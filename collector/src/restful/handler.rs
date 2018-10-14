//! Handlers for the RESTful resources
use futures::sync::oneshot;
use futures::{Stream, Future, future};
use actix::Arbiter;
use actix_web::{
    HttpRequest, HttpResponse, HttpMessage, Responder, error, AsyncResponder,
};
use bytes::BytesMut;
use serde_json;

use common::trade::TradeHistoryItem;
use common::exchange::Exchange;
use common::asset::{self, Asset};

use super::State;
use filter::UnfilteredTradeHistory;

const PAYLOAD_4MB: usize = 4194304;

/*
macro_rules! parse_path_segment {
    ($segment:expr) => {
        match $segment.parse() {
            Ok(s) => s,
            Err(_e) => return HttpResponse::BadRequest().finish(),
        };
    };
}
*/

macro_rules! parse_path_segment {
    ($segment:expr) => {
        match $segment.parse() {
            Ok(s) => s,
            Err(_e) => return Box::new(future::ok(HttpResponse::BadRequest().finish())),
        };
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TradeHistoryResponse {
    received: u64,
}

impl TradeHistoryResponse {
    pub fn new(received: u64) -> Self {
        TradeHistoryResponse { received }
    }
}

pub fn trade_match_root(_req: &HttpRequest<State>) -> impl Responder {
    // TODO

    // Return a static string for now.
    let json = r##"[{"assetpair":"BTC/USD","exchanges":["kraken"]}]"##;
    HttpResponse::Ok().body(json)
}

pub fn trade_match_left_asset(req: &HttpRequest<State>) -> impl Responder {
    let _lasset = req.match_info().get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");

    // TODO
    // Match with state on valid asset or not
    //println!("Left Asset: {}", &lasset);

    // Return a static string for now.
    let json = r##"["USD"]"##;
    HttpResponse::Ok().body(json)
}

pub fn trade_match_asset_pair(req: &HttpRequest<State>) -> impl Responder {
    let _lasset = req.match_info().get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let _rasset = req.match_info().get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");

    // TODO
    // Match with state on valid assets or not. Then check that the pair is valid.
    //println!("Left Asset: {}", &lasset);
    //println!("Right Asset: {}", &rasset);

    // Return a static string for now.
    let json = r##"["kraken", "binance"]"##;
    HttpResponse::Ok().body(json)
}

pub fn trade_match_put(
    req: &HttpRequest<State>
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");
    let exchange = params.get("exchange")
        .expect("Invalid use of function. Need to have {exchange} on path.");

    // TODO
    // Match with state on valid assets or not. Then check that the pair is valid. Then
    // check if the exchange is valid.
    //println!("Left Asset: {}", &lasset);
    //println!("Right Asset: {}", &rasset);
    //println!("Exchange: {}", &exchange);
    
    let left_asset: Asset = parse_path_segment!(lasset);
    let right_asset: Asset = parse_path_segment!(rasset);
    let exchange: Exchange = parse_path_segment!(exchange);

    let asset_pair = asset::Pair::new(left_asset, right_asset);

    // TODO
    // Validate that the exchange/asset_pair is valid.

    //let k_filter = req.state().kraken_filter().clone();
    let state = req.state().clone();

    // Now grab the raw JSON data and deal with it.
    req.payload()
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            // The payload comes in chunks. We read up to a limit... (we can remove limit).
            if (body.len() + chunk.len()) > PAYLOAD_4MB {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        .and_then(|body| {
            // Then we deserialize it.
            let history: Vec<TradeHistoryItem> = serde_json::from_slice(&body)?;

            Ok(history)
        })
        .and_then(move |history| {
            // Count the number of records (we'll return the count in the response).
            let count = history.len();

            // Forward the deserialized data off to the filter.
            let message = UnfilteredTradeHistory::new(asset_pair, history);
            let (inform, callback) = oneshot::channel();
            match exchange {
                Exchange::Kraken => {
                    let future = state.kraken_filter()
                        .send(message)
                        .then(move |result| inform.send(result))
                        .map_err(|e| error!("Can't inform filter response: {:?}", &e));
                    Arbiter::spawn(future);
                },
                Exchange::Binance => {
                    let future = state.binance_filter()
                        .send(message)
                        .then(move |result| inform.send(result))
                        .map_err(|e| error!("Can't inform filter response: {:?}", &e));
                    Arbiter::spawn(future);
                },
            }

            callback                
                .then(move |result| match result {
                    Ok(result) => match result {
                        Ok(()) => {
                            // Return the count of records received to the client.
                            let received = TradeHistoryResponse::new(count as u64);
                            Ok(HttpResponse::Ok().json(received))
                        },
                        Err(e) => {
                            error!("Actix error: {}", &e);
                            // TODO: Make the origin clearer.
                            Ok(HttpResponse::InternalServerError().finish())
                        },
                    },
                    Err(e) => {
                        error!("Callback channel error: {}", &e);
                        // TODO: Make the origin clearer.
                        Ok(HttpResponse::InternalServerError().finish())
                    },
                })
        })
        .responder()
}
