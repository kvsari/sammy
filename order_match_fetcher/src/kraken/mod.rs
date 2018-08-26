//! Code for kraken
use futures::Future;
use hyper::{Client, Body};
use hyper_tls::HttpsConnector;
use tokio;

pub fn test_fire1() {
    let https = HttpsConnector::new(2).expect("TLS init failed.");
    let client = Client::builder()
        .build::<_, Body>(https);

    let uri = "https://api.kraken.com/0/public/AssetPairs".parse().unwrap();

    let future = client
        .get(uri)
        .map(|res| {
        println!("Response: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        });

    tokio::run(future)
}
