<h1 align="center">
  <img src="assets/icon.png?v=1.0.31" width="48" height="48" valign="middle"> Defend
</h1>

<p align="center">
  <b>Interactive self-hosted web security arcade and space defense simulator written in Rust.</b>
</p>

---

### Instant One-Line Install (Docker Container)

Run the official zero-dependency container on port 4504:

```bash
docker run -d --name defend -p 4504:4504 -v /mnt/user/appdata/defend:/config ghcr.io/studio2201/defend:latest
```

Open your browser to `http://localhost:4504` to start playing immediately.

---

### One-Line Install (Native Package Manager)

On Debian, Ubuntu, Fedora, or RHEL:

```bash
curl -fsSL https://studio2201.github.io/packages/install.sh | sudo bash
```

---

### Unraid NAS Deployment

Deploy via the official Unraid Template:

1. Copy [`defend.xml`](defend.xml) to your Unraid flash drive under `/boot/config/plugins/dockerMan/templates-user/`.
2. Open **Docker** -> **Add Container** -> Select **defend** from the template dropdown.
3. Click **Apply**.

---

### Environment Configuration

The backend service can be customized using the following environment variables:

| Variable | Description | Default |
| :--- | :--- | :---: |
| `PORT` | Network port the web server binds to | `4504` |
| `DEFEND_PIN` | Security PIN required for application access | *(Disabled)* |
| `DEFEND_DATA_DIR` | Directory path for persistent data and high scores | `/config` |
| `DEFEND_ALLOWED_ORIGINS` | CORS allowed origins list (comma-separated) | `*` |
| `TRUST_PROXY` | Honor reverse proxy headers (`X-Forwarded-For`) | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated CIDR list of trusted reverse proxies | *(None)* |
| `LOG_LEVEL` | Tracing filter (`error`, `warn`, `info`, `debug`) | `info` |

---

### Administration CLI & TUI Dashboard

Every container and package includes a built-in administration utility (`defend`).

Launch interactive TUI dashboard:
```bash
docker exec -it defend defend tui
```

System diagnostics and self-healing check:
```bash
docker exec -it defend defend doctor
```

CLI Command Reference:
- `defend tui` — Interactive terminal user interface.
- `defend doctor` — Diagnoses storage permissions, ports, and database health.
- `defend status` — Displays network configuration and security parameters.
- `defend data stats` — Shows storage utilization and entry metrics.
- `defend data list` — Lists high scores and player leaderboard entries.

---

### Architecture & Security

- **Axum Web Backend**: High-concurrency async HTTP runtime built on Tokio.
- **Yew WebAssembly Frontend**: Type-safe client bundle running natively in browser WASM runtime.
- **Strict Input & Path Sanitization**: Path canonicalization guards preventing directory traversal escapes.
- **Fail-Closed Security PIN Authentication**: Rate-limited brute force protection with automatic lockout timers.

---

### License

Distributed under the Apache 2.0 License. See [LICENSE](LICENSE) for details.

---

### Project Banner Showcase

Official **Defend** project banner depicting energetic retro-neon space defender gameplay visual aesthetics.

<p align="center">
  <a href="https://github.com/studio2201/defend">
    <img src="assets/defend-header.jpg" alt="studio2201 banner" width="100%">
  </a>
</p>
