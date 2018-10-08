//! Configuration
use std::env;

use common::errors::ConfigError;

static COLLECTOR: &str = "SAMMY_COLLECTOR";

#[derive(Debug, Clone)]
pub struct Configuration {
    collector: String,
}

impl Configuration {
    pub fn collector(&self) -> &str {
        self.collector.as_str()
    }
}

pub fn config_from_environment() -> Result<Configuration, ConfigError> {
    let collector = env::var(COLLECTOR).map_err(|e| (COLLECTOR, e))?;

    Ok(Configuration {
        collector,
    })
}
