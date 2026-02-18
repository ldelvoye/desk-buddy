use crate::error::CoreResult;
use async_trait::async_trait;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// Saves the hydration interval in minutes.
    async fn set_hydration_interval_minutes(&self, interval_minutes: u64) -> CoreResult<()>;
    /// Loads the hydration interval in minutes, if configured.
    async fn hydration_interval_minutes(&self) -> CoreResult<Option<u64>>;
}
