//! Domain types and policies for hydration reminder behavior.
use std::time::Duration;

pub const DEFAULT_REMINDER_MESSAGE: &str = "Reminder: drink water!";
pub const DEFAULT_REMINDER_MINUTES: u64 = 30;
pub const DEFAULT_SNOOZE_MINUTES: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReminderConfig {
    pub interval_minutes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnoozeConfig {
    pub snooze_minutes: u64,
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

impl SnoozeConfig {
    /// Creates a snooze config and clamps the value to at least one minute.
    pub fn new(snooze_minutes: u64) -> Self {
        Self {
            snooze_minutes: snooze_minutes.max(1),
        }
    }
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            interval_minutes: DEFAULT_REMINDER_MINUTES,
        }
    }
}

impl Default for SnoozeConfig {
    fn default() -> Self {
        Self {
            snooze_minutes: DEFAULT_SNOOZE_MINUTES,
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

    #[test]
    fn default_snooze_is_10_minutes() {
        assert_eq!(SnoozeConfig::default().snooze_minutes, 10);
    }

    #[test]
    fn zero_snooze_is_clamped_to_one_minute() {
        assert_eq!(SnoozeConfig::new(0).snooze_minutes, 1);
    }
}
