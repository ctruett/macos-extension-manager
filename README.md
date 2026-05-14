# macOS System Manager

A terminal UI for managing macOS startup items, background services, and system extensions.

![Rust](https://img.shields.io/badge/rust-2021-orange) ![macOS](https://img.shields.io/badge/macOS-12%2B-blue)

## Overview

macOS System Manager gives you a unified view of everything that runs at login or in the background, with keyboard-driven controls to enable, disable, or delete items without navigating System Settings.

**Managed item types:**

| Type | Source |
|---|---|
| Login Items | Items registered to launch at login |
| Launch Agents | User-level background services (`~/Library/LaunchAgents`) |
| Launch Daemons | System-level background services (`/Library/LaunchDaemons`) |
| System Extensions | Kernel/network/endpoint extensions |
| Background Items | App-registered background tasks (BTM) |

## Requirements

- macOS 12 Monterey or later
- Rust 1.70+
- Administrator privileges required for Launch Daemons and System Extensions

## Build & Run

```bash
cargo build --release
./target/release/system-extension-manager
```

Or run directly:

```bash
cargo run
```

## Keyboard Shortcuts

### View Controls (top bar)

| Key | Action |
|---|---|
| `F` | Toggle filter input |
| `S` | Cycle scope: All → User → System |
| `Esc` | Clear filter |

### Actions (bottom bar)

| Key | Action |
|---|---|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `g` | Jump to top |
| `G` | Jump to bottom |
| `E` | Enable selected item |
| `D` | Disable selected item |
| `O` | Open item location in Finder |
| `C` | Copy identifier to clipboard |
| `^D` | Delete selected item (confirm with second `^D`) |
| `R` | Refresh all items |
| `?` | Show help |
| `Q` | Quit |

## Notes

- **System Extensions** cannot be deleted from the CLI — use System Settings → General → Login Items & Extensions.
- **Background Items** are managed via `launchctl` in the user domain and require no elevated privileges.
- Deleting a Launch Daemon or system-level Launch Agent will prompt for administrator credentials via macOS's standard auth dialog.
- Logs are written to `/tmp/system-extension-manager.log`.
