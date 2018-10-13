//! Postgres migrations using [schemamama](https://crates.io/crates/schemamama).
//!
//! This code replaces the `diesel` migration stuff since it's not good for dev/ops. With
//! a migration binary being possible with `schemamama`, [init containers](https://kubernetes.io/docs/concepts/workloads/pods/init-containers/) are easy
//! to build and use to setup DB's.
//!
//! Also, since postgres will be the DB of choice, what reason to use an ORM? This is doubly
//! so because we are still wrapping the ORM methods anyway in a `crud` struct.

use postgres::Connection;
use schemamama::Migrator;
use schemamama_postgres::PostgresAdapter;

use error::Error;

mod m01_initial;
mod m02_add_binance;
mod m03_extend_trade_history_items;

/// Prepare all migrations to be run returning the migrator.
pub fn setup<'a>(
    connection: &'a Connection
) -> Result<Migrator<PostgresAdapter<'a>>, Error> {
    let adapter = PostgresAdapter::new(connection);
    adapter.setup_schema()?;

    let mut migrator = Migrator::new(adapter);

    // Load in all the migrations. Any new migrations must be added in here.
    migrator.register(Box::new(m01_initial::CreateInitial));
    migrator.register(Box::new(m02_add_binance::MakeAdditions));
    migrator.register(Box::new(m03_extend_trade_history_items::ExtendTradeHistoryItems));

    Ok(migrator)
}
