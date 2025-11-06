//! Secret redaction utilities for safe logging
//!
//! Provides functions to sanitize log output by redacting common secret patterns.

use regex::Regex;
use std::sync::OnceLock;

/// Redact common secret patterns from text
///
/// This function masks various types of secrets commonly found in logs:
/// - GitHub tokens (ghp_, gho_, ghs_, etc.)
/// - GitLab tokens (glpat_)
/// - Generic Bearer tokens
/// - JWT tokens
/// - URL-embedded credentials
/// - Key-value pairs with sensitive keys
///
/// # Arguments
/// * `text` - The text to redact
///
/// # Returns
/// A new string with secrets masked as `***REDACTED***`
pub fn redact(text: &str) -> String {
    let mut result = text.to_string();

    // GitHub tokens (ghp_, gho_, ghs_, ghr_, ghv_, github_pat_)
    static GITHUB_TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();
    let github_regex = GITHUB_TOKEN_REGEX.get_or_init(|| {
        Regex::new(r"(gh[psorv]_[a-zA-Z0-9]{36,}|github_pat_[a-zA-Z0-9_]{82})").unwrap()
    });
    result = github_regex.replace_all(&result, "***REDACTED***").to_string();

    // GitLab tokens (glpat-)
    static GITLAB_TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();
    let gitlab_regex = GITLAB_TOKEN_REGEX.get_or_init(|| {
        Regex::new(r"glpat-[a-zA-Z0-9_-]{20,}").unwrap()
    });
    result = gitlab_regex.replace_all(&result, "***REDACTED***").to_string();

    // Bearer tokens
    static BEARER_REGEX: OnceLock<Regex> = OnceLock::new();
    let bearer_regex = BEARER_REGEX.get_or_init(|| {
        Regex::new(r"(?i)bearer\s+[a-zA-Z0-9_\-\.]{20,}").unwrap()
    });
    result = bearer_regex.replace_all(&result, "Bearer ***REDACTED***").to_string();

    // Generic JWT tokens (three base64 segments separated by dots)
    static JWT_REGEX: OnceLock<Regex> = OnceLock::new();
    let jwt_regex = JWT_REGEX.get_or_init(|| {
        Regex::new(r"eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*").unwrap()
    });
    result = jwt_regex.replace_all(&result, "***REDACTED_JWT***").to_string();

    // URL-embedded credentials (username:password@host)
    static URL_CRED_REGEX: OnceLock<Regex> = OnceLock::new();
    let url_cred_regex = URL_CRED_REGEX.get_or_init(|| {
        Regex::new(r"://([^:@\s]+):([^@\s]+)@").unwrap()
    });
    result = url_cred_regex.replace_all(&result, "://***:***@").to_string();

    // Key-value pairs with sensitive keys (token=, password=, secret=, key=, api_key=, auth=)
    static KEY_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();
    let key_value_regex = KEY_VALUE_REGEX.get_or_init(|| {
        Regex::new(r#"(?i)(token|password|secret|key|api_key|auth|passwd|pwd)([=:]\s*['"]?)([^\s'"&,;]+)"#).unwrap()
    });
    result = key_value_regex.replace_all(&result, "$1$2***REDACTED***").to_string();

    // AWS keys
    static AWS_KEY_REGEX: OnceLock<Regex> = OnceLock::new();
    let aws_key_regex = AWS_KEY_REGEX.get_or_init(|| {
        Regex::new(r"(?:A3T[A-Z0-9]|AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}").unwrap()
    });
    result = aws_key_regex.replace_all(&result, "***REDACTED_AWS***").to_string();

    result
}

/// Redact secrets specifically for command-line output
///
/// Similar to `redact()` but also masks common CLI flag patterns
pub fn redact_cli(text: &str) -> String {
    let mut result = redact(text);

    // CLI flags with sensitive values (--token, --password, --secret, -t, -p)
    static CLI_FLAG_REGEX: OnceLock<Regex> = OnceLock::new();
    let cli_flag_regex = CLI_FLAG_REGEX.get_or_init(|| {
        Regex::new(r"(?i)(-{1,2}(?:token|password|secret|auth|key|passwd|pwd)(?:[=\s]+))([^\s]+)").unwrap()
    });
    result = cli_flag_regex.replace_all(&result, "$1***REDACTED***").to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_github_tokens() {
        let text = "Token: ghp_1234567890abcdefghijklmnopqrstuvwxyz";
        let redacted = redact(text);
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("ghp_"));
    }

    #[test]
    fn test_redact_gitlab_tokens() {
        let text = "GitLab token: glpat-1234567890abcdefghij";
        let redacted = redact(text);
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("glpat-"));
    }

    #[test]
    fn test_redact_bearer_tokens() {
        let text = "Authorization: Bearer abc123xyz456def789ghijklmnop";
        let redacted = redact(text);
        assert!(redacted.contains("Bearer ***REDACTED***"));
        assert!(!redacted.contains("abc123xyz456def789"));
    }

    #[test]
    fn test_redact_jwt() {
        let text = "JWT: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let redacted = redact(text);
        assert!(redacted.contains("***REDACTED_JWT***"));
        assert!(!redacted.contains("eyJ"));
    }

    #[test]
    fn test_redact_url_credentials() {
        let text = "URL: https://user:password123@github.com/repo.git";
        let redacted = redact(text);
        assert!(redacted.contains("://***:***@"));
        assert!(!redacted.contains("password123"));
    }

    #[test]
    fn test_redact_key_value_pairs() {
        let text = "token=abc123 password=secret123 api_key=xyz789";
        let redacted = redact(text);
        assert!(redacted.contains("token=***REDACTED***"));
        assert!(redacted.contains("password=***REDACTED***"));
        assert!(redacted.contains("api_key=***REDACTED***"));
        assert!(!redacted.contains("abc123"));
        assert!(!redacted.contains("secret123"));
    }

    #[test]
    fn test_redact_aws_keys() {
        let text = "AWS Key: AKIAIOSFODNN7EXAMPLE";
        let redacted = redact(text);
        eprintln!("Original: {text}");
        eprintln!("Redacted: {redacted}");
        // AWS key pattern requires all uppercase, but key-value regex catches it first
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("AKIAIOSFODNN7EXAMPLE"));
    }

    #[test]
    fn test_redact_cli_flags() {
        let text = "Command: multigit --token ghp_1234567890abcdefghijklmnopqrstuvwxyz --password secret123";
        let redacted = redact_cli(text);
        assert!(redacted.contains("--token ***REDACTED***"));
        assert!(redacted.contains("--password ***REDACTED***"));
        assert!(!redacted.contains("secret123"));
        assert!(!redacted.contains("ghp_"));
    }

    #[test]
    fn test_redact_preserves_safe_text() {
        let text = "This is a normal log message with no secrets.";
        let redacted = redact(text);
        assert_eq!(redacted, text);
    }

    #[test]
    fn test_redact_multiple_secrets() {
        let text = "Token ghp_1234567890abcdefghijklmnopqrstuvwxyz and password=secret123 at https://user:pass@host.com";
        let redacted = redact(text);
        assert!(redacted.contains("***REDACTED***"));
        assert!(redacted.contains("://***:***@"));
        assert!(!redacted.contains("ghp_1234567890abcdefghijklmnopqrstuvwxyz"));
        assert!(!redacted.contains("secret123"));
        assert!(!redacted.contains("pass@"));
    }
}
