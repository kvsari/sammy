//! Database actor

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

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Trade history storer started.");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
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
