use crate::application::hydration_service::HydrationService;
use crate::domain::hydration::ReminderConfig;
use crate::error::{CoreError, CoreResult};
use crate::ports::reminder_event_sink::ReminderEventSink;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use tokio::task::JoinHandle;

pub struct HydrationScheduler {
    shutdown_tx: Option<watch::Sender<bool>>,
    join_handle: Option<JoinHandle<CoreResult<()>>>,
}

impl HydrationScheduler {
    /// Creates an idle hydration scheduler.
    pub fn new() -> Self {
        Self {
            shutdown_tx: None,
            join_handle: None,
        }
    }

    /// Returns true when a scheduler task is currently active.
    pub fn is_running(&self) -> bool {
        self.join_handle.is_some()
    }

    /// Starts the background reminder loop using the provided config.
    pub fn start(
        &mut self,
        service: HydrationService,
        event_sink: Arc<dyn ReminderEventSink>,
        config: ReminderConfig,
    ) -> CoreResult<()> {
        if self.is_running() {
            return Err(CoreError::SchedulerAlreadyRunning);
        }

        let (shutdown_tx, mut shutdown_rx): (watch::Sender<bool>, watch::Receiver<bool>) =
            watch::channel(false);
        let interval: Duration = config.interval_duration();

        let join_handle: JoinHandle<CoreResult<()>> = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(interval) => {
                        let message: String = service.trigger_reminder().await?;
                        event_sink.emit_hydration_reminder(&message).await?;
                    }
                    changed = shutdown_rx.changed() => {
                        if changed.is_err() || *shutdown_rx.borrow() {
                            break;
                        }
                    }
                }
            }

            Ok(())
        });

        self.shutdown_tx = Some(shutdown_tx);
        self.join_handle = Some(join_handle);

        Ok(())
    }

    /// Stops the background reminder loop and waits for task shutdown.
    pub async fn stop(&mut self) -> CoreResult<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result: Result<(), tokio::sync::watch::error::SendError<bool>> =
                shutdown_tx.send(true);
        }

        if let Some(join_handle) = self.join_handle.take() {
            let result: CoreResult<()> = join_handle.await?;
            result?;
        }

        Ok(())
    }
}

impl Default for HydrationScheduler {
    fn default() -> Self {
        Self::new()
    }
}
