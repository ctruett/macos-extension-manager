# System Extension Manager

## Overview

**Project:** System Extension Manager  
**Type:** Terminal User Interface (TUI) Application  
**Purpose:** Unified TUI to manage Login Items, Launch Agents, Launch Daemons, and System Extensions  
**Tech Stack:** Rust 1.70+, ratatui, crossterm, clap, plist

---

## Features

- [ ] **Login Items** - View, enable/disable, add, and remove login items
- [ ] **Launch Agents** - List, load, unload, create, and delete user-level agents
- [ ] **Launch Daemons** - List, load, unload, create, and delete system-level daemons
- [ ] **System Extensions** - List, activate, and deactivate system extensions

---

## Architecture

```
src/
├── main.rs              # Entry point with argument parsing
├── lib.rs               # Library entry point
├── app.rs               # Main TUI application loop
├── error.rs             # Error types
├── models/              # Data models
│   ├── item_type.rs     # Item category enum
│   ├── login_item.rs    # Login item model
│   ├── launch_agent.rs  # Launch agent model
│   ├── launch_daemon.rs # Launch daemon model
│   └── system_extension.rs
├── services/            # Business logic
│   ├── login_items_service.rs
│   ├── launch_agents_service.rs
│   ├── launch_daemons_service.rs
│   ├── system_extensions_service.rs
│   └── privilege_service.rs
├── state/               # Application state
│   └── app_state.rs     # Global state store
└── ui/                  # TUI views
    ├── app.rs           # Main view composition
    ├── layouts/        # Layout components
    │   ├── sidebar.rs
    │   ├── split_view.rs
    │   └── list_view.rs
    ├── views/          # Section views
    │   ├── login_items_view.rs
    │   ├── launch_agents_view.rs
    │   ├── launch_daemons_view.rs
    │   ├── system_extensions_view.rs
    │   └── detail_view.rs
    └── components/     # Reusable widgets
        ├── status_badge.rs
        ├── search_bar.rs
        ├── table_view.rs
        └── loading_spinner.rs
```

---

## Data Models

### ItemType Enum
```rust
enum ItemType {
    LoginItem,
    LaunchAgent,
    LaunchDaemon,
    SystemExtension,
}
```

### LoginItem
```rust
struct LoginItem {
    id: String,
    name: String,
    path: PathBuf,
    url: Option<Url>,
    enabled: bool,
    hidden: bool,
}
```

### LaunchAgent / LaunchDaemon
```rust
struct LaunchAgent {
    label: String,
    program: PathBuf,
    program_arguments: Vec<String>,
    run_at_load: bool,
    keep_alive: bool,
    standard_paths: StandardPaths,
}
```

### SystemExtension
```rust
struct SystemExtension {
    identifier: String,
    version: String,
    extension_types: Vec<ExtensionType>,
    status: ExtensionStatus,
}
```

---

## Services

### LaunchAgentsService / LaunchDaemonsService
Uses `launchctl` commands:
- `launchctl list` - List all agents/daemons
- `launchctl load <path>` - Load an agent/daemon
- `launchctl unload <path>` - Unload an agent/daemon
- `launchctl remove <label>` - Remove an agent/daemon

### LoginItemsService
Uses deprecated LSSharedFileList API via shell script or AppleScript.

### SystemExtensionsService
Uses `systemextensionsctl` command:
- `systemextensionsctl list` - List system extensions
- `systemextensionsctl setidentities` - Activate/deactivate

### PrivilegeService
Handles admin authorization via AppleScript dialogs for privileged operations.

---

## TUI Layout

```
┌─────────────────────────────────────────────────────┐
│  System Extension Manager                    [?]    │
├────────────┬────────────────────────────────────────┤
│            │                                        │
│  → Login   │  Name               Status    Path     │
│    Items   │  ────────────────────────────────────  │
│            │  Docker Desktop      ●        /Appl... │
│  ▶ Launch  │  Rectangle Pro       ●        /Appl... │
│    Agents  │  Raycast             ○        /Appl... │
│            │                                        │
│  ⚙ Launch  │                                        │
│    Daemons │                                        │
│            │                                        │
│  ◉ System  │                                        │
│    Exts    │                                        │
│            │                                        │
├────────────┴────────────────────────────────────────┤
│  ↑↓ Navigate  Enter: Select  r: Refresh  q: Quit   │
└─────────────────────────────────────────────────────┘
```

---

## Key Bindings

| Key | Action |
|-----|--------|
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `←` / `h` | Navigate to sidebar |
| `→` / `l` | Navigate to detail |
| `Enter` | Select / Toggle item |
| `r` | Refresh current list |
| `/` | Focus search |
| `Esc` | Clear search / Go back |
| `q` | Quit application |
| `?` | Show help |

---

## Dependencies

```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.28"
clap = { version = "4.5", features = ["derive"] }
plist = "1.6"
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## Build & Run

```bash
# Build
cargo build

# Run
cargo run

# Release
cargo build --release
cargo run --release

# Test
cargo test

# Lint
cargo clippy -- -D warnings
```

---

## Notes

- LSSharedFileList is deprecated but functional for Login Items
- SMJobBless not needed (agents/daemons installed manually)
- System Extensions entitlement requires Apple Developer Program
- Admin operations use AppleScript authorization dialogs
- No App Sandbox (TUI requires system-level access)