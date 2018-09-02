//! Kraken models.
use std::fmt::Debug;

use rust_decimal::Decimal;

use common::asset;

/// Static check to ensure only kraken model inners are wrapped in outers.
pub trait Inner { }

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum TradeMatchItem {
    Text(String),
    Timestamp(Decimal),
}

#[derive(Debug, Clone, Deserialize)]
pub enum Items {
    XXBTZUSD(Vec<Vec<TradeMatchItem>>),
    XETHZUSD(Vec<Vec<TradeMatchItem>>),
}

impl Items {
    fn pair(&self) -> asset::Pair {
        match self {
            Items::XETHZUSD(_) => asset::ETH_USD,
            Items::XXBTZUSD(_) => asset::BTC_USD,
        }
    }

    fn items(&self) -> &Vec<Vec<TradeMatchItem>> {
        match self {
            Items::XETHZUSD(ref items) => items,
            Items::XXBTZUSD(ref items) => items,
        }
    }
}

/// Trade history as [returned by kraken](https://www.kraken.com/en-us/help/api#get-recent-trades).
#[derive(Debug, Clone, Deserialize)]
pub struct TradeHistory {
    #[serde(flatten)]
    items: Items,

    last: String,
}

impl TradeHistory {
    pub fn new(items: Items, last: String) -> Self {
        TradeHistory {
            items, last,
        }
    }
    
    pub fn last(&self) -> &str {
        self.last.as_str()
    }

    pub fn pair(&self) -> asset::Pair {
        self.items.pair()
    }

    pub fn items(&self) -> &Vec<Vec<TradeMatchItem>> {
        self.items.items()
    }
}

impl Inner for TradeHistory { }

/// Outer object that contains either an error or the result itself.
#[derive(Debug, Clone, Deserialize)]
pub struct Outer<T: Inner + Clone + Debug> {
    error: Vec<String>,
    result: Option<T>,
}

impl<T: Inner + Clone + Debug> Outer<T> {
    fn error(&self) -> &[String] {
        self.error.as_slice()
    }

    fn result(&self) -> Option<&T> {
        self.result.as_ref()
    }

    pub fn consume(self) -> (Vec<String>, Option<T>) {
        (self.error, self.result)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    
    use super::*;
    
    static TRADE_HISTORY_BTC_USD_JSON: &str = r##"{"error":[],"result":{"XXBTZUSD":[["6650.00000","0.00100000",1535271158.4026,"b","m",""],["6650.00000","0.19900000",1535271158.4217,"b","m",""],["6650.00000","0.10000000",1535271158.4299,"b","m",""]],"last":"1535290179989384853"}}"##;

    static TRADE_HISTORY_ETH_USD_JSON: &str = r##"{"error":[],"result":{"XETHZUSD":[["6650.00000","0.00100000",1535271158.4026,"b","m",""],["6650.00000","0.19900000",1535271158.4217,"b","m",""],["6650.00000","0.10000000",1535271158.4299,"b","m",""]],"last":"1535290179989384853"}}"##;

    #[test]
    fn deserialize_trade_history() {
        let history: Outer<TradeHistory> = serde_json::from_str(TRADE_HISTORY_BTC_USD_JSON)
            .expect("Failed to deserialize.");
        assert!(history.result().unwrap().last.as_str() == "1535290179989384853");

        let history: Outer<TradeHistory> = serde_json::from_str(TRADE_HISTORY_ETH_USD_JSON)
            .expect("Failed to deserialize.");
        assert!(history.result().unwrap().last.as_str() == "1535290179989384853");
    }
}
