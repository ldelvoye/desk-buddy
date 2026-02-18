use crate::adapters::inbound::scheduler::HydrationScheduler;
use crate::adapters::outbound::events::StdoutReminderSink;
use crate::adapters::outbound::sqlite::{
    connect_sqlite, run_migrations, SqliteHydrationLogRepository, SqliteSettingsRepository,
};
use crate::application::hydration_service::HydrationService;
use crate::application::settings_service::SettingsService;
use crate::domain::hydration::ReminderConfig;
use crate::error::CoreResult;
use crate::ports::reminder_event_sink::ReminderEventSink;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct CoreOptions {
    pub database_url: String,
}

impl CoreOptions {
    /// Returns options configured for an in-memory SQLite database.
    pub fn in_memory() -> Self {
        Self {
            database_url: "sqlite::memory:".to_string(),
        }
    }
}

impl Default for CoreOptions {
    fn default() -> Self {
        Self {
            database_url: "sqlite://desk-buddy.db".to_string(),
        }
    }
}

pub struct CoreFacade {
    hydration_service: HydrationService,
    settings_service: SettingsService,
    scheduler: Mutex<HydrationScheduler>,
    reminder_sink: Arc<dyn ReminderEventSink>,
}

impl CoreFacade {
    /// Builds the core facade with SQLite, migrations, and default adapters.
    pub async fn new(options: CoreOptions) -> CoreResult<Self> {
        let pool: SqlitePool = connect_sqlite(&options.database_url).await?;
        run_migrations(&pool).await?;

        let hydration_repository: Arc<SqliteHydrationLogRepository> =
            Arc::new(SqliteHydrationLogRepository::new(pool.clone()));
        let settings_repository: Arc<SqliteSettingsRepository> =
            Arc::new(SqliteSettingsRepository::new(pool));

        let hydration_service: HydrationService = HydrationService::new(hydration_repository);
        let settings_service: SettingsService = SettingsService::new(settings_repository);
        let reminder_sink: Arc<dyn ReminderEventSink> = Arc::new(StdoutReminderSink);

        Ok(Self {
            hydration_service,
            settings_service,
            scheduler: Mutex::new(HydrationScheduler::new()),
            reminder_sink,
        })
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

    /// Starts the scheduler using interval configuration stored in settings.
    pub async fn start_with_saved_hydration_config(&self) -> CoreResult<()> {
        let config: ReminderConfig = self.settings_service.hydration_reminder_config().await?;
        self.start(config).await
    }

    /// Starts the scheduler with an explicit reminder configuration.
    pub async fn start(&self, config: ReminderConfig) -> CoreResult<()> {
        let mut scheduler: MutexGuard<'_, HydrationScheduler> = self.scheduler.lock().await;
        scheduler.start(
            self.hydration_service.clone(),
            self.reminder_sink.clone(),
            config,
        )
    }

    /// Stops the running scheduler task, if any.
    pub async fn stop(&self) -> CoreResult<()> {
        let mut scheduler: MutexGuard<'_, HydrationScheduler> = self.scheduler.lock().await;
        scheduler.stop().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn trigger_once_writes_to_sqlite() -> CoreResult<()> {
        let core: CoreFacade = CoreFacade::new(CoreOptions::in_memory()).await?;

        let _: String = core.trigger_hydration_reminder_once().await?;

        assert_eq!(core.hydration_log_count().await?, 1);
        Ok(())
    }

    #[tokio::test]
    async fn hydration_interval_defaults_then_round_trips() -> CoreResult<()> {
        let core: CoreFacade = CoreFacade::new(CoreOptions::in_memory()).await?;

        assert_eq!(
            core.hydration_reminder_config().await?.interval_minutes,
            ReminderConfig::default().interval_minutes
        );

        let persisted: u64 = core.set_hydration_interval_minutes(0).await?;
        assert_eq!(persisted, 1);
        assert_eq!(core.hydration_reminder_config().await?.interval_minutes, 1);
        Ok(())
    }

    #[tokio::test]
    async fn scheduler_uses_saved_config_smoke_test() -> CoreResult<()> {
        let core: CoreFacade = CoreFacade::new(CoreOptions::in_memory()).await?;

        let _: u64 = core.set_hydration_interval_minutes(1).await?;
        core.start_with_saved_hydration_config().await?;
        core.stop().await?;

        Ok(())
    }
}
