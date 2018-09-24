//! Apply DB migrations
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate postgres;

extern crate trade_history;

use std::env::var;
use std::{thread, time};

pub fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL env var not set.");
    let wait = time::Duration::from_secs(30);

    let connection = loop {
        info!("Connecting to DB.");
        match postgres::Connection::connect(db_url.clone(), postgres::TlsMode::None) {
            Ok(c) => break c,
            Err(e) => {
                warn!("Failed to connect to DB: {}", &e);
                info!("Retrying in {:?}", &wait);
                thread::sleep(wait);
            }
        }
    };
    
    info!("Setting up migrations.");
    let migrator = trade_history::migration::setup(&connection)
        .expect("Can't confirm migration state.");

    info!("Running migrations (if any outstanding).");
    migrator.up(None).expect("Failed to run migrations.");

    info!("DB migrations confirmed!");
}
