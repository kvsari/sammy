//! Database actor.
use std::convert::From;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use actix::prelude::*;

use common::{asset, exchange};
use ticker_db::model::{FreshTick, Tick};
use ticker_db::crud;

#[derive(Message)]
pub struct NewTicks(pub Vec<FreshTick>);

pub struct TickDbExecutor {
    executor: crud::Ticks,
}

impl TickDbExecutor {
    pub fn new(db_url: &str) -> Self {
        TickDbExecutor {
            executor: crud::Ticks::connect(db_url).expect("Database failure."),
        }
    }
}

impl Actor for TickDbExecutor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Tick Database executor started.");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        debug!("Tick Database executor stopped.");
    }
}

impl Handler<NewTicks> for TickDbExecutor {
    type Result = ();

    fn handle(&mut self, msg: NewTicks, ctx: &mut Self::Context) {
        let fresh_ticks = msg.0;
        trace!("Received new tick to insert: {:?}", &fresh_ticks);

        for fresh_tick in fresh_ticks {
            self.executor.create(&fresh_tick).expect("Couldn't insert fresh tick.");
        }
    }
}
