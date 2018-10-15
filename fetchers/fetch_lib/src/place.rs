//! Place stream items onto a RESTful api.
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

use futures::{Future, Stream};
use hyper::{Uri, Request};
use serde_json;

use common::{trade, exchange, asset};

use https_client::HttpsClient;

/// Contains placement URI's for the putting information on the `collector` API. This struct
/// is for a single `Exchange`. Allows 
#[derive(Debug, Clone)]
pub struct Target {
    exchange: exchange::Exchange,
    trade_history_uri: HashMap<asset::Pair, Uri>,
}

impl Target {
    pub fn new(
        base: &str, exchange: exchange::Exchange, asset_pairs: Vec<asset::Pair>
    ) -> Self {
        let trade_history_insert = asset_pairs
            .clone()
            .into_iter()
            .map(|ap| {
                let target = format!(
                    "{}/trade_history/{}/{}/{}", base, &ap.left(), &ap.right(), &exchange
                );
                (ap, Uri::from_str(target.as_str()).unwrap())
            });

        Target {
            exchange: exchange,
            trade_history_uri: HashMap::from_iter(trade_history_insert),
        }
    }

     /// Return the PUT URI for the asset pair.
    pub fn trade_history_uri(&self, ap: &asset::Pair) -> Option<Uri> {
        self.trade_history_uri.get(&ap).map(|u| u.clone())
    }
}

/// Receives a stream of common trade history items. Places them using the provided client.
/// Placement stream yields, the items input that have been successfully placed. Otherwise
/// yields an error.
pub fn put_trade_history(
    client: HttpsClient,
    target: Target,
    stream: impl Stream<Item = (asset::Pair, Vec<trade::TradeHistoryItem>), Error = ()>
) -> impl Future<Item = (), Error = ()> {    
    stream
        .and_then(move |(asset_pair, items)| {
            let req = Request::put(
                target.trade_history_uri(&asset_pair).expect("Missing asset pair!")
            )
                .body(serde_json::to_string(&items).unwrap().into())
                .unwrap();
            client.request(req).map_err(|e| error!("Failed to place history: {}", &e))
        })
        .then(|result| match result {            
            Ok(rsp) => {
                trace!("Placement success: {}", &rsp.status());
                Ok(())
            },
            Err(()) => {
                trace!("Placement failure");

                // Return `Ok` anyway. We don't want the stream to stop because the
                // collector may be momentarily down.
                Ok(())
            },
        })
        .for_each(|()| {
            Ok(())
        })
}
