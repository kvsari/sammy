//! Models

use common::{exchange, asset};

/// Url query parameters for a request of a stream of ticks.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Getters)]
pub struct TicksRequest {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,

    /// Second timestamp from.
    from: u64,

    /// Second timestamp to. If absent, up to 'now' will be assumed.
    to: Option<u64>,

    /// Time size of the ticks in Seconds
    span: u32,
}

impl TicksRequest {
    pub fn _new(
        exchange: exchange::Exchange,
        asset_pair: asset::Pair,
        from: u64,
        to: Option<u64>,
        span: u32,
    ) -> Self {
        TicksRequest {
            exchange, asset_pair, from, to, span,
        }
    }
}

/*
/// Prepared parameters for preparing a request.
pub struct FolderTickRequestParams {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    from: u64,
    to: u64,
    
}
*/
