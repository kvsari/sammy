//! Abstracted subscription
use std::iter::{Iterator, IntoIterator, Extend};
use std::fmt;

use common::asset;

const DEFAULT_BINANCE_WEBSOCKET_BASE_URI: &str = "wss://stream.binance.com:9443";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum StreamType {
    TradeHistoryItems(asset::Pair),
}

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamType::TradeHistoryItems(pair) => {
                let l = pair.left().as_str().to_lowercase();
                let r = pair.right().as_str().to_lowercase();                
                write!(f, "{}{}@trade", &l, &r)
            },
        }
    }
}

/// Build a websocket subscription request.
#[derive(Debug, Clone)]
pub struct StreamRequest {
    base: String,
    streams: Vec<StreamType>,
}

impl StreamRequest {
    pub fn new() -> Self {
        StreamRequest {
            base: DEFAULT_BINANCE_WEBSOCKET_BASE_URI.into(),
            streams: Vec::new(),
        }
    }

    pub fn set_base_uri<T: Into<String>>(mut self, base: T) -> Self {
        let base: String = base.into();
        self.base = base;
        self
    }

    pub fn add_trade_history_item_stream(mut self, pair: asset::Pair) -> Self {
        self.streams.push(StreamType::TradeHistoryItems(pair));
        self
    }

    /// Generate URL that can be passed into websocket client.
    pub fn url(&self) -> String {
        let mut streams = self.streams.clone();
        streams.sort();
        streams.dedup();

        let streams = streams
            .into_iter()
            .fold(String::new(), |mut all, st| {
                let item = st.to_string();
                if all.is_empty() {
                    all.extend("/stream?streams=".chars());
                } else {
                    all.extend("/".chars());
                }
                all.extend(item.chars());
                all
            });

        format!("{}{}", &self.base, &streams)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscribe_one_trade_history_stream() {
        let req = StreamRequest::new()
            .add_trade_history_item_stream(asset::BTC_USD);

        assert_eq!(req.url(), "wss://stream.binance.com:9443/stream?streams=btcusd@trade");
    }

    #[test]
    fn subscribe_two_trade_history_streams() {
        let req = StreamRequest::new()
            .add_trade_history_item_stream(asset::BNB_BTC)
            .add_trade_history_item_stream(asset::BNB_USD);

        assert_eq!(
            req.url(),
            "wss://stream.binance.com:9443/stream?streams=bnbbtc@trade/bnbusd@trade"
        );
    }
}
