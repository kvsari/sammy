//! Filter trade history actors.
use std::collections::HashMap;
use std::io;

use futures::Future;
use futures::future::{self, lazy, FutureResult};
use chrono::{DateTime, Utc, TimeZone};
use actix::prelude::*;

use common::{asset, trade, exchange};

use ticker;

lazy_static! {
    static ref YR2000: DateTime<Utc> = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
}

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
    ticker: Addr<ticker::TickGenerator>,
}

impl KrakenTradeHistory {
    pub fn new(ticker: Addr<ticker::TickGenerator>) -> Self {
        KrakenTradeHistory {
            timestamp_marker: HashMap::new(),
            ticker: ticker,
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
                        Ok(()) => (),
                        Err(e) => error!("Couldn't send internally: {}", &e),
                    }
                    Ok(())
                })
        }))
    }
}

impl Handler<ToFilterTradeHistory> for KrakenTradeHistory {
    type Result = ();

    fn handle(&mut self, msg: ToFilterTradeHistory, ctx: &mut Self::Context) {
        let asset_pair = msg.asset_pair;
        let mut history = msg.history;
        
        let ts = *self.timestamp_marker.get(&asset_pair).unwrap_or(&YR2000);

        history.retain(move |item| item.timestamp() > ts);

        // Only process further if there's data.
        if let Some(item) = history.last().map(|i| *i) {
            self.timestamp_marker.insert(asset_pair, item.timestamp());
            trace!(
                "{} new {} kraken trade history item(s). Sending to ticker generator.",
                &asset_pair,
                &history.len(),
            );
            
            // Send off to the tick generator
            let data = ticker::RawTradeData::new(
                exchange::Exchange::Kraken, asset_pair, history
            );
            let send_future = self.ticker.send(data)
                .map_err(|e| error!("Can't send to ticker generator! {}", &e));

            Arbiter::spawn(send_future);
        }
    }
}
