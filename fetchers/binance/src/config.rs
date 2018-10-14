//! Configuration
use std::env;

use common::asset;
use common::errors::ConfigError;

use lib::StreamRequest;

static COLLECTOR: &str = "SAMMY_COLLECTOR";
static BINANCE_BASE_URI: &str = "BINANCE_BASE_URI";
static TRADE_HISTORY: &str = "TRADE_HISTORY_STREAMS";

#[derive(Debug, Clone)]
pub struct Configuration {
    collector: String,
    subscribe: StreamRequest,
}

impl Configuration {
    pub fn collector(&self) -> &str {
        self.collector.as_str()
    }

    pub fn subscribe(&self) -> StreamRequest {
        self.subscribe.clone()
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    let collector = env::var(COLLECTOR).map_err(|e| (COLLECTOR, e))?;
    let base_uri = env::var(BINANCE_BASE_URI).ok();
    let trade_history_streams = env::var(TRADE_HISTORY).map_err(|e| (TRADE_HISTORY, e))?;

    let th_asset_pairs: Vec<asset::Pair> = trade_history_streams
        .split(':')
        .map(|ap_str| ap_str.parse().expect("Invalid asset pair code."))
        .collect();

    let mut subscribe = StreamRequest::new();

    if let Some(uri) = base_uri {
        subscribe = subscribe.set_base_uri(uri);
    }

    for ap in th_asset_pairs.into_iter() {
        subscribe = subscribe.add_trade_history_item_stream(ap);
    }

    Ok(Configuration {
        collector, subscribe        
    })
}
