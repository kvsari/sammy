//! Filter trade history actors.
use std::collections::HashMap;
use std::io;

use futures::Future;
use futures::future::{self, lazy, FutureResult};
use chrono::{DateTime, Utc};
use actix::prelude::*;

use common::{asset, trade};

#[derive(Message)]
pub struct UnfilteredTradeHistory {
    asset_pair: asset::Pair,
    history: Vec<trade::TradeHistoryItem>,
}

impl UnfilteredTradeHistory {
    pub fn new(asset_pair: asset::Pair, history: Vec<trade::TradeHistoryItem>) -> Self {
        UnfilteredTradeHistory {
            asset_pair,
            history,
        }
    }
}

#[derive(Message)]
struct ToFilterTradeHistory {
    asset_pair: asset::Pair,
    history: Vec<trade::TradeHistoryItem>,
}

/// Filter optimized for kraken trade history. This will ensure that only new items are
/// forwarded on through the system.
pub struct KrakenTradeHistory {
    timestamp_marker: HashMap<asset::Pair, DateTime<Utc>>,
}

impl KrakenTradeHistory {
    pub fn new() -> Self {
        KrakenTradeHistory {
            timestamp_marker: HashMap::new(),
        }
    }
}

impl Actor for KrakenTradeHistory {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Kraken Trade History filter started.");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        debug!("Kraken Trade History filter stopped.");
    }
}

impl Handler<UnfilteredTradeHistory> for KrakenTradeHistory {
    type Result = ();

    fn handle(&mut self, msg: UnfilteredTradeHistory, ctx: &mut Self::Context) {
        let self_addr = ctx.address();
        let message = ToFilterTradeHistory {
            asset_pair: msg.asset_pair,
            history: msg.history,
        };

        // Send to itself. This is to detach completion of this method from actual
        // completion of the work. Therefore the method can return immediately whilst the
        // work is now being handled elsewhere (still in the same actor though).
        Arbiter::spawn(lazy(move || {
            self_addr
                .send(message)
                .then(|result| {
                    match result {
                        Ok(()) => println!("Sent internally"),
                        Err(e) => println!("Internal error!"),
                    }
                    Ok(())
                })
        }))
    }
}

impl Handler<ToFilterTradeHistory> for KrakenTradeHistory {
    type Result = ();

    fn handle(&mut self, msg: ToFilterTradeHistory, ctx: &mut Self::Context) {
        println!("Alright! Time to sleep...");
        ::std::thread::sleep_ms(5000);
        println!("Finished sleeping.");
        
    }
}
