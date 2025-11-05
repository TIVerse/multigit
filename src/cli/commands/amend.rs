//! Quick amend last commit
//!
//! Easily amend the last commit with new changes or updated message

use crate::cli::interactive;
use crate::utils::error::{MultiGitError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Select};
use std::process::Command;

/// Execute amend
pub fn execute(no_edit: bool) -> Result<()> {
    println!("\n✏️  Amend Last Commit\n");

    // Show last commit
    let output = Command::new("git")
        .args(["log", "-1", "--pretty=format:%h - %s"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get last commit: {e}")))?;

    let last_commit = String::from_utf8_lossy(&output.stdout);
    println!("Last commit: {last_commit}\n");

    let options = vec![
        "📝 Amend with staged changes (keep message)",
        "✏️  Amend and edit message",
        "📦 Stage all changes and amend",
        "🔄 Amend author/committer info",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(usize::from(!no_edit))
        .interact()?;

    match choice {
        0 => amend_no_edit()?,
        1 => amend_with_edit()?,
        2 => amend_all()?,
        3 => amend_author()?,
        _ => {}
    }

    Ok(())
}

/// Amend without editing message
fn amend_no_edit() -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "--amend", "--no-edit"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to amend: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ Commit amended!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to amend: {error}")));
    }

    Ok(())
}

/// Amend with message edit
fn amend_with_edit() -> Result<()> {
    // Get current commit message
    let output = Command::new("git")
        .args(["log", "-1", "--pretty=format:%B"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to get commit message: {e}")))?;

    let current_message = String::from_utf8_lossy(&output.stdout);

    if let Some(new_message) = Editor::new().edit(&current_message)? {
        let output = Command::new("git")
            .args(["commit", "--amend", "-m", &new_message])
            .output()
            .map_err(|e| MultiGitError::other(format!("Failed to amend: {e}")))?;

        if output.status.success() {
            interactive::print_success("✅ Commit amended with new message!");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(MultiGitError::other(format!("Failed to amend: {error}")));
        }
    } else {
        println!("Amend cancelled.");
    }

    Ok(())
}

/// Stage all and amend
fn amend_all() -> Result<()> {
    // Stage all changes
    let output = Command::new("git")
        .args(["add", "-A"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to stage: {e}")))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to stage: {error}")));
    }

    let edit = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Edit commit message?")
        .default(false)
        .interact()?;

    let mut args = vec!["commit", "--amend"];
    if !edit {
        args.push("--no-edit");
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to amend: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ All changes staged and commit amended!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to amend: {error}")));
    }

    Ok(())
}

/// Amend author info
fn amend_author() -> Result<()> {
    use dialoguer::Input;

    let author: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author name and email (e.g., 'John Doe <john@example.com>')")
        .interact_text()?;

    let output = Command::new("git")
        .args(["commit", "--amend", "--author", &author, "--no-edit"])
        .output()
        .map_err(|e| MultiGitError::other(format!("Failed to amend: {e}")))?;

    if output.status.success() {
        interactive::print_success("✅ Author updated!");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(MultiGitError::other(format!("Failed to amend: {error}")));
    }

    Ok(())
}
