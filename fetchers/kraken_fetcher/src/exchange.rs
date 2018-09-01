//! Exchanges
use std::{fmt, error, str};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Exchange {
    Kraken,
}

impl str::FromStr for Exchange {
    type Err = ParseExchangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "KRAKEN" => Ok(Exchange::Kraken),
            _ => Err(ParseExchangeError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParseExchangeError;

impl fmt::Display for ParseExchangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse text into exchange.")
    }
}

impl error::Error for ParseExchangeError {
    fn description(&self) -> &str {
        "Failure to parse into exchange."
    }
}
