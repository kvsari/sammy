//! Trade history inserts

extern crate chrono;

extern crate trade_history;
extern crate common;

use chrono::{Utc, DateTime, TimeZone};

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

    let _tis = trades.create(&ftis).expect("DB error.");
}

fn gen_trade_item(ts: DateTime<Utc>) -> model::FreshTradeItem {
    model::FreshTradeItem::new(
        exchange::Exchange::Kraken,
        asset::BTC_USD,
        ts,
        10.into(),
        100.into(),
        trade::Market::Maker,
        trade::Type::Limit,
    )
}

#[test]
fn search_date_range() {
    let trades = crud::Trades::connect(DATABASE_URL).expect("Can't connect to DB.");

    let ts1 = Utc.ymd(2014, 1, 1).and_hms(1, 1, 1);
    let ts2 = Utc.ymd(2014, 1, 1).and_hms(1, 1, 2);
    let ts3 = Utc.ymd(2014, 1, 1).and_hms(1, 1, 3);
    let ts4 = Utc.ymd(2014, 1, 1).and_hms(1, 1, 4);
    let ts5 = Utc.ymd(2014, 1, 1).and_hms(1, 1, 5);

    let ftis = vec![
        gen_trade_item(ts1),
        gen_trade_item(ts2),
        gen_trade_item(ts3),
        gen_trade_item(ts4),
        gen_trade_item(ts5),
    ];

    let _tis = trades.create(&ftis).expect("DB error.");

    let fetched = trades.read_between(
        exchange::Exchange::Kraken, asset::BTC_USD, ts2, ts5,
    ).expect("DB read error.");
        

    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert!(fetched.len() == 3);
    assert!(*fetched[0].timestamp() == ts2);
    assert!(*fetched[1].timestamp() == ts3);
    assert!(*fetched[2].timestamp() == ts4);
}
