//! Git hosting provider implementations

pub mod bitbucket;
pub mod codeberg;
pub mod factory;
pub mod gitea;
pub mod github;
pub mod gitlab;
pub mod traits;

pub use factory::{create_provider, is_supported_provider, supported_providers};
pub use traits::{Protocol, Provider, RepoConfig};
