//! Git hosting provider implementations

pub mod bitbucket;
pub mod codeberg;
pub mod gitea;
pub mod github;
pub mod gitlab;
pub mod traits;

pub use traits::{Protocol, Provider, RepoConfig};

// TODO: Implement providers in Phase 3
