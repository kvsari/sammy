//! Code
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

pub mod https_client;
pub mod kraken;

pub use self::https_client::{HttpsClient, FetchError};
