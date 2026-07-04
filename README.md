# Defend - Retro Neon Space Shooter

<p align="center">
  <img src="https://raw.githubusercontent.com/UberMetroid/unraid-templates/main/icons/defend.png" alt="Defend Logo" width="128" height="128">
</p>

Defend is a clean, secure, and optimized retro-neon vertical space shooter arcade game built in Rust and WebAssembly, served by a high-performance Axum backend.

---

## Key Features

*   **Standardized UI Alignment**: Completely integrated with `shared-assets` for a uniform theme engine, navigation header, footer, and authentication layout.
*   **SVG Gameplay Viewport**: Smooth responsive vector-based ship movement, cyan laser pulses, and particle explosion sparks scaling dynamically to screen size.
*   **Keyboard & Touch Controls**: Playable on desktop (A/D or Arrow keys + Space to fire) and mobile/touchscreens (built-in virtual dpad controllers).
*   **Secure PIN Access**: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
*   **Performance First**: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

---

## Container Registry

The Docker image is built with **Nix** (no Alpine, fully reproducible) and published to Docker Hub:

*   **Docker Hub**: [ubermetroid/defend](https://hub.docker.com/r/ubermetroid/defend)

---

## Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4504` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Defend` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4504` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `DEFEND_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header (true/false). | `true` |
| `ENABLE_THEMES` | Enable the theme selector in the navigation header (true/false). | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header (true/false). | `true` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before locking out the user client IP address. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer (true/false). | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer (true/false). | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |

---

## License

Licensed under the Apache License, Version 2.0. Copyright 2026 UberMetroid.
