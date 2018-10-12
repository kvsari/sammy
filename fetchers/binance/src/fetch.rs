//! Code that manages the fetching of the data from binance. Will run a websocket connection
//! inside an infinite loop (binance disconnects you automatically every 24 hours) that will
//! forward payload items to the rest of the system.
//!
//! Binance websocket API allows us to fetch multiple streams within a single connection.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde_json;
use ws::{self, Sender, Handler, Message, Handshake, util::Token, CloseCode};

use super::StreamRequest;
use payload;

const CHECK_FOR_STOP: Token = Token(1);
const TIMEOUT_MILLIS: u64 = 2000; // Two second timeout.

struct Client {
    /// Websocket sender. Link to `mio` event loop backend. Also used for initiating close.
    ws: Sender,

    /// To permit graceful shutdown (and trapping ctlr-c plus other signals).
    stop: Arc<AtomicBool>,
}

impl Client {
    fn new(ws: Sender, stop: Arc<AtomicBool>) -> Self {
        Client {
            ws, stop,
        }
    }
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        //println!("WS TOKEN: {:?}", self.ws.token());

        // Setup stop check timeout every second
        self.ws.timeout(TIMEOUT_MILLIS, CHECK_FOR_STOP)
    }

    fn on_timeout(&mut self, event: ws::util::Token) -> Result<(), ws::Error> {
        if event == CHECK_FOR_STOP {
            // We check if stop is true
            if self.stop.load(Ordering::Relaxed) {
                self.ws.close(CloseCode::Normal)?;
            }
            
            // Reschedule the timeout
            self.ws.timeout(TIMEOUT_MILLIS, CHECK_FOR_STOP)
        } else {
            Err(ws::Error::new(
                ws::ErrorKind::Internal, "Invalid timeout token encountered!"
            ))
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        let json = msg.as_text()?;
        match serde_json::from_str::<payload::StreamItem>(json) {
            Ok(payload) => {
                println!("Received payload: {:?}", &payload);
            },
            Err(e) => {
                error!("Payload deserialization failed: {}", &e);
                // TODO: Do Something with the error.
            },
        }
        Ok(())
    }
}



/// Starts websocket connection within reconnect loop. Blocks calling thread.
pub fn stream(subscription: StreamRequest, stop: Arc<AtomicBool>
) -> Result<(), String> {
    while !stop.load(Ordering::Relaxed) {
        match ws::connect(subscription.url(), |sender| Client::new(sender, stop.clone())) {
            Ok(()) => (), // Stopped normally.
            Err(e) => {
                error!("Encountered error: {}", &e);
                // TODO: Do something with the error.
                // TODO: Back off on reconnecting. Add a delay.
            },
        }
    }

    Ok(())
}
