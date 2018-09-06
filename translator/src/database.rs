//! Database actor.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use common::{asset, exchange};

#[derive(Debug, Copy, Clone)]
pub struct FreshTick {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    first_price: Decimal,
    first_size: Decimal,
    highest_price: Decimal,
    highest_size: Decimal,
    lowest_price: Decimal,
    lowest_size: Decimal,
    last_price: Decimal,
    last_size: Decimal,
    count: i64,
}

impl FreshTick {
    pub fn new(
        exchange: exchange::Exchange,
        asset_pair: asset::Pair,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        first_price: Decimal,
        first_size: Decimal,
        highest_price: Decimal,
        highest_size: Decimal,
        lowest_price: Decimal,
        lowest_size: Decimal,
        last_price: Decimal,
        last_size: Decimal,
        count: i64,
    ) -> Self {
        FreshTick {
            exchange,
            asset_pair,
            start_time,
            end_time,
            first_price,
            first_size,
            highest_price,
            highest_size,
            lowest_price,
            lowest_size,
            last_price,
            last_size,
            count,
        }
    }
}
