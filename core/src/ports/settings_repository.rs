//! Output port for reading and writing persisted settings.
use crate::error::CoreResult;
use async_trait::async_trait;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// Saves the hydration interval in minutes.
    async fn set_hydration_interval_minutes(&self, interval_minutes: u64) -> CoreResult<()>;
    /// Loads the hydration interval in minutes, if configured.
    async fn hydration_interval_minutes(&self) -> CoreResult<Option<u64>>;
    /// Saves hydration snooze minutes.
    async fn set_hydration_snooze_minutes(&self, snooze_minutes: u64) -> CoreResult<()>;
    /// Loads hydration snooze minutes, if configured.
    async fn hydration_snooze_minutes(&self) -> CoreResult<Option<u64>>;
}
