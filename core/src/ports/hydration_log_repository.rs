//! Output port for persisting and querying hydration reminder logs.
use crate::error::CoreResult;
use async_trait::async_trait;

#[async_trait]
pub trait HydrationLogRepository: Send + Sync {
    /// Persists a hydration reminder log entry.
    async fn insert_hydration_log(&self, message: &str) -> CoreResult<()>;
    /// Returns the number of hydration reminder log entries.
    async fn hydration_log_count(&self) -> CoreResult<i64>;
}
