//! Common code for all the fetchers.
#[macro_use] extern crate log;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate tokio_timer;

extern crate common;

pub mod https_client;
pub mod place;
mod retry;
