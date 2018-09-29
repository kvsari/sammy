//! RESTful handlers. 

use futures::{future, Future};
use chrono::{NaiveDateTime, DateTime, Utc};
use actix::Addr;
use actix_web::{HttpRequest, HttpResponse, Responder, AsyncResponder, error};

use common::asset::{self, Asset};
use common::exchange::Exchange;

use output;
use database;
use fold;
use super::State;

macro_rules! parse_path_segment {
    ($segment:expr) => {
        match $segment.parse() {
            Ok(s) => s,
            Err(_e) => return Box::new(future::ok(HttpResponse::BadRequest().finish())),
        };
    };
}

macro_rules! extract_query {
    ($parameter:expr, $query_hash:expr) => {
        match $query_hash.get($parameter) {
            Some(param) => match param.parse() {
                Ok(p) => p,
                Err(_e) => return Box::new(future::ok(HttpResponse::BadRequest().finish())),
            },
            None => return Box::new(future::ok(HttpResponse::BadRequest().finish())),
        }
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

fn generate_tick(
    req: fold::RequestTick,
    addr: Addr<fold::TradeHistoryFolder>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    addr.send(req)
        .then(move |result| match result {
            Ok(Ok(tick)) => {
                trace!("Tick: {:?}", &tick);
                Ok(HttpResponse::Ok().json(tick))
            },
            Ok(Err(e)) => {
                error!("Tick fold failure: {}", &e);
                Ok(HttpResponse::InternalServerError().finish())
            },
            Err(e) => {
                error!("Trade history folding actor failure: {}", &e);
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

pub fn thf_match_asset_pair_tick(
    req: &HttpRequest<State>
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");

    let query = req.query();
    let from: u64 = extract_query!("from", query);
    let to: u64 = extract_query!("to", query);

    let from = DateTime::from_utc(NaiveDateTime::from_timestamp(from as i64, 0), Utc);
    let to = DateTime::from_utc(NaiveDateTime::from_timestamp(to as i64, 0), Utc);

    let left: Asset = parse_path_segment!(lasset);
    let right: Asset = parse_path_segment!(rasset);
    let pair = asset::Pair::new(left, right);
    let state = req.state();
    let folder = state.trade_history_folder().clone();
    let request = fold::RequestTick::new(pair, from, to);
    
    generate_tick(request, folder)
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

pub fn thf_match_exchange_tick(
    req: &HttpRequest<State>
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let params = req.match_info();
    let lasset = params.get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = params.get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");
    let exchange = params.get("exchange")
        .expect("Invalid use of function. Need to have {exchange} on path.");

    let query = req.query();
    let from: u64 = extract_query!("from", query);
    let to: u64 = extract_query!("to", query);

    let from = DateTime::from_utc(NaiveDateTime::from_timestamp(from as i64, 0), Utc);
    let to = DateTime::from_utc(NaiveDateTime::from_timestamp(to as i64, 0), Utc);

    let left: Asset = parse_path_segment!(lasset);
    let right: Asset = parse_path_segment!(rasset);
    let exchange: Exchange = parse_path_segment!(exchange);
    let pair = asset::Pair::new(left, right);
    let state = req.state();
    let folder = state.trade_history_folder().clone();
    let request = fold::RequestTick::new(pair, from, to).filter_exchange(exchange);
    
    generate_tick(request, folder)
}
