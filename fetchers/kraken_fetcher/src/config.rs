//! Configuration sourcing
use std::{env, error, fmt, convert, str};

use common::asset;

static TRANSLATOR: &str = "SAMMY_TRANSLATOR";
static ASSET_PAIR: &str = "KRAKEN_ASSET_PAIR";
static MODE: &str = "KRAKEN_FETCH_MODE";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FetchMode {
    TradeHistory,
    OrderBook,
}

impl str::FromStr for FetchMode {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRADE" | "TRADE_HISTORY" | "TRADEHISTORY" => Ok(FetchMode::TradeHistory),
            "ORDER" | "BOOK" | "ORDER_BOOK" | "ORDERBOOK" => Ok(FetchMode::OrderBook),
            _ => Err(ConfigError::InvalidMode(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Configuration {
    asset_pair: asset::Pair,
    fetch_mode: FetchMode,
    translator: String,
}

impl Configuration {
    pub fn asset_pair(&self) -> asset::Pair {
        self.asset_pair
    }

    pub fn fetch_mode(&self) -> FetchMode {
        self.fetch_mode
    }

    pub fn translator(&self) -> &str {
        self.translator.as_str()
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    let asset_pair = env::var(ASSET_PAIR).map_err(|e| (ASSET_PAIR, e))?;
    let fetch_mode = env::var(MODE).map_err(|e| (MODE, e))?;
    let translator = env::var(TRANSLATOR).map_err(|e| (TRANSLATOR, e))?;

    Ok(Configuration {
        asset_pair: asset_pair.parse().map_err(|e| (ASSET_PAIR, e))?,
        fetch_mode: fetch_mode.parse()?,
        translator: translator,
    })
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String, env::VarError),
    InvalidAsset(String, asset::ParseAssetError),
    InvalidMode(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var, err) => write!(f, "Missing {}:{}", &var, &err),
            ConfigError::InvalidAsset(var, err) => write!(f, "Invalid {}:{}", &var, &err),
            ConfigError::InvalidMode(var) => write!(f, "Invalid fetch mode: {}", &var),
        }
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        "Error with loading the configuration"
    }    
}

impl<'a> convert::From<(&'a str, env::VarError)> for ConfigError {
    fn from(e_tuple: (&str, env::VarError)) -> Self {
        ConfigError::MissingEnv(e_tuple.0.to_owned(), e_tuple.1)
    }
}

impl<'a> convert::From<(&'a str, asset::ParseAssetError)> for ConfigError {
    fn from(e_tuple: (&str, asset::ParseAssetError)) -> Self {
        ConfigError::InvalidAsset(e_tuple.0.to_owned(), e_tuple.1)
    }
}
