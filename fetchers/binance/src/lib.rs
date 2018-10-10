//! Code
extern crate ws;

use ws::{Sender, Handler, Message, Handshake};

pub struct Client {
    out: Sender,
}

impl Client {
    pub fn new(out: Sender) -> Self {
        Client {
            out,
        }
    }
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        println!("Connected!");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        println!("Got message: {}", &msg);
        Ok(())
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("Encountered error: {:?}", &err);
    }
}