//! Configuration
use std::{env, fmt, error, convert};

static LISTEN: &str = "TRANSLATOR_LISTEN_ADDR";

#[derive(Debug)]
pub struct Configuration {
    listen: String,    
}

impl Configuration {
    pub fn listen(&self) -> &str {
        self.listen.as_str()
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    Ok(Configuration {
        listen: env::var(LISTEN).map_err(|e| (LISTEN, e))?,
    })
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String, env::VarError),
    //InvalidAsset(String, asset::ParseAssetError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingEnv(var, err) => write!(f, "Missing {}:{}", &var, &err),
            //ConfigError::InvalidAsset(var, err) => write!(f, "Invalid {}:{}", &var, &err),
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

          /*
impl<'a> convert::From<(&'a str, asset::ParseAssetError)> for ConfigError {
    fn from(e_tuple: (&str, asset::ParseAssetError)) -> Self {
        ConfigError::InvalidAsset(e_tuple.0.to_owned(), e_tuple.1)
    }
}
     */
