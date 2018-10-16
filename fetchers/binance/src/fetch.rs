//! Code that manages the fetching of the data from binance. Will run a websocket connection
//! inside an infinite loop (binance disconnects you automatically every 24 hours) that will
//! forward payload items to the rest of the system.
//!
//! Binance websocket API allows us to fetch multiple streams within a single connection.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use futures::sync::mpsc::UnboundedSender;
use serde_json;
use ws::{self, Sender, Handler, Message, Handshake, util::Token, CloseCode};

use common::{asset, trade};

use super::StreamRequest;
use payload;

const CHECK_FOR_STOP: Token = Token(1);
const TIMEOUT_MILLIS: u64 = 2000; // Two second timeout.

struct Client {
    /// Websocket sender. Link to `mio` event loop backend. Also used for initiating close.
    ws: Sender,

    /// To permit graceful shutdown (and trapping ctlr-c plus other signals).
    stop: Arc<AtomicBool>,

    /// Forward received items on.
    th_sender: UnboundedSender<(asset::Pair, trade::TradeHistoryItem)>,
}

impl Client {
    fn new(
        ws: Sender,
        stop: Arc<AtomicBool>,
        th_sender: UnboundedSender<(asset::Pair, trade::TradeHistoryItem)>,
    ) -> Self {
        Client {
            ws, stop, th_sender,
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

    /// Inspects each message and depending on the type, will forward it on through the
    /// appropriate `UnboundedSender` channel.
    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        let json = msg.as_text()?;
        match serde_json::from_str::<payload::StreamItem>(json) {
            Ok(stream_item) => {
                trace!("Received stream item: {:?}", &stream_item);
                match stream_item.data().asset_pair() {
                    Ok(ap) => {
                        // Check if the payload is a trade history item.
                        if let Some(thi) = stream_item.data().as_trade_history_item() {
                            self.th_sender
                                .unbounded_send((ap, thi))
                                .unwrap_or_else(|_| {
                                    self.stop.store(true, Ordering::Relaxed)
                                });
                        }
                    },
                    Err(e) => {
                        error!("Invalid asset pair symbol: {}", &e);
                        // TODO: Do something with this error. Currently will ignore and
                        //       keep going with the next item. Maybie should keep an error
                        //       count in here and notify an external system?
                    },
                }
            },
            Err(e) => {
                error!("Payload deserialization failed: {}", &e);
                // TODO: Do Something with the error. If this starts happening consecutively
                //       for example it could indicate an API change. Thus some form of
                //       notification is essential for these kinds of errors.
            },
        }
        Ok(())
    }
}



/// Starts websocket connection within reconnect loop. Blocks calling thread.
pub fn stream(
    subscription: StreamRequest,
    stop: Arc<AtomicBool>,
    th_sender: UnboundedSender<(asset::Pair, trade::TradeHistoryItem)>,
) {
    while !stop.load(Ordering::Relaxed) {
        let url = subscription.url();
        match ws::connect(
            url, |sender| Client::new(sender, stop.clone(), th_sender.clone())
        ) {
            Ok(()) => (), // Stopped normally.
            Err(e) => {
                error!("Encountered error: {}", &e);
                // TODO: Do something with the error.
                // TODO: Back off on reconnecting. Add a delay.
            },
        }
    }
}
