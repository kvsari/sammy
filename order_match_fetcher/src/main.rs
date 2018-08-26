//! Entrypoint
extern crate futures;
extern crate tokio;

extern crate order_match_fetcher_lib as lib;

use futures::Future;

fn main() {
    let client = lib::https_client::produce(2).expect("Can't init TLS.");

    let future = lib::kraken::test_fire2(client)
        .map(|body| println!("Body: {}", &body))
        .map_err(|e| println!("Error: {}", &e));

    tokio::run(future);
}
