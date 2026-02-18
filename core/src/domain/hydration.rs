use std::time::Duration;

pub const DEFAULT_REMINDER_MESSAGE: &str = "Reminder: drink water!";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReminderConfig {
    pub interval_minutes: u64,
}

impl ReminderConfig {
    /// Creates a reminder config and clamps the interval to at least one minute.
    pub fn new(interval_minutes: u64) -> Self {
        Self {
            interval_minutes: interval_minutes.max(1),
        }
    }

    /// Converts the configured interval (minutes) into a `Duration`.
    pub fn interval_duration(self) -> Duration {
        Duration::from_secs(self.interval_minutes.saturating_mul(60))
    }
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            interval_minutes: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_30_minutes() {
        assert_eq!(ReminderConfig::default().interval_minutes, 30);
    }

    #[test]
    fn zero_interval_is_clamped_to_one_minute() {
        assert_eq!(ReminderConfig::new(0).interval_minutes, 1);
    }
}
