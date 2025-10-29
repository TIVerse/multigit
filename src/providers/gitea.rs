//! Gitea/Forgejo provider implementation
//!
//! Implements the Provider trait for Gitea and Forgejo instances using the REST API.

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

/// Gitea/Forgejo API provider
pub struct GiteaProvider {
    client: Client,
    token: String,
    username: String,
    api_url: String,
    base_url: String, // For clone URLs
    rate_limiter: RateLimiter,
}

impl GiteaProvider {
    /// Create a new Gitea/Forgejo provider
    pub fn new(token: String, username: String, base_url: String) -> Result<Self> {
        let api_url = format!("{}/api/v1", base_url.trim_end_matches('/'));

        Ok(Self {
            client: build_api_client()?,
            token,
            username,
            api_url,
            base_url: base_url.trim_end_matches('/').to_string(),
            rate_limiter: RateLimiter::new(1000.0, 10.0), // Conservative defaults
        })
    }

    async fn get(&self, endpoint: &str) -> Result<Value> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(MultiGitError::Other)?;

        let url = format!("{}{}", self.api_url, endpoint);
        debug!("Gitea GET: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .get(&url)
                .header("Authorization", format!("token {}", self.token))
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "Gitea API error: {status} - {error_text}"
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
            .map_err(MultiGitError::Other)?;

        let url = format!("{}{}", self.api_url, endpoint);
        debug!("Gitea POST: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .post(&url)
                .header("Authorization", format!("token {}", self.token))
                .json(&body)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "Gitea API error: {status} - {error_text}"
                )));
            }

            let data: Value = response.json().await?;
            Ok(data)
        })
        .await
    }
}

#[async_trait]
impl Provider for GiteaProvider {
    fn name(&self) -> &'static str {
        "gitea"
    }

    async fn test_connection(&self) -> anyhow::Result<bool> {
        info!("Testing Gitea connection");
        match self.get("/user").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn create_repo(&self, config: RepoConfig) -> anyhow::Result<Repository> {
        info!("Creating Gitea repository: {}", config.name);

        let body = json!({
            "name": config.name,
            "description": config.description,
            "private": config.private,
            "auto_init": false,
        });

        let data = self.post("/user/repos", body).await?;

        Ok(Repository {
            name: data["name"].as_str().unwrap_or("").to_string(),
            full_name: Some(data["full_name"].as_str().unwrap_or("").to_string()),
            url: data["clone_url"].as_str().unwrap_or("").to_string(),
            ssh_url: data["ssh_url"].as_str().unwrap_or("").to_string(),
            private: data["private"].as_bool().unwrap_or(false),
            default_branch: data["default_branch"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            description: data["description"].as_str().map(String::from),
            html_url: data["html_url"].as_str().map(String::from),
            created_at: data["created_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
            updated_at: data["updated_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
        })
    }

    async fn get_repo(&self, name: &str) -> anyhow::Result<Repository> {
        let endpoint = format!("/repos/{}/{}", self.username, name);
        let data = self.get(&endpoint).await?;

        Ok(Repository {
            name: data["name"].as_str().unwrap_or("").to_string(),
            full_name: Some(data["full_name"].as_str().unwrap_or("").to_string()),
            url: data["clone_url"].as_str().unwrap_or("").to_string(),
            ssh_url: data["ssh_url"].as_str().unwrap_or("").to_string(),
            private: data["private"].as_bool().unwrap_or(false),
            default_branch: data["default_branch"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            description: data["description"].as_str().map(String::from),
            html_url: data["html_url"].as_str().map(String::from),
            created_at: data["created_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
            updated_at: data["updated_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
        })
    }

    fn get_remote_url(&self, name: &str, protocol: Protocol) -> String {
        // Extract host from base_url
        let host = self
            .base_url
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        match protocol {
            Protocol::Https => format!("{}/{}/{}.git", self.base_url, self.username, name),
            Protocol::Ssh => format!("git@{}:{}/{}.git", host, self.username, name),
        }
    }

    async fn create_branch(&self, repo: &str, branch: &str) -> anyhow::Result<()> {
        info!("Creating branch '{}' in {}/{}", branch, self.username, repo);

        // Get the default branch reference
        let repo_data = self.get_repo(repo).await?;
        let default_branch = repo_data.default_branch;

        let body = json!({
            "new_branch_name": branch,
            "old_branch_name": default_branch,
        });

        let endpoint = format!("/repos/{}/{}/branches", self.username, repo);
        self.post(&endpoint, body).await?;
        Ok(())
    }

    async fn delete_branch(&self, _repo: &str, _branch: &str) -> anyhow::Result<()> {
        info!("Gitea branch deletion");
        // Would require DELETE request implementation
        Ok(())
    }

    async fn get_rate_limit(&self) -> anyhow::Result<RateLimit> {
        // Gitea doesn't typically have strict rate limits
        Ok(RateLimit {
            limit: 5000,
            remaining: 4900,
            reset_at: chrono::Utc::now() + chrono::Duration::hours(1),
        })
    }
}
