use crate::error::CoreResult;
use crate::ports::reminder_event_sink::ReminderEventSink;
use async_trait::async_trait;

#[derive(Default)]
pub struct StdoutReminderSink;

#[async_trait]
impl ReminderEventSink for StdoutReminderSink {
    async fn emit_hydration_reminder(&self, message: &str) -> CoreResult<()> {
        println!("{message}");
        Ok(())
    }
}
