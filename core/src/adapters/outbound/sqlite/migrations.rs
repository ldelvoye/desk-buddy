//! SQLite schema bootstrap for required core tables.
use crate::domain::hydration::ReminderConfig;
use crate::error::CoreResult;
use sqlx::SqlitePool;

/// Creates required SQLite tables for the current core schema.
pub async fn run_migrations(pool: &SqlitePool) -> CoreResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS hydration_reminder_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            message TEXT NOT NULL,
            triggered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS hydration_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            interval_minutes INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Seed a singleton default row so settings are visible in DB tools
    // and runtime reads have a concrete persisted value.
    let default_interval: i64 =
        i64::try_from(ReminderConfig::default().interval_minutes).unwrap_or(i64::MAX);
    sqlx::query(
        r#"
        INSERT INTO hydration_settings (id, interval_minutes)
        VALUES (1, ?1)
        ON CONFLICT(id) DO NOTHING
        "#,
    )
    .bind(default_interval)
    .execute(pool)
    .await?;

    Ok(())
}
