//! Unit tests for error handling

use multigit::utils::error::MultiGitError;

#[test]
fn test_auth_error() {
    let error = MultiGitError::auth("github", "Invalid token");
    assert!(error.is_auth_error());
    assert!(!error.is_retryable());

    let message = error.to_string();
    assert!(message.contains("github"));
    assert!(message.contains("Invalid token"));
}

#[test]
fn test_config_error() {
    let error = MultiGitError::config("Invalid TOML");
    let message = error.to_string();
    assert!(message.contains("Invalid TOML"));
}

#[test]
fn test_conflict_error() {
    let error = MultiGitError::conflict("Branches diverged");
    let message = error.to_string();
    assert!(message.contains("Branches diverged"));
}

#[test]
fn test_rate_limit_error() {
    let error = MultiGitError::rate_limit("gitlab", 3600);
    assert!(error.is_retryable());

    if let MultiGitError::RateLimitError { provider, minutes } = error {
        assert_eq!(provider, "gitlab");
        assert_eq!(minutes, 60); // 3600 seconds = 60 minutes
    } else {
        panic!("Expected RateLimitError");
    }
}

#[test]
fn test_provider_error() {
    let error = MultiGitError::provider("github", "API returned 500");

    match error {
        MultiGitError::ProviderError { provider, message } => {
            assert_eq!(provider, "github");
            assert_eq!(message, "API returned 500");
        }
        _ => panic!("Expected ProviderError"),
    }
}

#[test]
fn test_daemon_error() {
    let error = MultiGitError::daemon("Failed to start");
    let message = error.to_string();
    assert!(message.contains("Failed to start"));
}

#[test]
fn test_other_error() {
    let error = MultiGitError::other("Something went wrong");
    let message = error.to_string();
    assert_eq!(message, "Something went wrong");
}

#[test]
fn test_user_message_for_auth_error() {
    let error = MultiGitError::auth("github", "Token expired");
    let user_msg = error.user_message();

    assert!(user_msg.contains("github"));
    assert!(user_msg.contains("Token expired"));
    assert!(user_msg.contains("multigit remote test"));
}

#[test]
fn test_user_message_for_not_initialized() {
    let error = MultiGitError::NotInitialized;
    let user_msg = error.user_message();

    assert!(user_msg.contains("multigit init"));
}

#[test]
fn test_is_retryable() {
    assert!(MultiGitError::rate_limit("github", 60).is_retryable());
    assert!(!MultiGitError::auth("github", "bad token").is_retryable());
    assert!(!MultiGitError::config("bad config").is_retryable());
}
