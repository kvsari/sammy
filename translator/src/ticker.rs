//! Actor for building the ticker from the trade history items.
use std::collections::HashMap;
use std::default::Default;

use rust_decimal::Decimal;

use common::{asset, exchange};

struct PriceSize(Decimal, Decimal);

/// Maintain a running tally of trade history data as it comes in.
struct Collection {
    first: PriceSize,
    highest: PriceSize,
    lowest: PriceSize,
    last: PriceSize,
    count: u64,
}

/*
impl Default for Collection {
    fn default() -> Self {
        Collection {
        }
    }
}
*/

/// Collates incoming trade history into the right collections (sorted by asset pair and
/// exchange) and then processes a calculation based on a time span that contains basic
/// candle information and some other stuff.
pub struct TickGenerator {
    kraken: HashMap<asset::Pair, Collection>,
}

impl TickGenerator {
    pub fn new() -> Self {
        TickGenerator {
            kraken: HashMap::new(),
        }
    }
}

