//! Program entry
#[macro_use] extern crate log;
extern crate tokio;
extern crate dotenv;
extern crate env_logger;
extern crate ws;

extern crate common;

extern crate binance_lib as lib;

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let configuration = config::config_from_environment().expect("Can't load config.");
    debug!("Configuration: {:?}", &configuration);

    ws::connect("wss://stream.binance.com:9443/ws/bnbbtc@trade", |out| {
        lib::Client::new(out)
    }).unwrap();

    /*
    loop {
        ::std::thread::sleep_ms(1000);
    }
    */
}
