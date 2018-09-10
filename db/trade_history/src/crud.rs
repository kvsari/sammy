//! Create Read Update Delete executors
use std::collections::HashMap;

use common::{asset, exchange, trade};

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

fn fetch_exchanges(conn: &PgConnection) -> Result<Vec<(Exchange, i32)>, Error> {
    use schema::exchanges::dsl::*;
    let db_exchanges: Vec<DbExchange> = exchanges.load(conn)?;

    let mut kp_exchanges: Vec<(Exchange, i32)> = Vec::new();
    
    for db_exchange in db_exchanges {
        let exchange: Exchange = db_exchange.label.parse()?;
        kp_exchanges.push((exchange, db_exchange.id));
    }

    Ok(kp_exchanges)
}

fn fetch_asset_pairs(conn: &PgConnection) -> Result<Vec<(asset::Pair, i32)>, Error> {
    use schema::asset_pairs::dsl::*;
    let db_aps: Vec<DbAssetPair> = asset_pairs.load(conn)?;

    let mut aps: Vec<(asset::Pair, i32)> = Vec::new();

    for db_ap in db_aps {
        let ap: asset::Pair = db_ap.pair.parse()?;
        aps.push((ap, db_ap.id));
    }

    Ok(aps)
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

