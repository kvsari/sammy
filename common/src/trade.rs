//! Trade history models.
use std::{error, fmt, str};

use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Whether a market match was a taker or maker. A taker is a buyer or bidder. A maker is
/// a seller or asker. This depends on the perspective of the trading asset pair.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Market {
    Maker,
    Taker,
}

impl str::FromStr for Market {
    type Err = MarketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "B" | "buy" | "BUY" | "t" | "T" | "taker" | "TAKER" => Ok(Market::Taker),
            "s" | "S" | "sell" | "SELL" | "m" | "M" | "maker" | "MAKER" => {
                Ok(Market::Maker)
            },
            _ => Err(MarketParseError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MarketParseError;

impl fmt::Display for MarketParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing market.")
    }
}

impl error::Error for MarketParseError {
    fn description(&self) -> &str {
        "Error parsing market."
    }
}

/// What kind of trade it was. Whether is was a limit order or a market order.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Limit,
    Market,
}

impl str::FromStr for Type {
    type Err = TradeTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "l" | "L" | "limit" | "LIMIT" => Ok(Type::Limit),
            "m" | "M" | "market" | "MARKET" => Ok(Type::Market),
            _ => Err(TradeTypeParseError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TradeTypeParseError;

impl fmt::Display for TradeTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing trade type.")
    }
}

impl error::Error for TradeTypeParseError {
    fn description(&self) -> &str {
        "Error parsing trade type."
    }
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
    trade: Type,
    //meta: String,
}

impl TradeHistoryItem {
    pub fn new(
        timestamp: DateTime<Utc>,
        size: Decimal,
        price: Decimal,
        market: Market,
        trade: Type,
    ) -> Self {
        TradeHistoryItem {
            timestamp, size, price, market, trade
        }
    }
    
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

    pub fn trade(&self) -> Type {
        self.trade
    }
}
