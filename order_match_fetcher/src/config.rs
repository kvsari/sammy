//! Configuration sourcing
use std::{env, error, fmt, convert};

use lib::exchange::Exchange;
use lib::asset;

static EXCHANGES: &str = "EXCHANGES";
static ASSET_PAIR: &str = "ASSET_PAIR";

#[derive(Debug, Clone)]
pub struct Configuration {
    //exchanges: Vec<Exchange>,
    asset_pair: asset::Pair,
}

impl Configuration {
    pub fn asset_pair(&self) -> asset::Pair {
        self.asset_pair
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    //let exchanges_str = env::var(EXCHANGES).map_err(

    let asset_pair = env::var(ASSET_PAIR).map_err(|e| (ASSET_PAIR, e))?;

    Ok(Configuration {
        asset_pair: asset_pair.parse().map_err(|e| (ASSET_PAIR, e))?,
    })
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String, env::VarError),
    InvalidAsset(String, asset::ParseAssetError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var, err) => write!(f, "Missing {}:{}", &var, &err),
            ConfigError::InvalidAsset(var, err) => write!(f, "Invalid {}:{}", &var, &err),
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
