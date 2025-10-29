//! Integration tests for Git operations

use multigit::git::operations::GitOperations;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_git_init_creates_repository() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    let git_ops = GitOperations::init(repo_path).unwrap();

    // Verify .git directory exists
    assert!(repo_path.join(".git").exists());
    assert!(!git_ops.is_bare());
}

#[test]
fn test_git_open_existing_repository() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // First init
    GitOperations::init(repo_path).unwrap();

    // Then open
    let git_ops = GitOperations::open(repo_path).unwrap();
    assert!(git_ops.is_clean().unwrap());
}

#[test]
fn test_git_new_alias() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    GitOperations::init(repo_path).unwrap();

    // Test that 'new' works as alias for 'open'
    let git_ops = GitOperations::new(repo_path).unwrap();
    assert!(git_ops.is_clean().unwrap());
}

#[test]
fn test_git_is_clean_on_new_repo() {
    let temp_dir = TempDir::new().unwrap();
    let git_ops = GitOperations::init(temp_dir.path()).unwrap();

    assert!(git_ops.is_clean().unwrap());
}

#[test]
fn test_git_is_not_clean_with_changes() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    let git_ops = GitOperations::init(repo_path).unwrap();

    // Create a file
    fs::write(repo_path.join("test.txt"), "content").unwrap();

    // Repository should not be clean
    assert!(!git_ops.is_clean().unwrap());
}

#[test]
fn test_git_add_and_list_remotes() {
    let temp_dir = TempDir::new().unwrap();
    let git_ops = GitOperations::init(temp_dir.path()).unwrap();

    // Add remotes
    git_ops
        .add_remote("origin", "https://github.com/user/repo.git")
        .unwrap();
    git_ops
        .add_remote("upstream", "https://github.com/other/repo.git")
        .unwrap();

    // Verify remotes exist
    let origin_url = git_ops.get_remote_url("origin").unwrap();
    assert_eq!(origin_url, "https://github.com/user/repo.git");

    let upstream_url = git_ops.get_remote_url("upstream").unwrap();
    assert_eq!(upstream_url, "https://github.com/other/repo.git");
}

#[test]
fn test_git_remove_remote() {
    let temp_dir = TempDir::new().unwrap();
    let git_ops = GitOperations::init(temp_dir.path()).unwrap();

    // Add a remote
    git_ops
        .add_remote("origin", "https://github.com/user/repo.git")
        .unwrap();
    assert!(git_ops.get_remote_url("origin").is_ok());

    // Remove it
    git_ops.remove_remote("origin").unwrap();

    // Should be gone
    let result = git_ops.get_remote_url("origin");
    assert!(result.is_err());
}

#[test]
fn test_git_remove_nonexistent_remote() {
    let temp_dir = TempDir::new().unwrap();
    let git_ops = GitOperations::init(temp_dir.path()).unwrap();

    // Try to remove a remote that doesn't exist
    let result = git_ops.remove_remote("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_git_workdir() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    let git_ops = GitOperations::init(repo_path).unwrap();

    let workdir = git_ops.workdir().unwrap();
    assert_eq!(workdir, repo_path);
}

#[test]
fn test_git_path() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    let git_ops = GitOperations::init(repo_path).unwrap();

    let git_path = git_ops.path();
    assert!(git_path.to_string_lossy().contains(".git"));
}

#[test]
fn test_git_list_local_branches() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    let git_ops = GitOperations::init(repo_path).unwrap();

    // Create initial commit so we have branches
    fs::write(repo_path.join("test.txt"), "content").unwrap();

    let repo = git_ops.inner();
    let mut index = repo.index().unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = repo.signature().unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .unwrap();

    // Now list branches
    let branches = git_ops.list_local_branches().unwrap();
    assert!(!branches.is_empty());
}
