//! Kraken fetch targets
use std::collections::HashMap;
use std::str::FromStr;
use std::iter::FromIterator;

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

#[derive(Debug, Clone)]
pub struct TranslatorTargets {
    trade_history_uri: HashMap<asset::Pair, Uri>,
}

impl TranslatorTargets {
    pub fn new(base: &str, asset_pairs: Vec<asset::Pair>) -> Self {

        let trade_history_insert = asset_pairs
            .clone()
            .into_iter()
            .map(|ap| {
                let target = format!(
                    "{}/trade_history/{}/{}/kraken", base, &ap.left(), &ap.right()
                );
                (ap, Uri::from_str(target.as_str()).unwrap())
            });

        TranslatorTargets {
            trade_history_uri: HashMap::from_iter(trade_history_insert),
        }
    }
    
    /// Return the PUT URI for the asset pair.
    pub fn trade_history_uri(&self, ap: &asset::Pair) -> Option<Uri> {
        self.trade_history_uri.get(&ap).map(|u| u.clone())
    }
}

pub fn targets(base: &str) -> TranslatorTargets {
    TranslatorTargets::new(base, vec![asset::BTC_USD, asset::ETH_USD])
}
