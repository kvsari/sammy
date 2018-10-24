//! Entrypoint
#![recursion_limit="128"]
#[macro_use] extern crate log;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate dotenv;

extern crate common;

extern crate kraken_lib as lib;
extern crate fetch_lib;

use std::time::Duration;

use futures::future::{Either, lazy, result, FutureResult};

use common::{exchange, asset};
use fetch_lib::{https_client, place};

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &config);

    let fetch_aps = config.asset_pairs();
    let targets = place::Target::new(
        config.translator(),
        exchange::Exchange::Kraken,
        vec![asset::BTC_USD, asset::ETH_USD, asset::ETH_BTC],
    );
    let client = https_client::produce(1).expect("Can't init TLS.");
    let future = match config.fetch_mode() {
        config::FetchMode::TradeHistory => {
            debug!("Trade history fetching chosen.");
            let raw_fetch_stream = lib::poll_trade_history(
                client.clone(),
                fetch_aps[0],
                lib::KrakenFetchTargets,
                Duration::from_secs(15),
            );
            let filtered_fetch_stream = lib::filter_benign_errors(raw_fetch_stream);
            let converted_stream = lib::convert_into_common(filtered_fetch_stream);
            let place_future = place::put_trade_history(
                client.clone(), targets, converted_stream,
            );
            Either::A(place_future)
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
