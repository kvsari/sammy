//! Trade history models.
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Whether a market match was a taker or maker. A taker is a buyer or bidder. A maker is
/// a seller or asker. This depends on the perspective of the trading asset pair.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Market {
    Maker,
    Taker,
}

/// Common trade history item that is used by various components and thus transmitted. Needs
/// to be general over several variants of a trade history item thus not all fields may have
/// values in them due to the source exchange they came from.
///
/// The asset pair and source exchange is not stored in the struct because this information
/// aught to be derived from its storage/usage context.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeHistoryItem {
    timestamp: DateTime<Utc>,
    size: Decimal,
    price: Decimal,
    market: Market,
    //meta: String,
}

impl TradeHistoryItem {
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn size(&self) -> Decimal {
        self.size
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn market(&self) -> Market {
        self.market
    }
}

