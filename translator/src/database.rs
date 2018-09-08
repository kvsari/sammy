//! Database actor.
use std::convert::From;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use common::{asset, exchange};
use ticker_db::model::{FreshTick, Tick};
use ticker_db::crud;
