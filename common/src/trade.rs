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
    /// When when this item was created on the exchange. This is the default time entry
    /// used for searches.
    timestamp: DateTime<Utc>,
    size: Decimal,
    price: Decimal,
    market: Market,
    trade: Type,

    //meta: String,

    /// If the trade match has an ID
    #[serde(skip_serializing_if = "Option::is_none")]
    match_id: Option<i64>,

    /// ID of the buy order.
    #[serde(skip_serializing_if = "Option::is_none")]
    buy_order_id: Option<i64>,

    /// ID of the sell order
    #[serde(skip_serializing_if = "Option::is_none")]
    sell_order_id: Option<i64>,

    /// Time when the match actually occured.
    #[serde(skip_serializing_if = "Option::is_none")]
    match_timestamp: Option<DateTime<Utc>>,
}

impl TradeHistoryItem {
    pub fn new(
        timestamp: DateTime<Utc>,
        size: Decimal,
        price: Decimal,
        market: Market,
        trade: Type,
        match_id: Option<i64>,
        buy_order_id: Option<i64>,
        sell_order_id: Option<i64>,
        match_timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        TradeHistoryItem {
            timestamp,
            size,
            price,
            market,
            trade,
            match_id,
            buy_order_id,
            sell_order_id,
            match_timestamp,
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

    pub fn match_id(&self) -> Option<i64> {
        self.match_id
    }

    pub fn buy_order_id(&self) -> Option<i64> {
        self.buy_order_id
    }

    pub fn sell_order_id(&self) -> Option<i64> {
        self.sell_order_id
    }

    pub fn match_timestamp(&self) -> Option<DateTime<Utc>> {
        self.match_timestamp
    }
}
