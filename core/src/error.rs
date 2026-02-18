use std::fmt::{Display, Formatter};

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
    Sqlite(sqlx::Error),
    InvalidPersistedValue { field: &'static str, value: i64 },
    SchedulerAlreadyRunning,
    TaskJoin(tokio::task::JoinError),
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
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
