//! Filter for kraken
use std::collections::HashMap;

use futures::Future;
use futures::future::lazy;
use chrono::{DateTime, Utc, TimeZone};
use actix::prelude::*;

use common::{asset, trade, exchange};
use super::UnfilteredTradeHistory;

use database::{
    NewTradeHistory, TradeHistoryStorer, ReqLastHistoryItem, ReqAllLloadAssetPairs
};

lazy_static! {
    static ref YR2000: DateTime<Utc> = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
}

#[derive(Message)]
struct ToFilterTradeHistory {
    asset_pair: asset::Pair,
    history: Vec<trade::TradeHistoryItem>,
}

#[derive(Message)]
struct UpdateTimestampMarker {
    asset_pair: asset::Pair,
    timestamp: DateTime<Utc>,
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
        // TODO: Setup future that will fill the timestamp_marker with timestamps for all
        //       asset pairs in the database.
        let self_addr = ctx.address();
        let storer_addr = self.storer.clone();
        let kraken = exchange::Exchange::Kraken;

        let update_future = self.storer
            .send(ReqAllLloadAssetPairs)
            .map_err(|e| error!("Can't get asset pair list from database actor: {}", &e))
            .map(|option| option.expect("Must always be `Some(Vec<asset::Pair>)`!"))
            .and_then(move |list| {
                list.into_iter()
                    .for_each(|ap| {
                        //println!("Asset Pair: {}", &ap);
                        let self_addr_clone = self_addr.clone();
                        let request = ReqLastHistoryItem::new(kraken, ap);
                        let ts_fut = storer_addr.send(request)
                            //.map_err(|e| error!("Can't get last trade history item."))
                            .map(move |maybie| maybie
                                 .map(|item| (*item.timestamp(), ap))
                                 .unwrap_or((*YR2000, ap)))
                            .and_then(move |(ts, ap)| {
                                let update = UpdateTimestampMarker {
                                    asset_pair: ap,
                                    timestamp: ts,
                                };
                                self_addr_clone.send(update)
                            })
                            .map_err(|e| error!(
                                "Can't update asset pair timestamp: {}", &e
                            ));

                        Arbiter::spawn(ts_fut);
                    });
                Ok(())
            });

        Arbiter::spawn(update_future);

        debug!("Kraken Trade History filter started.");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
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

    fn handle(&mut self, msg: ToFilterTradeHistory, _ctx: &mut Self::Context) {
        let asset_pair = msg.asset_pair;

        let ts = *self.timestamp_marker.get(&asset_pair).unwrap_or(&YR2000);

        let mut history = msg.history;
        history.retain(move |item| item.timestamp() > ts);

        // Only process further if there's data.
        if let Some(item) = history.last().map(|i| *i) {
            self.timestamp_marker.insert(asset_pair, item.timestamp());
            trace!(
                "{} new {} kraken trade history item(s). Sending to DB store.",
                &asset_pair,
                &history.len(),
            );

            // Send off to the tick generator
            let new_trade_history = NewTradeHistory::new(
                exchange::Exchange::Kraken, asset_pair, history
            );

            let send_future = self.storer.send(new_trade_history)
                .map_err(|e| error!("Kraken filter can't send to storer! {}", &e));

            Arbiter::spawn(send_future);
        }
    }
}

impl Handler<UpdateTimestampMarker> for KrakenTradeHistory {
    type Result = ();

    fn handle(&mut self, msg: UpdateTimestampMarker, _ctx: &mut Self::Context) {
        self.timestamp_marker.insert(msg.asset_pair, msg.timestamp);
    }
}
