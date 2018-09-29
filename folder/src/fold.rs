//! Tick generation actor. Sources data from the database actor and folds over to produce
//! a single tick which is then returned to the consumer.
use std::convert::From;

use futures::Future;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use actix::prelude::*;

use common::{exchange, asset};

use output;
use database;

/// Request a tick for all the trade history items that fall within the set criteria. Take
/// care to not issue a date range that is extremely wide as that will force the fold to
/// occur over a very large data set that may exceed memory.
pub struct RequestTick {
    exchange: Option<exchange::Exchange>,
    asset_pair: asset::Pair,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl RequestTick {
    pub fn new(asset_pair: asset::Pair, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        RequestTick {
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

impl Message for RequestTick {
    type Result = Result<output::Tick, String>;
}

impl From<RequestTick> for database::TradeHistoryRequest {
    fn from(rt: RequestTick) -> Self {
        let thr = database::TradeHistoryRequest::new(rt.asset_pair, rt.from, rt.to);

        if let Some(exchange) = rt.exchange {
            thr.filter_exchange(exchange);
        }

        thr
    }
}

/// Folding actor that fetches raw trade history data within params and folds over it
/// generating a single tick (for now).
pub struct TradeHistoryFolder {
    source: Addr<database::TradeHistoryFetcher>,
}

impl TradeHistoryFolder {
    pub fn new(source: Addr<database::TradeHistoryFetcher>) -> Self {
        TradeHistoryFolder {
            source,
        }
    }
}

impl Actor for TradeHistoryFolder {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("Trade history folder started.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("Trade history folder stopped.");
    }
}

impl Handler<RequestTick> for TradeHistoryFolder {
    type Result = ResponseFuture<output::Tick, String>;

    fn handle(&mut self, msg: RequestTick, ctx: &mut Self::Context) -> Self::Result {
        let thr: database::TradeHistoryRequest = msg.into();
        let fetch_fut = self.source.send(thr);

        let z: Decimal = 0.into();

        let fold = fetch_fut
            .map_err(|e| e.to_string())
            .and_then(move |result| match result {
                Ok(items) => Ok(
                    items
                        .into_iter()
                        .fold(output::Tick::new(z, z, z, z, 0), |mut tick, item| {
                            tick.folding_add(*item.price());
                            tick
                        })
                ),
                Err(e) => Err(e),
            });

        Box::new(fold)
    }
}
