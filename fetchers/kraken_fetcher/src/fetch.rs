//! Fetching code
use std::time::Duration;

use futures::{Future, Stream};
use futures::future::FutureResult;
use serde_json;
use tokio_timer;

use common::{asset, trade};

use super::KrakenFetchTargets;
use super::{HttpsClient, FetchError};
use model::{Outer, TradeHistory};
use conversion::trade_history;

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

/// Return stream that polls the trade history. Only polls for a single asset pair. This
/// stream is expected to have combinators attached to it to drive it and deal with the
/// items yielded.
pub fn poll_trade_history2(
    client: HttpsClient,
    pair: asset::Pair,
    targets: KrakenFetchTargets,
) -> impl Stream<Item = Outer<TradeHistory>, Error = FetchError> {
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

            // Can have a receiver here to get the results from the previous request
            // and back off if there was a throttle warning. The sent future can be
            // wrapped in an option and filtered.
            
            client.get(uri)
        })
        .and_then(|res| {
            let status = res.status().as_u16();
            // TODO: Handle errors here.
            // Handle throttling errors here. Need to back off.
            //
            // Can use a channel to call back to the previous future and tell it to back off
            // for a while.
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
}

/// Takes in the fetch stream and deals with all benign errors only propagating the stream
/// killing errors that would need to be handled by an overarching process.
pub fn filter_benign_errors(
    input: impl Stream<Item = Outer<TradeHistory>, Error = FetchError>
) -> impl Stream<Item = TradeHistory, Error = ()> {
    // First we remove fetch errors depending on their severity.
    let trimmed = input
        .then(|result| {
            match result {
                Ok(history) => Ok(Some(history)),
                Err(fetch_err) => match fetch_err {
                    FetchError::Client(e) => {
                        error!("Client error: {}", &e);
                        Ok(None)
                    },
                    FetchError::SerdeJson(e) => {
                        error!("Deserialize error: {}", &e);
                        // This could be serious? Kraken API may have changed.
                        Ok(None)
                    },
                    FetchError::Status(e) => {
                        error!("HTTP Status: {}", &e);
                        // Likely we got throttled.
                        Ok(None)
                    },
                    FetchError::Utf8(e) => {
                        error!("Utf8: {}", &e);
                        // Possibly a transmission error. Perhaps we can continue.
                        Ok(None)
                    },
                },
            }
        })
        .filter_map(|item| item);

    // Then we unwrap the TradeHistory from the outer object removing any errors there.
    trimmed
        .and_then(|history| {
            let (error, result) = history.consume();
            if let Some(history) = result {
                Ok(history) // we send back the history inner.
            } else {
                // We have errors. Likely a bad command so this stream will never work.
                // Therefore should exit.
                error!("Stream broken. Kraken response error: {:?}", &error);
                Err(())
            }
        })
}

/// Takes a filtered fetch stream and converts it into the common format for placement into
/// other systems, likely the translator.
pub fn convert_into_common(
    input: impl Stream<Item = TradeHistory, Error = ()>
) -> impl Stream<Item = Vec<trade::TradeHistoryItem>, Error = ()> {
    input.and_then(|history| {
        trade_history(&history).map_err(|e| {
            error!("Failure to convert into common format: {}", &e);
            // TODO: Should we continue with the stream? For now we consider this error
            //       terminal (perhaps the kraken API has changed?) and we exit the stream.
        })
    })
}
