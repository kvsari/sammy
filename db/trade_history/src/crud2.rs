//! Create Read Update Delete executors using the `postgres` driver.
use std::collections::HashMap;

use postgres::Connection;

use common::{asset, trade, exchange};

use error::Error;

fn exchanges(
    conn: &Connection
) -> Result<(HashMap<exchange::Exchange, i32>, HashMap<i32, exchange::Exchange>), Error> {
    let mut ex_ids = HashMap::new();
    let mut ids_ex = HashMap::new();

    let rows = conn.query("SELECT id, label FROM exchanges", &[])?;

    for row in rows.iter() {
        let id: i32 = row.get(0);
        let exchange: String = row.get(1);
        let exchange: exchange::Exchange = exchange.parse()?;
        ex_ids.insert(exchange, id);
        ids_ex.insert(id, exchange);
    }

    Ok((ex_ids, ids_ex))
}

fn pairs(
    conn: &Connection
) -> Result<(HashMap<asset::Pair, i32>, HashMap<i32, asset::Pair>), Error> {
    let mut ap_ids = HashMap::new();
    let mut ids_ap = HashMap::new();

    let rows = conn.query("SELECT id, pair FROM asset_pairs", &[])?;

    for row in rows.iter() {
        let id: i32 = row.get(0);
        let pair: String = row.get(1);
        let pair: asset::Pair = pair.parse()?;
        ap_ids.insert(pair, id);
        ids_ap.insert(id, pair);
    }

    Ok((ap_ids, ids_ap))
}

fn markets(
    conn: &Connection        
) -> Result<(HashMap<trade::Market, i32>, HashMap<i32, trade::Market>), Error> {
    let mut tm_ids = HashMap::new();
    let mut ids_tm = HashMap::new();

    Ok((tm_ids, ids_tm))
}

fn types(
    conn: &Connection
) -> Result<(HashMap<trade::Type, i32>, HashMap<i32, trade::Type>), Error> {
    let mut tt_ids = HashMap::new();
    let mut ids_tt = HashMap::new();

    Ok((tt_ids, ids_tt))
}

pub struct Trades {
    connection: Connection,
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
    pub fn new(connection: Connection) -> Result<Self, Error> {
        let (ex_ids, ids_ex) = exchanges(&connection)?;
        let (ap_ids, ids_ap) = pairs(&connection)?;
        let (tm_ids, ids_tm) = markets(&connection)?;
        let (tt_ids, ids_tt) = types(&connection)?;

        Ok(Trades {
            connection, ex_ids, ap_ids, tm_ids, tt_ids, ids_ex, ids_ap, ids_tm, ids_tt,
        })
    }
}
