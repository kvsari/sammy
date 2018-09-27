//! Application state for the RESTful handlers.

use actix::Addr;

use database;

#[derive(Clone)]
pub struct State {
    trade_history_fetcher: Addr<database::TradeHistoryFetcher>,
}

impl State {
    pub fn new(trade_history_fetcher: Addr<database::TradeHistoryFetcher>) -> Self {
        State {
            trade_history_fetcher,
        }
    }

    pub fn trade_history_fetcher(&self) -> &Addr<database::TradeHistoryFetcher> {
        &self.trade_history_fetcher
    }
}
