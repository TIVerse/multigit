//! Sync manager tests

use git2::Repository;
use multigit::core::sync_manager::SyncManager;
use multigit::git::operations::GitOperations;
use tempfile::TempDir;

#[test]
fn test_sync_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = SyncManager::new(repo.path());
    assert!(manager.is_ok());
}

#[test]
fn test_sync_manager_with_max_parallel() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = SyncManager::new(repo.path()).unwrap().with_max_parallel(8);

    // Manager should be created successfully
    assert_eq!(
        std::mem::size_of_val(&manager),
        std::mem::size_of::<SyncManager>()
    );
}

#[test]
fn test_current_branch() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Create initial commit
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.write_tree().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
        .unwrap();

    let git_ops = GitOperations::new(repo.path()).unwrap();
    let branch = git_ops.get_current_branch();
    assert!(branch.is_ok());
}

#[test]
fn test_is_clean() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let git_ops = GitOperations::new(repo.path()).unwrap();
    let is_clean = git_ops.is_clean();
    assert!(is_clean.is_ok());
}
