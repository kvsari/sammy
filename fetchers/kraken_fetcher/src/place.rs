//! Place stream items onto a RESTful api.
use futures::{Future, Stream};
use hyper::{Uri, Request};
use serde_json;

use common::trade;

use https_client::HttpsClient;

/// Receives a stream of common trade history items. Places them using the provided client.
/// Placement stream yields, the items input that have been successfully placed. Otherwise
/// yields an error.
pub fn put_trade_history(
    client: HttpsClient,
    target: Uri,
    history_stream: impl Stream<Item = Vec<trade::TradeHistoryItem>, Error = ()>
) -> impl Stream<Item = (), Error = ()> {
    history_stream
        .and_then(move |items| {           
            let req = Request::put(target.clone())
                .body(serde_json::to_string(&items).unwrap().into())
                .unwrap();
            client.request(req).map_err(|_| ())
        })
        .then(|result| match result {            
            Ok(rsp) => {
                trace!("Placement success: {}", &rsp.status());
                Ok(())
            },
            Err(()) => {
                trace!("Placement failure");
                Err(())
            },
        })
}
