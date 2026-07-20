# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.6] - 2026-07-19

### Changed
- **Container Cleanup**: Renamed misleading `Containerfile.ubi` to standard `Containerfile` (the build target remains optimized Alpine Linux).

## [1.0.5] - 2026-07-19

### Changed
- **RFC Compliance**: Updated `shared-assets` to `v3.0.32` to inherit standard RFC 7235 compliance (`WWW-Authenticate` headers for unauthorized responses).

## [1.0.4] - 2026-07-19

### Changed
- **Code Review & Splitting**: Partitioned large monolithic files (including the sh admin console) into logical modules to ensure no single file exceeds 250 lines of code.
- **UI Links Cleanup**: Updated shared assets dependency to remove the version link in the header and the GitHub/Coffee links in the footer.


## [1.0.3] - 2026-07-19

### Fixed
- **RNG Dependency Fix**: Reverted rand version from 0.10 to 0.9 to resolve compilation errors in session generation.


## [1.0.2] - 2026-07-19

### Fixed
- **Dependency Repair**: Fixed corrupted axum dependency version range in Cargo.toml to restore clean compilation.


## [0.8.16] - 2026-07-19

### Security
- **CI Workflow Security**: Declared GITHUB_TOKEN read-only content permissions in the GitHub Actions workflow to resolve CodeQL scanning warnings.


## [0.8.15] - 2026-07-19

### Changed
- **Favicon update**: Synchronized the web application favicon with the new 2-color neon squircle icon.


## [0.8.14] - 2026-07-19

### Fixed
- **TUI execution fix**: Resolved argument routing issue in the main entry point of the admin tool, enabling the "tui" parameter to launch the interactive dashboard successfully.


## [0.8.13] - 2026-07-19

### Changed
- **Uniform Rounded Icon**: Applied a rounded corner mask with white-filled borders to make all application icons perfectly uniform.


## [0.8.12] - 2026-07-19

### Changed
- **Simple Bright Icon**: Replaced application icon with a simple high-contrast 2-color flat-art neon cyan and purple vector illustration on a dark navy blue background.


## [0.8.11] - 2026-07-19

### Fixed
- **Warning fix**: Removed unused mutable keyword on server command spawn to prevent warning compilation failures in CI runners.


## [0.8.10] - 2026-07-19

### Changed
- **Release build bump**: Preparing new version release to trigger automated package container compilation on GHCR.


## [0.8.9] - 2026-07-19

### Changed
- **Premium Flat-Art Icon**: Updated the repository application icon to a beautiful, modern flat neon glowing asset.


## [0.8.8] - 2026-07-19

### Changed
- **Slim Branding Banner**: Replaced the repository header banner with a slim, flat-art twilight landscape of Cheney, WA (home of the server) featuring rolling hills, Ponderosa pines, and a soaring neon eagle.


## [0.8.7] - 2026-07-19

### Changed
- **Containerized Admin Console integration**: Named the admin tool after the application (`defend`) and copied it to the container's system path `/usr/local/bin/defend`. The TUI can now be launched by simply executing `defend tui` (or `defend`) inside the container shell.
- **Documentation Modernization**: Rewrote `README.md` to remove CI details, format CLI commands as tables, and purge local development guides.


## [0.8.6] - 2026-07-19

### Changed
- **Containerized Admin Console integration**: Named the admin tool after the application (`defend`) and copied it to the container's system path `/usr/local/bin/defend`. The TUI can now be launched by simply executing `defend tui` (or `defend`) inside the container shell.
- **Documentation Modernization**: Rewrote `README.md` to remove CI details, format CLI commands as tables, and purge local development guides.


## [1.0.1] - 2026-07-19

### Changed
- Update README, clean file tree, and remove contributing/license files.


## [0.8.5] - 2026-07-19

### Changed
- **Standardized CLI & TUI command interface**: Aligned all admin commands and options with industry standard conventions. Added aliases for starting (`up`, `run`), stopping (`stop`, `down`), restarting (`restart`, `reload`), and diagnosing (`check`, `diagnose`) the application services.


## [0.8.4] - 2026-07-19

### Added
- **TUI & CLI Diagnostic Commands**: Added `doctor`, `start`, and `end`/`close` commands. Added the interactive system health check (doctor report) to the TUI panel menu.


## [0.8.3] - 2026-07-19

### Added
- **CLI Version Flag**: Added support for checking version details in the admin CLI using `version`, `-v`, or `--version` flags.


## [0.8.2] - 2026-07-19

### Added
- **Interactive Admin CLI & TUI Console**: Replaced the stub `sh` binary with a fully-featured, zero-dependency command-line interface and terminal user interface (TUI) dashboard for managing settings, checking database/storage file statistics, and viewing database records.


## [0.8.1] - 2026-07-19

### Added
- **Interactive Admin CLI & TUI Console**: Replaced the stub `sh` binary with a fully-featured, zero-dependency command-line interface and terminal user interface (TUI) dashboard for managing settings, checking database/storage file statistics, and viewing database records.


## [0.1.21] - 2026-07-19

### Changed
- **Rebrand to studio2201**: README, container labels, docker-compose, and Cargo
  metadata now reference `studio2201/defend`. CI badge URL and GHCR image name
  updated accordingly.
- **Fixed SCAN_ env-var copy-paste** in `docker-compose.yml`: all `SCAN_*`
  environment variables renamed to `DEFEND_*` (data path, PIN, allowed origins,
  base URL, site title). Defend now correctly uses its own env-var namespace.
- **Fixed manifest description typo** ("defendner" → "defender") and aligned
  wording with the README tagline.
- **Favicon cache-bust query** bumped `?v=0.1.0` → `?v=0.1.21` in
  `frontend/index.html` to invalidate stale PWA icon cache.

## [0.1.0] - 2026-07-03

### Added
- **Initial Release**: Launch of `defend` retro-neon space shooter game.
- **HUD Glassmorphism Styling**: Styled panel views with blur and dark cyberpunk layout elements.
- **SVG Canvas Renderer**: High-performance responsive vector display scaling to different aspect ratios.
- **Keyboard & Touch Inputs**: Supports Arrow Keys / A/D movement & Space firing, as well as touchscreen D-pad buttons.
- **Nix Scaffolding**: Pinned packages compiling in Trunk/Yew, ready for container deployments.
- **Localization Integration**: Added 8 language translations support.