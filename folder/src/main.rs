//! Entrypoint
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;

mod config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = config::config_from_environment().expect("Can't load configuration.");
    debug!("{:?}", &config);

    
}
