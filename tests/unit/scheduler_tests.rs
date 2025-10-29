//! Unit tests for scheduler

use multigit::daemon::scheduler::{Schedule, Scheduler};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[test]
fn test_schedule_parsing() {
    assert_eq!(
        Schedule::from_duration_str("30s")
            .unwrap()
            .interval_seconds(),
        30
    );
    assert_eq!(
        Schedule::from_duration_str("5m")
            .unwrap()
            .interval_seconds(),
        300
    );
    assert_eq!(
        Schedule::from_duration_str("2h")
            .unwrap()
            .interval_seconds(),
        7200
    );
    assert_eq!(
        Schedule::from_duration_str("120")
            .unwrap()
            .interval_seconds(),
        120
    );

    // Test uppercase
    assert_eq!(
        Schedule::from_duration_str("5M")
            .unwrap()
            .interval_seconds(),
        300
    );
    assert_eq!(
        Schedule::from_duration_str("1H")
            .unwrap()
            .interval_seconds(),
        3600
    );
}

#[test]
fn test_schedule_parsing_invalid() {
    assert!(Schedule::from_duration_str("invalid").is_err());
    assert!(Schedule::from_duration_str("").is_err());
    assert!(Schedule::from_duration_str("abc123").is_err());
}

#[test]
fn test_schedule_creation() {
    let schedule = Schedule::every_seconds(45);
    assert_eq!(schedule.interval_seconds(), 45);

    let schedule = Schedule::every_minutes(10);
    assert_eq!(schedule.interval_seconds(), 600);

    let schedule = Schedule::every_hours(2);
    assert_eq!(schedule.interval_seconds(), 7200);
}

#[test]
fn test_schedule_default() {
    let schedule = Schedule::default();
    assert_eq!(schedule.interval_seconds(), 300); // 5 minutes
}

#[test]
fn test_scheduler_creation() {
    let scheduler = Scheduler::new(300);
    assert_eq!(scheduler.interval_seconds(), 300);
    assert!(!scheduler.is_running());
}

#[test]
fn test_scheduler_handle() {
    let scheduler = Scheduler::new(60);
    let handle = scheduler.stop_handle();

    assert!(!handle.is_running());

    // Stop via handle
    handle.stop();
    assert!(!handle.is_running());
}

#[tokio::test]
async fn test_scheduler_runs_task() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let scheduler = Scheduler::new(1); // 1 second interval
    let handle = scheduler.stop_handle();

    // Spawn scheduler
    let scheduler_task = tokio::spawn(async move {
        scheduler
            .start(move || {
                let counter = counter_clone.clone();
                async move {
                    let mut count = counter.lock().unwrap();
                    *count += 1;
                    Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                }
            })
            .await
    });

    // Wait for a few ticks
    sleep(Duration::from_millis(2500)).await;

    // Stop scheduler
    handle.stop();

    // Wait for task to finish
    let _ = scheduler_task.await;

    // Check that task ran at least twice
    let count = *counter.lock().unwrap();
    assert!(count >= 2, "Expected at least 2 executions, got {}", count);
}

#[tokio::test]
async fn test_scheduler_continues_on_task_error() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let scheduler = Scheduler::new(1);
    let handle = scheduler.stop_handle();

    let scheduler_task = tokio::spawn(async move {
        scheduler
            .start(move || {
                let counter = counter_clone.clone();
                async move {
                    let mut count = counter.lock().unwrap();
                    *count += 1;

                    // Always fail, but scheduler should continue
                    Err::<(), Box<dyn std::error::Error + Send + Sync>>("Task failed".into())
                }
            })
            .await
    });

    sleep(Duration::from_millis(2500)).await;
    handle.stop();
    let _ = scheduler_task.await;

    // Task should have run multiple times despite errors
    let count = *counter.lock().unwrap();
    assert!(count >= 2);
}
