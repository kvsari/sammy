//! Configuration
use std::{env, net};

use common::errors::ConfigError;

static LISTEN: &str = "COLLECTOR_LISTEN_ADDR";
static DB_URL: &str = "DATABASE_URL";
static DB_CONNS: &str = "DATABASE_CONNECTIONS";

#[derive(Debug)]
pub struct Configuration {
    listen: net::SocketAddr,
    database_url: String,
    database_connections: u8,
}

impl Configuration {
    pub fn listen(&self) -> net::SocketAddr {
        self.listen
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }

    pub fn database_connections(&self) -> u8 {
        self.database_connections
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    let listen = env::var(LISTEN).map_err(|e| (LISTEN, e))?;
    let db_conns = env::var(DB_CONNS).map_err(|e| (DB_CONNS, e))?;
    
    Ok(Configuration {
        listen: listen.parse().map_err(|e| (LISTEN, e))?,
        database_url: env::var(DB_URL).map_err(|e| (DB_URL, e))?,
        database_connections: db_conns.parse().map_err(|e|(DB_CONNS, e))?,
    })
}

