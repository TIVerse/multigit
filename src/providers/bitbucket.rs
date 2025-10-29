//! Bitbucket provider implementation
//!
//! Implements the Provider trait for Bitbucket using the REST API 2.0.

use crate::api::{
    client::build_api_client, rate_limiter::RateLimiter, retry::retry_async, retry::RetryConfig,
};
use crate::models::{RateLimit, Repository};
use crate::providers::traits::{Protocol, Provider, RepoConfig};
use crate::utils::error::{MultiGitError, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use tracing::{debug, info};

/// Bitbucket API provider
pub struct BitbucketProvider {
    client: Client,
    username: String,
    password: String, // App password
    rate_limiter: RateLimiter,
}

impl BitbucketProvider {
    /// Create a new Bitbucket provider with app password
    pub fn new(username: String, password: String) -> Result<Self> {
        Ok(Self {
            client: build_api_client()?,
            username,
            password,
            rate_limiter: RateLimiter::bitbucket(),
        })
    }

    async fn get(&self, endpoint: &str) -> Result<Value> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(|e| MultiGitError::Other(e))?;

        let url = format!("https://api.bitbucket.org/2.0{}", endpoint);
        debug!("Bitbucket GET: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .get(&url)
                .basic_auth(&self.username, Some(&self.password))
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "Bitbucket API error: {} - {}",
                    status, error_text
                )));
            }

            let data: Value = response.json().await?;
            Ok(data)
        })
        .await
    }

    async fn post(&self, endpoint: &str, body: Value) -> Result<Value> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(|e| MultiGitError::Other(e))?;

        let url = format!("https://api.bitbucket.org/2.0{}", endpoint);
        debug!("Bitbucket POST: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .post(&url)
                .basic_auth(&self.username, Some(&self.password))
                .json(&body)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "Bitbucket API error: {} - {}",
                    status, error_text
                )));
            }

            let data: Value = response.json().await?;
            Ok(data)
        })
        .await
    }
}

#[async_trait]
impl Provider for BitbucketProvider {
    fn name(&self) -> &str {
        "bitbucket"
    }

    async fn test_connection(&self) -> anyhow::Result<bool> {
        info!("Testing Bitbucket connection");
        match self.get(&format!("/users/{}", self.username)).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn create_repo(&self, config: RepoConfig) -> anyhow::Result<Repository> {
        info!("Creating Bitbucket repository: {}", config.name);

        let body = json!({
            "scm": "git",
            "is_private": config.private,
            "description": config.description,
        });

        let endpoint = format!("/repositories/{}/{}", self.username, config.name);
        let data = self.post(&endpoint, body).await?;

        let clone_links = &data["links"]["clone"];
        let https_url = clone_links
            .as_array()
            .and_then(|arr| arr.iter().find(|link| link["name"] == "https"))
            .and_then(|link| link["href"].as_str())
            .unwrap_or("");

        let ssh_url = clone_links
            .as_array()
            .and_then(|arr| arr.iter().find(|link| link["name"] == "ssh"))
            .and_then(|link| link["href"].as_str())
            .unwrap_or("");

        Ok(Repository {
            name: data["name"].as_str().unwrap_or("").to_string(),
            full_name: Some(data["full_name"].as_str().unwrap_or("").to_string()),
            url: https_url.to_string(),
            ssh_url: ssh_url.to_string(),
            private: data["is_private"].as_bool().unwrap_or(false),
            default_branch: data["mainbranch"]["name"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            description: data["description"].as_str().map(String::from),
            html_url: data["links"]["html"]["href"].as_str().map(String::from),
            created_at: data["created_on"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.into())
            }),
            updated_at: data["updated_on"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.into())
            }),
        })
    }

    async fn get_repo(&self, name: &str) -> anyhow::Result<Repository> {
        let endpoint = format!("/repositories/{}/{}", self.username, name);
        let data = self.get(&endpoint).await?;

        let clone_links = &data["links"]["clone"];
        let https_url = clone_links
            .as_array()
            .and_then(|arr| arr.iter().find(|link| link["name"] == "https"))
            .and_then(|link| link["href"].as_str())
            .unwrap_or("");

        let ssh_url = clone_links
            .as_array()
            .and_then(|arr| arr.iter().find(|link| link["name"] == "ssh"))
            .and_then(|link| link["href"].as_str())
            .unwrap_or("");

        Ok(Repository {
            name: data["name"].as_str().unwrap_or("").to_string(),
            full_name: Some(data["full_name"].as_str().unwrap_or("").to_string()),
            url: https_url.to_string(),
            ssh_url: ssh_url.to_string(),
            private: data["is_private"].as_bool().unwrap_or(false),
            default_branch: data["mainbranch"]["name"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            description: data["description"].as_str().map(String::from),
            html_url: data["links"]["html"]["href"].as_str().map(String::from),
            created_at: data["created_on"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.into())
            }),
            updated_at: data["updated_on"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.into())
            }),
        })
    }

    fn get_remote_url(&self, name: &str, protocol: Protocol) -> String {
        match protocol {
            Protocol::Https => format!("https://bitbucket.org/{}/{}.git", self.username, name),
            Protocol::Ssh => format!("git@bitbucket.org:{}/{}.git", self.username, name),
        }
    }

    async fn create_branch(&self, _repo: &str, _branch: &str) -> anyhow::Result<()> {
        // Bitbucket creates branches on push, not via API
        info!("Bitbucket branches are created on push");
        Ok(())
    }

    async fn delete_branch(&self, _repo: &str, _branch: &str) -> anyhow::Result<()> {
        info!("Bitbucket branch deletion not implemented");
        Ok(())
    }

    async fn get_rate_limit(&self) -> anyhow::Result<RateLimit> {
        // Bitbucket doesn't expose rate limits via API
        Ok(RateLimit {
            limit: 1000,
            remaining: 900,
            reset_at: chrono::Utc::now() + chrono::Duration::hours(1),
        })
    }
}
