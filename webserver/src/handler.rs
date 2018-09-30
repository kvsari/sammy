//! Restful handlers
use std::iter;

use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use actix_web::{HttpRequest, HttpResponse, Responder};

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

pub fn ticks_last_24h_10_min_spans(_req: &HttpRequest<State>) -> impl Responder {
    let now: DateTime<Utc> = Utc::now();
    let mut minutes = 1440;

    let mut numbers: Vec<Decimal> = Vec::new();
    
    for _ in 0..144 {
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

        
    }

    let blurb = r##"{"todo":"Finish me."}"##;
    HttpResponse::Ok().body(blurb)
}
