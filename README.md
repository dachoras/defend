# Defend

`defend` is a retro-neon vertical space shooter arcade game built in Rust using Yew and WebAssembly on the frontend, served by a native Axum backend. It features vibrant glassmorphic HUD styling, smooth SVG-based entity rendering, and internationalized string support.

---

## Features

- **Responsive SVG Canvas**: Highly performant vector rendering that scales dynamically to fit different screen viewports.
- **Dynamic Wave Difficulty**: Speed and hazard frequency scale upward as wave counter increases.
- **Retro Neon Glow**: Accentuated cyan laser pulses, cyan chevron defender ship, and red hazard debris styled with CSS drop-shadows.
- **Access PIN Security**: Support for optional backend PIN locks.
- **Multilingual Support**: Supports 8 default language configurations via a built-in i18n switcher.
- **Keyboard & Touch Controls**: Playable on desktop (A/D or Arrows + Space) and mobile/touchscreens (built-in virtual dpad controllers).

---

## Development

All commands are run from the workspace root:

### Format Check
```bash
cargo fmt --check
```

### Lints & Compilation Check
```bash
cargo clippy --all-targets
```

### Integration Tests
```bash
cargo test --all-targets
```

### Nix Hermetic Build
```bash
nix build
```

---

## License

Licensed under the Apache License, Version 2.0. Copyright 2026 UberMetroid.
