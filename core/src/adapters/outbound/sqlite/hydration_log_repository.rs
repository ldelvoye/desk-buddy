//! SQLite implementation of the hydration log repository port.
use crate::error::CoreResult;
use crate::ports::hydration_log_repository::HydrationLogRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteHydrationLogRepository {
    pool: SqlitePool,
}

impl SqliteHydrationLogRepository {
    /// Creates a SQLite-backed hydration log repository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HydrationLogRepository for SqliteHydrationLogRepository {
    async fn insert_hydration_log(&self, message: &str) -> CoreResult<()> {
        sqlx::query("INSERT INTO hydration_reminder_logs (message) VALUES (?1)")
            .bind(message)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn hydration_log_count(&self) -> CoreResult<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM hydration_reminder_logs")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }
}
