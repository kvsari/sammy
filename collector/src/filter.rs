//! Filter trade history actors.
use std::collections::HashMap;
use std::io;

use futures::Future;
use futures::future::{self, lazy, FutureResult};
use chrono::{DateTime, Utc, TimeZone};
use actix::fut::{self, IntoActorFuture};
use actix::prelude::*;

use common::{asset, trade, exchange};

use database::{NewTradeHistory, TradeHistoryStorer, ReqLastHistoryItem};

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
    storer: Addr<TradeHistoryStorer>,
}

impl KrakenTradeHistory {
    pub fn new(storer: Addr<TradeHistoryStorer>) -> Self {
        KrakenTradeHistory {
            timestamp_marker: HashMap::new(),
            storer: storer,
        }
    }
}

impl Actor for KrakenTradeHistory {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Kraken Trade History filter started.");

        // TODO: Setup future that will fill the timestamp_marker with timestamps for all
        //       asset pairs in the database.
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

        /*
        // Add fetching of timestamp marker from storer here.
        let ts = match self.timestamp_marker.get(&asset_pair) {
            Some(timestamp) => *timestamp,
            None => {
                // TODO: Fetch the timestamp from the DB if any
                let request = ReqLastHistoryItem::new(
                    exchange::Exchange::Kraken, asset_pair
                );
                let fetch_last_timestamp = self.storer
                    .send(request)
                    .map_err(|_| ());
                let update_self = fut::wrap_future::<_, Self>(fetch_last_timestamp)
                    .and_then(move |item, actor, ctx| {
                        let timestamp = item
                            .map(|hi| *hi.timestamp())
                            .unwrap_or(*YR2000);
                        actor.timestamp_marker.insert(asset_pair, timestamp);

                        // Resend the msg to the handler
                        let self_addr = ctx.address();
                        Arbiter::spawn(lazy(move || {
                            self_addr
                                .send(msg)
                                .map_err(|_| ())
                        }));
                        
                        fut::ok::<(), (), Self>(())
                    })
                    .into_future();

                Arbiter::spawn(update_self);

                // We break out of the method.
                return;
            },
        };
        */

        let mut history = msg.history;
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
            let new_trade_history = NewTradeHistory::new(
                exchange::Exchange::Kraken, asset_pair, history
            );

            let send_future = self.storer.send(new_trade_history)
                .map_err(|e| error!("Can't send to storer! {}", &e));

            Arbiter::spawn(send_future);
        }
    }
}
