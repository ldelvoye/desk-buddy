//! Runtime container that owns scheduler state and exposes a pure API.
use crate::adapters::inbound::scheduler::HydrationScheduler;
use crate::api::CoreApi;
use crate::domain::hydration::ReminderConfig;
use crate::error::CoreResult;
use tokio::sync::{Mutex, MutexGuard};

pub struct CoreRuntime {
    api: CoreApi,
    scheduler: Mutex<HydrationScheduler>,
}

impl CoreRuntime {
    /// Creates a runtime wrapper around a prewired core API.
    pub fn new(api: CoreApi) -> Self {
        Self {
            api,
            scheduler: Mutex::new(HydrationScheduler::new()),
        }
    }

    /// Returns the pure core API for use-case calls.
    pub fn api(&self) -> &CoreApi {
        &self.api
    }

    /// Starts the scheduler using interval configuration stored in settings.
    pub async fn start_with_saved_hydration_config(&self) -> CoreResult<()> {
        let config: ReminderConfig = self.api.hydration_reminder_config().await?;
        self.start(config).await
    }

    /// Starts the scheduler with an explicit reminder configuration.
    pub async fn start(&self, config: ReminderConfig) -> CoreResult<()> {
        let mut scheduler: MutexGuard<'_, HydrationScheduler> = self.scheduler.lock().await;
        scheduler.start(
            self.api.hydration_service(),
            self.api.reminder_sink(),
            config,
        )
    }

    /// Stops the running scheduler task, if any.
    pub async fn stop(&self) -> CoreResult<()> {
        let mut scheduler: MutexGuard<'_, HydrationScheduler> = self.scheduler.lock().await;
        scheduler.stop().await
    }
}
