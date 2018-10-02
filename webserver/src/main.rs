#[macro_use] extern crate log;
extern crate dotenv;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate rust_decimal;
extern crate chrono;
extern crate futures;
extern crate bytes;
extern crate serde;
extern crate serde_json;

extern crate common;

use actix_web::{server, App, fs, http::Method, middleware::Logger};

mod config;
mod handler;
mod middle;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let configuration = config::config_from_environment().expect("Can't load config.");
    info!("Starting sammy webserver.");
    debug!("Configuration: {:?}", &configuration);
    
    let system = actix::System::new("sammy webserver");

    let state = handler::State::new(configuration.folder_url());    
    
    server::HttpServer::new(move || {
        App::with_state(state.clone())
            .middleware(Logger::default())
            //.middleware(middle::DebugRequestHeaders)
            .scope("/tick", |scope| {
                scope
                    .resource("", |r| {
                        r.method(Method::GET).f(handler::info)
                    })
                    .resource("/24h_10_min_spans", |r| {
                        r.method(Method::GET).f(handler::ticks_last_24h_10_min_spans)
                        //r.method(Method::GET).f(handler::dummy_ticks_144)
                    })
            })
            .scope("/", |scope| {
                scope
                    .handler("", fs::StaticFiles::new("www").unwrap().show_files_listing())
            })
    })
        .bind(configuration.listen())
        .expect("Can't bind address.")
        .start();    

    system.run();
}
