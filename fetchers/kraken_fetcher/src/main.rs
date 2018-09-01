//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate dotenv;

extern crate kraken_fetcher_lib as lib;

use futures::Future;
use futures::future::{Either, lazy, result, FutureResult};

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &config);
    
    let client = lib::https_client::produce(2).expect("Can't init TLS.");

    let future = match config.fetch_mode() {
        config::FetchMode::TradeHistory => {
            debug!("Trade history fetching chosen.");
            Either::A(lib::kraken::poll_trade_history(
                client.clone(), lib::asset::BTC_USD, lib::kraken::KrakenFetchTargets,
            ))
        },
        config::FetchMode::OrderBook => {
            debug!("Order book fetching chosen.");
            Either::B(lazy(|| -> FutureResult<(), ()> {
                println!("TODO: Implement order book fetcher.");
                result::<(), ()>(Ok(()))
            }))
        },
    };

    tokio::run(future);
}
