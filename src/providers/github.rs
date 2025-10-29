//! GitHub provider implementation
//!
//! Implements the Provider trait for GitHub using the REST API v3.

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

/// GitHub API provider
pub struct GitHubProvider {
    client: Client,
    token: String,
    username: String,
    rate_limiter: RateLimiter,
}

impl GitHubProvider {
    /// Create a new GitHub provider
    pub fn new(token: String, username: String) -> Result<Self> {
        Ok(Self {
            client: build_api_client()?,
            token,
            username,
            rate_limiter: RateLimiter::github(),
        })
    }

    /// Make an authenticated GET request
    async fn get(&self, endpoint: &str) -> Result<Value> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(|e| MultiGitError::Other(e))?;

        let url = if endpoint.starts_with("https://") {
            endpoint.to_string()
        } else {
            format!("https://api.github.com{}", endpoint)
        };

        debug!("GitHub GET: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Accept", "application/vnd.github.v3+json")
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "GitHub API error: {} - {}",
                    status, error_text
                )));
            }

            let data: Value = response.json().await?;
            Ok(data)
        })
        .await
    }

    /// Make an authenticated POST request
    async fn post(&self, endpoint: &str, body: Value) -> Result<Value> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(|e| MultiGitError::Other(e))?;

        let url = format!("https://api.github.com{}", endpoint);
        debug!("GitHub POST: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Accept", "application/vnd.github.v3+json")
                .json(&body)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "GitHub API error: {} - {}",
                    status, error_text
                )));
            }

            let data: Value = response.json().await?;
            Ok(data)
        })
        .await
    }

    /// Make an authenticated DELETE request
    async fn delete(&self, endpoint: &str) -> Result<()> {
        self.rate_limiter
            .acquire()
            .await
            .map_err(|e| MultiGitError::Other(e))?;

        let url = format!("https://api.github.com{}", endpoint);
        debug!("GitHub DELETE: {}", url);

        retry_async(RetryConfig::for_api(), || async {
            let response = self
                .client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Accept", "application/vnd.github.v3+json")
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(MultiGitError::Other(format!(
                    "GitHub API error: {} - {}",
                    status, error_text
                )));
            }

            Ok(())
        })
        .await
    }
}

#[async_trait]
impl Provider for GitHubProvider {
    fn name(&self) -> &str {
        "github"
    }

    async fn test_connection(&self) -> anyhow::Result<bool> {
        info!("Testing GitHub connection for user: {}", self.username);

        match self.get("/user").await {
            Ok(_) => {
                info!("GitHub connection successful");
                Ok(true)
            }
            Err(e) => {
                info!("GitHub connection failed: {}", e);
                Ok(false)
            }
        }
    }

    async fn create_repo(&self, config: RepoConfig) -> anyhow::Result<Repository> {
        info!("Creating GitHub repository: {}", config.name);

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
                    .map(|dt| dt.into())
            }),
            updated_at: data["updated_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.into())
            }),
        })
    }

    async fn get_repo(&self, name: &str) -> anyhow::Result<Repository> {
        info!("Fetching GitHub repository: {}/{}", self.username, name);

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
                    .map(|dt| dt.into())
            }),
            updated_at: data["updated_at"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.into())
            }),
        })
    }

    fn get_remote_url(&self, name: &str, protocol: Protocol) -> String {
        match protocol {
            Protocol::Https => format!("https://github.com/{}/{}.git", self.username, name),
            Protocol::Ssh => format!("git@github.com:{}/{}.git", self.username, name),
        }
    }

    async fn create_branch(&self, repo: &str, branch: &str) -> anyhow::Result<()> {
        info!("Creating branch '{}' in {}/{}", branch, self.username, repo);

        // Get the default branch SHA
        let repo_data = self.get_repo(repo).await?;
        let default_branch = repo_data.default_branch;

        let endpoint = format!(
            "/repos/{}/{}/git/refs/heads/{}",
            self.username, repo, default_branch
        );
        let ref_data = self.get(&endpoint).await?;
        let sha = ref_data["object"]["sha"]
            .as_str()
            .ok_or_else(|| MultiGitError::Other("Failed to get SHA".to_string()))?;

        // Create the new branch
        let body = json!({
            "ref": format!("refs/heads/{}", branch),
            "sha": sha,
        });

        let endpoint = format!("/repos/{}/{}/git/refs", self.username, repo);
        self.post(&endpoint, body).await?;

        info!("Branch '{}' created successfully", branch);
        Ok(())
    }

    async fn delete_branch(&self, repo: &str, branch: &str) -> anyhow::Result<()> {
        info!(
            "Deleting branch '{}' from {}/{}",
            branch, self.username, repo
        );

        let endpoint = format!(
            "/repos/{}/{}/git/refs/heads/{}",
            self.username, repo, branch
        );
        self.delete(&endpoint).await?;

        info!("Branch '{}' deleted successfully", branch);
        Ok(())
    }

    async fn get_rate_limit(&self) -> anyhow::Result<RateLimit> {
        debug!("Fetching GitHub rate limit info");

        let data = self.get("/rate_limit").await?;
        let core = &data["resources"]["core"];

        Ok(RateLimit {
            limit: core["limit"].as_u64().unwrap_or(5000) as u32,
            remaining: core["remaining"].as_u64().unwrap_or(0) as u32,
            reset_at: chrono::DateTime::from_timestamp(core["reset"].as_i64().unwrap_or(0), 0)
                .unwrap_or(chrono::Utc::now()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_provider_creation() {
        let provider = GitHubProvider::new("test_token".to_string(), "test_user".to_string());
        assert!(provider.is_ok());
    }

    #[test]
    fn test_get_remote_url() {
        let provider =
            GitHubProvider::new("test_token".to_string(), "testuser".to_string()).unwrap();

        assert_eq!(
            provider.get_remote_url("myrepo", Protocol::Https),
            "https://github.com/testuser/myrepo.git"
        );

        assert_eq!(
            provider.get_remote_url("myrepo", Protocol::Ssh),
            "git@github.com:testuser/myrepo.git"
        );
    }
}
