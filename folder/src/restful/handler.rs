//! RESTful handlers. 

use actix_web::{HttpRequest, HttpResponse, Responder};

use super::State;

pub fn thf_match_root(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_left_asset(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_asset_pair(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1/2"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_asset_pair_tick(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1/2/tick"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_exchange(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1/2/exchange"}"##;
    HttpResponse::Ok().body(finish_me)
}

pub fn thf_match_exchange_tick(_req: &HttpRequest<State>) -> impl Responder {
    // TODO: Finish me
    let finish_me = r##"{"todo":"/trade_history/1/2/exchange/tick"}"##;
    HttpResponse::Ok().body(finish_me)
}
