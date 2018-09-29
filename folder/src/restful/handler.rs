//! RESTful handlers. 

use futures::{future, Future, Stream};
use actix::Addr;
use actix_web::{HttpRequest, HttpResponse, Responder, HttpMessage, AsyncResponder, error};

use common::asset::{self, Asset};
use common::exchange::Exchange;

use output;
use database;
use super::State;

macro_rules! parse_path_segment {
    ($segment:expr) => {
        match $segment.parse() {
            Ok(s) => s,
            Err(_e) => return Box::new(future::ok(HttpResponse::BadRequest().finish())),
        };
    };
}

fn fetch_summary(
    req: database::TradeSummaryRequest,
    addr: Addr<database::TradeHistoryFetcher>,
    fops: Vec<output::FoldOperation>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    addr.send(req)
        .then(move |result| match result {
            Ok(Ok(mut summary)) => {
                fops.into_iter().for_each(|fop| summary.add_operation(fop));
                trace!("Summary: {:?}", &summary);
                Ok(HttpResponse::Ok().json(summary))
            },
            Ok(Err(e)) => {
                error!("Database fetch failure: {}", &e);
                Ok(HttpResponse::InternalServerError().finish())
            },
            Err(e) => {
                error!("Database actor failure: {}", &e);
                Ok(HttpResponse::InternalServerError().finish())
            },
        })
        .responder()
}

pub fn thf_match_root(req: &HttpRequest<State>) -> impl Responder {
    let state = req.state();
    let fetcher_addr = state.trade_history_fetcher().clone();
    let request = database::TradeSummaryRequest::new();

    fetch_summary(request, fetcher_addr, Vec::new())
}

pub fn thf_match_left_asset(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_asset_pair(
    req: &HttpRequest<State>
) -> Box<Future<Item = HttpResponse, Error = error::Error>>  {
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");

    let left: Asset = parse_path_segment!(lasset);
    let right: Asset = parse_path_segment!(rasset);
    let pair = asset::Pair::new(left, right);
    let state = req.state();
    let fetcher_addr = state.trade_history_fetcher().clone();
    let request = database::TradeSummaryRequest::new().filter_asset_pair(pair);

    fetch_summary(request, fetcher_addr, vec![output::FoldOperation::Tick])
}

pub fn thf_match_asset_pair_tick(req: &HttpRequest<State>) -> impl Responder {
    /*
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");

    let left: Asset = parse_path_segment!(lasset);
    let right: Asset = parse_path_segment!(rasset);
    let state = req.state();
    let fetcher = state.trade_history_fetcher();
    */
    
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1/2/tick"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_exchange(
    req: &HttpRequest<State>
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");
    let exchange = params.get("exchange")
        .expect("Invalid use of function. Need to have {exchange} on path.");

    let left: Asset = parse_path_segment!(lasset);
    let right: Asset = parse_path_segment!(rasset);
    let exchange: Exchange = parse_path_segment!(exchange);
    let pair = asset::Pair::new(left, right);
    let state = req.state();
    let fetcher_addr = state.trade_history_fetcher().clone();
    let request = database::TradeSummaryRequest::new()
        .filter_asset_pair(pair)
        .filter_exchange(exchange);

    fetch_summary(request, fetcher_addr, vec![output::FoldOperation::Tick])
}

pub fn thf_match_exchange_tick(req: &HttpRequest<State>) -> impl Responder {
    /*
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");
    let exchange = params.get("exchange")
        .expect("Invalid use of function. Need to have {exchange} on path.");
     */
    
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1/2/exchange/tick"}"##;
    HttpResponse::Ok().body(finish_me)
}
