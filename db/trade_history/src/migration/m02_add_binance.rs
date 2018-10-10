//! Second migration. Adding Binance and several more asset pairs.
use postgres::error::Error as PostgresError;
use postgres::transaction::Transaction;
use schemamama_postgres::PostgresMigration;

pub struct MakeAdditions;

migration!(MakeAdditions, 2, "Adding binance and asset pairs.");

impl PostgresMigration for MakeAdditions {
    fn up(&self, transaction: &Transaction) -> Result<(), PostgresError> {
        transaction.batch_execute(
            "INSERT INTO exchanges ( label ) VALUES ( 'binance' ); \
             \
             INSERT INTO asset_pairs ( left_side, right_side, pair ) \
             VALUES ( 'ETH', 'USD', 'ETH/USD' ); \
             \
             INSERT INTO asset_pairs ( left_side, right_side, pair ) \
             VALUES ( 'BNB', 'USD', 'BNB/USD' ); \
             \
             INSERT INTO asset_pairs ( left_side, right_side, pair ) \
             VALUES ( 'BNB', 'BTC', 'BNB/BTC' ); \
             \
             INSERT INTO asset_pairs ( left_side, right_side, pair ) \
             VALUES ( 'ETH', 'BTC', 'ETH/BTC' ); \
             \
             INSERT INTO asset_pairs ( left_side, right_side, pair ) \
             VALUES ( 'BNB', 'ETH', 'BNB/ETH' ); \
             \
             INSERT INTO asset_pairs ( left_side, right_side, pair ) \
             VALUES ( 'BNB', 'USD', 'BNB/USD' );"
        )
    }

    fn down(&self, transaction: &Transaction) -> Result<(), PostgresError> {
        transaction.batch_execute(
            "DELETE FROM trade_history_items \
             WHERE exchange = (SELECT id FROM exchanges WHERE label = 'binance'); \
             \
             DELETE FROM trade_history_items \
             WHERE asset_pair IN (SELECT id FROM asset_pairs WHERE pair IN \
             ( 'ETH/USD', 'BNB/USD', 'BNB/BTC', 'ETH/BTC', 'BNB/ETH', 'BNB/USD' )); \
             \
             DELETE FROM exchanges WHERE label = 'binance'; \
             \
             DELETE FROM asset_pairs WHERE pair IN \
             ( 'ETH/USD', 'BNB/USD', 'BNB/BTC', 'ETH/BTC', 'BNB/ETH', 'BNB/USD' );"
        )
    }
}
