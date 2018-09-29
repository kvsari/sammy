//! Database synchronous actor

use chrono::{DateTime, Utc};
use actix::prelude::*;

use common::{exchange, asset};
use trade_history::{crud, model};

use output;

/// Request a table summary of the trade history data set within the criteria
#[derive(Debug, Copy, Clone)]
pub struct TradeSummaryRequest {
    asset_pair: Option<asset::Pair>,
    exchange: Option<exchange::Exchange>,
}

impl TradeSummaryRequest {
    pub fn new() -> Self {
        TradeSummaryRequest {
            asset_pair: None,
            exchange: None,
        }
    }

    pub fn filter_asset_pair(mut self, asset_pair: asset::Pair) -> Self {
        self.asset_pair = Some(asset_pair);
        self
    }

    pub fn filter_exchange(mut self, exchange: exchange::Exchange) -> Self {
        self.exchange = Some(exchange);
        self
    }
}

impl Message for TradeSummaryRequest {
    type Result = Result<output::TradeHistorySummary, String>;
}

/// Request a chronologically ordered list of trade history items that meet the critera.
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

/// Database actor that processes information requests.
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

impl Handler<TradeSummaryRequest> for TradeHistoryFetcher {
    type Result = Result<output::TradeHistorySummary, String>;

    fn handle(
        &mut self, msg: TradeSummaryRequest, _ctx: &mut Self::Context
    ) -> Self::Result {
        let set_summary = self.fetcher.read_set_summary(msg.exchange, msg.asset_pair)
            .map_err(|e| e.to_string())?;

        // If no exchange is set, we searched for all exchanges
        let exchanges = match msg.exchange {
            Some(exchange) => vec![exchange],
            None => self.fetcher.exchanges(),
        };

        // If no asset_pair is set, we searched for all asset pairs.
        let asset_pairs = match msg.asset_pair {
            Some(asset_pair) => vec![asset_pair],
            None => self.fetcher.asset_pairs(),
        };

        let summary = output::TradeHistorySummary::new(
            asset_pairs,
            exchanges,
            *set_summary.count() as u64,
            *set_summary.first(),
            *set_summary.last(),
            None,
        );
        
        Ok(summary)
    }
}
