//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate actix;
extern crate actix_web;

extern crate translator_lib as lib;

use actix_web::{server::HttpServer, App, HttpRequest, http::Method};

mod config;

fn yo(_req: &HttpRequest) -> &'static str {
    "Yo"
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load configuration.");
    debug!("{:?}", &config);

    let system = actix::System::new("Translator");

    let rest_state = lib::restful::State;

    HttpServer::new(move || {
        App::with_state(rest_state)
            //.resource("/", |r| r.f(yo))
            .scope("/trade_history", |scope| {
                scope
                    .resource("", |r| {
                        r.method(Method::GET).f(lib::restful::trade_match_root)
                    })
            })
    })
        .bind(config.listen())
        .expect("Can't bind address.")
        .start();
    
    system.run();
}
