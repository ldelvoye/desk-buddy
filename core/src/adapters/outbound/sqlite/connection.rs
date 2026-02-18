use crate::error::CoreResult;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

/// Creates a SQLite connection pool for the provided database URL.
pub async fn connect_sqlite(database_url: &str) -> CoreResult<SqlitePool> {
    let options: SqliteConnectOptions =
        SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    // One connection keeps sqlite::memory: behavior deterministic for tests.
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;

    Ok(pool)
}
