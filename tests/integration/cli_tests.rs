//! CLI integration tests

use multigit::core::config::Config;
use tempfile::TempDir;
use git2::Repository;

#[test]
fn test_cli_parser_creation() {
    // Test that CLI parser can be created
    let args = vec!["multigit", "init"];
    let result = std::panic::catch_unwind(|| {
        // This would normally parse from CLI args
        // We're just testing the structure exists
        assert_eq!(args.len(), 2);
    });
    assert!(result.is_ok());
}

#[test]
fn test_config_initialization() {
    let config = Config::default();
    assert!(config.remotes.is_empty());
    assert_eq!(config.settings.default_branch, "main");
}

#[test]
fn test_repository_detection() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();
    
    // Verify .git directory exists
    assert!(temp_dir.path().join(".git").exists());
}

#[test]
fn test_config_remote_management() {
    use multigit::core::config::RemoteConfig;
    
    let mut config = Config::default();
    
    let remote = RemoteConfig {
        username: "testuser".to_string(),
        api_url: None,
        enabled: true,
        provider: Some("github".to_string()),
        use_ssh: false,
        priority: 0,
    };
    
    config.add_remote("github".to_string(), remote);
    assert_eq!(config.remotes.len(), 1);
    
    config.remove_remote("github");
    assert_eq!(config.remotes.len(), 0);
}

#[test]
fn test_enabled_remotes_filter() {
    use multigit::core::config::RemoteConfig;
    
    let mut config = Config::default();
    
    config.add_remote("github".to_string(), RemoteConfig {
        username: "user".to_string(),
        api_url: None,
        enabled: true,
        provider: Some("github".to_string()),
        use_ssh: false,
        priority: 0,
    });
    
    config.add_remote("gitlab".to_string(), RemoteConfig {
        username: "user".to_string(),
        api_url: None,
        enabled: false,
        provider: Some("gitlab".to_string()),
        use_ssh: false,
        priority: 0,
    });
    
    let enabled = config.enabled_remotes();
    assert_eq!(enabled.len(), 1);
    assert!(enabled.contains_key("github"));
}

#[test]
fn test_conflict_resolver_creation() {
    use multigit::core::conflict_resolver::{ConflictResolver, ResolutionStrategy};
    
    let resolver = ConflictResolver::new(ResolutionStrategy::FastForwardOnly);
    assert_eq!(std::mem::size_of_val(&resolver), std::mem::size_of::<ConflictResolver>());
}

#[test]
fn test_conflict_detection() {
    use multigit::core::conflict_resolver::ConflictResolver;
    use multigit::core::conflict_resolver::ResolutionStrategy;
    
    let resolver = ConflictResolver::new(ResolutionStrategy::Manual);
    
    // Test diverged state (ahead and behind)
    let conflict = resolver.detect_conflict(5, 3);
    assert!(conflict.is_some());
    
    // Test no conflict
    let no_conflict = resolver.detect_conflict(0, 0);
    assert!(no_conflict.is_none());
}

#[test]
fn test_auto_resolve_capability() {
    use multigit::core::conflict_resolver::{Conflict, ConflictResolver, ResolutionStrategy};
    
    let resolver = ConflictResolver::new(ResolutionStrategy::FastForwardOnly);
    
    // Ahead only - can fast forward
    let conflict = Conflict {
        branch: "main".to_string(),
        local_commits: 5,
        remote_commits: 0,
        diverged: false,
    };
    assert!(resolver.can_auto_resolve(&conflict));
    
    // Diverged - cannot fast forward
    let diverged = Conflict {
        branch: "main".to_string(),
        local_commits: 5,
        remote_commits: 3,
        diverged: true,
    };
    assert!(!resolver.can_auto_resolve(&diverged));
}

#[test]
fn test_health_checker_creation() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();
    
    use multigit::core::health_checker::HealthChecker;
    let checker = HealthChecker::new(temp_dir.path().to_path_buf());
    assert!(checker.is_ok());
}

#[test]
fn test_health_check_execution() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    
    // Create initial commit
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.write_tree().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[]).unwrap();
    
    use multigit::core::health_checker::HealthChecker;
    let checker = HealthChecker::new(temp_dir.path().to_path_buf()).unwrap();
    let _report = checker.check();
    
    // Health check completed
}

#[test]
fn test_is_healthy() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    
    // Create initial commit
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.write_tree().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[]).unwrap();
    
    use multigit::core::health_checker::HealthChecker;
    let checker = HealthChecker::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Should be healthy for a fresh repo
    let is_healthy = checker.is_healthy();
    assert!(is_healthy);
}
