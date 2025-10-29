//! OS keyring integration for secure credential storage
//!
//! Provides a cross-platform interface to store credentials securely using the OS keyring.

use crate::utils::error::{MultiGitError, Result};
use keyring::Entry;
use tracing::{debug, info};

/// Service name for keyring entries
const SERVICE_NAME: &str = "multigit";

/// Keyring manager for secure credential storage
pub struct KeyringManager {
    service: String,
}

impl KeyringManager {
    /// Create a new keyring manager
    pub fn new() -> Self {
        Self {
            service: SERVICE_NAME.to_string(),
        }
    }

    /// Create a keyring manager with a custom service name
    pub fn with_service(service: String) -> Self {
        Self { service }
    }

    /// Store a credential in the keyring
    pub fn store(&self, key: &str, value: &str) -> Result<()> {
        debug!("Storing credential for key: {}", key);

        let entry = Entry::new(&self.service, key)
            .map_err(|e| MultiGitError::Other(format!("Failed to create keyring entry: {}", e)))?;

        entry
            .set_password(value)
            .map_err(|e| MultiGitError::Other(format!("Failed to store credential: {}", e)))?;

        info!("Successfully stored credential for key: {}", key);
        Ok(())
    }

    /// Retrieve a credential from the keyring
    pub fn retrieve(&self, key: &str) -> Result<String> {
        debug!("Retrieving credential for key: {}", key);

        let entry = Entry::new(&self.service, key)
            .map_err(|e| MultiGitError::Other(format!("Failed to create keyring entry: {}", e)))?;

        let password = entry
            .get_password()
            .map_err(|e| MultiGitError::Other(format!("Failed to retrieve credential: {}", e)))?;

        debug!("Successfully retrieved credential for key: {}", key);
        Ok(password)
    }

    /// Delete a credential from the keyring
    pub fn delete(&self, key: &str) -> Result<()> {
        debug!("Deleting credential for key: {}", key);

        let entry = Entry::new(&self.service, key)
            .map_err(|e| MultiGitError::Other(format!("Failed to create keyring entry: {}", e)))?;

        entry
            .delete_password()
            .map_err(|e| MultiGitError::Other(format!("Failed to delete credential: {}", e)))?;

        info!("Successfully deleted credential for key: {}", key);
        Ok(())
    }

    /// Check if a credential exists in the keyring
    pub fn exists(&self, key: &str) -> bool {
        match self.retrieve(key) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Store a provider token
    pub fn store_provider_token(&self, provider: &str, username: &str, token: &str) -> Result<()> {
        let key = format!("{}:{}:token", provider, username);
        self.store(&key, token)
    }

    /// Retrieve a provider token
    pub fn retrieve_provider_token(&self, provider: &str, username: &str) -> Result<String> {
        let key = format!("{}:{}:token", provider, username);
        self.retrieve(&key)
    }

    /// Delete a provider token
    pub fn delete_provider_token(&self, provider: &str, username: &str) -> Result<()> {
        let key = format!("{}:{}:token", provider, username);
        self.delete(&key)
    }
}

impl Default for KeyringManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyring_store_and_retrieve() {
        let manager = KeyringManager::new();
        let test_key = "test_key_12345";
        let test_value = "test_secret_value";

        // Store
        let store_result = manager.store(test_key, test_value);
        if store_result.is_err() {
            // Keyring might not be available in CI/test environments
            eprintln!("Keyring not available in test environment");
            return;
        }

        // Retrieve
        let retrieved = manager.retrieve(test_key).unwrap();
        assert_eq!(retrieved, test_value);

        // Cleanup
        let _ = manager.delete(test_key);
    }

    #[test]
    fn test_keyring_exists() {
        let manager = KeyringManager::new();
        let test_key = "test_key_exists_67890";

        // Initially should not exist
        assert!(!manager.exists(test_key));

        // Store a value
        if manager.store(test_key, "value").is_ok() {
            // Should now exist
            assert!(manager.exists(test_key));

            // Cleanup
            let _ = manager.delete(test_key);
        }
    }

    #[test]
    fn test_provider_token_methods() {
        let manager = KeyringManager::new();

        let provider = "github";
        let username = "testuser";
        let token = "ghp_test_token_123";

        // Store provider token
        if manager
            .store_provider_token(provider, username, token)
            .is_ok()
        {
            // Retrieve provider token
            let retrieved = manager.retrieve_provider_token(provider, username).unwrap();
            assert_eq!(retrieved, token);

            // Cleanup
            let _ = manager.delete_provider_token(provider, username);
        }
    }
}
