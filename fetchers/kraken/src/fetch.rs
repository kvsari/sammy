//! Fetching code
use std::time::Duration;

use futures::{Future, Stream};
use futures::sync::mpsc;
use futures::future::{lazy, result, FutureResult};
use serde_json;
use tokio_timer;
use tokio;

use fetch_lib::https_client::{HttpsClient, FetchError};
use common::{asset, trade};

use super::KrakenFetchTargets;
use model::{Outer, TradeHistory};
use conversion::trade_history;

/// Return stream that polls the trade history. Only polls for a single asset pair. This
/// stream is expected to have combinators attached to it to drive it and deal with the
/// items yielded.
pub fn poll_trade_history(
    client: HttpsClient,
    pair: asset::Pair,
    targets: KrakenFetchTargets,
    poll_delay: Duration,
) -> impl Stream<Item = Outer<TradeHistory>, Error = FetchError> {
    // Start the interval loop
    let frequency = tokio_timer::Interval::new_interval(poll_delay);

    // Setup the since var.
    // TODO: Make mutable. We may need to track last fetch if we are fetching many
    //       different asset pairs which would lengthen the delay between any update.
    let since: Option<u64> = None;
    
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
            let _status = res.status().as_u16();
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
            let history: Outer<TradeHistory> = serde_json::from_slice(&body)?;
            Ok(history)
        })
}

/// Spawn multiple fetchers, one for each asset pair. Combine their outputs using a channel
/// and return the entire thing packaged into a stream.
///
/// ## Todo
/// 1. Try re-writing this function to use `select` instead of a channel to combine the
///    fetch streams together. This aught to remove the risk of a `SendError`.
pub fn poll_trade_histories(
    client: HttpsClient,
    pairs: Vec<asset::Pair>,
    targets: KrakenFetchTargets,
    poll_delay: Duration,
) -> impl Stream<Item = Outer<TradeHistory>, Error = FetchError> {
    let (tx, rx) = mpsc::unbounded();
    
    lazy(move || -> FutureResult<
        Box<Stream<Item = Outer<TradeHistory>, Error = FetchError> + Send>, ()
            > {
        pairs.into_iter()
            .for_each(move |pair| {
                let sender = tx.clone();
                let fut = poll_trade_history(
                    client.clone(), pair, targets.clone(), poll_delay,
                ).then(move |emission| {
                    sender.unbounded_send(emission)
                        .expect(
                            "This should never happen. TODO: Refactor me to use select."
                        );
                    Ok(())
                }).for_each(|()| Ok(()));
                tokio::spawn(fut);
            });        

        let recv = rx
            .map_err(|()| FetchError::InternalChannel)
            .and_then(|emission| emission);

        result::<_, _>(Ok(Box::new(recv)))
        
    })
        .map_err(|()| FetchError::InternalChannel)
        .flatten_stream()
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
                    FetchError::InternalChannel => {
                        error!("Internal channel failure");
                        // This error should never happen. Unbounded channels were being
                        // used to combine the fetch streams into one and somehow they
                        // failed. It shouldn't ever happen as the fetches occur based on
                        // a timer and thus a failure to fetch should mean that it'll be
                        // tried again on the next timeout. Going to return Ok(None) here
                        // but it might make more sense to just error the stream, exit and
                        // then rely on orchestration to restart the fetcher.
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
                Ok(Some(history)) // we send back the history inner.
            } else {
                // We have errors. It could be a bad command or the service is down
                // temporarily and will be back up. Need to analyze the error returned
                // here. For now we'll just try again.
                error!("Stream broken. Kraken response error: {:?}", &error);

                // Errors that have happened here are;
                // 1. ["EService:Unavailable"]

                // We continue anyway...
                Ok(None)
            }
        })
        .filter_map(|item| item) // another layer of filtering
}

/// Takes a filtered fetch stream and converts it into the common format for placement into
/// other systems, likely the translator.
pub fn convert_into_common(
    input: impl Stream<Item = TradeHistory, Error = ()>
) -> impl Stream<Item = (asset::Pair, Vec<trade::TradeHistoryItem>), Error = ()> {
    input.and_then(|history| {
        trade_history(&history).map_err(|e| {
            error!("Failure to convert into common format: {}", &e);
            // TODO: Should we continue with the stream? For now we consider this error
            //       terminal (perhaps the kraken API has changed?) and we exit the stream.
        })
    })
}
