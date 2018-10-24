//! Kraken fetch targets
use hyper::Uri;

use common::asset;

/// Fetch targets. Unit struct for now with hardcoded values. We're just trying to explore
/// an API. In the future we can instantiate and load in the base paths and other stuff.
#[derive(Debug, Clone)]
pub struct KrakenFetchTargets;

impl KrakenFetchTargets {

    /// Return the URI for the asset pair.
    pub fn trade_history(&self, ap: asset::Pair, since: Option<u64>) -> Option<Uri> {
        let base = "https://api.kraken.com/0/public/Trades";
        let pair = match ap {
            asset::BTC_USD => "?pair=XBTUSD",
            asset::ETH_USD => "?pair=ETHUSD",
            asset::ETH_BTC => "?pair=ETHXBT",
            _ => return None,
        };

        let uri = if let Some(since) = since {
            format!("{}{}&since={}", base, pair, &since)
        } else {
            format!("{}{}", base, pair)
        };

        // This part shouldn't fail as we're controlling URI construction.
        Some(uri.parse().expect("Invalid URI constructed. This shouldn't happen. Fix me."))
    }
}
