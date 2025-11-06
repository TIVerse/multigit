# Re-Release Instructions for v2.0.0

## 🎯 Problem
The current v2.0.0 release only contains source code archives, but no platform-specific binaries (Windows .exe, macOS, Linux binaries).

## ✅ Solution
The release workflow has been updated to build and upload proper distribution packages for all platforms.

## 📋 Steps to Re-Release

### 1. Commit the Updated Workflow

```bash
# Add the updated release workflow
git add .github/workflows/release.yml

# Commit the changes
git commit -m "fix: update release workflow to include all platform binaries

- Build both multigit and mg binaries
- Create proper tar.gz/zip packages with install scripts
- Include README, LICENSE, and CHANGELOG
- Generate SHA256SUMS for verification
- Add detailed release notes"

# Push to main
git push origin main
```

### 2. Delete the Old Tag (Locally and Remotely)

```bash
# Delete the local tag
git tag -d v2.0.0

# Delete the remote tag
git push origin :refs/tags/v2.0.0
```

### 3. Delete the GitHub Release

Go to: https://github.com/TIVerse/multigit/releases

1. Click on the `v2.0.0` release
2. Click **"Delete"** button
3. Confirm deletion

### 4. Create a New Tag and Push

```bash
# Create a new annotated tag
git tag -a v2.0.0 -m "Release v2.0.0

🎉 MultiGit v2.0.0 - Production Ready

## Features
- Multi-remote synchronization
- Smart conflict detection
- OS keyring integration  
- Daemon mode with scheduler
- Conventional commits wizard
- Cross-platform support

See CHANGELOG.md for full details."

# Push the tag (this will trigger the release workflow)
git push origin v2.0.0
```

### 5. Monitor the Release Build

1. Go to: https://github.com/TIVerse/multigit/actions
2. Watch the "Release" workflow run
3. It will build binaries for:
   - ✅ Linux x86_64 (GNU)
   - ✅ Linux x86_64 (MUSL - static)
   - ✅ macOS x86_64 (Intel)
   - ✅ macOS ARM64 (Apple Silicon)
   - ✅ Windows x86_64

### 6. Verify the Release

Once the workflow completes:

1. Go to: https://github.com/TIVerse/multigit/releases
2. Check that v2.0.0 now has these assets:
   - `multigit-2.0.0-linux-x86_64.tar.gz`
   - `multigit-2.0.0-linux-x86_64-musl.tar.gz`
   - `multigit-2.0.0-macos-x86_64.tar.gz`
   - `multigit-2.0.0-macos-arm64.tar.gz`
   - `multigit-2.0.0-windows-x86_64.zip`
   - `SHA256SUMS`
   - Source code (zip)
   - Source code (tar.gz)

## 🎁 What Each Package Contains

Each release package includes:
- ✅ `multigit` binary (or `multigit.exe` on Windows)
- ✅ `mg` binary (or `mg.exe` on Windows) - command alias
- ✅ `README.md` - Project documentation
- ✅ `LICENSE` - MIT License
- ✅ `CHANGELOG.md` - Version history
- ✅ `install.sh` (Unix) or `install.bat` (Windows) - Easy installation script

## 🚀 User Installation Experience

**Linux/macOS:**
```bash
# Download
curl -LO https://github.com/TIVerse/multigit/releases/download/v2.0.0/multigit-2.0.0-linux-x86_64.tar.gz

# Extract
tar -xzf multigit-2.0.0-linux-x86_64.tar.gz

# Install
cd multigit-2.0.0-linux-x86_64
./install.sh

# Verify
multigit --version
mg --version
```

**Windows:**
```powershell
# Download from GitHub releases page
# Extract the ZIP file
# Run install.bat
# Done!
```

## ⏱️ Expected Build Time
- Total workflow time: ~15-20 minutes
- Building all 5 platforms in parallel

## 🔧 Troubleshooting

### If the workflow fails:
1. Check the Actions tab for error logs
2. Common issues:
   - MUSL build: May need `musl-tools` in runner
   - Windows build: Check for `7z` availability
   - Permission errors: Check `GITHUB_TOKEN` permissions

### If binaries are missing:
1. Check artifact upload step in workflow logs
2. Verify all build jobs completed successfully
3. Check the `create-release` job logs

## ✨ One-Time Fix
Once you complete these steps, all future releases will automatically include all platform binaries when you push a new tag!

---

**Need help?** Check the workflow logs or open an issue.
