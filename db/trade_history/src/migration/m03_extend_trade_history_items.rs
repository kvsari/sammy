//! Extend the `trade_history_items` database table to store extra binance data.
use postgres::error::Error as PostgresError;
use postgres::transaction::Transaction;
use schemamama_postgres::PostgresMigration;

pub struct ExtendTradeHistoryItems;

migration!(ExtendTradeHistoryItems, 3, 
