//! Core crate exports for domain, application, ports, and adapters.
pub mod adapters;
pub mod api;
pub mod application;
pub mod bootstrap;
pub mod domain;
pub mod error;
pub mod ports;

pub use domain::hydration::{ReminderConfig, DEFAULT_REMINDER_MESSAGE};
pub use error::{CoreError, CoreResult};
pub use api::CoreApi;
pub use bootstrap::{ConfigResolver, CoreBuilder, CoreOptions, CoreRuntime, RuntimeConfig};
