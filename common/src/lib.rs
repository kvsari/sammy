//! Common models for the entire project.
#[macro_use] extern crate derive_getters;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate rust_decimal;
extern crate chrono;

pub mod trade;
pub mod exchange;
pub mod asset;
pub mod tick;
pub mod errors;
