//! SQLite schema bootstrap for required core tables.
use crate::error::{CoreError, CoreResult};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Runs versioned SQL migrations for the core SQLite schema.
pub async fn run_migrations(pool: &SqlitePool) -> CoreResult<()> {
    if let Err(err) = MIGRATOR.run(pool).await {
        return Err(CoreError::SqliteMigrate(err));
    }

    Ok(())
}
