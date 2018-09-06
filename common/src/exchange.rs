//! Exchanges
use std::{fmt, error, str};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Exchange {
    Kraken,
}

impl str::FromStr for Exchange {
    type Err = ParseExchangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kraken" | "KRAKEN" => Ok(Exchange::Kraken),
            _ => Err(ParseExchangeError),
        }
    }
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exchange::Kraken => write!(f, "kraken"),
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
