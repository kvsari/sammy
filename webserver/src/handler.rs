//! Restful handlers
use std::iter;

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

pub fn ticks_last_24h_10_min_spans(req: &HttpRequest<State>) -> impl Responder {
    let mut count = 0;
    let numbers: Vec<String> = iter::repeat_with(move || { count += 1; count })
        .take(144)
        .map(|x| x.to_string())
        .collect();
    
    HttpResponse::Ok().json(numbers)
}
