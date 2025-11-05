# 🚀 MultiGit - Easy Setup Guide

**New in v1.0.1**: Super simple setup wizard! Get started in 2 minutes. ⏱️

---

## ✨ The Easiest Way to Get Started

### Step 1: Run the Setup Wizard

```bash
multigit setup
```

That's it! The wizard will guide you through everything:
- ✅ Initialize MultiGit
- ✅ Choose your Git providers
- ✅ Set up authentication
- ✅ Configure preferences

### What You'll See:

```
╔══════════════════════════════════════════════╗
║                                              ║
║     🚀 Welcome to MultiGit Setup Wizard     ║
║                                              ║
╚══════════════════════════════════════════════╝

This wizard will help you set up MultiGit in 3 easy steps:

  1️⃣  Initialize MultiGit
  2️⃣  Add your Git hosting providers
  3️⃣  Configure your preferences

? Ready to start? (Y/n)
```

---

## 🎯 Quick Setup (For Single Provider)

If you just want to add one provider quickly:

```bash
# Quick setup for GitHub
multigit setup --provider github --username YourUsername

# Quick setup for GitLab
multigit setup --provider gitlab --username YourUsername
```

---

## 📝 What Information You'll Need

### For GitHub:
- **Username**: Your GitHub username
- **Token**: Personal Access Token from https://github.com/settings/tokens
  - Just click "Generate new token (classic)"
  - Select: `repo`, `read:user`
  - Copy the token (starts with `ghp_`)

### For GitLab:
- **Username**: Your GitLab username  
- **Token**: Personal Access Token from https://gitlab.com/-/profile/personal_access_tokens
  - Click "Add new token"
  - Select: `api`, `write_repository`
  - Copy the token (starts with `glpat-`)

### For Bitbucket:
- **Username**: Your Bitbucket username
- **App Password**: From https://bitbucket.org/account/settings/app-passwords/
  - Select: Repositories (Read, Write)

---

## 🎬 Example Session

```bash
$ multigit setup

╔══════════════════════════════════════════════╗
║     🚀 Welcome to MultiGit Setup Wizard     ║
╚══════════════════════════════════════════════╝

? Ready to start? Yes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📝 Step 1: Initialize MultiGit
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ MultiGit initialized successfully!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔗 Step 2: Add Git Hosting Providers
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Select which providers you want to use:
(Use Space to select, Enter to confirm)

❯ ◉ GitHub
  ◯ GitLab
  ◯ Bitbucket
  ◯ Codeberg
  ◯ Gitea (self-hosted)

╭──────────────────────────────────────────╮
│  Setting up: GitHub                      │
╰──────────────────────────────────────────╯

? Enter your github username: TIVerse

📝 How to get your github token:

   1. Go to: https://github.com/settings/tokens
   2. Click 'Generate new token (classic)'
   3. Select scopes: repo, read:user
   4. Click 'Generate token' and copy it

🔒 Your token will be stored securely in your OS keyring.
   It will NEVER be stored in plain text.

? Enter your github token: ••••••••••••••••••••

🔍 Testing connection...
✅ Connection successful!
✅ Credentials stored securely
✅ github added to configuration

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⚙️  Step 3: Configure Preferences (Optional)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

? Configure advanced settings? No
✅ Using default settings

╔══════════════════════════════════════════════╗
║     🎉 Setup Complete! You're ready!         ║
╚══════════════════════════════════════════════╝

📚 Next Steps:

   1. Check your configuration:
      multigit status

   2. Test your connections:
      multigit remote test --all

   3. Push to all remotes:
      multigit push

   4. Start background sync:
      multigit daemon start --interval 15m
```

---

## 🆚 Old Way vs New Way

### ❌ Old Way (Complex):
```bash
# Step 1: Initialize
multigit init

# Step 2: Add remote with all details
multigit remote add github TIVerse

# Prompted for token...
# Need to know token URL...
# Need to know scopes...
# Need to test connection manually...
```

### ✅ New Way (Simple):
```bash
# One command does everything!
multigit setup
# Wizard guides you through each step
# Clear instructions for each provider
# Automatic connection testing
# Everything configured for you
```

---

## 🔄 Complete Example: GitHub + GitLab

```bash
# 1. Run setup
$ multigit setup

# 2. Select both GitHub and GitLab
# 3. Follow prompts for each provider
# 4. Done!

# Now you can:
$ multigit status
Current branch: main
Working directory: clean

Remote status:
  ✓ github (@TIVerse)
  ✓ gitlab (@TIVisionOSS)

# Push to both with one command
$ multigit push

🚀 Pushing 'main' to 2 remote(s)...

✓ github - pushed in 1250ms
✓ gitlab - pushed in 980ms

📊 Summary: 2 succeeded, 0 failed
```

---

## 💡 Tips

1. **Get tokens ready beforehand**
   - Have your GitHub/GitLab token pages open
   - Generate tokens before running setup
   - Copy-paste makes it faster

2. **Use quick setup for single provider**
   ```bash
   multigit setup --provider github --username YourName
   ```

3. **Run setup multiple times**
   - You can run `multigit setup` again to add more providers
   - It won't affect existing configuration

4. **Skip advanced settings**
   - Defaults work great for most users
   - You can change settings later

---

## 🔒 Security

- ✅ Tokens stored in OS keyring (never plain text)
- ✅ Automatic connection validation
- ✅ Clear permission requirements
- ✅ No secrets in git history
- ✅ Audit logging (optional)

---

## 🐛 Troubleshooting

**"Connection failed" error:**
- Check your token has correct permissions
- Make sure token hasn't expired
- Verify username is correct

**"Provider not supported" error:**
- Currently supported: GitHub, GitLab (more coming soon)
- Use manual setup for other providers:
  ```bash
  multigit remote add <provider> <username>
  ```

**Want to start over?**
```bash
# Remove .multigit directory
rm -rf .multigit

# Run setup again
multigit setup
```

---

## 📚 More Help

- **View current setup**: `multigit status`
- **Test connections**: `multigit remote test --all`
- **Add more remotes**: `multigit setup` (run again)
- **Manual setup**: `multigit remote add --help`
- **Full docs**: https://github.com/TIVerse/multigit

---

**Made getting started easier? ⭐ Star us on GitHub!**
