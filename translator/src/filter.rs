//! Filter trade history actors.
use std::collections::HashMap;

use chrono::{DateTime, Utc};

use common::asset;

/// Filter optimized for kraken trade history. This will ensure that only new items are
/// forwarded on through the system.
pub struct KrakenTradeHistory {
    timestamp_marker: HashMap<asset::Pair, DateTime<Utc>>,
}

