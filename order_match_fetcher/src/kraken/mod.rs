//! Code for kraken
use std::time::Duration;

use futures::{Future, Stream};
use hyper::{Client, Body, Uri};
use hyper_tls::HttpsConnector;
use tokio_timer;
use tokio;

use asset;
use super::{HttpsClient, FetchError};

pub fn test_fire1() {
    let https = HttpsConnector::new(2).expect("TLS init failed.");
    let client = Client::builder()
        .build::<_, Body>(https);

    let uri = "https://api.kraken.com/0/public/AssetPairs".parse().unwrap();

    let future = client
        .get(uri)
        .map(|res| {
            println!("Response: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });

    tokio::run(future)
}

pub fn test_fire2(client: HttpsClient) -> impl Future<Item = String, Error = FetchError> {
    let uri = "https://api.kraken.com/0/public/AssetPairs".parse().unwrap();

    client
        .get(uri)
        .and_then(|res| {
            println!("Response: {}", res.status());
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let body = String::from_utf8(body.to_vec())?;
            Ok(body)
        })
        .from_err()
}

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
            // TODO: Handle HTTP error codes here. Right now we're assuming happy path.

            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let body = String::from_utf8(body.to_vec())?;
            Ok(body)
        })
        .then(|result| {
            match result {
                Ok(body) => println!("Body: {}", &body),
                Err(e) => println!("Error: {}", &e),
            }
            Ok(())
        })
        .for_each(|_| Ok(()))
}
