//! Asset types.
use std::{fmt, error, str};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Asset {
    BTC,
    ETH,
    USD,
    BNB,
}

impl Asset {
    pub fn as_str(&self) -> &str {
        match self {
            Asset::BTC => "BTC",
            Asset::ETH => "ETH",
            Asset::USD => "USD",
            Asset::BNB => "BNB",
        }
    }
}

impl str::FromStr for Asset {
    type Err = ParseAssetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BTC" | "btc" => Ok(Asset::BTC),
            "ETH" | "eth" => Ok(Asset::ETH),
            "USD" | "usd" => Ok(Asset::USD),
            "BNB" | "bnb" => Ok(Asset::BNB),
            _ => Err(ParseAssetError),
        }
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pair {
    left: Asset,
    right: Asset,
}

impl Pair {
    pub fn new(left: Asset, right: Asset) -> Self {
        Pair { left, right }
    }

    pub fn left(&self) -> Asset {
        self.left
    }

    pub fn right(&self) -> Asset {
        self.right
    }
}

impl str::FromStr for Pair {
    type Err = ParseAssetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BTCUSD" | "BTC_USD" | "BTC/USD" | "btcusd" | "btc_usd" | "btc/usd" => {
                Ok(BTC_USD)
            },
            _ => Err(ParseAssetError),
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.left.as_str(), self.right.as_str())
    }
}

macro_rules! asset_pair {
    ($name:ident, $left:expr, $right:expr) => {
        pub const $name: Pair = Pair { left: $left, right: $right };
    };
}

asset_pair!(BTC_USD, Asset::BTC, Asset::USD);
asset_pair!(ETH_USD, Asset::ETH, Asset::USD);

#[derive(Debug, Copy, Clone)]
pub struct ParseAssetError;

impl fmt::Display for ParseAssetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse text into asset.")
    }
}

impl error::Error for ParseAssetError {
    fn description(&self) -> &str {
        "Failure to parse into asset."
    }
}
