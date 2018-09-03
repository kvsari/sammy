//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate actix;
extern crate actix_web;

extern crate translator_lib as lib;

use actix::Actor;
use actix_web::{server::HttpServer, App, http::Method};

use lib::{restful, filter};

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load configuration.");
    debug!("{:?}", &config);

    let system = actix::System::new("Translator");

    let kraken_filter = filter::KrakenTradeHistory::new();
    let kf_addr = kraken_filter.start();

    let rest_state = lib::restful::State::new(kf_addr);

    HttpServer::new(move || {
        App::with_state(rest_state.clone())
            .scope("/trade_history", |scope| {
                scope
                    .resource("", |r| {
                        r.method(Method::GET).f(restful::trade_match_root)
                    })
                    .resource("/{left_asset}", |r| {
                        r.method(Method::GET).f(restful::trade_match_left_asset)
                    })
                    .resource("/{left_asset}/{right_asset}", |r| {
                        r.method(Method::GET).f(restful::trade_match_asset_pair)
                    })
                    .resource("/{left_asset}/{right_asset}/{exchange}", |r| {
                        r.method(Method::PUT).f(restful::trade_match_put)
                    })
            })
    })
        .bind(config.listen())
        .expect("Can't bind address.")
        .start();
    
    system.run();
}
