//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate actix;
extern crate actix_web;

use actix_web::{server::HttpServer, App, HttpRequest};

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

    HttpServer::new(|| {
        App::new()
            .resource("/", |r| r.f(yo))
    })
        .bind(config.listen())
        .expect("Can't bind address.")
        .start();
    
    system.run();
}
