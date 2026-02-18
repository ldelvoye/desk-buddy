//! Bootstrap/runtime layer that wires adapters and owns background tasks.
mod core_builder;
mod core_runtime;

pub use core_builder::{CoreBuilder, CoreOptions};
pub use core_runtime::CoreRuntime;
