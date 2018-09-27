//! Database synchronous actor

use chrono::{DateTime, Utc};
use actix::prelude::*;

use common::{exchange, asset};
use trade_history::{crud, model};

#[derive(Debug, Copy, Clone)]
pub struct TradeHistoryRequest {
    exchange: Option<exchange::Exchange>,
    asset_pair: asset::Pair,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl TradeHistoryRequest {
    pub fn new(asset_pair: asset::Pair, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        TradeHistoryRequest {
            exchange: None,
            asset_pair: asset_pair,
            from: from,
            to: to,
        }
    }

    pub fn filter_exchange(mut self, exchange: exchange::Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }
}

impl Message for TradeHistoryRequest {
    type Result = Result<Vec<model::TradeItem>, String>;
}

pub struct TradeHistoryFetcher {
    fetcher: crud::Trades,
}

impl TradeHistoryFetcher {
    pub fn new(db_url: &str) -> Self {
        TradeHistoryFetcher {
            fetcher: crud::Trades::connect(db_url).expect("Database connect failure."),
        }
    }
}

impl Actor for TradeHistoryFetcher {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("Trade history fetcher started.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("Trade history fetcher stopped.");
    }
}

impl Handler<TradeHistoryRequest> for TradeHistoryFetcher {
    type Result = Result<Vec<model::TradeItem>, String>;

    fn handle(
        &mut self, msg: TradeHistoryRequest, _ctx: &mut Self::Context
    ) -> Self::Result {

        // TODO
        Ok(Vec::new())
    }
}
