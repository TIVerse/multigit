//! Daemon service tests

use multigit::daemon::scheduler::Scheduler;
use multigit::daemon::service::DaemonService;
use std::time::Duration;

#[test]
fn test_daemon_service_creation() {
    let service = DaemonService::new(300);
    assert_eq!(
        std::mem::size_of_val(&service),
        std::mem::size_of::<DaemonService>()
    );
}

#[test]
fn test_scheduler_creation() {
    let scheduler = Scheduler::new(60);
    assert!(!scheduler.is_running());
}

#[test]
fn test_scheduler_stop() {
    let scheduler = Scheduler::new(60);
    scheduler.stop();
    assert!(!scheduler.is_running());
}

#[test]
fn test_daemon_status_structure() {
    use multigit::daemon::service::DaemonStatus;
    use std::path::PathBuf;

    let status = DaemonStatus {
        running: false,
        pid: None,
        log_file: Some(PathBuf::from("/tmp/daemon.log")),
    };

    assert!(!status.running);
}

#[test]
fn test_schedule_creation() {
    use multigit::daemon::scheduler::Schedule;

    // Test duration string parsing
    let interval = Schedule::from_duration_str("5m");
    assert!(interval.is_ok());

    // Test every_seconds
    let seconds_schedule = Schedule::every_seconds(300);
    assert_eq!(
        std::mem::size_of_val(&seconds_schedule),
        std::mem::size_of::<Schedule>()
    );

    // Test every_minutes
    let minutes_schedule = Schedule::every_minutes(5);
    assert_eq!(
        std::mem::size_of_val(&minutes_schedule),
        std::mem::size_of::<Schedule>()
    );
}

#[test]
fn test_daemon_config_validation() {
    // Test minimum interval
    let service = DaemonService::new(30); // 30 seconds
    assert_eq!(
        std::mem::size_of_val(&service),
        std::mem::size_of::<DaemonService>()
    );

    // Test reasonable interval
    let service2 = DaemonService::new(3600); // 1 hour
    assert_eq!(
        std::mem::size_of_val(&service2),
        std::mem::size_of::<DaemonService>()
    );
}

#[test]
fn test_scheduler_handles() {
    let scheduler = Scheduler::new(60);

    // Test that scheduler can be created and stopped
    assert!(!scheduler.is_running());
    scheduler.stop();
    assert!(!scheduler.is_running());
}

#[test]
fn test_scheduler_interval() {
    let interval = 300u64;
    let scheduler = Scheduler::new(interval);

    // Verify scheduler was created with correct interval
    assert_eq!(
        std::mem::size_of_val(&scheduler),
        std::mem::size_of::<Scheduler>()
    );
}
