//! DB models and their conversions

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use common::{exchange, asset, trade};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Getters)]
pub struct FreshTradeItem {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    timestamp: DateTime<Utc>,
    size: Decimal,
    price: Decimal,
    market: trade::Market,
    trade: trade::Type,

    match_id: Option<i64>,
    buy_order_id: Option<i64>,
    sell_order_id: Option<i64>,
    match_timestamp: Option<DateTime<Utc>>,
}

impl FreshTradeItem {
    pub fn new(
        exchange: exchange::Exchange,
        asset_pair: asset::Pair,
        timestamp: DateTime<Utc>,
        size: Decimal,
        price: Decimal,
        market: trade::Market,
        trade: trade::Type,
        match_id: Option<i64>,
        buy_order_id: Option<i64>,
        sell_order_id: Option<i64>,
        match_timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        FreshTradeItem {
            exchange,
            asset_pair,
            timestamp,
            size,
            price,
            market,
            trade,
            match_id,
            buy_order_id,
            sell_order_id,
            match_timestamp,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Getters)]
pub struct TradeItem {
    id: i64,
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    timestamp: DateTime<Utc>,
    size: Decimal,
    price: Decimal,
    market: trade::Market,
    trade: trade::Type,
}

impl TradeItem {
    pub fn new(
        id: i64,
        exchange: exchange::Exchange,
        asset_pair: asset::Pair,
        timestamp: DateTime<Utc>,
        size: Decimal,
        price: Decimal,
        market: trade::Market,
        trade: trade::Type,
    ) -> Self {
        TradeItem {
            id, exchange, asset_pair, timestamp, size, price, market, trade
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Getters)]
pub struct TradeSetSummary {
    first: DateTime<Utc>,
    last: DateTime<Utc>,
    count: i64,
}

impl TradeSetSummary {
    pub fn new(first: DateTime<Utc>, last: DateTime<Utc>, count: i64) -> Self {
        TradeSetSummary {
            first, last, count
        }
    }
}
