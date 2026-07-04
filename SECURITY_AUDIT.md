# Security Audit — v0.1.0

Audit performed: 2026-07-03

Scope: backend (`backend/src/**`) and frontend (`frontend/src/**`) Rust sources.
Reference version: v0.1.0.

This audit documents the security posture of Defend at the v0.1.0 release. It is informational.

## Findings

### Hardcoded secrets
- **None**: No API keys, database passwords, or private keys were found in source.
- Notes:
  - `backend/src/routes/auth/mod.rs` and `backend/src/state.rs` reference literal strings for session-id testing inside test blocks, not real credentials.
  - `frontend/src/components/pin.rs` references `type="password"`. That is the HTML `<input>` attribute, not a credential.

### Hardcoded URLs / hosts
- **None**: No external hosts are baked into backend or frontend source.
- Notes: Git dependencies to `shared-assets` are declared in Cargo.toml. Internal routes are directed to `/api/...`.

### Input sanitization
- **Leaderboards**: Player names submitted to leaderboards are sanitized.
- **PIN authentication**: Handled inside `shared-backend` using constant-time comparison to prevent timing attacks.

### Memory safety
- **Memory safety**: Evaluated with cargo-deny. The codebase is annotated with `#![deny(unsafe_code)]` in `Cargo.toml`.