# Defend — Retro Neon Space Shooter <img src="https://raw.githubusercontent.com/UberMetroid/unraid-templates/main/icons/defend.png?v=2" width="48" height="48" alt="defend logo" align="right">

Defend is a clean, secure, and optimized retro-neon vertical space shooter arcade game built in Rust and WebAssembly, served by a high-performance Axum backend.

---

## 🏛️ Architecture & Stack
*   **Frontend**: Yew (WASM)
*   **Backend**: Axum (Rust) / Tokio
*   **Deployment**: Nix-built Container / Unraid native / Docker Compose

---

## 🟢 Key Features
*   **Standardized UI Alignment**: Completely integrated with `shared-assets` for a uniform theme engine, navigation header, footer, and authentication layout.
*   **SVG Gameplay Viewport**: Smooth responsive vector-based ship movement, cyan laser pulses, and particle explosion sparks scaling dynamically to screen size.
*   **Keyboard & Touch Controls**: Playable on desktop (A/D or Arrow keys + Space to fire) and mobile/touchscreens (built-in virtual dpad controllers).
*   **Secure PIN Access**: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
*   **Performance First**: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

---

## 💾 Deployment & Installation

### Docker Compose
Create a `docker-compose.yml` file with the following service definition:

```yaml
services:
  defend:
    image: ubermetroid/defend:latest
    container_name: defend
    restart: unless-stopped
    volumes:
      - ${SCAN_DATA_PATH:-./data}:/app/data
    ports:
      - ${PORT:-4504}:4504
    environment:
      PORT: 4504
      BASE_URL: ${SCAN_BASE_URL:-http://localhost:4504}
      SCAN_PIN: ${SCAN_PIN:-}
      ALLOWED_ORIGINS: ${SCAN_ALLOWED_ORIGINS:-*}
      MAX_ATTEMPTS: ${MAX_ATTEMPTS:-5}
      SITE_TITLE: ${SCAN_SITE_TITLE:-Defend}
      ENABLE_TRANSLATION: ${ENABLE_TRANSLATION:-true}
      ENABLE_THEMES: ${ENABLE_THEMES:-true}
      ENABLE_PRINT: ${ENABLE_PRINT:-true}
      TZ: ${TZ:-UTC}
```

---

## ⚙️ Configuration Options

| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4504` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Defend` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies. | `http://localhost:4504` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). | `*` |
| `DEFEND_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. | None |
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

---

## 🛠️ Local Development

Ensure you have the Rust toolchain and Trunk installed.

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

---

## 📄 License
Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 UberMetroid.
