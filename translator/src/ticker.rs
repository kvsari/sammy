//! Actor for building the ticker from the trade history items.
use std::collections::HashMap;
use std::time::Duration;
use std::default::Default;

use rust_decimal::Decimal;
use actix::prelude::*;

use common::{asset, exchange};

lazy_static! {
    static ref TIME_PERIOD: Duration = Duration::from_secs(60 * 15); // 15 minutes
}

struct PriceSize(Decimal, Decimal);

/// Maintain a running tally of trade history data as it comes in.
struct Accumulation {
    first: PriceSize,
    highest: PriceSize,
    lowest: PriceSize,
    last: PriceSize,
    count: u64,
}

/*
impl Default for Collection {
    fn default() -> Self {
        Collection {
        }
    }
}
 */

/// Message to signal that a tick is to be emitted from the 
#[derive(Message)]
struct ItsTime;

/// Collates incoming trade history into the right collections (sorted by asset pair and
/// exchange) and then processes a calculation based on a time span that contains basic
/// candle information and some other stuff.
pub struct TickGenerator {
    period: Duration,
    p_start: DateTime<Utc>,
    kraken: HashMap<asset::Pair, Accumulation>,
}

impl TickGenerator {
    pub fn new() -> Self {
        TickGenerator {
            period: *TIME_PERIOD, // grab a local copy for... reasons.
            p_start: Utc::now(),
            kraken: HashMap::new(),
        }
    }
}

impl Actor for TickGenerator {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {

        
        
        debug!("Tick Generator started.");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        debug!("Tick Generator stopped.");
    }
}
