//! Outbound SQLite adapter modules for persistence and schema setup.
mod connection;
mod hydration_log_repository;
mod migrations;
mod settings_repository;

pub use connection::connect_sqlite;
pub use hydration_log_repository::SqliteHydrationLogRepository;
pub use migrations::run_migrations;
pub use settings_repository::SqliteSettingsRepository;
