//! Handlers for the RESTful resources
use actix_web::{HttpRequest, HttpResponse};

use super::State;

pub fn trade_match_root(_req: &HttpRequest<State>) -> HttpResponse {
    // TODO

    // Return a static string for now.
    let json = r##"[{"assetpair":"BTC/USD","exchanges":["kraken"]}]"##;
    HttpResponse::Ok().body(json)
}

pub fn trade_match_left_asset(req: &HttpRequest<State>) -> HttpResponse {
    let lasset = req.match_info().get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");

    // TODO
    // Match with state on valid asset or not
    println!("Left Asset: {}", &lasset);

    // Return a static string for now.
    let json = r##"["USD"]"##;
    HttpResponse::Ok().body(json)
}

pub fn trade_match_asset_pair(req: &HttpRequest<State>) -> HttpResponse {
    let lasset = req.match_info().get("left_asset")
        .expect("Invalid use of function. Need to have {left_asset} on path.");
    let rasset = req.match_info().get("right_asset")
        .expect("Invalid use of function. Need to have {right_asset} on path.");

    // TODO
    // Match with state on valid assets or not. Then check that the pair is valid.
    println!("Left Asset: {}", &lasset);
    println!("Right Asset: {}", &rasset);

    // Return a static string for now.
    let json = r##"["kraken"]"##;
    HttpResponse::Ok().body(json)
}

pub fn trade_match_put(req: &HttpRequest<State>) -> HttpResponse {
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
    println!("Left Asset: {}", &lasset);
    println!("Right Asset: {}", &rasset);
    println!("Exchange: {}", &exchange);

    // Return a static string for now.
    let json = r##"{"received":10}"##;
    HttpResponse::Ok().body(json)
}
