# Security Findings Task List

This checklist captures all identified security issues, with actionable tasks, file references, and acceptance criteria. Tackle HIGH first.

## HIGH

- [ ] Bind credentials to host (prevent confused-deputy)
  - Description: Tokens are stored under `"{provider}:{username}:token"` (host-agnostic). Self‑hosted URLs could exfiltrate tokens to attacker hosts.
  - Files:
    - `src/security/keyring.rs`
    - Call sites: `src/providers/{github.rs,gitlab.rs,gitea.rs,bitbucket.rs,codeberg.rs}`, `src/providers/factory.rs`, `src/cli/commands/{remote.rs,setup.rs}`
  - Tasks:
    - [ ] Change key format to `"{provider}:{host}:{username}:token"` in `store_*`/`retrieve_*`/`delete_*`.
    - [ ] Derive `host` via `url::Url` from `base_url`/`api_url`; SaaS hosts are constants (`github.com`, `gitlab.com`, `bitbucket.org`, `codeberg.org`).
    - [ ] Add migration: on retrieve, if new key missing, try legacy key, then re-store under new key.
    - [ ] Add tests covering SaaS, self-hosted, and migration.
  - Acceptance: Provider calls only succeed when host-bound token present; legacy tokens transparently migrated; tests pass.

- [ ] Enforce HTTPS for self‑hosted providers by default
  - Description: Custom `api_url`/`base_url` are accepted as-is; `http://` allows MitM/token leak.
  - Files:
    - `src/cli/interactive.rs`, `src/cli/commands/setup.rs`
    - `src/providers/{gitlab.rs,gitea.rs}`
    - `src/utils/validation.rs` (new helper)
  - Tasks:
    - [ ] Add `validate_https_url(url: &str) -> Result<()>` helper (reject `http://`).
    - [ ] Validate/normalize URLs during setup and provider construction; error with actionable message.
    - [ ] Optional opt‑in: config flag `security.allow_insecure_http` (default false) to bypass with clear warning.
    - [ ] Unit tests for acceptance/rejection.
  - Acceptance: `http://` URLs are rejected unless explicit insecure flag is set; tests pass.

## MEDIUM

- [ ] Pin GitHub Actions to commit SHAs and restrict permissions
  - Description: Workflows use moving tags (e.g., `actions/checkout@v4`).
  - Files: `.github/workflows/test.yml`, `.github/workflows/coverage.yml`, `.github/workflows/release.yml`
  - Tasks:
    - [ ] Replace all `uses: org/action@version` with `@<commit-sha>`; add a comment with the original version tag.
    - [ ] Add top-level `permissions: contents: read` (or least needed). For release, grant only the minimal `contents: write` where required.
    - [ ] Add `concurrency:` groups to avoid duplicate runs.
  - Acceptance: All actions pinned; workflow permissions least-privilege; CI green.

- [ ] Make env token usage opt‑in and clearly logged
  - Description: `src/core/auth.rs::retrieve_credential()` prefers env tokens by default.
  - Files: `src/models/config.rs`, `src/core/auth.rs`
  - Tasks:
    - [ ] Add `SecurityConfig { allow_env_tokens: bool }` (default false).
    - [ ] In `retrieve_credential()`, consult config; only read `MULTIGIT_{PROVIDER}_TOKEN` when allowed; emit a warning log stating which provider/host (no token value).
    - [ ] Tests for precedence and config gating.
  - Acceptance: With default config, env tokens are ignored; enabling flag makes them take precedence; tests pass.

- [ ] Redact daemon log output of sync subprocess
  - Description: `src/daemon/service.rs` logs `stdout`/`stderr` from `multigit sync` directly.
  - Files: `src/daemon/service.rs`, `src/utils/redact.rs` (new)
  - Tasks:
    - [ ] Implement `redact(text: &str) -> String` to mask common secret patterns (e.g., `ghp_`, `glpat_`, `Bearer`, generic `token=`/`password=` pairs, JWTs).
    - [ ] Apply redaction before `debug!/warn!` logging of sync outputs.
    - [ ] Tests with sample strings to verify masking.
  - Acceptance: Logs never contain raw secrets in covered patterns; tests pass.

- [ ] Add CI security jobs: cargo-audit / cargo-deny / gitleaks
  - Files: `.github/workflows/test.yml` (or new `security.yml`)
  - Tasks:
    - [ ] Add job running `cargo audit` (or `cargo-deny`) and fail on vulnerabilities.
    - [ ] Add job running `gitleaks` secret scan.
  - Acceptance: Jobs run in CI; failing conditions block merges.

## LOW / HYGIENE

- [ ] Handle passphrases with secrecy + zeroize
  - Description: `EncryptedCredentialStore` holds passphrase in a `String`.
  - Files: `src/core/auth.rs`
  - Tasks:
    - [ ] Use `secrecy::SecretString` (and ensure `zeroize` enabled) for passphrase fields; minimize copies.
    - [ ] Adjust encryption calls to borrow secret without exposing content; drop securely.
  - Acceptance: No persistent `String` of passphrase remains; tests pass.

- [ ] Standardize Codecov action and pin to SHA
  - Files: `.github/workflows/{coverage.yml,test.yml}`
  - Tasks:
    - [ ] Choose a single Codecov action version, pin to SHA, and align both workflows.
  - Acceptance: Both workflows reference the same pinned Codecov action.

- [ ] Commit `Cargo.lock` for reproducible release builds
  - Files: `.gitignore`, repo root `Cargo.lock`
  - Tasks:
    - [ ] Remove `Cargo.lock` from `.gitignore`.
    - [ ] Commit `Cargo.lock` (for binary application best practice).
  - Acceptance: `Cargo.lock` tracked and used in CI/release.

- [ ] Add Dependabot to keep Actions/Cargo up to date
  - Files: `.github/dependabot.yml` (new)
  - Tasks:
    - [ ] Configure daily/weekly updates for `github-actions` and `cargo` ecosystems.
  - Acceptance: PRs are opened for updates automatically.

- [ ] Pre-commit hooks for local hygiene
  - Files: `.pre-commit-config.yaml` (new)
  - Tasks:
    - [ ] Add hooks for `rustfmt`, `clippy` (as advisory), and `gitleaks`.
  - Acceptance: Contributors can enable pre-commit for consistent checks locally.

## DOCUMENTATION

- [ ] Document HTTPS requirement and host-bound tokens
  - Files: `README.md`, `docs/USER_GUIDE.md`
  - Tasks:
    - [ ] Add note that self-hosted instances must use HTTPS unless explicitly allowed.
    - [ ] Explain that credentials are bound to `{provider, host, username}`.
  - Acceptance: Docs updated and visible in Security and Setup sections.

---

## Notes and References

- Host-agnostic keying observed in `src/security/keyring.rs` (`"{provider}:{username}:token"`).
- Self‑hosted URL acceptance without validation in `src/cli/commands/setup.rs` and provider constructors.
- Daemon logging of sync output in `src/daemon/service.rs`.
- CI actions currently reference tags (e.g., `actions/checkout@v4`, `codecov/codecov-action@v3/@v5`).
