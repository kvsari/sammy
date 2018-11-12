//! Restful handlers
use std::{self, fmt};

use futures::{Future, Stream};
use futures::future::ok;
use futures::stream::iter_ok;
use bytes::Bytes;
use chrono::{DateTime, Utc, Duration};
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    AsyncResponder,
    client,
    error,
    http::header,
    HttpMessage,
    ResponseError,
    State,
    Query,
};
use serde_json;

use common::tick::Tick;

use model::TicksRequest;
use function;

pub fn generate_fold_future(url: &str) -> impl Future<Item = Tick, Error = FetchFoldError> {
    client::get(url)
        .finish()
        .expect("Can't prepare client for folder request.")
        .send()
        .map_err(|e| {
            error!("Fold fetch error: {}", &e);
            FetchFoldError::Client(e)
        })
        .and_then(|response| {
            trace!("RESPONSE: {:?}", &response);
            response
                .body()
                .map_err(|e| {
                    error!("Can't get response payload: {}", &e);
                    FetchFoldError::Payload(e)
                })
        })
        .and_then(|bytes: Bytes| {
            trace!("RESPONSE BODY: {:?}", &bytes);
            let tick: Tick = serde_json::from_slice(&bytes)
                .map_err(|e| {
                    error!("Failed to deserialize tick: {}", &e);
                    FetchFoldError::Json(e)
                })?;
            Ok(tick)
        })
}

macro_rules! fold_futures_responder {
    ($fold_futures:expr) => {
        iter_ok($fold_futures.into_iter())
            .and_then(|fold_fut| fold_fut)
            .fold(vec![], |mut ticks, tick| {
                trace!("Tick: {:?}", &tick);
                ticks.push(tick);
                ok(ticks)
            })
            .and_then(|numbers| Ok(
                HttpResponse::Ok()
                    .header(header::CACHE_CONTROL, "no-cache")
                    .json(numbers)
            ))
            .from_err()
            .responder()
    };
}

#[derive(Debug, Clone)]
pub struct ServerState {
    folder_url: String,
}

impl ServerState {
    pub fn new(folder_url: &str) -> Self {
        ServerState {
            folder_url: folder_url.to_owned(),
        }
    }
}

pub fn info(_req: &HttpRequest<ServerState>) -> impl Responder {
    let blurb = r##"{"into":"Emit ticks."}"##;
    HttpResponse::Ok().body(blurb)
}

pub fn ticks_last_24h_10_min_spans(
    req: &HttpRequest<ServerState>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    let mut minutes = 1440;

    let state = req.state();
    let folder_url = state.folder_url.clone();

    let mut fold_futures = Vec::new();
    
    for _ in 0..144 {
        let subtract = Duration::minutes(minutes);
        let start = now - subtract;
        minutes -= 10;

        // It's perfectly balanced. 1440 negated by 10 144 times balances out.
        // Should never break. Just here in case the `minutes` variable is ever changed.
        if minutes < 0 {
            //panic!("Over extended!");
            break;
        } 
        
        let subtract = Duration::minutes(minutes);
        let end = now - subtract;

        let start_ts = start.timestamp();
        let end_ts = end.timestamp();

        let url = format!(
            "{}/trade_history/btc/usd/tick?from={}&to={}", &folder_url, &start_ts, &end_ts
        );
        
        fold_futures.push(generate_fold_future(url.as_str()));
    }

    fold_futures_responder!(fold_futures)
}

pub fn ticks(
    state: State<ServerState>, query: Query<TicksRequest>
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let folder_url = state.folder_url.clone();
    
    // 1. Create an inner function that does the actual work of preparing the vector of tick
    // requests. Inner function so it can be unit tested.
    let req_urls = function::prepare_folder_requests(&folder_url, &query);

    //debug!("Request URLS: {:?}", &req_urls);

    // 2. Take this vec of tick requests and map into a vec of request futures.
    // 3. Turn the vec of request futures into a stream future.
    // 4. Return as responder future.
    fold_futures_responder!(
        req_urls
            .into_iter()
            .map(|url| generate_fold_future(url.as_str()))
    )
}

#[derive(Debug)]
enum FetchFoldError {
    Client(client::SendRequestError),
    Payload(error::PayloadError),
    Json(serde_json::error::Error),
}

impl fmt::Display for FetchFoldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchFoldError::Client(ref err) => write!(f, "Fold fetch error: {}", err),
            FetchFoldError::Payload(ref err) => write!(f, "Fold fetch error: {}", err),
            FetchFoldError::Json(ref err) => write!(f, "Fold fetch error: {}", err),
        }       
    }
}

impl std::error::Error for FetchFoldError {
    fn description(&self) -> &str {
        "An error with the tick fetching process."
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            FetchFoldError::Json(ref err) => Some(err),
            _ => None,
        }
    }
}

impl ResponseError for FetchFoldError { }

