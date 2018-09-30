#[macro_use] extern crate log;
extern crate dotenv;
extern crate env_logger;
extern crate actix;
extern crate actix_web;

use actix_web::{server, App, http::Method};

mod config;
mod handler;

fn main() {
    dotenv::dotenv().ok();
    let configuration = config::config_from_environment().expect("Can't load config.");
    info!("Starting sammy webserver.");
    debug!("Configuration: {:?}", &configuration);
    
    let system = actix::System::new("sammy webserver");

    let state = handler::State::new(configuration.folder_url());
    
    server::HttpServer::new(move || {
        App::with_state(state.clone())
            .scope("/tick", |scope| {
                scope
                    .resource("", |r| {
                        r.method(Method::GET).f(handler::info)
                    })
                    .resource("/24h_10_min_spans", |r| {
                        r.method(Method::GET).f(handler::ticks_last_24h_10_min_spans)
                    })
            })
    })
        .bind(configuration.listen())
        .expect("Can't bind address.")
        .start();    

    system.run();
}
