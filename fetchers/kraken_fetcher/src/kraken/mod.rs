//! Code for kraken
use std::time::Duration;

use futures::{Future, Stream};
use hyper::{Client, Body, Uri};
use hyper_tls::HttpsConnector;
use serde_json;
use tokio_timer;
use tokio;

use asset;
use super::{HttpsClient, FetchError};

mod model;

use self::model::{Outer, TradeHistory};

/// Fetch targets. Unit struct for now with hardcoded values. We're just trying to explore
/// an API. In the future we can instantiate and load in the base paths and other stuff.
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

/// Return future that polls the trade history. Only polls for a single asset pair.
///
/// TODO: Pass in a channel for forwarding items onwards to other parts.
pub fn poll_trade_history(
    client: HttpsClient,
    pair: asset::Pair,
    targets: KrakenFetchTargets,
) -> impl Future<Item = (), Error = ()> {
    // Start the interval loop at polling per 15 seconds.
    let frequency = tokio_timer::Interval::new_interval(Duration::from_secs(15));

    // Setup the since var
    let mut since: Option<u64> = None;
    
    // Feed in the client and send out a poll at every iteration.
    frequency
        .map_err(|e| error!("Couldn't setup timer: {}", &e))
        .then(move |_| {
            let uri = targets.trade_history(pair, since)
                .expect("Invalid asset pair. TODO: Return error here.");
            
            client.get(uri)
        })
        .and_then(|res| {
            let status = res.status().as_u16();
            // TODO: Handle errors here.
            // Handle throttling errors here. Need to back off.
            /*
            match status {
                200 => res.into_body().concat2(),
                _ => Err(status),
            }
             */
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            //let body = String::from_utf8(body.to_vec())?;
            //Ok(body)
            let history: Outer<TradeHistory> = serde_json::from_slice(&body)?;
            Ok(history)
        })
        .then(|result| {
            match result {
                Ok(body) => println!("Body: {:?}", &body),
                Err(e) => println!("Error: {}", &e),
            }
            Ok(())
        })
        .for_each(|_| Ok(()))
}
