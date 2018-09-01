//! Handlers for the RESTful resources
use actix_web::{HttpRequest, HttpResponse};

use super::State;

pub fn trade_match_root(req: &HttpRequest<State>) -> HttpResponse {
    // TODO

    // Return a static string for now.
    let json = r##"[{"assetpair":"BTC/USD","exchanges":["kraken"]}]"##;
    HttpResponse::Ok().body(json)
}
