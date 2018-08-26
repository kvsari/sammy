//! Single HTTPS client.
use std::{fmt, error, convert, string};

use hyper::{self, Client, Body, client::HttpConnector};
use hyper_tls::{HttpsConnector, Error};

pub type HttpsClient = Client<HttpsConnector<HttpConnector>>;

pub fn produce(threads: usize) -> Result<HttpsClient, Error> {
    let https = HttpsConnector::new(threads)?;
    Ok(Client::builder().build::<_, Body>(https))
}

#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Utf8(string::FromUtf8Error),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::Http(err) => write!(f, "HTTP fail: {}", &err),
            FetchError::Utf8(err) => write!(f, "Response body invalid UTF8: {}", &err),
        }
    }
}

impl error::Error for FetchError {
    fn description(&self) -> &str {
        "Error occured fetching from exchange."
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            FetchError::Http(ref err) => Some(err),
            FetchError::Utf8(ref err) => Some(err),
        }
    }
}

impl convert::From<hyper::Error> for FetchError {
    fn from(e: hyper::Error) -> Self {
        FetchError::Http(e)
    }
}

impl convert::From<string::FromUtf8Error> for FetchError {
    fn from(e: string::FromUtf8Error) -> Self {
        FetchError::Utf8(e)
    }
}
