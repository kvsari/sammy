//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate dotenv;

extern crate order_match_fetcher_lib as lib;

use futures::Future;

mod config;

fn main() {
    dotenv::dotenv().ok();
    let config = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &config);
    
    let client = lib::https_client::produce(2).expect("Can't init TLS.");

    let future = lib::kraken::poll_trade_history(
        client.clone(), lib::asset::BTC_USD, lib::kraken::KrakenFetchTargets,
    );

    tokio::run(future);
}
