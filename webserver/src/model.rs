//! Models

use common::{exchange, asset};

/// Url query parameters for a request of a stream of ticks.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TicksRequest {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,

    /// Millisecond timestamp from.
    from: u64,

    /// Millisond timestamp to. If absent, up to 'now' will be assumed.
    to: Option<u64>,

    /// Time size of the ticks in milliseconds
    span: u64,
}
