//! Application state for the RESTful handlers.

use actix::Addr;

use fold;
use database;

#[derive(Clone)]
pub struct State {
    trade_history_fetcher: Addr<database::TradeHistoryFetcher>,
    trade_history_folder: Addr<fold::TradeHistoryFolder>,
}

impl State {
    pub fn new(
        trade_history_fetcher: Addr<database::TradeHistoryFetcher>,
        trade_history_folder: Addr<fold::TradeHistoryFolder>,
    ) -> Self {
        State {
            trade_history_fetcher, trade_history_folder,
        }
    }

    pub fn trade_history_fetcher(&self) -> &Addr<database::TradeHistoryFetcher> {
        &self.trade_history_fetcher
    }

    pub fn trade_history_folder(&self) -> &Addr<fold::TradeHistoryFolder> {
        &self.trade_history_folder
    }
}
