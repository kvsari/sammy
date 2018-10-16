//! Program entry
#[macro_use] extern crate log;
extern crate futures;
extern crate tokio;
extern crate dotenv;
extern crate env_logger;
extern crate ws;

extern crate common;
extern crate fetch_lib;

extern crate binance_lib as lib;

use std::thread;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use futures::Stream;
use futures::sync::mpsc;

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let configuration = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &configuration);

    //let request = configuration.subscribe();
    let target = fetch_lib::place::Target::new(
        configuration.collector(), 
        common::exchange::Exchange::Binance,
        configuration.subscribe().trade_history_asset_pairs(),
    );

    let (th_place_tx, th_place_rx) = mpsc::unbounded();
    let stop = Arc::new(AtomicBool::new(false));

    let _thread_handle = thread::spawn(move || {
        lib::stream(configuration.subscribe(), stop.clone(), th_place_tx);
    });

    let place_future = fetch_lib::place::put_trade_history(
        fetch_lib::https_client::produce(1).expect("Can't init TLS."),
        target,
        th_place_rx
            .map(|(ap, thi)| (ap, vec![thi]))
            .map_err(|e| error!("Receive failure: {:?}", &e))
            .inspect(|tuple| println!("ITEMS: {:?}", &tuple)),
    );

    tokio::run(place_future);

    //thread_handle.join().expect("Websocket thread termination error.");
}
