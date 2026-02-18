use crate::domain::hydration::DEFAULT_REMINDER_MESSAGE;
use crate::error::CoreResult;
use crate::ports::hydration_log_repository::HydrationLogRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct HydrationService {
    repository: Arc<dyn HydrationLogRepository>,
}

impl HydrationService {
    /// Builds the hydration use-case service with a log repository implementation.
    pub fn new(repository: Arc<dyn HydrationLogRepository>) -> Self {
        Self { repository }
    }

    /// Records one hydration reminder event and returns the emitted message.
    pub async fn trigger_reminder(&self) -> CoreResult<String> {
        self.repository
            .insert_hydration_log(DEFAULT_REMINDER_MESSAGE)
            .await?;
        Ok(DEFAULT_REMINDER_MESSAGE.to_string())
    }

    /// Returns how many hydration reminders have been logged.
    pub async fn reminder_count(&self) -> CoreResult<i64> {
        self.repository.hydration_log_count().await
    }
}
