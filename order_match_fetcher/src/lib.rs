//! Code
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rust_decimal;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate tokio_timer;

pub mod https_client;
pub mod asset;
pub mod kraken;
pub mod exchange;

pub use self::https_client::{HttpsClient, FetchError};
