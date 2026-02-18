use desk_buddy_core::{CoreFacade, CoreOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options: CoreOptions = match std::env::var("DESK_BUDDY_DATABASE_URL") {
        Ok(database_url) => CoreOptions { database_url },
        Err(_) => CoreOptions::default(),
    };

    let core: CoreFacade = CoreFacade::new(options).await?;

    if let Ok(raw_interval) = std::env::var("DESK_BUDDY_INTERVAL_MINUTES") {
        let requested: u64 = raw_interval.parse::<u64>()?;
        let normalized: u64 = core.set_hydration_interval_minutes(requested).await?;
        println!("Hydration interval set to {normalized} minute(s).");
    }

    let _: String = core.trigger_hydration_reminder_once().await?;
    let total: i64 = core.hydration_log_count().await?;
    println!("Total hydration reminders logged: {total}");

    Ok(())
}
