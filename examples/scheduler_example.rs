//! Scheduler example - Periodic task execution
//!
//! This example demonstrates how to use the scheduler for periodic operations.

use multigit::daemon::scheduler::{Schedule, Scheduler};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("MultiGit Scheduler Example\n");

    // 1. Parse schedule from string
    let schedule = Schedule::from_duration_str("5s")?;
    println!(
        "✓ Created schedule: every {} seconds",
        schedule.interval_seconds()
    );

    // 2. Create scheduler
    let scheduler = Scheduler::new(schedule.interval_seconds());
    println!("✓ Scheduler created\n");

    // 3. Create shared counter to track executions
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    // 4. Get scheduler handle for stopping
    let handle = scheduler.stop_handle();

    // 5. Spawn a task to stop scheduler after 12 seconds
    let stop_handle = handle.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        println!("\n⏹  Stopping scheduler...");
        stop_handle.stop();
    });

    // 6. Start scheduler with task
    println!("▶️  Starting scheduler (will run for 12 seconds)...\n");

    scheduler
        .start(move || {
            let counter = counter_clone.clone();
            async move {
                let mut count = counter.lock().unwrap();
                *count += 1;
                println!(
                    "  Tick #{}: Task executed at {:?}",
                    *count,
                    std::time::SystemTime::now()
                );
                Ok(())
            }
        })
        .await?;

    // 7. Show results
    let final_count = *counter.lock().unwrap();
    println!("\n✅ Scheduler stopped");
    println!("   Total executions: {}", final_count);

    Ok(())
}
