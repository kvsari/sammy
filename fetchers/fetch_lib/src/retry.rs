//! Retry logic.
//!
//! Sometimes a placement fails. Therefore a few retries are in order. This is for a quick
//! retry and is not a full-fledged reliability system where every message has guaranteed
//! delivery like in AMQP and other protocols. This code tackles the low hanging fruit that
//! trying again will overcome the problem of a momentary transmission error.
//!
//! ## TODO
//! 1. Do something if there are X number of failures. Perhaps notify on a side channel?
use std::time::{Instant, Duration};
use std::fmt;

use futures::{Poll, Async, Future};
use hyper::{self, Uri, Request};
use tokio;
use tokio_timer::{self, Delay};

use https_client::HttpsClient;

enum RetryError {
    Timer(tokio_timer::Error),
    Hyper(hyper::Error),    
}

impl fmt::Display for RetryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RetryError::Timer(ref e) => write!(f, "{}", e),
            RetryError::Hyper(ref e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PutRetry {
    destination: Option<Uri>,
    json_body: Option<String>,
    client: Option<HttpsClient>,
    delay: Duration,
    count: usize,
    limit: usize,
}

impl PutRetry {
    pub fn new(
        destination: Uri,
        json_body: String,
        client: HttpsClient,
        delay: Duration,
        limit: usize,
    ) -> Self {
        PutRetry {
            destination: Some(destination),
            json_body: Some(json_body),
            client: Some(client),
            delay: delay,
            count: 1,
            limit: limit,
        }
    }

    /// Produce a clone of self incrementing the count by 1.
    fn increment(&self) -> Self {
        let mut put_retry = self.clone();
        put_retry.count += 1;
        put_retry
    }

    /// Whether the retries has exceeded the limit.
    fn give_up(&self) -> bool {
        self.count > self.limit
    }
}

impl Future for PutRetry {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.give_up() {
            error!(
                "{} of {} retries attempted. All failed. Giving up.",
                &self.count - 1,
                &self.limit,
            );
            return Err(());
            
            // TODO: Pass the error onto another system? It could be that the collector
            //       is down and can't be restarted so a notification to sysadmin needed.
        }
        
        // Prepare the next retry just in case first before we move data out.
        let retry_again = self.increment();

        // Move data out to be moved into future.
        let count = self.count;
        let limit = self.limit;
        let client = self.client.take().unwrap();
        let destination = self.destination.take().unwrap();
        let json_body = self.json_body.take().unwrap();
        
        // Setup a delay which will retry the connection.
        let retry_fut = Delay::new(Instant::now() + self.delay)
            .map_err(|e| RetryError::Timer(e))
            .and_then(move |()| {
                let req = Request::put(destination)
                    .body(json_body.into())
                    .unwrap();
                client
                    .request(req)
                    .map_err(|e| RetryError::Hyper(e))
            })
            .then(move |result| match result {
                Ok(rsp) => {
                    trace!("Retry placement success: {}", &rsp.status());
                    Ok(())
                },
                Err(e) => {
                    warn!("Retry {} of {} failed: {}", count, limit, &e);
                    tokio::spawn(retry_again);
                    Ok(())
                },
            });

        tokio::spawn(retry_fut);

        Ok(Async::Ready(()))
    }
}
