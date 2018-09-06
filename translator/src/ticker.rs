//! Actor for building the ticker from the trade history items.
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::default::Default;

use futures::{Future, Stream};
use chrono::{self, DateTime, Utc, Timelike, Datelike};
use rust_decimal::Decimal;
use actix::prelude::*;
use tokio_timer;

use common::{asset, exchange, trade};

lazy_static! {
    //static ref TIME_PERIOD: Duration = Duration::from_secs(60 * 15); // 15 minutes
    static ref TIME_PERIOD: Duration = Duration::from_secs(60 * 1); // 1 minute
}

/// Return an instant that is locked to the next quarter in the future. It assumes that the
/// supplied instant and datetime parameters are near identical chronologically. This
/// function will not return valid results if the instant and datetime differ.
fn closest_quarter(instant: Instant, datetime: DateTime<Utc>) -> Instant {
    let minute = datetime.minute();
    let hour = datetime.hour();
    let day = datetime.ordinal();
    let year = datetime.year();

    //println!("Minute: {}, Hour: {}, Day: {}, Year: {}", &minute, &hour, &day, &year);

    // Our time, at the next quarter on the hour.
    let fifteen = match () {
        _ if minute >= 45 => {
            let day = if hour == 23 {
                let year = if day >= 365 {                    
                    datetime.with_year(year + 1).unwrap()
                } else {
                    datetime
                };
                year.with_ordinal(day + 1).unwrap()
            } else {
                datetime
            };
            day
                .with_hour(hour + 1)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap()
        },
        _ if minute >= 30 => {
            datetime
                .with_minute(45)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap()
        },
        _ if minute >= 15 => {
            datetime
                .with_minute(30)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap()
        },
        _ => {
               datetime
                .with_minute(15)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap()
        }
    };

    //println!("DateTime: {} \nFifteen : {}", &datetime, &fifteen);

    let n_datetime = datetime.naive_utc();
    let n_fifteen = fifteen.naive_utc();

    let diff: chrono::Duration = n_fifteen - n_datetime;
    let diff = diff.to_std().expect("Cannot convert duration.");

    // How many monotic time units from the supplied instant to the future instant that is
    // more or less the next quarter on the hour.
    instant + diff
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
        let self_addr = ctx.address();
        
        // Start the self timing tigger stream future.
        let start = closest_quarter(Instant::now(), Utc::now());
        let timing = tokio_timer::Interval::new(start, self.period)
            .map_err(|e| error!("Problem with timing loop: {}", &e))
            .and_then(move |i| {
                // send the it's time message to itself.
                self_addr.send(ItsTime)
                    .map_err(|e| error!("Can't send the `it's time` event: {}", &e))
            })
            .for_each(|()| {
                trace!("Generate tick timing issued.");
                Ok(())
            });

        Arbiter::spawn(timing);
        
        debug!("Tick Generator started.");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        debug!("Tick Generator stopped.");
    }
}

impl Handler<ItsTime> for TickGenerator {
    type Result = ();

    fn handle(&mut self, msg: ItsTime, ctx: &mut Self::Context) {
        trace!("It's time to generate a new tick.");
        // TODO
    }
}

#[derive(Message)]
pub struct RawTradeData {
    exchange: exchange::Exchange,
    asset_pair: asset::Pair,
    items: Vec<trade::TradeHistoryItem>,
}

impl RawTradeData {
    pub fn new(
        exchange: exchange::Exchange,
        asset_pair: asset::Pair,
        items: Vec<trade::TradeHistoryItem>,
    ) -> Self {
        RawTradeData {
            exchange, asset_pair, items
        }
    }
}

impl Handler<RawTradeData> for TickGenerator {
    type Result = ();

    fn handle(&mut self, msg: RawTradeData, ctx: &mut Self::Context) {
        trace!("Received raw {} trade data from: {}", &msg.asset_pair, &msg.exchange);
        // TODO
    }
}
