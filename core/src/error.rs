//! Shared core error types and result alias used across modules.
use std::fmt::{Display, Formatter};

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
    Io(std::io::Error),
    ConfigParse(toml::de::Error),
    SqliteMigrate(sqlx::migrate::MigrateError),
    InvalidConfigValue {
        field: &'static str,
        value: String,
        expected: &'static str,
    },
    Sqlite(sqlx::Error),
    InvalidPersistedValue { field: &'static str, value: i64 },
    SchedulerAlreadyRunning,
    TaskJoin(tokio::task::JoinError),
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {err}"),
            Self::ConfigParse(err) => write!(f, "config parse error: {err}"),
            Self::SqliteMigrate(err) => write!(f, "sqlite migration error: {err}"),
            Self::InvalidConfigValue {
                field,
                value,
                expected,
            } => write!(
                f,
                "invalid config value for {field}: {value} (expected {expected})"
            ),
            Self::Sqlite(err) => write!(f, "sqlite error: {err}"),
            Self::InvalidPersistedValue { field, value } => {
                write!(f, "invalid persisted value for {field}: {value}")
            }
            Self::SchedulerAlreadyRunning => write!(f, "scheduler is already running"),
            Self::TaskJoin(err) => write!(f, "scheduler task join error: {err}"),
        }
    }
}

impl std::error::Error for CoreError {}

impl From<std::io::Error> for CoreError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<toml::de::Error> for CoreError {
    fn from(value: toml::de::Error) -> Self {
        Self::ConfigParse(value)
    }
}

impl From<sqlx::migrate::MigrateError> for CoreError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::SqliteMigrate(value)
    }
}

impl From<sqlx::Error> for CoreError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<tokio::task::JoinError> for CoreError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::TaskJoin(value)
    }
}
