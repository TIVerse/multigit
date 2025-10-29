//! HTTP client for API requests
//!
//! Provides a configured HTTP client with sensible defaults for API calls.

use crate::utils::error::Result;
use reqwest::{header, Client, ClientBuilder};
use std::time::Duration;

/// Build a configured HTTP client for API requests
pub fn build_api_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("multigit/0.1.0"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .build()
        .map_err(crate::utils::error::MultiGitError::NetworkError)?;

    Ok(client)
}

/// API client wrapper with common functionality
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    /// Create a new API client
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: build_api_client()?,
        })
    }

    /// Get the underlying reqwest client
    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new().expect("Failed to create API client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_api_client() {
        let client = build_api_client();
        assert!(client.is_ok());
    }

    #[test]
    fn test_api_client_creation() {
        let api_client = ApiClient::new();
        assert!(api_client.is_ok());
    }
}
