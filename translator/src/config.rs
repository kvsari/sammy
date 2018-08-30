//! Configuration
use std::{env, fmt, error, convert, net};

static LISTEN: &str = "TRANSLATOR_LISTEN_ADDR";

#[derive(Debug)]
pub struct Configuration {
    listen: net::SocketAddr,
}

impl Configuration {
    pub fn listen(&self) -> net::SocketAddr {
        self.listen
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    let listen = env::var(LISTEN).map_err(|e| (LISTEN, e))?;
    
    Ok(Configuration {
        listen: listen.parse().map_err(|e| (LISTEN, e))?,
    })
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String, env::VarError),
    InvalidAddr(String, net::AddrParseError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var, err) => write!(f, "Missing {}:{}", &var, &err),
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

impl<'a> convert::From<(&'a str, net::AddrParseError)> for ConfigError {
    fn from(e_tuple: (&str, net::AddrParseError)) -> Self {
        ConfigError::InvalidAddr(e_tuple.0.to_owned(), e_tuple.1)
    }
}
