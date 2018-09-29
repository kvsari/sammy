//! Code
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate serde;
extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate futures;
extern crate rust_decimal;

extern crate common;
extern crate trade_history;

pub mod database;
pub mod restful;
pub mod fold;
mod output;
