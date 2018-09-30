//! Restful handlers
use std::iter;

use futures::{Future, Stream, future};
use futures::future::{ok, lazy, result, FutureResult};
use futures::stream::iter_ok;
use bytes::Bytes;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use actix::Arbiter;
use actix_web::{HttpRequest, HttpResponse, Responder, client, HttpMessage};
use serde_json;

use common::tick::Tick;

#[derive(Debug, Clone)]
pub struct State {
    folder_url: String,
}

impl State {
    pub fn new(folder_url: &str) -> Self {
        State {
            folder_url: folder_url.to_owned(),
        }
    }
}

pub fn info(_req: &HttpRequest<State>) -> impl Responder {
    let blurb = r##"{"into":"Emit ticks."}"##;
    HttpResponse::Ok().body(blurb)
}

pub fn dummy_ticks_144(_req: &HttpRequest<State>) -> impl Responder {
    let mut count = 0;
    let numbers: Vec<String> = iter::repeat_with(move || { count += 1; count })
        .take(144)
        .map(|x| x.to_string())
        .collect();
    
    HttpResponse::Ok().json(numbers)
}

pub fn ticks_last_24h_10_min_spans(req: &HttpRequest<State>) -> impl Responder {
    let now: DateTime<Utc> = Utc::now();
    let mut minutes = 1440;

    let state = req.state();
    let folder_url = state.folder_url.clone();

    println!("Base URL: {}", &folder_url);

    /*
    let full_future = lazy(|| -> FutureResult<(), ()> {
        result::<(), ()>(Ok(()))
    });

    let mut full_future = Box::new(full_future);
     */

    let mut fold_futures = Vec::new();
    
    //for _ in 0..144 {
    for _ in 0..5 {
        let subtract = Duration::minutes(minutes);
        let start = now - subtract;
        minutes -= 10;
        if minutes <= 0 {
            break;
        } 
        let subtract = Duration::minutes(minutes);
        let end = now - subtract;

        let start_ts = start.timestamp();
        let end_ts = end.timestamp();

        let url = format!(
            "{}/trade_history/btc/usd?from={}&to={}", &folder_url, &start_ts, &end_ts
        );
        println!("URL: {}", &url);

        let fold_fut = client::get(url.as_str())
            .finish()
            .unwrap()
            .send()
            .map_err(|_| ())
            .and_then(|response| {
                //println!("Response: {:?}", &response);
                response.body().map_err(|_| ())
            })
            .and_then(|bytes: Bytes| {
                println!("Response Body: {:?}", &bytes);
                let tick: Tick = serde_json::from_slice(&bytes).map_err(|_| ())?;
                Ok(tick)
            });

        fold_futures.push(fold_fut);
    }

    iter_ok(fold_futures.into_iter())
        .and_then(|fold_fut| fold_fut)
        .fold(vec![], |numbers, tick| {
            println!("Tick: {:?}", &tick);
            numbers.push(*tick.high());
            ok(numbers)
        })
        .and_then(|numbers| {
            Ok(HttpResponse::Ok().json(numbers))
        })
        .responder()

/*        
    let blurb = r##"{"todo":"Finish me."}"##;
    HttpResponse::Ok().body(blurb)
*/
}
