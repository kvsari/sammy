//! Create Read Update Delete executors using the `postgres` driver.
use std::collections::HashMap;

use postgres::{Connection, TlsMode};

use common::{asset, trade, exchange};

use model::{FreshTradeItem, TradeItem};
use error::Error;

macro_rules! fetch_data_types {
    ($name:ident, $data:ty, $sql:expr) => {
        fn $name(
            conn: &Connection
        ) -> Result<(HashMap<$data, i32>, HashMap<i32, $data>), Error> {
            let mut data_ids = HashMap::new();
            let mut ids_data = HashMap::new();

            let rows = conn.query($sql, &[])?;

            for row in rows.iter() {
                let id: i32 = row.get(0);
                let data: String = row.get(1);
                let data: $data = data.parse()?;
                data_ids.insert(data, id);
                ids_data.insert(id, data);
            }
            
            Ok((data_ids, ids_data))
        }
    }
}

fetch_data_types!(exchanges, exchange::Exchange, "SELECT id, label FROM exchanges");
fetch_data_types!(pairs, asset::Pair, "SELECT id, pair FROM asset_pairs");
fetch_data_types!(markets, trade::Market, "SELECT id, market FROM trade_markets");
fetch_data_types!(types, trade::Type, "SELECT id, trade FROM trade_types");

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
    /// Create a `Trades` executor by passing a live postgres connection.
    pub fn new(connection: Connection) -> Result<Self, Error> {
        let (ex_ids, ids_ex) = exchanges(&connection)?;
        let (ap_ids, ids_ap) = pairs(&connection)?;
        let (tm_ids, ids_tm) = markets(&connection)?;
        let (tt_ids, ids_tt) = types(&connection)?;

        Ok(Trades {
            connection,
            ex_ids,
            ap_ids,
            tm_ids,
            tt_ids,
            ids_ex,
            ids_ap,
            ids_tm,
            ids_tt,
        })
    }

    /// Create a `Trades` executor by connecting to the DB at `url`.
    pub fn connect(url: &str) -> Result<Self, Error> {
        let connection = Connection::connect(url, TlsMode::None)?;
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
        let transaction = self.connection.transaction()?;
        let create_stmt = transaction.prepare_cached(
            "INSERT INTO trade_history_items \
             ( exchange, asset_pair, happened, match_size, match_price, market, trade ) \
             VALUES ( $1, $2, $3, $4, $5, $6, $7 ) \
             RETURNING \
             ( id, exchange, asset_pair, happened, match_size, match_price, market, trade )"
        )?;

        let itis = ftis.iter()
            .try_fold(vec![], |mut itis, fti| -> Result<Vec<TradeItem>, Error> {
                let ex_id = self.ex_ids.get(fti.exchange()).ok_or("Invalid exchange")?;
                let ap_id = self.ap_ids.get(fti.asset_pair()).ok_or("Invalid asset pair")?;
                let tm_id = self.tm_ids.get(fti.market()).ok_or("Invalid market")?;
                let tt_id = self.tt_ids.get(fti.trade()).ok_or("Invalid trade")?;
                
                let rows = create_stmt.query(&[
                    ex_id, ap_id, fti.timestamp(), fti.size(), fti.price(), tm_id, tt_id
                ])?;

                // Only one row should be returned here.
                if rows.len() != 1 {
                    return Err(Error::InvalidRows(
                        "Single trade item insert should only return one row.".to_owned(),
                    ));
                }

                let row = rows.get(0);

                let iti = TradeItem::new(
                    row.get(0),
                    *self.ids_ex.get(&row.get(1)).ok_or("Exchange in DB not in index.")?,
                    *self.ids_ap.get(&row.get(2)).ok_or("Asset Pair in DB not in index.")?,
                    row.get(3),
                    row.get(4),
                    row.get(5),
                    *self.ids_tm.get(&row.get(6)).ok_or("Market in DB not in index.")?,
                    *self.ids_tt.get(&row.get(7)).ok_or("Trade type in DB not in index.")?,
                );
                
                itis.push(iti);
                Ok(itis)
            })?;

        transaction.finish()?;
        
        Ok(itis)
    }
}
