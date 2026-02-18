//! Minimal runnable entrypoint that exercises the current hydration slice.
use desk_buddy_core::{CoreApi, CoreBuilder, CoreOptions, CoreRuntime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options: CoreOptions = match std::env::var("DESK_BUDDY_DATABASE_URL") {
        Ok(database_url) => CoreOptions { database_url },
        Err(_) => CoreOptions::default(),
    };

    let runtime: CoreRuntime = CoreBuilder::new(options).build().await?;
    let api: &CoreApi = runtime.api();

    if let Ok(raw_interval) = std::env::var("DESK_BUDDY_INTERVAL_MINUTES") {
        let requested: u64 = raw_interval.parse::<u64>()?;
        let normalized: u64 = api.set_hydration_interval_minutes(requested).await?;
        println!("Hydration interval set to {normalized} minute(s).");
    }

    let _: String = api.trigger_hydration_reminder_once().await?;
    let total: i64 = api.hydration_log_count().await?;
    println!("Total hydration reminders logged: {total}");

    Ok(())
}
