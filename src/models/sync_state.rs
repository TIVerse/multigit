//! Synchronization state tracking models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Overall synchronization state for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// Repository path
    pub repo_path: String,

    /// Current branch
    pub current_branch: String,

    /// State for each configured remote
    pub remotes: HashMap<String, RemoteState>,

    /// Detected conflicts
    #[serde(default)]
    pub conflicts: Vec<Conflict>,

    /// Last successful sync timestamp
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,

    /// Overall sync status
    pub status: SyncStatus,
}

impl SyncState {
    /// Create a new sync state
    pub fn new(repo_path: impl Into<String>, current_branch: impl Into<String>) -> Self {
        Self {
            repo_path: repo_path.into(),
            current_branch: current_branch.into(),
            remotes: HashMap::new(),
            conflicts: Vec::new(),
            last_sync: None,
            status: SyncStatus::Unknown,
        }
    }

    /// Add a remote state
    pub fn add_remote(&mut self, name: impl Into<String>, state: RemoteState) {
        self.remotes.insert(name.into(), state);
    }

    /// Check if there are any conflicts
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Check if all remotes are in sync
    pub fn is_fully_synced(&self) -> bool {
        self.remotes.values().all(|state| state.is_synced()) && !self.has_conflicts()
    }

    /// Get remotes that are out of sync
    pub fn out_of_sync_remotes(&self) -> Vec<&String> {
        self.remotes
            .iter()
            .filter(|(_, state)| !state.is_synced())
            .map(|(name, _)| name)
            .collect()
    }

    /// Update the overall status based on remote states
    pub fn update_status(&mut self) {
        if self.has_conflicts() {
            self.status = SyncStatus::Conflict;
        } else if self.is_fully_synced() {
            self.status = SyncStatus::Synced;
        } else if self
            .remotes
            .values()
            .any(|s| s.status == RemoteSyncStatus::Error)
        {
            self.status = SyncStatus::Error;
        } else {
            self.status = SyncStatus::Dirty;
        }
    }
}

/// State of a specific remote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteState {
    /// Remote name
    pub name: String,

    /// Number of commits ahead of local
    pub ahead: usize,

    /// Number of commits behind local
    pub behind: usize,

    /// Last fetch timestamp
    pub last_fetch: Option<chrono::DateTime<chrono::Utc>>,

    /// Last push timestamp
    pub last_push: Option<chrono::DateTime<chrono::Utc>>,

    /// Sync status
    pub status: RemoteSyncStatus,

    /// Error message if status is Error
    pub error_message: Option<String>,
}

impl RemoteState {
    /// Create a new remote state
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ahead: 0,
            behind: 0,
            last_fetch: None,
            last_push: None,
            status: RemoteSyncStatus::Unknown,
            error_message: None,
        }
    }

    /// Check if this remote is in sync (no ahead/behind commits)
    pub fn is_synced(&self) -> bool {
        self.ahead == 0 && self.behind == 0 && self.status == RemoteSyncStatus::Synced
    }

    /// Mark as synced
    pub fn mark_synced(&mut self) {
        self.ahead = 0;
        self.behind = 0;
        self.status = RemoteSyncStatus::Synced;
        self.last_push = Some(chrono::Utc::now());
    }

    /// Mark as error
    pub fn mark_error(&mut self, message: impl Into<String>) {
        self.status = RemoteSyncStatus::Error;
        self.error_message = Some(message.into());
    }
}

/// Sync status for a remote
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RemoteSyncStatus {
    /// Status unknown (not yet checked)
    Unknown,

    /// Fully synchronized
    Synced,

    /// Out of sync (ahead or behind)
    Dirty,

    /// Sync in progress
    Syncing,

    /// Error occurred
    Error,
}

impl std::fmt::Display for RemoteSyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Synced => write!(f, "synced"),
            Self::Dirty => write!(f, "dirty"),
            Self::Syncing => write!(f, "syncing"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// Overall sync status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    /// Status unknown
    Unknown,

    /// All remotes synced
    Synced,

    /// Some remotes out of sync
    Dirty,

    /// Conflicts detected
    Conflict,

    /// Error occurred
    Error,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Synced => write!(f, "synced"),
            Self::Dirty => write!(f, "dirty"),
            Self::Conflict => write!(f, "conflict"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// Represents a detected conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    /// Type of conflict
    pub conflict_type: ConflictType,

    /// Remotes involved in the conflict
    pub remotes: Vec<String>,

    /// Branch where conflict was detected
    pub branch: String,

    /// Detailed description
    pub description: String,

    /// When the conflict was detected
    pub detected_at: chrono::DateTime<chrono::Utc>,

    /// Suggested resolution strategy
    pub suggested_resolution: Option<String>,
}

impl Conflict {
    /// Create a new conflict
    pub fn new(
        conflict_type: ConflictType,
        remotes: Vec<String>,
        branch: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            conflict_type,
            remotes,
            branch: branch.into(),
            description: description.into(),
            detected_at: chrono::Utc::now(),
            suggested_resolution: None,
        }
    }

    /// Add a suggested resolution
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggested_resolution = Some(suggestion.into());
        self
    }
}

/// Type of conflict detected
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictType {
    /// Divergent branches (different commits)
    DivergentBranches,

    /// Different HEAD commits
    DifferentHeads,

    /// Force push detected
    ForcePush,

    /// Branch exists on some remotes but not others
    BranchMismatch,

    /// Merge conflict
    MergeConflict,
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivergentBranches => write!(f, "divergent-branches"),
            Self::DifferentHeads => write!(f, "different-heads"),
            Self::ForcePush => write!(f, "force-push"),
            Self::BranchMismatch => write!(f, "branch-mismatch"),
            Self::MergeConflict => write!(f, "merge-conflict"),
        }
    }
}

/// Result of a push operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushResult {
    /// Remote that was pushed to
    pub remote: String,

    /// Whether the push succeeded
    pub success: bool,

    /// Status message
    pub message: String,

    /// Number of commits pushed
    pub commits_pushed: usize,

    /// Duration of the push operation
    pub duration: Option<std::time::Duration>,
}

impl PushResult {
    /// Create a successful push result
    pub fn success(remote: impl Into<String>, commits: usize) -> Self {
        Self {
            remote: remote.into(),
            success: true,
            message: "Success".to_string(),
            commits_pushed: commits,
            duration: None,
        }
    }

    /// Create a failed push result
    pub fn failure(remote: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            remote: remote.into(),
            success: false,
            message: message.into(),
            commits_pushed: 0,
            duration: None,
        }
    }

    /// Add duration information
    pub fn with_duration(mut self, duration: std::time::Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_state_creation() {
        let state = SyncState::new("/repo", "main");
        assert_eq!(state.repo_path, "/repo");
        assert_eq!(state.current_branch, "main");
        assert!(!state.has_conflicts());
    }

    #[test]
    fn test_remote_state() {
        let mut state = RemoteState::new("github");
        assert!(!state.is_synced());

        state.mark_synced();
        assert!(state.is_synced());
        assert_eq!(state.status, RemoteSyncStatus::Synced);
    }

    #[test]
    fn test_conflict_creation() {
        let conflict = Conflict::new(
            ConflictType::DivergentBranches,
            vec!["github".to_string(), "gitlab".to_string()],
            "main",
            "Branches have diverged",
        )
        .with_suggestion("Manually resolve and force push");

        assert_eq!(conflict.conflict_type, ConflictType::DivergentBranches);
        assert_eq!(conflict.remotes.len(), 2);
        assert!(conflict.suggested_resolution.is_some());
    }

    #[test]
    fn test_push_result() {
        let result =
            PushResult::success("github", 5).with_duration(std::time::Duration::from_secs(2));

        assert!(result.success);
        assert_eq!(result.commits_pushed, 5);
        assert!(result.duration.is_some());
    }

    #[test]
    fn test_sync_state_status() {
        let mut state = SyncState::new("/repo", "main");

        let mut remote1 = RemoteState::new("github");
        remote1.mark_synced();
        state.add_remote("github", remote1);

        state.update_status();
        assert_eq!(state.status, SyncStatus::Synced);

        let mut remote2 = RemoteState::new("gitlab");
        remote2.ahead = 2;
        remote2.status = RemoteSyncStatus::Dirty;
        state.add_remote("gitlab", remote2);

        state.update_status();
        assert_eq!(state.status, SyncStatus::Dirty);
    }
}
