//! Code
#[macro_use] extern crate log;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate tokio_timer;

pub mod https_client;
pub mod asset;
pub mod kraken;

pub use self::https_client::{HttpsClient, FetchError};
