//! Data model for display to the client

use chrono::{DateTime, Utc};

use common::{asset, exchange};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FoldOperation {
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
