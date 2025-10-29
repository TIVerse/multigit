//! Conflict detection and resolution
//!
//! Handles conflicts between divergent branches across remotes.

use crate::utils::error::Result;
use tracing::{info, warn};

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionStrategy {
    /// Fast-forward only (safest, no merges)
    FastForwardOnly,
    /// Prefer a specific remote as source of truth
    PreferRemote,
    /// Manual resolution required
    Manual,
    /// Force push (dangerous!)
    Force,
}

/// A detected conflict between remotes
#[derive(Debug, Clone)]
pub struct Conflict {
    /// Branch name
    pub branch: String,
    /// Number of local commits ahead
    pub local_commits: usize,
    /// Number of remote commits ahead
    pub remote_commits: usize,
    /// Whether branches have diverged
    pub diverged: bool,
}

impl Conflict {
    /// Create a new conflict
    #[must_use] 
    pub fn new(branch: String, ahead: usize, behind: usize) -> Self {
        Self {
            branch,
            local_commits: ahead,
            remote_commits: behind,
            diverged: ahead > 0 && behind > 0,
        }
    }
}

/// Conflict resolver
pub struct ConflictResolver {
    strategy: ResolutionStrategy,
}

impl ConflictResolver {
    /// Create a new conflict resolver with the given strategy
    #[must_use] 
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self { strategy }
    }

    /// Detect conflicts between local and remote branches
    #[must_use] 
    pub fn detect_conflict(&self, ahead: usize, behind: usize) -> Option<Conflict> {
        if ahead > 0 && behind > 0 {
            Some(Conflict {
                branch: String::new(), // Will be filled by caller
                local_commits: ahead,
                remote_commits: behind,
                diverged: true,
            })
        } else {
            None
        }
    }

    /// Resolve a conflict based on the configured strategy
    pub fn resolve(&self, conflict: &Conflict) -> Result<Resolution> {
        info!(
            "Resolving conflict: {} commits ahead, {} behind",
            conflict.local_commits, conflict.remote_commits
        );

        match self.strategy {
            ResolutionStrategy::FastForwardOnly => {
                if conflict.diverged {
                    warn!("Fast-forward not possible - branches have diverged");
                    Ok(Resolution::RequiresManual)
                } else if conflict.local_commits > 0 {
                    Ok(Resolution::Push)
                } else if conflict.remote_commits > 0 {
                    Ok(Resolution::Pull)
                } else {
                    Ok(Resolution::NoAction)
                }
            }
            ResolutionStrategy::PreferRemote => {
                if conflict.remote_commits > 0 {
                    Ok(Resolution::Pull)
                } else {
                    Ok(Resolution::Push)
                }
            }
            ResolutionStrategy::Manual => Ok(Resolution::RequiresManual),
            ResolutionStrategy::Force => Ok(Resolution::ForcePush),
        }
    }

    /// Check if a conflict can be auto-resolved
    #[must_use] 
    pub fn can_auto_resolve(&self, conflict: &Conflict) -> bool {
        match self.strategy {
            ResolutionStrategy::FastForwardOnly => !conflict.diverged,
            ResolutionStrategy::PreferRemote => true,
            ResolutionStrategy::Force => true,
            ResolutionStrategy::Manual => false,
        }
    }
}

/// Resolution action to take
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// No action needed - already in sync
    NoAction,
    /// Push local commits to remote
    Push,
    /// Pull remote commits to local
    Pull,
    /// Force push (overwrites remote)
    ForcePush,
    /// Requires manual intervention
    RequiresManual,
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new(ResolutionStrategy::FastForwardOnly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_conflict() {
        let resolver = ConflictResolver::default();

        // No conflict - in sync
        assert!(resolver.detect_conflict(0, 0).is_none());

        // No conflict - ahead only
        assert!(resolver.detect_conflict(5, 0).is_none());

        // No conflict - behind only
        assert!(resolver.detect_conflict(0, 3).is_none());

        // Conflict - diverged
        let conflict = resolver.detect_conflict(2, 3);
        assert!(conflict.is_some());
        assert!(conflict.unwrap().diverged);
    }

    #[test]
    fn test_resolve_fast_forward() {
        let resolver = ConflictResolver::new(ResolutionStrategy::FastForwardOnly);

        // Ahead only - should push
        let conflict = Conflict::new("main".to_string(), 3, 0);
        assert_eq!(resolver.resolve(&conflict).unwrap(), Resolution::Push);

        // Behind only - should pull
        let conflict = Conflict::new("main".to_string(), 0, 2);
        assert_eq!(resolver.resolve(&conflict).unwrap(), Resolution::Pull);

        // Diverged - requires manual
        let conflict = Conflict::new("main".to_string(), 2, 3);
        assert_eq!(
            resolver.resolve(&conflict).unwrap(),
            Resolution::RequiresManual
        );
    }

    #[test]
    fn test_can_auto_resolve() {
        let resolver = ConflictResolver::new(ResolutionStrategy::FastForwardOnly);

        let conflict = Conflict::new("main".to_string(), 2, 0);
        assert!(resolver.can_auto_resolve(&conflict));

        let conflict = Conflict::new("main".to_string(), 2, 3);
        assert!(!resolver.can_auto_resolve(&conflict));
    }
}
