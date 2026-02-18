pub mod adapters;
pub mod application;
pub mod domain;
pub mod error;
pub mod ports;

pub use domain::hydration::{ReminderConfig, DEFAULT_REMINDER_MESSAGE};
pub use error::{CoreError, CoreResult};
pub use adapters::inbound::{CoreFacade, CoreOptions};
