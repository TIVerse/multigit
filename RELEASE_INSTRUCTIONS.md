# Release Instructions for v2.0.0

## Files Updated
- ✅ `Cargo.toml` - Version bumped to 2.0.0
- ✅ `CHANGELOG.md` - Unreleased section moved to [2.0.0] with date 2025-11-06
- ✅ `RELEASE_NOTES_v2.0.0.md` - Created for GitHub release
- ✅ `devto.md` - Dev.to article ready for publishing

---

## Step 1: Commit Version Bump

```bash
# Stage the version changes
git add Cargo.toml CHANGELOG.md RELEASE_NOTES_v2.0.0.md devto.md RELEASE_INSTRUCTIONS.md

# Commit with conventional commit message
git commit -m "chore(release): bump version to v2.0.0

- Update Cargo.toml version to 2.0.0
- Move unreleased changes to 2.0.0 in CHANGELOG.md
- Add release notes for v2.0.0
- Add comprehensive dev.to article"
```

---

## Step 2: Create Git Tag

```bash
# Create annotated tag
git tag -a v2.0.0 -m "Release v2.0.0

Major release with:
- Conventional Commit helper (mg cc)
- Short command alias (mg)
- Unified provider factory
- Remote health checks
- Multiple bug fixes and improvements

See RELEASE_NOTES_v2.0.0.md for full details."
```

---

## Step 3: Push to GitHub

```bash
# Push commits
git push origin main

# Push tag
git push origin v2.0.0
```

---

## Step 4: Create GitHub Release

### Option A: Using GitHub CLI (Recommended)

```bash
# Install gh CLI if not already installed
# https://cli.github.com/

# Create release with release notes
gh release create v2.0.0 \
  --title "MultiGit v2.0.0 - Conventional Commits & Enhanced UX" \
  --notes-file RELEASE_NOTES_v2.0.0.md \
  --latest

# The release will be automatically published
```

### Option B: Using GitHub Web UI

1. Go to: https://github.com/TIVerse/multigit/releases/new

2. **Choose a tag**: Select `v2.0.0` from dropdown

3. **Release title**: `MultiGit v2.0.0 - Conventional Commits & Enhanced UX`

4. **Description**: Copy-paste content from `RELEASE_NOTES_v2.0.0.md`

5. **Set as latest release**: ✅ Check this box

6. **Publish release**: Click the green button

---

## Step 5: Publish to crates.io (Optional)

```bash
# Ensure you're logged in to crates.io
cargo login

# Publish to crates.io
cargo publish --dry-run  # Test first
cargo publish            # Actually publish
```

---

## Step 6: Verify Release

After publishing, verify:

1. **GitHub Release**: https://github.com/TIVerse/multigit/releases/tag/v2.0.0
2. **Crates.io**: https://crates.io/crates/multigit (should show v2.0.0)
3. **Docs.rs**: https://docs.rs/multigit (will auto-update from crates.io)

---

## Step 7: Announce Release (Optional)

### Update README.md badges (if needed)
Badges should auto-update, but verify they show v2.0.0:
- Crates.io version badge
- Docs.rs badge

### Social Media / Community
Consider announcing on:
- Twitter/X
- Reddit (r/rust, r/programming)
- Hacker News
- Dev.to (publish the devto.md article)
- Discord communities

### Dev.to Article
The `devto.md` file is ready to publish:
1. Go to https://dev.to/new
2. Copy content from `devto.md`
3. Update front matter: set `published: true`
4. Add cover image (optional)
5. Publish!

---

## Rollback (If Needed)

If something goes wrong:

```bash
# Delete the tag locally
git tag -d v2.0.0

# Delete the tag on GitHub
git push --delete origin v2.0.0

# Delete GitHub release via web UI or:
gh release delete v2.0.0

# Revert the version commit
git revert HEAD
git push origin main
```

---

## Post-Release Checklist

- [ ] GitHub release created and visible
- [ ] Tag v2.0.0 pushed to GitHub
- [ ] Crates.io shows v2.0.0 (if published)
- [ ] Docs.rs updated to v2.0.0
- [ ] README badges show v2.0.0
- [ ] Release announcement posted (optional)
- [ ] Dev.to article published (optional)
- [ ] Update project board/issues as needed

---

## Next Steps

After v2.0.0 is released, consider:

1. Start a new `[Unreleased]` section in CHANGELOG.md
2. Plan roadmap items for v2.1.0 or v3.0.0
3. Respond to user feedback and issues
4. Continue development on new features

---

## Need Help?

- **GitHub Releases docs**: https://docs.github.com/en/repositories/releasing-projects-on-github
- **Cargo publish docs**: https://doc.rust-lang.org/cargo/reference/publishing.html
- **GitHub CLI**: https://cli.github.com/manual/

---

**Ready to release? Start with Step 1! 🚀**
