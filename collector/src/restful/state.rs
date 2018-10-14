//! Shared state for all the handlers.

use actix::Addr;

use filter;

#[derive(Clone)]
pub struct State {
    kraken_filter: Addr<filter::KrakenTradeHistory>,
    binance_filter: Addr<filter::BinanceTradeHistory>,
}

impl State {
    pub fn new(
        kraken_filter: Addr<filter::KrakenTradeHistory>,
        binance_filter: Addr<filter::BinanceTradeHistory>,
    ) -> Self {
        State {
            kraken_filter, binance_filter
        }
    }

    pub fn kraken_filter(&self) -> &Addr<filter::KrakenTradeHistory> {
        &self.kraken_filter
    }

    pub fn binance_filter(&self) -> &Addr<filter::BinanceTradeHistory> {
        &self.binance_filter
    }
}
