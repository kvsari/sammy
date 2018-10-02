//! Middlewares

use actix_web::{error, HttpRequest, HttpResponse};
use actix_web::middleware::{Middleware, Started, Response};

pub struct DebugRequestHeaders;

impl<S> Middleware<S> for DebugRequestHeaders {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, error::Error> {
        let path = req.uri().path();
        let headers = req.request().headers();

        debug!("REQUEST HEADERS for path {}", path);        
        headers
            .iter()
            .for_each(|(key, value)| {
                debug!("{} = {:?}", key, value);
            });

        match path {
            "/tick/24h_10_min_spans" => {
                debug!("Requesting ticks!");
            },
            _ => (),
        }
        
        Ok(Started::Done)
    }

    fn response(
        &self, req: &HttpRequest<S>, resp: HttpResponse
    ) -> Result<Response, error::Error> {
        let path = req.uri().path();

        {
            let headers = resp.headers();

            debug!("RESPONSE HEADERS for path {}", path);        
            headers
                .iter()
                .for_each(|(key, value)| {
                    debug!("{} = {:?}", key, value);
                });
        }

        Ok(Response::Done(resp))
    }
}
