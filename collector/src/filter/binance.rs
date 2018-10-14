//! Filter for binance

use futures::Future;
use actix::prelude::*;

use common::exchange;

use database::{TradeHistoryStorer, NewTradeHistory};
use super::UnfilteredTradeHistory;

/// Filter for Binance.
pub struct BinanceTradeHistory {
    storer: Addr<TradeHistoryStorer>,
}

impl BinanceTradeHistory {
    pub fn new(storer: Addr<TradeHistoryStorer>) -> Self {
        BinanceTradeHistory {
            storer,
        }
    }
}

impl Actor for BinanceTradeHistory {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("Binance Trade History filter started.");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("Binance Trade History filter stopped.");
    }
}

impl Handler<UnfilteredTradeHistory> for BinanceTradeHistory {
    type Result = ();

    fn handle(&mut self, msg: UnfilteredTradeHistory, _ctx: &mut Self::Context) {
        let asset_pair = msg.asset_pair;
        let history = msg.history;

        // Only process if there are items.
        if !history.is_empty() {
            trace!(
                "{} new {} binance trade history item(s). Sending to DB Store.",
                &asset_pair,
                &history.len(),
            );

            let new_trade_history = NewTradeHistory::new(
                exchange::Exchange::Binance, asset_pair, history
            );

            let send_future = self.storer.send(new_trade_history)
                .map_err(|e| error!("Binance filter can't send to storer! {}", &e));

            Arbiter::spawn(send_future);
        }
    }
}
