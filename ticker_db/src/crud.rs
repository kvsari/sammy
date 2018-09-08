//! Create Read Update Delete
use std::{convert, fmt, error};
use std::collections::HashMap;
use std::iter::FromIterator;

use diesel::{Connection, RunQueryDsl};
use diesel::result::{self, ConnectionError};
use diesel::pg::PgConnection;
use chrono::{NaiveDateTime};
use bigdecimal::{BigDecimal, ParseBigDecimalError};

use common::exchange::{Exchange, ParseExchangeError};
use common::asset;

use model::{FreshTick, Tick};

#[derive(Debug, Clone)]
struct InsertableTick {
    exchange: i32,
    asset_pair: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    first_price: BigDecimal,
    first_size: BigDecimal,
    highest_price: BigDecimal,
    highest_size: BigDecimal,
    lowest_price: BigDecimal,
    lowest_size: BigDecimal,
    last_price: BigDecimal,
    last_size: BigDecimal,
    count: i64,
}

fn freshtick_into_insertabletick(
    ft: &FreshTick, ex: &HashMap<Exchange, i32>, ap: &HashMap<asset::Pair, i32>
) -> Result<InsertableTick, TicksError> {
    let itick = InsertableTick {
        exchange: *ex.get(ft.exchange()).ok_or("Invalid exchange.")?,
        asset_pair: *ap.get(ft.asset_pair()).ok_or("Invalid asset pair.")?,
        start_time: ft.start_time().naive_utc(),
        end_time: ft.end_time().naive_utc(),
        first_price: ft.first_price().to_string().parse()?,
        first_size: ft.first_size().to_string().parse()?,
        highest_price: ft.highest_price().to_string().parse()?,
        highest_size: ft.highest_size().to_string().parse()?,
        lowest_price: ft.lowest_price().to_string().parse()?,
        lowest_size: ft.lowest_size().to_string().parse()?,
        last_price: ft.last_price().to_string().parse()?,
        last_size: ft.last_size().to_string().parse()?,
        count: *ft.count(),
    };

    Ok(itick)
}

#[derive(Debug, Clone)]
struct RawTick {
    id: i64,
    exchange: i32,
    asset_pair: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    first_price: BigDecimal,
    first_size: BigDecimal,
    highest_price: BigDecimal,
    highest_size: BigDecimal,
    lowest_price: BigDecimal,
    lowest_size: BigDecimal,
    last_price: BigDecimal,
    last_size: BigDecimal,
    count: i64,
}

#[derive(Queryable)]
struct DbExchange {
    id: i32,
    label: String,
}

#[derive(Queryable)]
struct DbAssetPair {
    id: i32,
    left_side: String,
    right_side: String,
    pair: String,
}

fn fetch_exchanges(conn: &PgConnection) -> Result<Vec<(Exchange, i32)>, TicksError> {
    use schema::exchanges::dsl::*;
    let db_exchanges: Vec<DbExchange> = exchanges.load(conn)?;

    let mut kp_exchanges: Vec<(Exchange, i32)> = Vec::new();
    
    for db_exchange in db_exchanges {
        let exchange: Exchange = db_exchange.label.parse()?;
        kp_exchanges.push((exchange, db_exchange.id));
    }

    Ok(kp_exchanges)
}

fn fetch_asset_pairs(conn: &PgConnection) -> Result<Vec<(asset::Pair, i32)>, TicksError> {
    use schema::asset_pairs::dsl::*;
    let db_aps: Vec<DbAssetPair> = asset_pairs.load(conn)?;

    let mut aps: Vec<(asset::Pair, i32)> = Vec::new();

    for db_ap in db_aps {
        let ap: asset::Pair = db_ap.pair.parse()?;
        aps.push((ap, db_ap.id));
    }

    Ok(aps)
}

pub struct Ticks {
    connection: PgConnection,
    ex_ids: HashMap<Exchange, i32>,
    ap_ids: HashMap<asset::Pair, i32>,
    ids_ex: HashMap<i32, Exchange>,
    ids_ap: HashMap<i32, asset::Pair>,
}

impl Ticks {
    pub fn new(db_url: &str) -> Result<Self, TicksError> {
        let connection = PgConnection::establish(db_url)?;

        // Fetch exchanges
        let ex_data = fetch_exchanges(&connection)?;
        let ex_data_iter = ex_data.clone().into_iter();        
        let ex_ids: HashMap<Exchange, i32> = HashMap::from_iter(ex_data_iter);
        let ex_rev: Vec<(i32, Exchange)> = ex_data
            .into_iter()
            .map(|(ex, id)| (id, ex))
            .collect();
        let ids_ex: HashMap<i32, Exchange> = HashMap::from_iter(ex_rev.into_iter());

        // Fetch asset pairs
        let ap_data = fetch_asset_pairs(&connection)?;
        let ap_data_iter = ap_data.clone().into_iter();
        let ap_ids: HashMap<asset::Pair, i32> = HashMap::from_iter(ap_data_iter);
        let ap_rev: Vec<(i32, asset::Pair)> = ap_data
            .into_iter()
            .map(|(ap, id)| (id, ap))
            .collect();
        let ids_ap: HashMap<i32, asset::Pair> = HashMap::from_iter(ap_rev.into_iter());
        
        Ok(Ticks {
            connection, ex_ids, ap_ids, ids_ex, ids_ap
        })
    }

    /*
    pub fn create(&self, ft: FreshTick) -> Result<Tick, Error> {
        
    }
    */
}

#[derive(Debug)]
pub enum TicksError {
    Connect(ConnectionError),
    Sql(result::Error),
    Exchange(ParseExchangeError),
    AssetPair(asset::ParseAssetError),
    Convert(String),
    Decimal(ParseBigDecimalError),
}

impl fmt::Display for TicksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TicksError::Connect(ref err) => write!(f, "Connect: {}", &err),
            TicksError::Sql(ref err) => write!(f, "SQL: {}", &err),
            TicksError::Exchange(ref err) => write!(f, "Exchange parse: {}", &err),
            TicksError::AssetPair(ref err) => write!(f, "Asset pair parse: {}", &err),
            TicksError::Convert(ref err) => {
                write!(f, "Can't convert before DB OP: {}", &err)
            },
            TicksError::Decimal(ref err) => write!(f, "Bad decimal: {}", &err),
        }
    }
}

impl error::Error for TicksError {
    fn description(&self) -> &str {
        "Error with Ticks DB CRUD."
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            TicksError::Connect(ref err) => Some(err),
            TicksError::Sql(ref err) => Some(err),
            TicksError::Exchange(ref err) => Some(err),
            TicksError::AssetPair(ref err) => Some(err),
            TicksError::Decimal(ref err) => Some(err),
            TicksError::Convert(_) => None,
            TicksError::Decimal(ref err) => Some(err),
        }
    }
}

impl convert::From<ConnectionError> for TicksError {
    fn from(ce: ConnectionError) -> Self {
        TicksError::Connect(ce)
    }
}

impl convert::From<result::Error> for TicksError {
    fn from(re: result::Error) -> Self {
        TicksError::Sql(re)
    }
}

impl convert::From<ParseExchangeError> for TicksError {
    fn from(p: ParseExchangeError) -> Self {
        TicksError::Exchange(p)
    }
}

impl convert::From<asset::ParseAssetError> for TicksError {
    fn from(p: asset::ParseAssetError) -> Self {
        TicksError::AssetPair(p)
    }
}

impl convert::From<String> for TicksError {
    fn from(s: String) -> Self {
        TicksError::Convert(s)
    }
}

impl<'a> convert::From<&'a str> for TicksError {
    fn from(s: &str) -> Self {
        TicksError::Convert(s.to_owned())
    }
}

impl convert::From<ParseBigDecimalError> for TicksError {
    fn from(p: ParseBigDecimalError) -> Self {
        TicksError::Decimal(p)
    }
}
