use crate::error::CoreResult;
use crate::ports::settings_repository::SettingsRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteSettingsRepository {
    pool: SqlitePool,
}

impl SqliteSettingsRepository {
    /// Creates a SQLite-backed settings repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
    async fn set_hydration_interval_minutes(&self, interval_minutes: u64) -> CoreResult<()> {
        sqlx::query(
            r#"
            INSERT INTO hydration_settings (id, interval_minutes)
            VALUES (1, ?1)
            ON CONFLICT(id)
            DO UPDATE SET interval_minutes = excluded.interval_minutes
            "#,
        )
        .bind(i64::try_from(interval_minutes).unwrap_or(i64::MAX))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn hydration_interval_minutes(&self) -> CoreResult<Option<u64>> {
        let maybe_interval: Option<i64> = sqlx::query_scalar::<_, i64>(
            "SELECT interval_minutes FROM hydration_settings WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        // Treat invalid persisted values as absent so callers fall back to defaults.
        let interval: Option<u64> = maybe_interval
            .and_then(|raw: i64| u64::try_from(raw).ok())
            .map(|value: u64| value.max(1));

        Ok(interval)
    }
}
