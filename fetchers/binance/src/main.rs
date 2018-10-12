//! Program entry
#[macro_use] extern crate log;
extern crate tokio;
extern crate dotenv;
extern crate env_logger;
extern crate ws;

extern crate common;

extern crate binance_lib as lib;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use common::asset;

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let configuration = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &configuration);

    let request = lib::StreamRequest::new()
        .add_trade_history_item_stream(asset::BNB_BTC);

    lib::stream(request, Arc::new(AtomicBool::new(false)))
        .expect("Can't start stream.");
}
