//! Filter trade history actors.

use common::{asset, trade};

mod kraken;
mod binance;

pub use self::kraken::KrakenTradeHistory;
pub use self::binance::BinanceTradeHistory;

/// Common message for new data input for all filter actors.
#[derive(Message)]
pub struct UnfilteredTradeHistory {
    asset_pair: asset::Pair,
    history: Vec<trade::TradeHistoryItem>,
}

impl UnfilteredTradeHistory {
    pub fn new(asset_pair: asset::Pair, history: Vec<trade::TradeHistoryItem>) -> Self {
        UnfilteredTradeHistory {
            asset_pair,
            history,
        }
    }
}
