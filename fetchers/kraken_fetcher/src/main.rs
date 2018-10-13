//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate futures;
extern crate tokio;
extern crate dotenv;

extern crate common;

extern crate kraken_fetcher_lib as lib;

use futures::Stream;
use futures::future::{Either, lazy, result, FutureResult};

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &config);
    let targets = lib::targets::targets(config.translator());
    let client = lib::https_client::produce(1).expect("Can't init TLS.");
    let future = match config.fetch_mode() {
        config::FetchMode::TradeHistory => {
            debug!("Trade history fetching chosen.");
            let put_target = targets.trade_history_uri(&config.asset_pair())
                .expect("Invalid Asset Pair target.");
            debug!("Translator put target: {}", &put_target);
            let raw_fetch_stream = lib::poll_trade_history(
                client.clone(), common::asset::BTC_USD, lib::KrakenFetchTargets,
            );
            let filtered_fetch_stream = lib::filter_benign_errors(raw_fetch_stream);
            let converted_stream = lib::convert_into_common(filtered_fetch_stream);
            let place_stream = lib::put_trade_history(
                client.clone(), put_target, converted_stream
            );
            let future = place_stream.for_each(|()| {
                Ok(())
            });
            
            Either::A(future)
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
