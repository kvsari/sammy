//! Place stream items onto a RESTful api.
use model::{Outer, TradeHistory};
use https_client::{FetchError, HttpsClient};

pub fn trade_history_onto_translator_rest_api(
    client: HttpsClient,
    stream: impl Stream<Item = Outer<TradeHistory>, Error = FetchError>,
) -> impl Future<Item = (), Error = ()> {
    // First we remove fetch errors depending on their severity.
    let trimmed = stream
        .and_then(|result| {
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
                },
            }
        })
        .filter_map(|item| item);
}
