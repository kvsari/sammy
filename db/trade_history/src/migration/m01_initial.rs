//! The first migration for the system.
use postgres::error::Error as PostgresError;
use postgres::transaction::Transaction;
use schemamama_postgres::PostgresMigration;

pub struct CreateExchanges;

migration!(CreateExchanges, 1, "Create the exchanges table.");

impl PostgresMigration for CreateExchanges {
    fn up(&self, transaction: &Transaction) -> Result<(), PostgresError> {
        transaction.batch_execute(
            "CREATE TABLE exchanges ( \
             id SERIAL NOT NULL PRIMARY KEY, \
             label VARCHAR(32) NOT NULL \
             ); \
             \
             INSERT INTO exchanges ( label ) VALUES ( 'kraken' );",
        )
    }

    fn down(&self, transaction: &Transaction) -> Result<(), PostgresError> {
        transaction.execute("DROP TABLE exchanges;", &[]).map(|_| ())
    }
}

