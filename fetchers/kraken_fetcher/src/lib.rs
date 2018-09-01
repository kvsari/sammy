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

extern crate common;

pub mod https_client;
pub mod fetch;
pub mod targets;
mod model;

pub use self::https_client::{HttpsClient, FetchError};
pub use self::fetch::poll_trade_history;
pub use self::targets::KrakenFetchTargets;
