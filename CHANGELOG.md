# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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