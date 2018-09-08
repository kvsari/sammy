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
use ticker_db::model::FreshTick;

use database;

lazy_static! {
    static ref TIME_PERIOD: Duration = Duration::from_secs(60 * 15); // 15 minutes
    //static ref TIME_PERIOD: Duration = Duration::from_secs(60 * 1); // 1 minute
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

#[derive(Debug, Copy, Clone)]
struct PriceSize(Decimal, Decimal);

/// Maintain a running tally of trade history data as it comes in.
#[derive(Debug, Copy, Clone)]
struct Accumulation {
    first: PriceSize,
    highest: PriceSize,
    lowest: PriceSize,
    last: PriceSize,
    count: u64,
}

impl Accumulation {
    /// Never build a new accumulation without a starting item.
    fn new(item: &trade::TradeHistoryItem) -> Self {
        Accumulation {
            first: PriceSize(item.price(), item.size()),
            highest: PriceSize(item.price(), item.size()),
            lowest: PriceSize(item.price(), item.size()),
            last: PriceSize(item.price(), item.size()),
            count: 1,
        }
    }
}

struct Accumulator;

impl Accumulator {
    fn new_accumulation(item: &trade::TradeHistoryItem) -> Accumulation {
        Accumulation::new(item)
    }

    /*
    fn add(accumulation: Accumulation, item: trade::TradeHistoryItem) -> Accumulation {
    }
     */

    fn mut_add(accumulation: &mut Accumulation, item: &trade::TradeHistoryItem) {
        let price = item.price();
        let size = item.size();
        
        match () {
            _ if price > accumulation.highest.0 => {
                accumulation.highest = PriceSize(price, size);
            },
            _ if price < accumulation.lowest.0 => {
                accumulation.lowest = PriceSize(price, size);
            },
            _ => (),
        }

        accumulation.last = PriceSize(price, size);
        accumulation.count += 1;
    }
}

/// Message to signal that a tick is to be emitted from the 
#[derive(Message)]
struct ItIsTime;

/// Collates incoming trade history into the right collections (sorted by asset pair and
/// exchange) and then processes a calculation based on a time span that contains basic
/// candle information and some other stuff.
pub struct TickGenerator {
    period: Duration,
    p_start: DateTime<Utc>,
    accumulation: HashMap<(exchange::Exchange, asset::Pair), Accumulation>,
    db_executor: Addr<database::TickDbExecutor>,
}

impl TickGenerator {
    pub fn new(db_executor: Addr<database::TickDbExecutor>) -> Self {
        TickGenerator {
            period: *TIME_PERIOD, // grab a local copy for... reasons.
            p_start: Utc::now(),
            accumulation: HashMap::new(),
            db_executor: db_executor,
        }
    }
}

impl Actor for TickGenerator {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        let self_addr = ctx.address();
        
        // Reset the start time to now as this is when the actor is really running. There
        // could be any number of delays between instantiation and execution after all.
        self.p_start = Utc::now();       
        
        // Start the self timing tigger stream future.
        let start = closest_quarter(Instant::now(), self.p_start);
        let timing = tokio_timer::Interval::new(start, self.period)
            .map_err(|e| error!("Problem with timing loop: {}", &e))
            .and_then(move |i| {
                // send the `it's time` message to itself.
                self_addr.send(ItIsTime)
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

impl Handler<ItIsTime> for TickGenerator {
    type Result = ();

    fn handle(&mut self, msg: ItIsTime, ctx: &mut Self::Context) {
        trace!("It's time to generate a new tick.");
        
        let now = Utc::now();
        let last = self.p_start;
        
        // Drain accumulation into batch data set for batch insertion.
        let batch: Vec<FreshTick> = self.accumulation
            .drain()
            .map(|((exchange, asset_pair), accumulation)| {
                FreshTick::new(
                    exchange,
                    asset_pair,
                    last,
                    now,
                    accumulation.first.0,
                    accumulation.first.1,
                    accumulation.highest.0,
                    accumulation.highest.1,
                    accumulation.lowest.0,
                    accumulation.lowest.1,
                    accumulation.last.0,
                    accumulation.last.1,
                    accumulation.count as i32,
                )
            })
            .collect();

        println!("Ticks generated: {:?}", &batch);

        // Send batched accumulations to DB actor
        let msg = database::NewTicks(batch);
        let send_future = self.db_executor
            .send(msg)
            .map_err(|e| error!("Couldn't send tick batch to DB executor: {}", &e));

        Arbiter::spawn(send_future);

        // New tick start time.
        self.p_start = now;
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

        // Add raw data to accumulation
        let key = (msg.exchange, msg.asset_pair);
        self.accumulation
            .entry(key)
            .and_modify(|accumulation| {
                msg.items
                    .iter()
                    .for_each(|item| Accumulator::mut_add(accumulation, item));
            })
            .or_insert_with(|| {
                let mut iter = msg.items.iter();

                // There must be at least one item.
                let item = iter.next().expect("Emtpy raw data sent to ticker generator.");
                let mut accumulation = Accumulation::new(item);

                // Handle the rest, if any
                iter.for_each(|item| Accumulator::mut_add(&mut accumulation, item));

                accumulation
            });

        // TODO
        // Send raw data to DB for storage
    }
}
