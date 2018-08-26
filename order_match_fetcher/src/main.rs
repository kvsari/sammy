//! Entrypoint
extern crate futures;
extern crate tokio;

extern crate order_match_fetcher_lib as lib;

use futures::Future;

fn main() {
    let client = lib::https_client::produce(2).expect("Can't init TLS.");

    let future = lib::kraken::poll_trade_history(
        client.clone(), lib::asset::BTC_USD, lib::kraken::KrakenFetchTargets,
    );

    tokio::run(future);
}
