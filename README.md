# Defend

Defend is a retro-neon vertical space shooter arcade game built in Rust using Yew and WebAssembly on the frontend, served by a native Axum backend.

## Architecture and Stack

* Frontend: Yew (WASM)
* Backend: Axum (Rust) / Tokio
* Deployment: UBI container (Red Hat UBI9) on Docker Hub / Unraid / Podman / Docker Compose

## Code Map

The project is structured as a cargo workspace containing a Rust-based backend and a WebAssembly frontend built with Yew.

* [backend/src/main.rs](backend/src/main.rs): Application process entrypoint and HTTP server initialization.
* [backend/src/bootstrap.rs](backend/src/bootstrap.rs): Runtime builder, state initialization, and application bootstrapper.
* [backend/src/config.rs](backend/src/config.rs): Configuration loader and validation.
* [backend/src/state.rs](backend/src/state.rs): State definitions and global application state wrapper.
* [backend/src/services/paths.rs](backend/src/services/paths.rs): Path resolution and environment variable directory overrides.
* [frontend/src/components/defend_logic.rs](frontend/src/components/defend_logic.rs): Core game loop logic, grid state, threat generation, and collision detection.
* [frontend/src/components/defend_game.rs](frontend/src/components/defend_game.rs): Main gameplay container and viewport layout wrapper.
* [frontend/src/components/defend_board.rs](frontend/src/components/defend_board.rs): SVG canvas drawing logic for rendering ships, lasers, and threats.

## Key Features

* Standardized UI Alignment: Completely integrated with shared-assets for a uniform theme engine, navigation header, footer, and authentication layout.
* SVG Gameplay Viewport: Smooth responsive vector-based ship movement, cyan laser pulses, and particle explosion sparks scaling dynamically to screen size.
* Keyboard and Touch Controls: Playable on desktop (A/D or Arrow keys + Space to fire) and mobile/touchscreens (built-in virtual dpad controllers).
* Secure PIN Access: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
* Performance First: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

## Local Setup

Ensure you have the Rust toolchain (stable) and Trunk installed.

### Prerequisites

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk compiler
cargo install --locked trunk
```

### Development Commands

```bash
# 1. Run workspace tests
cargo test

# 2. Run clippy workspace checks
cargo clippy --workspace --all-targets

# 3. Start frontend Yew dev server (from frontend/)
cd frontend && trunk serve

# 4. Start backend Axum server (from backend/)
cd backend && cargo run
```

## Deployment and Hosting

Defend is optimized for self-hosting on Unraid, Docker, and Podman. Official images are built on Red Hat Universal Base Image (UBI9-minimal).

### Unraid Deployment Details

Defend templates are available through the community application repository.
* Docker Hub Repository: `ubermetroid/defend` (tags: `latest`, `ubi`, or version pins)
* Network Mode: Bridge (default port: `4504`)
* Volume Configuration: Mapped host folder to `/app/data` for leaderboard persistence (`leaderboard.json`).
* Security: The container runs with non-root privileges (`--user 99:100`). Ensure the mapped host path has appropriate read/write permissions for UID 99 and GID 100.

### Docker Compose

Create a `docker-compose.yml` file with the following service definition:

```yaml
services:
  defend:
    image: ubermetroid/defend:latest
    container_name: defend
    restart: unless-stopped
    volumes:
      - ${DEFEND_DATA_PATH:-./data}:/app/data
    ports:
      - ${PORT:-4504}:4504
    environment:
      PORT: 4504
      BASE_URL: ${BASE_URL:-http://localhost:4504}
      DEFEND_PIN: ${DEFEND_PIN:-}
      ALLOWED_ORIGINS: ${ALLOWED_ORIGINS:-*}
      MAX_ATTEMPTS: ${MAX_ATTEMPTS:-5}
      SITE_TITLE: ${SITE_TITLE:-Defend}
      ENABLE_TRANSLATION: ${ENABLE_TRANSLATION:-true}
      ENABLE_THEMES: ${ENABLE_THEMES:-true}
      ENABLE_PRINT: ${ENABLE_PRINT:-true}
      TZ: ${TZ:-UTC}
```

### Build UBI Image Locally

```bash
# From the repository root
podman build --format docker -f Containerfile.ubi \
  -t docker.io/ubermetroid/defend:0.1.11 \
  -t docker.io/ubermetroid/defend:latest \
  -t docker.io/ubermetroid/defend:ubi \
  .
```

## Configuration Options

| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server binds to inside the container. | `4504` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Defend` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies. | `http://localhost:4504` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). | `*` |
| `DEFEND_PIN` | Optional 4–64 character PIN to lock access to the interface. | None |
| `SNAKE_DATA_DIR` | Directory where runtime state is persisted (`leaderboard.json`). | `./data` |
| `SNAKE_FRONTEND_DIR` | Path to the prebuilt Trunk SPA bundle. | `./frontend/dist` |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header. | `true` |
| `ENABLE_THEMES` | Enable the theme selector in the navigation header. | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header. | `true` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before rate lockout. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer. | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer. | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |

> [!WARNING]
> Due to implementation inheritances from the snake template, the environment variables `SNAKE_DATA_DIR` and `SNAKE_FRONTEND_DIR` are currently used to override the data and frontend directory paths respectively.

## License

Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 UberMetroid.
