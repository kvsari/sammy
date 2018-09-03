//! Filter trade history actors.
use std::collections::HashMap;
use std::io;

use chrono::{DateTime, Utc};
use actix::prelude::*;

use common::{asset, trade};

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

impl Message for UnfilteredTradeHistory {
    type Result = Result<(), io::Error>;
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
    type Result = Result<(), io::Error>;

    fn handle(
        &mut self, msg: UnfilteredTradeHistory, ctx: &mut Context<Self>
    ) -> Self::Result {        
        println!("Filter received: {:?}", &msg.history);
        Ok(())
    }
}
