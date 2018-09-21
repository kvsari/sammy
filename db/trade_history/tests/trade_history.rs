//! Trade history inserts

extern crate chrono;

extern crate trade_history;
extern crate common;

use chrono::Utc;

use common::{exchange, asset, trade};
use trade_history::{model, crud};

static DATABASE_URL: &str = "postgres://stephan@localhost/sammy_trade_history";

#[test]
fn insert_fresh_trade_item() {
    let trades = crud::Trades::connect(DATABASE_URL).expect("Can't connect to DB.");

    let fti_1 = model::FreshTradeItem::new(
        exchange::Exchange::Kraken,
        asset::BTC_USD,
        Utc::now(),
        10.into(),
        100.into(),
        trade::Market::Maker,
        trade::Type::Limit,
    );

    let ftis = vec![fti_1];

    let tis = trades.create(&ftis).expect("DB error.");
}
