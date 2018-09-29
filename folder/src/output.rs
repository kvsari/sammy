//! Data model for display to the client
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use common::{asset, exchange};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FoldOperation {
    #[serde(rename = "tick")]
    Tick,
}

/// Intermediate object that gives a summary over a trade history data set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistorySummary {
    assetpairs: Vec<asset::Pair>,
    exchanges: Vec<exchange::Exchange>,
    count: u64,
    earliest: DateTime<Utc>,
    latest: DateTime<Utc>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<Vec<FoldOperation>>,
}

impl TradeHistorySummary {
    pub fn new(
        assetpairs: Vec<asset::Pair>,
        exchanges: Vec<exchange::Exchange>,
        count: u64,
        earliest: DateTime<Utc>,
        latest: DateTime<Utc>,
        operations: Option<Vec<FoldOperation>>,
    ) -> Self {
        TradeHistorySummary {
            assetpairs, exchanges, count, earliest, latest, operations,
        }
    }

    /// TODO: Ignore duplicates
    pub fn add_operation(&mut self, operation: FoldOperation) {
        if let Some(operations) = self.operations.as_mut() {
            operations.push(operation);
        }

        if self.operations.is_none() {
            self.operations = Some(vec![operation]);
        }
    }
}

/// A single tick. The time from/to, asset_pair, exchange(s) are not present and to be
/// determined via the calling context.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Tick {
    first: Decimal,
    high: Decimal,
    low: Decimal,
    last: Decimal,
    count: u64,
}

impl Tick {
    pub fn new(
        first: Decimal,
        high: Decimal,
        low: Decimal,
        last: Decimal,
        count: u64
    ) -> Self {
        Tick {
            first, high, low, last, count,
        }
    }

    /// Struct needs to be initialized with the count set to 0 for this method to work
    /// properly as it uses the count to determine the start.
    pub fn folding_add(&mut self, number: Decimal) {
        if self.count > 0 {
            if number > self.high { self.high = number; }
            if number < self.low { self.low = number; }
            self.last = number;
        } else {
            self.first = number;
            self.high = number;
            self.low = number;
            self.last = number;
        }

        self.count += 1;
    }
}
