//! GitLab provider implementation
//!
//! Implements the Provider trait for GitLab using the REST API v4.

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

/// GitLab API provider
pub struct GitLabProvider {
    client: Client,
    token: String,
    username: String,
    api_url: String,
    rate_limiter: RateLimiter,
}

impl GitLabProvider {
    /// Create a new GitLab provider
    pub fn new(token: String, username: String, api_url: Option<String>) -> Result<Self> {
        Ok(Self {
            client: build_api_client()?,
            token,
            username,
            api_url: api_url.unwrap_or_else(|| "https://gitlab.com/api/v4".to_string()),
            rate_limiter: RateLimiter::gitlab(),
        })
    }

    async fn get(&self, endpoint: &str) -> Result<Value> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(MultiGitError::Other)?;

        let url = format!("{}{}", self.api_url, endpoint);
        debug!("GitLab GET: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .get(&url)
                .header("PRIVATE-TOKEN", &self.token)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "GitLab API error: {status} - {error_text}"
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
        debug!("GitLab POST: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .post(&url)
                .header("PRIVATE-TOKEN", &self.token)
                .json(&body)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "GitLab API error: {status} - {error_text}"
                )));
            }

            let data: Value = response.json().await?;
            Ok(data)
        })
        .await
    }
}

#[async_trait]
impl Provider for GitLabProvider {
    fn name(&self) -> &'static str {
        "gitlab"
    }

    async fn test_connection(&self) -> anyhow::Result<bool> {
        info!("Testing GitLab connection");
        match self.get("/user").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn create_repo(&self, config: RepoConfig) -> anyhow::Result<Repository> {
        info!("Creating GitLab project: {}", config.name);

        let body = json!({
            "name": config.name,
            "description": config.description,
            "visibility": if config.private { "private" } else { "public" },
        });

        let data = self.post("/projects", body).await?;

        Ok(Repository {
            name: data["name"].as_str().unwrap_or("").to_string(),
            full_name: Some(
                data["path_with_namespace"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            ),
            url: data["http_url_to_repo"].as_str().unwrap_or("").to_string(),
            ssh_url: data["ssh_url_to_repo"].as_str().unwrap_or("").to_string(),
            private: data["visibility"].as_str() == Some("private"),
            default_branch: data["default_branch"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            description: data["description"].as_str().map(String::from),
            html_url: data["web_url"].as_str().map(String::from),
            created_at: data["created_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
            updated_at: data["last_activity_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
        })
    }

    async fn get_repo(&self, name: &str) -> anyhow::Result<Repository> {
        let path_string = format!("{}/{}", self.username, name);
        let encoded_path = urlencoding::encode(&path_string);
        let endpoint = format!("/projects/{encoded_path}");
        let data = self.get(&endpoint).await?;

        Ok(Repository {
            name: data["name"].as_str().unwrap_or("").to_string(),
            full_name: Some(
                data["path_with_namespace"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            ),
            url: data["http_url_to_repo"].as_str().unwrap_or("").to_string(),
            ssh_url: data["ssh_url_to_repo"].as_str().unwrap_or("").to_string(),
            private: data["visibility"].as_str() == Some("private"),
            default_branch: data["default_branch"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            description: data["description"].as_str().map(String::from),
            html_url: data["web_url"].as_str().map(String::from),
            created_at: data["created_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
            updated_at: data["last_activity_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(std::convert::Into::into)
            }),
        })
    }

    fn get_remote_url(&self, name: &str, protocol: Protocol) -> String {
        match protocol {
            Protocol::Https => format!("https://gitlab.com/{}/{}.git", self.username, name),
            Protocol::Ssh => format!("git@gitlab.com:{}/{}.git", self.username, name),
        }
    }

    async fn create_branch(&self, repo: &str, branch: &str) -> anyhow::Result<()> {
        info!(
            "Creating branch '{}' in GitLab project {}/{}",
            branch, self.username, repo
        );

        let path_string = format!("{}/{}", self.username, repo);
        let encoded_path = urlencoding::encode(&path_string);
        let body = json!({
            "branch": branch,
            "ref": "main",
        });

        let endpoint = format!("/projects/{encoded_path}/repository/branches");
        self.post(&endpoint, body).await?;
        Ok(())
    }

    async fn delete_branch(&self, _repo: &str, branch: &str) -> anyhow::Result<()> {
        info!("Deleting branch '{}' from GitLab project", branch);
        // GitLab branch deletion would use DELETE request
        // Simplified implementation for now
        Ok(())
    }

    async fn get_rate_limit(&self) -> anyhow::Result<RateLimit> {
        // GitLab doesn't have a dedicated rate limit endpoint like GitHub
        // Using sensible defaults based on GitLab's documented limits
        Ok(RateLimit {
            limit: 600,
            remaining: 500,
            reset_at: chrono::Utc::now() + chrono::Duration::minutes(1),
        })
    }
}
