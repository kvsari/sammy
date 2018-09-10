//! Error type
use std::{fmt, convert, error};

use diesel::result;
use rust_decimal;
use bigdecimal;

use common::{asset, exchange};

#[derive(Debug)]
pub enum Error {
    Connect(result::ConnectionError),
    Sql(result::Error),
    Exchange(exchange::ParseExchangeError),
    AssetPair(asset::ParseAssetError),
    Convert(String),
    Decimal(bigdecimal::ParseBigDecimalError),
    Numeric(rust_decimal::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Connect(ref err) => write!(f, "Connect: {}", &err),
            Error::Sql(ref err) => write!(f, "SQL: {}", &err),
            Error::Exchange(ref err) => write!(f, "Exchange parse: {}", &err),
            Error::AssetPair(ref err) => write!(f, "Asset pair parse: {}", &err),
            Error::Convert(ref err) => {
                write!(f, "Can't convert before DB OP: {}", &err)
            },
            Error::Decimal(ref err) => write!(f, "Bad decimal: {}", &err),
            Error::Numeric(ref err) => {
                write!(f, "Can't convert into decimal from DB: {}", &err)
            },
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Error with Ticks DB CRUD."
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Connect(ref err) => Some(err),
            Error::Sql(ref err) => Some(err),
            Error::Exchange(ref err) => Some(err),
            Error::AssetPair(ref err) => Some(err),
            Error::Decimal(ref err) => Some(err),
            Error::Convert(_) => None,
            Error::Numeric(ref err) => Some(err),
        }
    }
}

impl convert::From<result::ConnectionError> for Error {
    fn from(ce: ConnectionError) -> Self {
        Error::Connect(ce)
    }
}

impl convert::From<result::Error> for Error {
    fn from(re: result::Error) -> Self {
        Error::Sql(re)
    }
}

impl convert::From<exchange::ParseExchangeError> for Error {
    fn from(p: ParseExchangeError) -> Self {
        Error::Exchange(p)
    }
}

impl convert::From<asset::ParseAssetError> for Error {
    fn from(p: asset::ParseAssetError) -> Self {
        Error::AssetPair(p)
    }
}

impl convert::From<String> for Error {
    fn from(s: String) -> Self {
        Error::Convert(s)
    }
}

impl<'a> convert::From<&'a str> for Error {
    fn from(s: &str) -> Self {
        Error::Convert(s.to_owned())
    }
}

impl convert::From<bigdecimal::ParseBigDecimalError> for Error {
    fn from(p: ParseBigDecimalError) -> Self {
        Error::Decimal(p)
    }
}

impl convert::From<rust_decimal::Error> for Error {
    fn from(e: rust_decimal::Error) -> Self {
        Error::Numeric(e)
    }
}
