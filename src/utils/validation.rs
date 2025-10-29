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
}
