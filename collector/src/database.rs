//! Database actor

use actix::dev::MessageResponse;
use actix::prelude::*;

use common::{exchange, trade, asset};
use trade_history::{crud, model};

#[derive(Debug, Clone, Message)]
pub struct NewTradeHistory {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    items: Vec<trade::TradeHistoryItem>,
}

impl NewTradeHistory {
    pub fn new(
        exchange: exchange::Exchange,
        asset_pair: asset::Pair,
        items: Vec<trade::TradeHistoryItem>,
    ) -> Self {
        NewTradeHistory {
            exchange, asset_pair, items,
        }
    }
}

pub struct TradeHistoryStorer  {
    executor: crud::Trades,
}

impl TradeHistoryStorer {
    pub fn new(db_url: &str) -> Self {
        TradeHistoryStorer {
            executor: crud::Trades::connect(db_url).expect("Database connect failure."),
        }
    }
}

impl Actor for TradeHistoryStorer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("Trade history storer started.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("Trade history storer stopped.");
    }
}

impl Handler<NewTradeHistory> for TradeHistoryStorer {
    type Result = ();

    fn handle(&mut self, msg: NewTradeHistory, ctx: &mut Self::Context) {
        let exchange = msg.exchange;
        let asset_pair = msg.asset_pair;
        let ftis: Vec<model::FreshTradeItem> = msg.items
            .iter()
            .map(|item| model::FreshTradeItem::new(
                exchange,
                asset_pair,
                item.timestamp(),
                item.size(),
                item.price(),
                item.market(),
                item.trade(),
            ))
            .collect();

        self.executor.create(&ftis).expect("Couldn't insert trade history.");
    }
}

/// A request to fetch the last history item stored in the DB for the exchange/asset_pair.
#[derive(Debug, Copy, Clone)]
pub struct ReqLastHistoryItem {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
}

impl ReqLastHistoryItem {
    pub fn new(exchange: exchange::Exchange, asset_pair: asset::Pair) -> Self {
        ReqLastHistoryItem {
            exchange, asset_pair,
        }
    }
}

impl Message for ReqLastHistoryItem {
    type Result = Option<model::TradeItem>;
}

impl Handler<ReqLastHistoryItem> for TradeHistoryStorer {
    type Result = Option<model::TradeItem>;

    fn handle(&mut self, msg: ReqLastHistoryItem, ctx: &mut Self::Context) -> Self::Result {
        self.executor.read_last_item(msg.exchange, msg.asset_pair)
            .expect("Couldn't read from DB.")
    }
}

/// Request all asset pairs that have been loaded into the database
#[derive(Debug, Copy, Clone)]
pub struct ReqAllLloadAssetPairs;

impl Message for ReqAllLloadAssetPairs {
    type Result = Option<Vec<asset::Pair>>;
}

impl Handler<ReqAllLloadAssetPairs> for TradeHistoryStorer {
    type Result = Option<Vec<asset::Pair>>;

    fn handle(&mut self, _: ReqAllLloadAssetPairs, _: &mut Self::Context) -> Self::Result {
        Some(self.executor.asset_pairs())
    }
}
