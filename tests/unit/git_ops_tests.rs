//! Additional git operations tests

use git2::Repository;
use multigit::git::branch::BranchManager;
use multigit::git::operations::GitOperations;
use multigit::git::remote::RemoteManager;
use tempfile::TempDir;

#[test]
fn test_git_operations_open() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();

    let ops = GitOperations::open(temp_dir.path());
    assert!(ops.is_ok());
}

#[test]
fn test_git_operations_new() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();

    let ops = GitOperations::new(temp_dir.path());
    assert!(ops.is_ok());
}

#[test]
fn test_git_operations_path() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();

    let ops = GitOperations::open(temp_dir.path()).unwrap();
    let path = ops.path();
    assert!(path.ends_with(".git"));
}

#[test]
fn test_is_bare() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();

    let ops = GitOperations::open(temp_dir.path()).unwrap();
    assert!(!ops.is_bare());
}

#[test]
fn test_inner_repository() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();

    let ops = GitOperations::open(temp_dir.path()).unwrap();
    let inner = ops.inner();
    assert!(!inner.is_bare());
}

#[test]
fn test_branch_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = BranchManager::new(&repo);
    assert_eq!(
        std::mem::size_of_val(&manager),
        std::mem::size_of::<BranchManager>()
    );
}

#[test]
fn test_create_and_list_branches() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Create initial commit
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.write_tree().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    let commit = repo
        .commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
        .unwrap();

    let manager = BranchManager::new(&repo);

    // Create a branch
    let result = manager.create_branch("feature", &repo.find_commit(commit).unwrap());
    assert!(result.is_ok());

    // List branches
    let branches = manager.list_branches();
    assert!(branches.is_ok());
    let branch_list = branches.unwrap();
    assert!(!branch_list.is_empty());
}

#[test]
fn test_delete_branch() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Create initial commit
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.write_tree().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    let commit = repo
        .commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
        .unwrap();

    let manager = BranchManager::new(&repo);

    // Create a branch
    manager
        .create_branch("to-delete", &repo.find_commit(commit).unwrap())
        .unwrap();

    // Delete the branch
    let result = manager.delete_branch("to-delete");
    assert!(result.is_ok());
}

#[test]
fn test_remote_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = RemoteManager::new(&repo);
    assert_eq!(
        std::mem::size_of_val(&manager),
        std::mem::size_of::<RemoteManager>()
    );
}

#[test]
fn test_add_remote() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = RemoteManager::new(&repo);
    let result = manager.add("origin", "https://github.com/user/repo.git");
    assert!(result.is_ok());
}

#[test]
fn test_list_remotes() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = RemoteManager::new(&repo);
    manager
        .add("origin", "https://github.com/user/repo.git")
        .unwrap();

    let remotes = manager.list();
    assert!(remotes.is_ok());
    let remote_list = remotes.unwrap();
    assert!(!remote_list.is_empty());
}

#[test]
fn test_remove_remote() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let manager = RemoteManager::new(&repo);
    manager
        .add("to-remove", "https://github.com/user/repo.git")
        .unwrap();

    let result = manager.remove("to-remove");
    assert!(result.is_ok());
}

#[test]
fn test_extract_repo_name() {
    let name = RemoteManager::extract_repo_name("https://github.com/user/repo.git");
    assert_eq!(name, Some("repo".to_string()));

    let name2 = RemoteManager::extract_repo_name("git@github.com:user/myproject.git");
    assert_eq!(name2, Some("myproject".to_string()));
}

#[test]
fn test_url_conversion() {
    let https = "https://github.com/user/repo.git";
    let ssh = RemoteManager::https_to_ssh(https);
    assert_eq!(ssh, Some("git@github.com:user/repo.git".to_string()));

    let back_to_https = RemoteManager::ssh_to_https(&ssh.unwrap());
    assert_eq!(back_to_https, Some(https.to_string()));
}

#[test]
fn test_is_clean_repo() {
    let temp_dir = TempDir::new().unwrap();
    let _repo = Repository::init(temp_dir.path()).unwrap();

    let ops = GitOperations::open(temp_dir.path()).unwrap();
    let result = ops.is_clean();
    assert!(result.is_ok());
}

#[test]
fn test_get_current_branch() {
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

    let ops = GitOperations::new(temp_dir.path()).unwrap();
    let branch = ops.get_current_branch();
    assert!(branch.is_ok());
}

#[test]
fn test_find_commit() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Create initial commit
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.write_tree().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    let oid = repo
        .commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
        .unwrap();

    let ops = GitOperations::new(temp_dir.path()).unwrap();
    let commit = ops.find_commit(oid);
    assert!(commit.is_ok());
}

#[test]
fn test_head_commit() {
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

    let ops = GitOperations::new(temp_dir.path()).unwrap();
    let commit = ops.head_commit();
    assert!(commit.is_ok());
}
