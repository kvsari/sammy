//! Time utilities.
use std::num::NonZeroU64;

use chrono::{DateTime, Utc, NaiveDateTime};

pub fn millisecond_timestamp_to_chrono(mts: NonZeroU64) -> DateTime<Utc> {
    let mts = mts.get();
    let seconds = (mts / 1000) as i64;
    let millis = mts % 1000;
    let nanos = (millis * 1000000) as u32;
    DateTime::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
}
