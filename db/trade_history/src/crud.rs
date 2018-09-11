//! Create Read Update Delete executors
use std::collections::HashMap;
use std::iter::FromIterator;

use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::{self, Connection, PgConnection, RunQueryDsl};
use bigdecimal::BigDecimal;

use common::{asset, exchange, trade};
use common::trade::Type as TradeType; // To avoid a `diesel` name collision.

use model::{FreshTradeItem, TradeItem};
use schema::trade_history_items;
use error::Error;

#[derive(Queryable)]
struct DbExchange {
    id: i32,
    label: String,
}

#[derive(Queryable)]
struct DbAssetPair {
    id: i32,
    _left_side: String,
    _right_side: String,
    pair: String,
}

#[derive(Queryable)]
struct DbTradeMarkets {
    id: i32,
    market: String,
}

#[derive(Queryable)]
struct DbTradeTypes {
    id: i32,
    trade: String,
}

/// Macro to abstract repetitive code.
///
/// TODO: This macro should be split into two. This will make the second half unit testable.
macro_rules! fetch_data_types {
    ($name:ident, $data:ty, $db_cont:ty, $db_field:ident, $db_table:ident) => {
        fn $name(
            conn: &PgConnection
        ) -> Result<(HashMap<$data, i32>, HashMap<i32, $data>), Error> {
            // Get the raw data.
            let data = {
                use schema::$db_table::dsl::*;
                let from_db: Vec<$db_cont> = $db_table.load(conn)?;

                let mut vector: Vec<($data, i32)> = Vec::new();

                for item in from_db {
                    let converted: $data = item.$db_field.parse()?;
                    vector.push((converted, item.id));
                }

                vector
            };

            // Convert into <Val, ID> hashmap
            let iter = data.clone().into_iter();
            let val_id: HashMap<$data, i32> = HashMap::from_iter(iter);

            // Convert into <ID, Val> hashmap
            let iter = data.into_iter();
            let reversed: Vec<(i32, $data)> = iter
                .map(|(a, b)| (b, a))
                .collect();
            let id_val: HashMap<i32, $data> = HashMap::from_iter(reversed.into_iter());

            // Return double index
            Ok((val_id, id_val))
        }
    };
}

fetch_data_types!(fetch_exchanges, exchange::Exchange, DbExchange, label, exchanges);
fetch_data_types!(fetch_asset_pairs, asset::Pair, DbAssetPair, pair, asset_pairs);
fetch_data_types!(fetch_markets, trade::Market, DbTradeMarkets, market, trade_markets);
fetch_data_types!(fetch_trade_types, TradeType, DbTradeTypes, trade, trade_types);

#[derive(Insertable)]
#[table_name = "trade_history_items"]
struct InsertableTradeItem {
    exchange: i32,
    asset_pair: i32,
    happened: NaiveDateTime,
    match_size: BigDecimal,
    match_price: BigDecimal,
    market: i32,
    trade: i32,
}

fn fresh_trade_item_into_insertable_trade_item(
    fti: &FreshTradeItem,
    ex_ids: &HashMap<exchange::Exchange, i32>,
    ap_ids: &HashMap<asset::Pair, i32>,
    tm_ids: &HashMap<trade::Market, i32>,
    tt_ids: &HashMap<trade::Type, i32>,
) -> Result<InsertableTradeItem, Error> {
    let iti = InsertableTradeItem {
        exchange: *ex_ids.get(fti.exchange()).ok_or("Invalid exchange.")?,
        asset_pair: *ap_ids.get(fti.asset_pair()).ok_or("Invalid asset pair.")?,
        happened: fti.timestamp().naive_utc(),
        match_size: fti.size().to_string().parse()?,
        match_price: fti.price().to_string().parse()?,
        market: *tm_ids.get(fti.market()).ok_or("Invalid market.")?,
        trade: *tt_ids.get(fti.trade()).ok_or("Invalid trade.")?,
    };

    Ok(iti)
}

#[derive(Queryable)]
struct RawTradeItem {
    id: i64,
    exchange: i32,
    asset_pair: i32,
    happened: NaiveDateTime,
    match_size: BigDecimal,
    match_price: BigDecimal,
    market: i32,
    trade: i32,
}

fn raw_trade_item_into_trade_item(
    rti: &RawTradeItem,
    ids_ex: &HashMap<i32, exchange::Exchange>,
    ids_ap: &HashMap<i32, asset::Pair>,
    ids_tm: &HashMap<i32, trade::Market>,
    ids_tt: &HashMap<i32, trade::Type>,
) -> Result<TradeItem, Error> {
    let ti = TradeItem::new(
        rti.id,
        *ids_ex.get(&rti.exchange).ok_or("Exchange in DB not present in index.")?,
        *ids_ap.get(&rti.asset_pair).ok_or("Asset Pair in DB not present in index.")?,
        DateTime::from_utc(rti.happened, Utc),
        rti.match_size.to_string().parse()?,
        rti.match_price.to_string().parse()?,
        *ids_tm.get(&rti.market).ok_or("Market in DB not present in index.")?,
        *ids_tt.get(&rti.trade).ok_or("Trade Type in DB not present in index.")?,
    );

    Ok(ti)
}

pub struct Trades {
    connection: PgConnection,
    ex_ids: HashMap<exchange::Exchange, i32>,
    ap_ids: HashMap<asset::Pair, i32>,
    tm_ids: HashMap<trade::Market, i32>,
    tt_ids: HashMap<trade::Type, i32>,
    ids_ex: HashMap<i32, exchange::Exchange>,
    ids_ap: HashMap<i32, asset::Pair>,
    ids_tm: HashMap<i32, trade::Market>,
    ids_tt: HashMap<i32, trade::Type>,
}

impl Trades {
    /// Create a `Trades` executor by passing a live postgres connection.
    pub fn new(connection: PgConnection) -> Result<Self, Error> {
        let (ex_ids, ids_ex) = fetch_exchanges(&connection)?;
        let (ap_ids, ids_ap) = fetch_asset_pairs(&connection)?;
        let (tm_ids, ids_tm) = fetch_markets(&connection)?;
        let (tt_ids, ids_tt) = fetch_trade_types(&connection)?;

        Ok(Trades {
            connection, ex_ids, ap_ids, tm_ids, tt_ids, ids_ex, ids_ap, ids_tm, ids_tt,
        })
    }

    /// Create a `Trades` executor by connecting to the DB at `url`.
    pub fn connect(url: &str) -> Result<Self, Error> {
        let connection = PgConnection::establish(url)?;
        Trades::new(connection)
    }

    /// Insert into the DB. The 'Create' part of CRUD. This one will check every
    /// item for it's exchange and asset pair.
    ///
    /// TODO:
    /// Create another method that let's you set the exchange and asset pair in
    /// advance to save a little time if this information is known in advance for large
    /// batch inserts.
    pub fn create(&self, ftis: &[FreshTradeItem]) -> Result<Vec<TradeItem>, Error> {
        // convert
        let itis: Vec<InsertableTradeItem> = ftis
            .iter()
            .try_fold(vec![], |mut itis, fti| -> Result<Vec<InsertableTradeItem>, Error> {
                let iti = fresh_trade_item_into_insertable_trade_item(
                    fti, &self.ex_ids, &self.ap_ids, &self.tm_ids, &self.tt_ids,
                )?;
                itis.push(iti);
                Ok(itis)
            })?;

        diesel::insert_into(trade_history_items::table)
            .values(&itis)
            .get_results(&self.connection)
            .map_err(|e| e.into())
            .and_then(|rtis| {
                rtis.iter()
                    .try_fold(vec![], |mut tis, rti| -> Result<Vec<TradeItem>, Error> {
                        let ti = raw_trade_item_into_trade_item(
                            rti, &self.ids_ex, &self.ids_ap, &self.ids_tm, &self.ids_tt,
                        )?;
                        tis.push(ti);
                        Ok(tis)
                    })
            })
    }
}
