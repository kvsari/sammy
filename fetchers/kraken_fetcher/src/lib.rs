//! Code
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rust_decimal;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate tokio_timer;
extern crate num_traits;
extern crate chrono;

extern crate common;

pub mod https_client;
pub mod fetch;
pub mod place;
pub mod targets;
mod conversion;
mod model;

pub use self::https_client::{HttpsClient, FetchError};
pub use self::targets::KrakenFetchTargets;
pub use self::place::put_trade_history;
pub use self::fetch::{
    poll_trade_history,
    filter_benign_errors,
    convert_into_common,
};
