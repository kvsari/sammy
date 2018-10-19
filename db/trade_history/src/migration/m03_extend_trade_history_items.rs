//! Extend the `trade_history_items` database table to store extra binance data.
//!
//! ## Warning
//! This migration contains a one-way column change to allow null values. This is the
//! `trade` column. This the deployment cannot be reversed to a prior version of the
//! source code if there's a screwup.
use postgres::error::Error as PostgresError;
use postgres::transaction::Transaction;
use schemamama_postgres::PostgresMigration;

pub struct ExtendTradeHistoryItems;

migration!(ExtendTradeHistoryItems, 3, "Extend trade_history_items.");

impl PostgresMigration for ExtendTradeHistoryItems {
    fn up(&self, transaction: &Transaction) -> Result<(), PostgresError> {
        transaction.batch_execute(
            "ALTER TABLE IF EXISTS trade_history_items \
             ALTER COLUMN trade DROP NOT NULL, \
             ADD COLUMN IF NOT EXISTS match_id BIGINT DEFAULT NULL, \
             ADD COLUMN IF NOT EXISTS buy_order_id BIGINT DEFAULT NULL, \
             ADD COLUMN IF NOT EXISTS sell_order_id BIGINT DEFAULT NULL, \
             ADD COLUMN IF NOT EXISTS match_ts TIMESTAMP WITH TIME ZONE DEFAULT NULL;"
        )
    }
    
    fn down(&self, transaction: &Transaction) -> Result<(), PostgresError> {
        transaction.batch_execute(
            "ALTER TABLE IF EXISTS trade_history_items \
             DROP COLUMN IF EXISTS match_id CASCADE, \
             DROP COLUMN IF EXISTS buy_order_id CASCADE, \
             DROP COLUMN IF EXISTS sell_order_id CASCADE, \
             DROP COLUMN IF EXISTS match_ts CASCADE;"
        )
    }
}
