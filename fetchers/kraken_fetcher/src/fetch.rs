//! Fetching code
use std::time::Duration;

use futures::{Future, Stream};
use serde_json;
use tokio_timer;

use common::asset;

use super::KrakenFetchTargets;
use super::{HttpsClient, FetchError};
use model::{Outer, TradeHistory};

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
