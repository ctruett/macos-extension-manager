# System Extension Manager - Project Plan

## Overview

**Project Name:** System Extension Manager  
**Type:** Terminal User Interface (TUI) Application  
**Core Functionality:** A TUI application that provides a unified interface to manage Login Items, Launch Agents, Launch Daemons, and System Extensions.  
**Target Users:** Power users, system administrators, and developers who need fine-grained control over macOS startup and background services.  
**macOS Version Support:** macOS 12.0+ (Monterey and later)

---

## 1. Project Scope

### 1.1 Core Features

#### 1.1.1 Login Items Management
- [ ] List all user login items (applications and URLs)
- [ ] Add/remove login items
- [ ] Enable/disable individual login items
- [ ] View login item properties (path, hidden status)

#### 1.1.2 Launch Agents Management
- [ ] List all user launch agents (`~/Library/LaunchAgents`)
- [ ] Load/unload agents via `launchctl`
- [ ] Create/delete agent plist files
- [ ] View agent properties (label, program, runAtLoad, keepAlive)

#### 1.1.3 Launch Daemons Management
- [ ] List all system launch daemons (`/Library/LaunchDaemons`)
- [ ] Load/unload daemons (requires admin privileges)
- [ ] Create/delete daemon plist files
- [ ] Admin authentication via AppleScript dialog

#### 1.1.4 System Extensions Management
- [ ] List installed system extensions
- [ ] Activate/deactivate extensions (requires admin privileges)
- [ ] View extension properties (identifier, version, type)
- [ ] Status indicators (activated, deactivated, pending)

### 1.2 TUI Features

#### 1.2.1 Navigation
- [ ] Sidebar with section icons
- [ ] Keyboard navigation (vim-style: j/k/h/l, arrow keys)
- [ ] Mouse support (click to select, scroll)
- [ ] Tab switching between sections

#### 1.2.2 List Views
- [ ] Sortable columns
- [ ] Search/filter functionality
- [ ] Status badges (enabled/disabled, loaded/unloaded)
- [ ] Pagination for large lists

#### 1.2.3 Detail Views
- [ ] Selected item details panel
- [ ] Plist content viewer/editor
- [ ] Action buttons (enable, disable, delete, etc.)

#### 1.2.4 Visual States
- [ ] Loading spinner
- [ ] Empty state messages
- [ ] Error banners with recovery actions
- [ ] Confirmation dialogs for destructive actions

---

## 2. Technical Approach

### 2.1 Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 1.70+ |
| TUI Framework | ratatui |
| Terminal I/O | crossterm |
| CLI Parsing | clap |
| Data Parsing | plist, serde |
| Error Handling | anyhow, thiserror |
| Logging | tracing |

### 2.2 Architecture

```
┌─────────────────────────────────────┐
│           TUI Application            │
├─────────────────────────────────────┤
│  State Management (AppState)         │
├──────────────┬──────────────────────┤
│  Services    │   Views (ratatui)    │
│              │                      │
│  - Login     │  - Sidebar           │
│  - Agents    │  - List views        │
│  - Daemons   │  - Detail panel      │
│  - SysExts   │  - Modals            │
└──────────────┴──────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│      Shell Commands (launchctl)       │
│      macOS APIs (SystemExtensions)   │
└─────────────────────────────────────┘
```

### 2.3 Data Models

```rust
enum ItemType {
    LoginItem,
    LaunchAgent,
    LaunchDaemon,
    SystemExtension,
}

struct LoginItem {
    id: String,
    name: String,
    path: PathBuf,
    enabled: bool,
}

struct LaunchAgent {
    label: String,
    path: PathBuf,
    program: PathBuf,
    run_at_load: bool,
    keep_alive: bool,
    loaded: bool,
}

struct LaunchDaemon {
    // Similar to LaunchAgent
}

struct SystemExtension {
    identifier: String,
    version: String,
    activated: bool,
}
```

### 2.4 Services Layer

Each service module handles:
- Listing items from the filesystem and via shell commands
- CRUD operations via shell (`launchctl`, `systemextensionsctl`)
- Admin privilege escalation when needed
- Error handling with typed errors

---

## 3. File Structure

```
system-extension-manager/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── error.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── item_type.rs
│   │   ├── login_item.rs
│   │   ├── launch_agent.rs
│   │   ├── launch_daemon.rs
│   │   └── system_extension.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── login_items_service.rs
│   │   ├── launch_agents_service.rs
│   │   ├── launch_daemons_service.rs
│   │   ├── system_extensions_service.rs
│   │   └── privilege_service.rs
│   ├── state/
│   │   ├── mod.rs
│   │   └── app_state.rs
│   └── ui/
│       ├── mod.rs
│       ├── app.rs
│       ├── layouts/
│       │   ├── mod.rs
│       │   ├── sidebar.rs
│       │   ├── split_view.rs
│       │   └── list_view.rs
│       ├── views/
│       │   ├── mod.rs
│       │   ├── login_items_view.rs
│       │   ├── launch_agents_view.rs
│       │   ├── launch_daemons_view.rs
│       │   ├── system_extensions_view.rs
│       │   └── detail_view.rs
│       └── components/
│           ├── mod.rs
│           ├── status_badge.rs
│           ├── search_bar.rs
│           ├── table_view.rs
│           └── loading_spinner.rs
├── BUILD.md
├── README.md
└── LICENSE
```

---

## 4. Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `←` / `h` | Navigate to sidebar |
| `→` / `l` | Navigate to detail |
| `Enter` / `Space` | Select / Toggle item |
| `r` | Refresh current list |
| `/` | Focus search |
| `Esc` | Clear search / Go back |
| `q` | Quit application |
| `?` | Show help |

---

## 5. Error Handling

| Error Type | User Feedback |
|------------|---------------|
| Permission Denied | "Admin privileges required" + prompt to authenticate |
| File Not Found | Inline error + option to remove from list |
| Invalid Plist | Error view with parse error details |
| Extension Activation Failed | Error with code and suggestion |
| launchctl Failure | Inline error with stderr output |

---

## 6. Implementation Priority

### Phase 1: Foundation
1. Project setup (Cargo.toml, build config)
2. Error types
3. Data models
4. Shell command utilities

### Phase 2: Core Services
1. LoginItemsService
2. LaunchAgentsService
3. LaunchDaemonsService
4. SystemExtensionsService

### Phase 3: State Management
1. AppState struct
2. State transitions
3. Loading/error states

### Phase 4: TUI Views
1. Main app loop
2. Layout components
3. Section list views
4. Detail views
5. Search and filtering

### Phase 5: Polish
1. Keyboard navigation
2. Loading states
3. Error handling UI
4. Help/shortcuts overlay

---

## 7. Testing Strategy

- Unit tests for models and services
- Integration tests for shell command execution
- Manual testing for TUI interactions

---

## 8. Future Considerations

- Configuration file for custom launch agent locations
- Batch operations (enable/disable multiple items)
- Export/import configurations
- macOS Notifications for extension status changes