//! Minimal runnable entrypoint that exercises the current hydration slice.
use desk_buddy_core::{ConfigResolver, CoreApi, CoreBuilder, CoreRuntime, RuntimeConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime_config: RuntimeConfig = ConfigResolver::load()?;
    let runtime: CoreRuntime = CoreBuilder::new(runtime_config.options).build().await?;
    let api: &CoreApi = runtime.api();

    let _: String = api.trigger_hydration_reminder_once().await?;
    let total: i64 = api.hydration_log_count().await?;
    println!("Total hydration reminders logged: {total}");

    Ok(())
}
