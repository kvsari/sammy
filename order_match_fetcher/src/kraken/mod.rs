//! Code for kraken
use futures::{Future, Stream};
use hyper::{Client, Body};
use hyper_tls::HttpsConnector;
use tokio;

use super::{HttpsClient, FetchError};

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

pub fn test_fire2(client: HttpsClient) -> impl Future<Item = String, Error = FetchError> {
    let uri = "https://api.kraken.com/0/public/AssetPairs".parse().unwrap();

    client
        .get(uri)
        .and_then(|res| {
            println!("Response: {}", res.status());
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let body = String::from_utf8(body.to_vec())?;
            Ok(body)
        })
        .from_err()
}
