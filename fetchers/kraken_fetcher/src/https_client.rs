//! Single HTTPS client.
use std::{fmt, error, convert, string};

use serde_json;
use hyper::{self, Client, Body, client::HttpConnector};
use hyper_tls::{HttpsConnector, Error};

pub type HttpsClient = Client<HttpsConnector<HttpConnector>>;

pub fn produce(threads: usize) -> Result<HttpsClient, Error> {
    let https = HttpsConnector::new(threads)?;
    Ok(Client::builder().build::<_, Body>(https))
}

#[derive(Debug)]
pub enum FetchError {
    Client(hyper::Error),
    Status(u16),
    Utf8(string::FromUtf8Error),
    SerdeJson(serde_json::Error),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::Client(err) => write!(f, "Client fail: {}", &err),
            FetchError::Status(err) => write!(f, "HTTP status code: {}", &err),
            FetchError::Utf8(err) => write!(f, "Response body invalid UTF8: {}", &err),
            FetchError::SerdeJson(err) => write!(f, "JSON serde error: {}", &err),
        }
    }
}

impl error::Error for FetchError {
    fn description(&self) -> &str {
        "Error occured fetching from exchange."
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            FetchError::Client(ref err) => Some(err),
            FetchError::Status(_) => None,
            FetchError::Utf8(ref err) => Some(err),
            FetchError::SerdeJson(ref err) => Some(err),
        }
    }
}

impl convert::From<hyper::Error> for FetchError {
    fn from(e: hyper::Error) -> Self {
        FetchError::Client(e)
    }
}

impl convert::From<string::FromUtf8Error> for FetchError {
    fn from(e: string::FromUtf8Error) -> Self {
        FetchError::Utf8(e)
    }
}

impl convert::From<u16> for FetchError {
    fn from(e: u16) -> Self {
        FetchError::Status(e)
    }
}

impl convert::From<serde_json::Error> for FetchError {
    fn from(e: serde_json::Error) -> Self {
        FetchError::SerdeJson(e)
    }
}
