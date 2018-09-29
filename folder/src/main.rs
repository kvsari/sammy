//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate actix;
extern crate actix_web;

extern crate folder_lib as lib;

use actix::prelude::*;
use actix_web::{server::HttpServer, App, http::Method};

use lib::{database, restful, fold};

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load configuration.");
    debug!("{:?}", &config);

    let system = actix::System::new("folder");

    // Setup our manual DB fetching pool. So far we have configured just one db actor.
    let db_url = config.database_url().to_owned();
    let th_fetch_addr = SyncArbiter::start(1, move || {
        database::TradeHistoryFetcher::new(db_url.as_str())
    });

    /*
    let th_fetch_addr_clone = th_fetch_addr.clone();
    let th_fold_addr = SyncArbiter::start(1, move || {
        fold::TradeHistoryFolder::new(th_fetch_addr_clone.clone())
    });
     */

    let th_fold = fold::TradeHistoryFolder::new(th_fetch_addr.clone());
    let th_fold_addr = th_fold.start();

    let state = restful::State::new(th_fetch_addr, th_fold_addr);

    HttpServer::new(move || {
        App::with_state(state.clone())
            .scope("/trade_history", |scope| {
                scope
                    .resource("", |r| {
                        r.method(Method::GET).f(restful::thf_match_root)
                    })
                    .resource("/{left_asset}", |r| {
                        r.method(Method::GET).f(restful::thf_match_left_asset)
                    })
                    .resource("/{left_asset}/{right_asset}", |r| {
                        r.method(Method::GET).f(restful::thf_match_asset_pair)
                    })
                    .resource("/{left_asset}/{right_asset}/tick", |r| {
                        r.method(Method::GET).f(restful::thf_match_asset_pair_tick)
                    })
                    .resource("/{left_asset}/{right_asset}/{exchange}", |r| {
                        r.method(Method::GET).f(restful::thf_match_exchange)
                    })
                    .resource("/{left_asset}/{right_asset}/{exchange}/tick", |r| {
                        r.method(Method::GET).f(restful::thf_match_exchange_tick)
                    })
            })
    })
        .bind(config.listen())
        .expect("Can't bind address.")
        .start();

    system.run();
}
