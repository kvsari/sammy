//! Common errors. Placed here as to prevent cut-n-pasting of code.
//!
//! If this module starts to fill up, consider creating a new crate to hold them.
use std::{fmt, env, error, convert};

use asset;

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
