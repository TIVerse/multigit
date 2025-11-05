//! Conventional commit helper
//!
//! Interactive tool for creating well-formatted conventional commits.

use crate::cli::interactive;
use crate::git::operations::GitOperations;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, MultiSelect, Select};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

/// Conventional commit types
const COMMIT_TYPES: &[(&str, &str)] = &[
    ("feat", "✨ A new feature"),
    ("fix", "🐛 A bug fix"),
    ("docs", "📚 Documentation only changes"),
    (
        "style",
        "💎 Code style changes (formatting, semicolons, etc.)",
    ),
    (
        "refactor",
        "♻️  Code refactoring without changing functionality",
    ),
    ("perf", "⚡ Performance improvements"),
    ("test", "✅ Adding or updating tests"),
    ("build", "🔨 Build system or external dependencies"),
    ("ci", "👷 CI/CD configuration changes"),
    (
        "chore",
        "🔧 Other changes that don't modify src or test files",
    ),
    ("revert", "⏪ Revert a previous commit"),
];

/// Common scopes based on project structure
const COMMON_SCOPES: &[&str] = &[
    "cli",
    "core",
    "git",
    "providers",
    "api",
    "daemon",
    "ui",
    "utils",
    "config",
    "auth",
    "sync",
    "health",
    "error",
    "docs",
    "test",
];

/// Execute conventional commit workflow
pub fn execute() -> Result<()> {
    println!("\n🎯 Conventional Commit Helper\n");

    // Check if we're in a git repository
    let git_ops = GitOperations::open(".").map_err(|_| {
        MultiGitError::other("Not in a git repository. Run this command from a git repository.")
    })?;

    // Get modified files
    let modified_files = get_modified_files(&git_ops)?;

    if modified_files.is_empty() {
        interactive::print_info("No modified files found. Make some changes first!");
        return Ok(());
    }

    // Step 1: Select files to stage
    let selected_files = select_files_to_stage(&modified_files)?;

    if selected_files.is_empty() {
        interactive::print_info("No files selected. Commit cancelled.");
        return Ok(());
    }

    // Stage the selected files
    stage_files(&selected_files)?;

    // Step 2: Select commit type
    let commit_type = select_commit_type()?;

    // Step 3: Enter scope (optional, with smart suggestions)
    let scope = select_scope(&selected_files)?;

    // Step 4: Enter short description
    let description = enter_description()?;

    // Step 5: Enter long description (optional)
    let body = enter_body()?;

    // Step 6: Breaking change?
    let breaking = is_breaking_change()?;

    // Step 7: Footer (e.g., issue references)
    let footer = enter_footer()?;

    // Build the commit message
    let commit_message = build_commit_message(
        &commit_type,
        scope.as_deref(),
        &description,
        body.as_deref(),
        breaking,
        footer.as_deref(),
    );

    // Step 8: Preview and confirm
    println!("\n📝 Commit Message Preview:\n");
    println!("{}", "─".repeat(60));
    println!("{commit_message}");
    println!("{}", "─".repeat(60));
    println!();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit with this message?")
        .default(true)
        .interact()?;

    if !confirm {
        let edit = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Edit in your editor?")
            .default(false)
            .interact()?;

        if edit {
            if let Some(edited) = Editor::new().edit(&commit_message)? {
                commit_with_message(&edited)?;
                interactive::print_success("✅ Commit created successfully!");
                return Ok(());
            }
            interactive::print_info("Commit cancelled.");
            return Ok(());
        }
        interactive::print_info("Commit cancelled.");
        return Ok(());
    }

    // Commit
    commit_with_message(&commit_message)?;
    interactive::print_success("✅ Commit created successfully!");

    Ok(())
}

/// Get list of modified files (excluding ignored files)
fn get_modified_files(git_ops: &GitOperations) -> Result<Vec<(String, String)>> {
    let repo = git_ops.inner();
    let mut files = Vec::new();

    // Get status, excluding ignored and untracked files in ignored directories
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);
    status_opts.exclude_submodules(true);

    let statuses = repo
        .statuses(Some(&mut status_opts))
        .map_err(|e| MultiGitError::other(format!("Failed to get status: {e}")))?;

    for entry in statuses.iter() {
        let status = entry.status();

        // Skip ignored files
        if status.is_ignored() {
            continue;
        }

        let path = entry.path().unwrap_or("").to_string();

        let status_str = if status.is_wt_new() || status.is_index_new() {
            "new"
        } else if status.is_wt_modified() || status.is_index_modified() {
            "modified"
        } else if status.is_wt_deleted() || status.is_index_deleted() {
            "deleted"
        } else if status.is_wt_renamed() || status.is_index_renamed() {
            "renamed"
        } else {
            "changed"
        };

        if !path.is_empty() {
            files.push((path, status_str.to_string()));
        }
    }

    Ok(files)
}

/// Let user select files to stage
fn select_files_to_stage(files: &[(String, String)]) -> Result<Vec<String>> {
    println!("📂 Select files to include in this commit:\n");

    // Quick options
    let quick_options = vec!["✅ All files", "🎯 Select individually"];

    let quick_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to select files?")
        .items(&quick_options)
        .default(0)
        .interact()?;

    if quick_choice == 0 {
        // All files selected
        println!("\n✅ All {} file(s) will be committed\n", files.len());
        for (path, status) in files {
            println!("  [{status}] {path}");
        }
        println!();
        return Ok(files.iter().map(|(path, _)| path.clone()).collect());
    }

    // Individual selection
    println!();
    let options: Vec<String> = files
        .iter()
        .map(|(path, status)| format!("[{status}] {path}"))
        .collect();

    let defaults = vec![true; files.len()]; // All selected by default

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Files to commit (Space to toggle, Enter to confirm)")
        .items(&options)
        .defaults(&defaults)
        .interact()?;

    let selected: Vec<String> = selections.iter().map(|&i| files[i].0.clone()).collect();

    Ok(selected)
}

/// Stage files using git add
fn stage_files(files: &[String]) -> Result<()> {
    for file in files {
        Command::new("git")
            .args(["add", file])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to stage {file}: {e}")))?;
    }
    Ok(())
}

/// Let user select commit type
fn select_commit_type() -> Result<String> {
    println!();
    let items: Vec<String> = COMMIT_TYPES
        .iter()
        .map(|(name, desc)| format!("{name:12} {desc}"))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commit type")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(COMMIT_TYPES[selection].0.to_string())
}

/// Let user select or enter a scope
fn select_scope(selected_files: &[String]) -> Result<Option<String>> {
    println!();

    // Detect scopes from selected files
    let detected_scopes = detect_scopes_from_files(selected_files);

    let mut scope_options: Vec<String> = Vec::new();
    scope_options.push("(no scope)".to_string());

    // Add detected scopes first
    for scope in &detected_scopes {
        scope_options.push(format!("{scope} (detected)"));
    }

    // Add common scopes that weren't detected
    for scope in COMMON_SCOPES {
        if !detected_scopes.contains(*scope) {
            scope_options.push((*scope).to_string());
        }
    }

    scope_options.push("(enter custom scope)".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scope (optional)")
        .items(&scope_options)
        .default(0)
        .interact()?;

    if selection == 0 {
        // No scope
        Ok(None)
    } else if selection == scope_options.len() - 1 {
        // Custom scope
        let custom: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom scope")
            .allow_empty(true)
            .interact_text()?;

        if custom.is_empty() {
            Ok(None)
        } else {
            Ok(Some(custom))
        }
    } else {
        // Selected from list
        let scope = scope_options[selection]
            .replace(" (detected)", "")
            .to_string();
        Ok(Some(scope))
    }
}

/// Detect scopes from file paths
fn detect_scopes_from_files(files: &[String]) -> HashSet<String> {
    let mut scopes = HashSet::new();

    for file in files {
        let path = PathBuf::from(file);

        // Try to extract scope from path
        if let Some(components) = path.parent() {
            let parts: Vec<_> = components.components().collect();

            // Check src/ subdirectories
            if parts.len() >= 2 {
                if let Some(dir) = parts[1].as_os_str().to_str() {
                    if COMMON_SCOPES.contains(&dir) {
                        scopes.insert(dir.to_string());
                    }
                }
            }

            // Also check immediate parent for files in src/
            if !parts.is_empty() {
                if let Some(dir) = parts[0].as_os_str().to_str() {
                    if dir == "tests" {
                        scopes.insert("test".to_string());
                    } else if std::path::Path::new(file)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
                    {
                        scopes.insert("docs".to_string());
                    } else if file == "Cargo.toml" || file == "Cargo.lock" {
                        scopes.insert("build".to_string());
                    }
                }
            }
        }
    }

    scopes
}

/// Enter short description
fn enter_description() -> Result<String> {
    println!();
    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Short description (imperative mood: 'add feature' not 'added feature')")
        .validate_with(|input: &String| -> std::result::Result<(), &str> {
            if input.trim().is_empty() {
                Err("Description cannot be empty")
            } else if input.len() > 72 {
                Err("Description should be 72 characters or less")
            } else if input.chars().next().is_some_and(char::is_uppercase) {
                Err("Start with lowercase (conventional commits style)")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    Ok(description.trim().to_string())
}

/// Enter long description/body
fn enter_body() -> Result<Option<String>> {
    println!();
    let add_body = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Add detailed description (body)?")
        .default(false)
        .interact()?;

    if !add_body {
        return Ok(None);
    }

    if let Some(body) = Editor::new().edit("# Enter detailed description here\n")? {
        let cleaned = body
            .lines()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        if cleaned.is_empty() {
            Ok(None)
        } else {
            Ok(Some(cleaned))
        }
    } else {
        Ok(None)
    }
}

/// Check if this is a breaking change
fn is_breaking_change() -> Result<bool> {
    println!();
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is this a BREAKING CHANGE?")
        .default(false)
        .interact()
        .map_err(|e| MultiGitError::other(format!("Input error: {e}")))
}

/// Enter footer (e.g., issue references)
fn enter_footer() -> Result<Option<String>> {
    println!();
    let footer: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Footer (e.g., 'Closes #123', 'Refs #456')")
        .allow_empty(true)
        .interact_text()?;

    if footer.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(footer.trim().to_string()))
    }
}

/// Build the conventional commit message
fn build_commit_message(
    commit_type: &str,
    scope: Option<&str>,
    description: &str,
    body: Option<&str>,
    breaking: bool,
    footer: Option<&str>,
) -> String {
    let mut message = String::new();

    // First line: type(scope): description
    message.push_str(commit_type);
    if let Some(s) = scope {
        message.push('(');
        message.push_str(s);
        message.push(')');
    }
    if breaking {
        message.push('!');
    }
    message.push_str(": ");
    message.push_str(description);

    // Body
    if let Some(b) = body {
        message.push_str("\n\n");
        message.push_str(b);
    }

    // Breaking change notice
    if breaking {
        message.push_str("\n\nBREAKING CHANGE: ");
        if let Some(b) = body {
            // Use first line of body as breaking change description
            if let Some(first_line) = b.lines().next() {
                message.push_str(first_line);
            }
        } else {
            message.push_str(description);
        }
    }

    // Footer
    if let Some(f) = footer {
        message.push_str("\n\n");
        message.push_str(f);
    }

    message
}

/// Commit with the given message
fn commit_with_message(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to commit: {e}")))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Commit failed: {error}")));
    }

    Ok(())
}
