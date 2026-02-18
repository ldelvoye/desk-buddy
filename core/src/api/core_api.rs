//! Pure core API that delegates to application services and ports.
use crate::application::hydration_service::HydrationService;
use crate::application::settings_service::SettingsService;
use crate::domain::hydration::ReminderConfig;
use crate::error::CoreResult;
use crate::ports::reminder_event_sink::ReminderEventSink;
use std::sync::Arc;

pub struct CoreApi {
    hydration_service: HydrationService,
    settings_service: SettingsService,
    reminder_sink: Arc<dyn ReminderEventSink>,
}

impl CoreApi {
    /// Creates a core API from already-wired services and output ports.
    pub fn new(
        hydration_service: HydrationService,
        settings_service: SettingsService,
        reminder_sink: Arc<dyn ReminderEventSink>,
    ) -> Self {
        Self {
            hydration_service,
            settings_service,
            reminder_sink,
        }
    }

    /// Triggers one hydration reminder and emits it through the event sink.
    pub async fn trigger_hydration_reminder_once(&self) -> CoreResult<String> {
        let message: String = self.hydration_service.trigger_reminder().await?;
        self.reminder_sink.emit_hydration_reminder(&message).await?;
        Ok(message)
    }

    /// Returns the number of hydration reminder log entries.
    pub async fn hydration_log_count(&self) -> CoreResult<i64> {
        self.hydration_service.reminder_count().await
    }

    /// Persists hydration interval minutes and returns the normalized value.
    pub async fn set_hydration_interval_minutes(&self, interval_minutes: u64) -> CoreResult<u64> {
        let config: ReminderConfig = self
            .settings_service
            .set_hydration_interval_minutes(interval_minutes)
            .await?;
        Ok(config.interval_minutes)
    }

    /// Reads the effective hydration reminder configuration.
    pub async fn hydration_reminder_config(&self) -> CoreResult<ReminderConfig> {
        self.settings_service.hydration_reminder_config().await
    }

    pub(crate) fn hydration_service(&self) -> HydrationService {
        self.hydration_service.clone()
    }

    pub(crate) fn reminder_sink(&self) -> Arc<dyn ReminderEventSink> {
        self.reminder_sink.clone()
    }
}
