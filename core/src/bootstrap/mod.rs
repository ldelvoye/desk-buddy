//! Bootstrap/runtime layer that wires adapters and owns background tasks.
mod config_resolver;
mod core_builder;
mod core_runtime;

pub use config_resolver::{ConfigResolver, RuntimeConfig};
pub use core_builder::{CoreBuilder, CoreOptions};
pub use core_runtime::CoreRuntime;
