# 🚀 Quick Multi-Remote Test Guide

## Option 1: Using Environment Variables (Non-Interactive)

```bash
# 1. Set your tokens as environment variables
export MULTIGIT_GITHUB_TOKEN="ghp_your_github_token_here"
export MULTIGIT_GITLAB_TOKEN="glpat_your_gitlab_token_here"

# 2. Run the test script
./RUN_MULTI_REMOTE_TEST.sh
```

## Option 2: Interactive (Prompted for Tokens)

```bash
# Just run the script - it will prompt you for tokens
./RUN_MULTI_REMOTE_TEST.sh
```

## Option 3: Manual Step-by-Step

### Get Your Tokens First:

**GitHub Token:**
1. Go to: https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select scopes: `repo`, `read:user`
4. Generate and copy the token (starts with `ghp_`)

**GitLab Token:**
1. Go to: https://gitlab.com/-/profile/personal_access_tokens
2. Click "Add new token"
3. Select scopes: `api`, `write_repository`
4. Create and copy the token (starts with `glpat-`)

### Then Run:

```bash
# Add GitHub
./target/release/multigit remote add github TIVerse
# Paste your GitHub token when prompted

# Add GitLab  
./target/release/multigit remote add gitlab TIVisionOSS
# Paste your GitLab token when prompted

# Test connections
./target/release/multigit remote test github
./target/release/multigit remote test gitlab

# List remotes
./target/release/multigit remote list

# Dry run (safe)
./target/release/multigit sync --dry-run

# Real push to BOTH remotes
./target/release/multigit push
```

## What Will Happen:

1. ✅ Tokens stored securely in OS keyring (never plain text)
2. ✅ Both remotes tested for connectivity
3. ✅ Dry-run shows what would be pushed
4. ✅ Real push sends to GitHub AND GitLab simultaneously
5. ✅ You'll see timing for each remote

## Expected Output:

```
🚀 Pushing 'master' to 2 remote(s)...

✓ github - pushed in 1250ms
✓ gitlab - pushed in 980ms

📊 Summary: 2 succeeded, 0 failed
```

---

## 🔒 Security Notes:

- Tokens are stored in OS keyring (Keychain on macOS, Credential Manager on Windows, Secret Service on Linux)
- Config file (.multigit/config.toml) does NOT contain tokens, only usernames
- Tokens never appear in git history or logs
- You can remove remotes anytime with: `multigit remote remove <name>`

---

## 🐛 Troubleshooting:

**If GitHub connection fails:**
```bash
# Check token has correct scopes
./target/release/multigit remote test github
# Update token if needed
./target/release/multigit remote update github
```

**If GitLab connection fails:**
```bash
# Check token has correct scopes  
./target/release/multigit remote test gitlab
# Update token if needed
./target/release/multigit remote update gitlab
```

**Remove and re-add a remote:**
```bash
./target/release/multigit remote remove github
./target/release/multigit remote add github TIVerse
```

---

## Ready to Test?

Choose one of the options above and let's test pushing to both remotes! 🚀
