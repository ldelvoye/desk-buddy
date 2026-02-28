//! SQLite schema bootstrap for required core tables.
use crate::error::{CoreError, CoreResult};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;
use std::collections::HashSet;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Runs versioned SQL migrations for the core SQLite schema.
pub async fn run_migrations(pool: &SqlitePool) -> CoreResult<()> {
    let applied_versions: HashSet<i64> = load_applied_versions(pool).await?;
    let pending_migrations: Vec<String> = MIGRATOR
        .iter()
        .filter(|migration| !applied_versions.contains(&migration.version))
        .map(|migration| {
            let slug: String = migration.description.replace(' ', "_");
            format!("{:04}_{slug}", migration.version)
        })
        .collect();

    if !pending_migrations.is_empty() {
        println!("Applying migrations:");
        for migration in pending_migrations {
            println!("> {migration}");
        }
    }

    if let Err(err) = MIGRATOR.run(pool).await {
        return Err(CoreError::SqliteMigrate(err));
    }

    Ok(())
}

async fn load_applied_versions(pool: &SqlitePool) -> CoreResult<HashSet<i64>> {
    // SQLx creates this metadata table after the first successful migration run.
    let rows: Vec<i64> = sqlx::query_scalar("SELECT version FROM _sqlx_migrations")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    Ok(rows.into_iter().collect())
}
