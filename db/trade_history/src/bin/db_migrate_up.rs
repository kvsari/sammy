//! Apply DB migrations
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate postgres;

extern crate trade_history;

use std::env::var;

pub fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL env var not set.");

    info!("Connecting to DB.");
    let connection = postgres::Connection::connect(db_url, postgres::TlsMode::None)
        .expect("Can't connect to database.");
    
    info!("Setting up migrations.");
    let migrator = trade_history::migration::setup(&connection)
        .expect("Can't confirm migration state.");

    info!("Running migrations (if any outstanding).");
    migrator.up(None).expect("Failed to run migrations.");

    info!("DB migrations confirmed!");
}
