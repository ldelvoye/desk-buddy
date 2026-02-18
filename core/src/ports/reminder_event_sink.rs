//! Output port for emitting reminder events to external channels.
use crate::error::CoreResult;
use async_trait::async_trait;

#[async_trait]
pub trait ReminderEventSink: Send + Sync {
    /// Emits a hydration reminder event to the configured output channel.
    async fn emit_hydration_reminder(&self, message: &str) -> CoreResult<()>;
}
