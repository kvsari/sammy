//! DB models and their conversions

use chrono::{DateTime, Utc, NaiveDateTime};
use rust_decimal::Decimal;

use common::{exchange, asset, trade};

pub struct FreshTradeItem {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    timestamp: DateTime<Utc>,
    size: Decimal,
    price: Decimal,
    market: trade::Market,
    trade: trade::Type,
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
    ) -> Self {
        FreshTradeItem {
            exchange, asset_pair, timestamp, size, price, market, trade
        }
    }
}

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
