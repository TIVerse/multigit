//! Input validation utilities

use crate::utils::error::{MultiGitError, Result};
use regex::Regex;
use std::sync::OnceLock;

/// Validate a repository name
pub fn validate_repo_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(MultiGitError::invalid_input(
            "Repository name cannot be empty",
        ));
    }

    if name.len() > 100 {
        return Err(MultiGitError::invalid_input(
            "Repository name cannot exceed 100 characters",
        ));
    }

    // Common Git repository name pattern
    static REPO_NAME_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REPO_NAME_REGEX
        .get_or_init(|| Regex::new(r"^[a-zA-Z0-9._-]+$").expect("Invalid regex pattern"));

    if !regex.is_match(name) {
        return Err(MultiGitError::invalid_input(
            "Repository name can only contain letters, numbers, dots, hyphens, and underscores",
        ));
    }

    Ok(())
}

/// Validate a username
pub fn validate_username(username: &str) -> Result<()> {
    static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();

    if username.is_empty() {
        return Err(MultiGitError::invalid_input("Username cannot be empty"));
    }

    if username.len() > 39 {
        // GitHub's max username length
        return Err(MultiGitError::invalid_input(
            "Username cannot exceed 39 characters",
        ));
    }

    let regex = USERNAME_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?$").expect("Invalid regex pattern")
    });

    if !regex.is_match(username) {
        return Err(MultiGitError::invalid_input(
            "Username must start and end with alphanumeric characters, and can contain hyphens",
        ));
    }

    Ok(())
}

/// Validate a branch name
pub fn validate_branch_name(branch: &str) -> Result<()> {
    if branch.is_empty() {
        return Err(MultiGitError::invalid_input("Branch name cannot be empty"));
    }

    // Git branch name restrictions
    if branch.starts_with('-') || branch.starts_with('.') || branch.ends_with('.') {
        return Err(MultiGitError::invalid_input(
            "Branch name cannot start with '-' or '.' or end with '.'",
        ));
    }

    if branch.contains("..") || branch.contains("//") {
        return Err(MultiGitError::invalid_input(
            "Branch name cannot contain '..' or '//'",
        ));
    }

    // Forbidden characters
    let forbidden = ['~', '^', ':', '?', '*', '[', '\\', ' ', '\t', '\n'];
    if branch.chars().any(|c| forbidden.contains(&c)) {
        return Err(MultiGitError::invalid_input(format!(
            "Branch name cannot contain: {}",
            forbidden
                .iter()
                .map(|c| format!("'{c}'"))
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    Ok(())
}

/// Validate a URL
pub fn validate_url(url_str: &str) -> Result<()> {
    url::Url::parse(url_str)
        .map_err(|e| MultiGitError::invalid_input(format!("Invalid URL: {e}")))?;

    Ok(())
}

/// Validate that a URL uses HTTPS (reject HTTP for security)
pub fn validate_https_url(url_str: &str, allow_insecure: bool) -> Result<String> {
    let parsed_url = url::Url::parse(url_str)
        .map_err(|e| MultiGitError::invalid_input(format!("Invalid URL: {e}")))?;

    if parsed_url.scheme() == "http" && !allow_insecure {
        return Err(MultiGitError::invalid_input(
            "HTTP URLs are not allowed for security reasons. Please use HTTPS.\n\
             If you really need to use HTTP (not recommended), set 'security.allow_insecure_http = true' in your config."
        ));
    }

    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err(MultiGitError::invalid_input(format!(
            "Unsupported URL scheme '{}'. Only HTTP(S) URLs are supported.",
            parsed_url.scheme()
        )));
    }

    // Return normalized URL (with trailing slashes removed)
    Ok(url_str.trim_end_matches('/').to_string())
}

/// Extract host from URL for credential binding
pub fn extract_host_from_url(url_str: &str) -> Result<String> {
    let parsed_url = url::Url::parse(url_str)
        .map_err(|e| MultiGitError::invalid_input(format!("Invalid URL: {e}")))?;

    parsed_url
        .host_str()
        .map(ToString::to_string)
        .ok_or_else(|| MultiGitError::invalid_input("URL does not contain a valid host"))
}

/// Validate an API token (basic checks)
pub fn validate_token(token: &str) -> Result<()> {
    if token.is_empty() {
        return Err(MultiGitError::invalid_input("Token cannot be empty"));
    }

    if token.len() < 20 {
        return Err(MultiGitError::invalid_input(
            "Token appears to be too short. Please check your token.",
        ));
    }

    if token.contains(char::is_whitespace) {
        return Err(MultiGitError::invalid_input(
            "Token cannot contain whitespace",
        ));
    }

    Ok(())
}

/// Sanitize a string for safe display (hide sensitive data)
#[must_use]
pub fn sanitize_token(token: &str) -> String {
    if token.len() <= 8 {
        "*".repeat(token.len())
    } else {
        format!("{}...{}", &token[..4], &token[token.len() - 4..])
    }
}

/// Check if a path is a valid Git repository
#[must_use]
pub fn is_git_repository(path: &std::path::Path) -> bool {
    path.join(".git").exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_repo_name() {
        assert!(validate_repo_name("my-repo").is_ok());
        assert!(validate_repo_name("my_repo").is_ok());
        assert!(validate_repo_name("my.repo").is_ok());
        assert!(validate_repo_name("MyRepo123").is_ok());

        assert!(validate_repo_name("").is_err());
        assert!(validate_repo_name("my repo").is_err());
        assert!(validate_repo_name("my@repo").is_err());
    }

    #[test]
    fn test_validate_username() {
        assert!(validate_username("username").is_ok());
        assert!(validate_username("user-name").is_ok());
        assert!(validate_username("user123").is_ok());

        assert!(validate_username("").is_err());
        assert!(validate_username("-username").is_err());
        assert!(validate_username("username-").is_err());
        assert!(validate_username("user_name").is_err());
    }

    #[test]
    fn test_validate_branch_name() {
        assert!(validate_branch_name("main").is_ok());
        assert!(validate_branch_name("feature/new-feature").is_ok());
        assert!(validate_branch_name("fix-123").is_ok());

        assert!(validate_branch_name("").is_err());
        assert!(validate_branch_name("-branch").is_err());
        assert!(validate_branch_name(".branch").is_err());
        assert!(validate_branch_name("branch.").is_err());
        assert!(validate_branch_name("branch..name").is_err());
        assert!(validate_branch_name("branch*name").is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("https://github.com").is_ok());
        assert!(validate_url("https://api.github.com/users").is_ok());

        assert!(validate_url("not-a-url").is_err());
        assert!(validate_url("").is_err());
    }

    #[test]
    fn test_validate_token() {
        assert!(validate_token("ghp_1234567890abcdefghijklmnop").is_ok());

        assert!(validate_token("").is_err());
        assert!(validate_token("short").is_err());
        assert!(validate_token("token with spaces").is_err());
    }

    #[test]
    fn test_sanitize_token() {
        assert_eq!(
            sanitize_token("ghp_1234567890abcdefghijklmnop"),
            "ghp_...mnop"
        );
        assert_eq!(sanitize_token("short"), "*****");
    }

    #[test]
    fn test_validate_https_url() {
        // HTTPS should always be allowed
        assert!(validate_https_url("https://github.com", false).is_ok());
        assert!(validate_https_url("https://git.example.com/api/v1", false).is_ok());

        // HTTP should be rejected when allow_insecure is false
        assert!(validate_https_url("http://example.com", false).is_err());

        // HTTP should be allowed when allow_insecure is true
        assert!(validate_https_url("http://example.com", true).is_ok());

        // Other schemes should be rejected
        assert!(validate_https_url("ftp://example.com", false).is_err());
        assert!(validate_https_url("git://example.com", false).is_err());
    }

    #[test]
    fn test_extract_host_from_url() {
        assert_eq!(
            extract_host_from_url("https://github.com").unwrap(),
            "github.com"
        );
        assert_eq!(
            extract_host_from_url("https://git.example.com:8080/api").unwrap(),
            "git.example.com"
        );
        assert_eq!(
            extract_host_from_url("https://192.168.1.1/gitlab").unwrap(),
            "192.168.1.1"
        );

        assert!(extract_host_from_url("not-a-url").is_err());
    }
}
