//! Code
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate serde;
extern crate serde_json;
extern crate rust_decimal;
extern crate ws;

extern crate common;

use ws::{Sender, Handler, Message, Handshake};

mod subscription;
mod payload;
mod fetch;

pub use self::fetch::stream;
