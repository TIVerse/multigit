//! Codeberg provider implementation
//!
//! Codeberg uses Forgejo, so we reuse the Gitea provider implementation.

use crate::providers::gitea::GiteaProvider;
use crate::utils::error::Result;

/// Codeberg provider (uses Forgejo/Gitea API)
pub type CodebergProvider = GiteaProvider;

/// Helper function to create a Codeberg provider
pub fn new_codeberg_provider(token: String, username: String) -> Result<CodebergProvider> {
    GiteaProvider::new(token, username, "https://codeberg.org".to_string())
}
