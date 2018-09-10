//! Shared state for all the handlers.

use actix::Addr;

use filter;

#[derive(Clone)]
pub struct State {
    kraken_filter: Addr<filter::KrakenTradeHistory>,
}

impl State {
    pub fn new(kraken_filter: Addr<filter::KrakenTradeHistory>) -> Self {
        State {
            kraken_filter,
        }
    }

    pub fn kraken_filter(&self) -> &Addr<filter::KrakenTradeHistory> {
        &self.kraken_filter
    }
}
