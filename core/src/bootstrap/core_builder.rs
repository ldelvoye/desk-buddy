//! Composition root that wires concrete adapters into runtime + API objects.
use crate::adapters::outbound::events::StdoutReminderSink;
use crate::adapters::outbound::sqlite::{
    connect_sqlite, run_migrations, SqliteHydrationLogRepository, SqliteSettingsRepository,
};
use crate::api::CoreApi;
use crate::application::hydration_service::HydrationService;
use crate::application::settings_service::SettingsService;
use crate::bootstrap::core_runtime::CoreRuntime;
use crate::error::CoreResult;
use crate::ports::reminder_event_sink::ReminderEventSink;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::{env, path::PathBuf};

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
            database_url: default_database_url(),
        }
    }
}

pub struct CoreBuilder {
    options: CoreOptions,
}

fn default_database_url() -> String {
    // Use a stable per-user app-data location for packaged desktop behavior.
    let base_dir: PathBuf = env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let db_path: PathBuf = base_dir.join("DeskBuddy").join("desk-buddy.db");
    let normalized_path: String = db_path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{normalized_path}")
}

impl CoreBuilder {
    /// Creates a builder that will construct a wired core runtime.
    pub fn new(options: CoreOptions) -> Self {
        Self { options }
    }

    /// Builds the runtime by wiring adapters, services, and the core API.
    pub async fn build(self) -> CoreResult<CoreRuntime> {
        let pool: SqlitePool = connect_sqlite(&self.options.database_url).await?;
        run_migrations(&pool).await?;

        let hydration_repository: Arc<SqliteHydrationLogRepository> =
            Arc::new(SqliteHydrationLogRepository::new(pool.clone()));
        let settings_repository: Arc<SqliteSettingsRepository> =
            Arc::new(SqliteSettingsRepository::new(pool));
        let reminder_sink: Arc<dyn ReminderEventSink> = Arc::new(StdoutReminderSink);

        let hydration_service: HydrationService = HydrationService::new(hydration_repository);
        let settings_service: SettingsService = SettingsService::new(settings_repository);
        let api: CoreApi = CoreApi::new(hydration_service, settings_service, reminder_sink);

        Ok(CoreRuntime::new(api))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::hydration::ReminderConfig;

    #[tokio::test]
    async fn trigger_once_writes_to_sqlite() -> CoreResult<()> {
        let runtime: CoreRuntime = CoreBuilder::new(CoreOptions::in_memory()).build().await?;

        let _: String = runtime.api().trigger_hydration_reminder_once().await?;

        assert_eq!(runtime.api().hydration_log_count().await?, 1);
        Ok(())
    }

    #[tokio::test]
    async fn hydration_interval_defaults_then_round_trips() -> CoreResult<()> {
        let runtime: CoreRuntime = CoreBuilder::new(CoreOptions::in_memory()).build().await?;

        assert_eq!(
            runtime.api().hydration_reminder_config().await?.interval_minutes,
            ReminderConfig::default().interval_minutes
        );

        let persisted: u64 = runtime.api().set_hydration_interval_minutes(0).await?;
        assert_eq!(persisted, 1);
        assert_eq!(runtime.api().hydration_reminder_config().await?.interval_minutes, 1);
        Ok(())
    }

    #[tokio::test]
    async fn scheduler_uses_saved_config_smoke_test() -> CoreResult<()> {
        let runtime: CoreRuntime = CoreBuilder::new(CoreOptions::in_memory()).build().await?;

        let _: u64 = runtime.api().set_hydration_interval_minutes(1).await?;
        runtime.start_with_saved_hydration_config().await?;
        runtime.stop().await?;

        Ok(())
    }
}
