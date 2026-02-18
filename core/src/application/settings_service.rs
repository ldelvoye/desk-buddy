use crate::domain::hydration::ReminderConfig;
use crate::error::CoreResult;
use crate::ports::settings_repository::SettingsRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct SettingsService {
    repository: Arc<dyn SettingsRepository>,
}

impl SettingsService {
    /// Builds the settings use-case service with a settings repository.
    pub fn new(repository: Arc<dyn SettingsRepository>) -> Self {
        Self { repository }
    }

    /// Persists hydration interval minutes and returns the normalized config.
    pub async fn set_hydration_interval_minutes(&self, interval_minutes: u64) -> CoreResult<ReminderConfig> {
        let config: ReminderConfig = ReminderConfig::new(interval_minutes);
        self.repository
            .set_hydration_interval_minutes(config.interval_minutes)
            .await?;
        Ok(config)
    }

    /// Loads hydration reminder config from storage or falls back to defaults.
    pub async fn hydration_reminder_config(&self) -> CoreResult<ReminderConfig> {
        let interval: u64 = self
            .repository
            .hydration_interval_minutes()
            .await?
            .unwrap_or(ReminderConfig::default().interval_minutes);

        Ok(ReminderConfig::new(interval))
    }
}
