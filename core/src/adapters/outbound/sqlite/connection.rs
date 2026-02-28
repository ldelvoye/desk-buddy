//! SQLite connection factory for creating the shared core connection pool.
use crate::error::CoreResult;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::Duration;

/// Creates a SQLite connection pool for the provided database URL.
pub async fn connect_sqlite(database_url: &str) -> CoreResult<SqlitePool> {
    let options: SqliteConnectOptions =
        SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_secs(5))
            .foreign_keys(true)
            .synchronous(SqliteSynchronous::Normal);

    // One connection keeps sqlite::memory: behavior deterministic for tests.
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;

    Ok(pool)
}
