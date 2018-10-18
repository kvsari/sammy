//! Common errors. Placed here as to prevent cut-n-pasting of code.
//!
//! If this module starts to fill up, consider creating a new crate to hold them.
use std::{fmt, env, error, convert, num, net};

use asset;

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String, env::VarError),
    InvalidAsset(String, asset::ParseAssetError),
    InvalidMode(String),
    InvalidInt(String, num::ParseIntError),
    InvalidAddr(String, net::AddrParseError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var, err) => write!(f, "Missing {}:{}", &var, &err),
            ConfigError::InvalidAsset(var, err) => write!(f, "Invalid {}:{}", &var, &err),
            ConfigError::InvalidMode(var) => write!(f, "Invalid fetch mode: {}", &var),
            ConfigError::InvalidInt(var, err) => write!(f, "Invalid {}:{}", &var, &err),
            ConfigError::InvalidAddr(var, err) => write!(f, "Invalid {}:{}", &var, &err),
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

impl<'a> convert::From<(&'a str, num::ParseIntError)> for ConfigError {
    fn from(e_tuple: (&str, num::ParseIntError)) -> Self {
        ConfigError::InvalidInt(e_tuple.0.to_owned(), e_tuple.1)
    }
}

impl<'a> convert::From<(&'a str, net::AddrParseError)> for ConfigError {
    fn from(e_tuple: (&str, net::AddrParseError)) -> Self {
        ConfigError::InvalidAddr(e_tuple.0.to_owned(), e_tuple.1)
    }
}
