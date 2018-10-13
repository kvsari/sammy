//! Apply and reverse DB migrations.
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate postgres;
extern crate clap;

extern crate trade_history;

use std::env::var;
use std::{thread, time};

use clap::{Arg, App};

pub fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = var("DATABASE_URL").expect("DATABASE_URL env var not set.");

    let matches = App::new("Trade History Migrator")
        .version("0.1.0")
        .author("Stephan Luther <kvsari@protonmail.com>")
        .arg(Arg::with_name("apply")
             .short("a")
             .long("apply")
             .help("Apply migrations. Default is to apply when param is absent")
             .takes_value(false))
        .arg(Arg::with_name("reverse")
             .short("r")
             .long("reverse")
             .help("Reverse migrations")
             .takes_value(false))
        .arg(Arg::with_name("migration")
             .short("m")
             .long("migration")
             .help("Apply/Reverse to migration number. When absent, goes to the highest or lowest depending on whether it was direction.")
             .takes_value(true))
        .get_matches();

    let apply = matches.is_present("apply");
    let reverse = matches.is_present("reverse");
    let migration: Option<i64> = matches.value_of("migration")
        .map(|m| m.parse().expect("Can't parse into number."));

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

    match (apply, reverse) {
        (false, false) | (true, false) => {
            info!("Applying migrations.");
            migrator.up(migration).expect("Failed to run migrations.");
        },
        (false, true) | (true, true) => {
            info!("Reversing migrations.");
            migrator.down(migration).expect("Failed to reverse migrations.");
        },
    }

    info!("DB migrations confirmed!");
}
