//! Abstracted subscription
use std::iter::{Iterator, IntoIterator, Extend};
use std::fmt;

use common::asset;

const DEFAULT_BINANCE_WEBSOCKET_BASE_URI: &str = "wss://stream.binance.com:9443";

/// Some assets have a different code on Binance. For example, USD is USDT. Thus, make
/// local amendments to some of these 
fn asset_amend(aa: asset::Asset) -> String {
    match aa {
        asset::Asset::USD => "usdt".into(),
        _ => aa.as_str().to_lowercase(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum StreamType {
    TradeHistoryItems(asset::Pair),
}

/*
impl StreamType {
    pub fn asset_pair(&self) -> asset::Pair {
        match self {
            StreamType::TradeHistoryItems(ap) => *ap,
        }
    }
}
*/

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamType::TradeHistoryItems(pair) => {
                let l = asset_amend(pair.left());
                let r = asset_amend(pair.right());
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

    /// Asset pairs to fetch for trade history streams.
    pub fn trade_history_asset_pairs(&self) -> Vec<asset::Pair> {
        self.streams
            .clone()
            .into_iter()
            .filter_map(|st| match st {
                StreamType::TradeHistoryItems(ap) => Some(ap),
                // _ => None,
            })
            .collect()
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
