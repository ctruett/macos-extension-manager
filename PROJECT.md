# System Extension Manager - macOS Application Plan

## Overview

**Project Name:** System Extension Manager  
**Bundle Identifier:** com.systemextensionmanager.app  
**Core Functionality:** A native macOS application that provides a unified graphical interface to manage Login Items, Launch Agents, Launch Daemons, and System Extensions (Background Items).  
**Target Users:** Power users, system administrators, and developers who need fine-grained control over macOS startup and background services.  
**macOS Version Support:** macOS 12.0+ (Monterey and later)

---

## 1. Project Scope

### 1.1 Core Features

#### 1.1.1 Login Items Management
- List all user login items (applications and URLs)
- Add/remove login items
- Enable/disable individual login items
- Show login item details (path, name, hidden status)

#### 1.1.2 Launch Agents Management
- List all user-level Launch Agents (`~/Library/LaunchAgents/`)
- Display agent properties (label, program arguments, run conditions, etc.)
- Add/edit/remove Launch Agents
- Enable/disable individual agents via `launchctl`
- Show agent plist contents

#### 1.1.3 Launch Daemons Management
- List all system-level Launch Daemons (`/Library/LaunchDaemons/`)
- Requires admin privileges to view/modify
- Display daemon properties
- Add/edit/remove Launch Daemons (admin required)
- Enable/disable individual daemons
- Show daemon plist contents

#### 1.1.4 System Extensions (Background Items) Management
- List installed system extensions via `SMExtensionManager`
- Enable/disable system extensions
- Show extension details (bundle ID, version, type)
- Note: Approving extensions requires System Preferences interaction

### 1.2 Secondary Features

- Search and filter across all item types
- Detailed inspector view for each item
- Import/export configurations
- Quick actions: Open file location, Reveal in Finder
- Status indicators (enabled/disabled/loaded/unloaded)
- Refresh capability
- Help tooltips for each section

---

## 2. Technical Architecture

### 2.1 Technology Stack

| Component | Technology |
|-----------|------------|
| UI Framework | SwiftUI (primary) with AppKit integration |
| Architecture | MVVM (Model-View-ViewModel) |
| Build System | Swift Package Manager (SPM) |
| Minimum macOS | 12.0 |
| Swift Version | 5.9+ |

### 2.2 Project Structure

```
SystemExtensionManager/
├── Sources/
│   ├── App/
│   │   ├── main.swift                    # Application entry point
│   │   ├── AppDelegate.swift             # NSApplication delegate
│   │   └── SystemExtensionManagerApp.swift
│   ├── Models/
│   │   ├── LoginItem.swift
│   │   ├── LaunchAgent.swift
│   │   ├── LaunchDaemon.swift
│   │   ├── SystemExtension.swift
│   │   └── ItemType.swift
│   ├── ViewModels/
│   │   ├── LoginItemsViewModel.swift
│   │   ├── LaunchAgentsViewModel.swift
│   │   ├── LaunchDaemonsViewModel.swift
│   │   ├── SystemExtensionsViewModel.swift
│   │   └── MainViewModel.swift
│   ├── Views/
│   │   ├── MainWindow/
│   │   │   ├── MainView.swift
│   │   │   └── SidebarView.swift
│   │   ├── LoginItems/
│   │   │   ├── LoginItemsView.swift
│   │   │   └── LoginItemRow.swift
│   │   ├── LaunchAgents/
│   │   │   ├── LaunchAgentsView.swift
│   │   │   └── LaunchAgentRow.swift
│   │   ├── LaunchDaemons/
│   │   │   ├── LaunchDaemonsView.swift
│   │   │   └── LaunchDaemonRow.swift
│   │   ├── SystemExtensions/
│   │   │   ├── SystemExtensionsView.swift
│   │   │   └── SystemExtensionRow.swift
│   │   ├── Detail/
│   │   │   ├── DetailView.swift
│   │   │   ├── PlistEditorView.swift
│   │   │   └── InspectorView.swift
│   │   └── Components/
│   │       ├── StatusBadge.swift
│   │       ├── SearchBar.swift
│   │       ├── ActionButton.swift
│   │       └── RefreshButton.swift
│   ├── Services/
│   │   ├── LoginItemsService.swift
│   │   ├── LaunchAgentsService.swift
│   │   ├── LaunchDaemonsService.swift
│   │   ├── SystemExtensionsService.swift
│   │   └── PrivilegeService.swift
│   ├── Utilities/
│   │   ├── PlistParser.swift
│   │   ├── ShellExecutor.swift
│   │   └── FileManager+Extensions.swift
│   └── Resources/
│       ├── Assets.xcassets/
│       └── Localizable.strings
├── project.yml                             # XcodeGen configuration
├── Package.swift                           # SPM manifest (if needed)
└── Info.plist
```

---

## 3. UI/UX Design

### 3.1 Window Structure

- **Main Window:** Single-window application with sidebar navigation
  - Minimum size: 900x600
  - Default size: 1100x700
  - Supports window resizing
  
- **Navigation:** NSSidebar-style sidebar (SwiftUI `NavigationSplitView`)
  - Collapsible
  - Icons + labels for each section

### 3.2 Visual Design

| Element | Specification |
|---------|---------------|
| Window Style | Unified title bar, automatic hide toolbar |
| Color Scheme | System appearance (light/dark mode support) |
| Typography | SF Pro (system font): Title 22pt, Heading 17pt, Body 13pt |
| Spacing | 8pt grid system |
| Sidebar Width | 200pt minimum, 280pt default |

### 3.3 Color Palette

| Purpose | Light Mode | Dark Mode |
|---------|------------|-----------|
| Primary | System Blue (#007AFF) | System Blue |
| Secondary | System Gray (#8E8E93) | System Gray |
| Success/Enabled | System Green (#34C759) | System Green |
| Warning | System Orange (#FF9500) | System Orange |
| Error/Disabled | System Red (#FF3B30) | System Red |
| Background | System Background | System Background |

### 3.4 Icons

| Section | SF Symbol |
|---------|-----------|
| Login Items | `person.crop.circle.badge.plus` |
| Launch Agents | `doc.badge.gearshape` |
| Launch Daemons | `gearshape.2` |
| System Extensions | `puzzlepiece.extension` |
| General | `house.fill` |

### 3.5 View States

| State | Visual Treatment |
|-------|------------------|
| Loading | ProgressView spinner centered |
| Empty | SF Symbol + descriptive message |
| Error | Alert or inline error message |
| Populated | List with rows |

---

## 4. Functionality Specification

### 4.1 User Flows

#### 4.1.1 View Login Items
1. User clicks "Login Items" in sidebar
2. App fetches login items via LSSharedFileList or SMLoginItemSetEnabled
3. Display items in table view with status badges
4. User can select item to see details

#### 4.1.2 Toggle Login Item
1. User selects login item row
2. User clicks toggle or right-click > Enable/Disable
3. App updates login item state
4. UI reflects new state

#### 4.1.3 Add Launch Agent
1. User clicks "+" button in Launch Agents view
2. File picker opens for .plist selection
3. App validates plist format
4. App copies file to ~/Library/LaunchAgents/
5. App loads agent via launchctl
6. List refreshes to show new agent

#### 4.1.4 Remove System Extension
1. User selects system extension
2. User clicks "-" or right-click > Remove
3. Confirmation dialog appears
4. App calls SMExtensionManager to deactivate
5. UI updates

### 4.2 Architecture Pattern (MVVM)

```
┌─────────────────────────────────────────────────────────────┐
│                          Views                               │
│  (SwiftUI Views - LoginItemsView, LaunchAgentsView, etc.)  │
└─────────────────────────┬───────────────────────────────────┘
                          │ @ObservedObject / @StateObject
┌─────────────────────────▼───────────────────────────────────┐
│                      ViewModels                              │
│  (LoginItemsViewModel, LaunchAgentsViewModel, etc.)        │
│  - @Published properties for UI binding                     │
│  - Actions triggered by user                               │
└─────────────────────────┬───────────────────────────────────┘
                          │ calls
┌─────────────────────────▼───────────────────────────────────┐
│                       Services                               │
│  (LoginItemsService, LaunchAgentsService, etc.)             │
│  - Business logic                                           │
│  - Shell command execution                                  │
│  - Data transformation                                      │
└─────────────────────────┬───────────────────────────────────┘
                          │ uses
┌─────────────────────────▼───────────────────────────────────┐
│                        Models                                │
│  (LoginItem, LaunchAgent, LaunchDaemon, etc.)              │
│  - Data structures                                          │
│  - Codable for plist parsing                                │
└─────────────────────────────────────────────────────────────┘
```

### 4.3 Error Handling

| Error Type | User Feedback |
|------------|---------------|
| Permission Denied | Alert with "Grant Permission" button, opens System Preferences |
| File Not Found | Inline message, option to remove from list |
| Invalid Plist | Alert with parse error details |
| Extension Activation Failed | Alert with error code and suggestion |
| Launchctl Failure | Inline error with stderr output |

### 4.4 Security Considerations

- Launch Daemon modification requires admin via AuthorizationServices
- System Extension management requires System Extensions entitlement
- Sensitive operations logged (without sensitive data)
- No hardcoded credentials

---

## 5. Implementation Details

### 5.1 Login Items

**API:** `LSSharedFileList` (deprecated but functional) or `SMLoginItemSetEnabled` via ServiceManagement framework

```swift
// Key APIs
LSSharedFileListCreate(nil, kLSSharedFileListSessionLoginItems, nil)
LSSharedFileListItemResolve(item, kLSSharedFileListDoNotMountVolumes, ...)
ServiceManagement.SMLoginItemSetEnabled(bundleID, enabled)
```

### 5.2 Launch Agents/Daemons

**Location:**
- Agents: `~/Library/LaunchAgents/`
- Daemons: `/Library/LaunchDaemons/`

**Tools:**
- `launchctl list` - List loaded agents/daemons
- `launchctl load / unload` - Load/unload agents
- `launchctl remove` - Remove agents

**Plist Format:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "...">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.example.agent</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/bin/python3</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
```

### 5.3 System Extensions

**Framework:** `SystemExtensions` framework (OSSystemExtensionRequest)

```swift
import SystemExtensions

// Key classes
OSSystemExtensionRequest.activationRequest(forExtensionWithIdentifier:)
OSSystemExtensionRequest.deactivationRequest(forExtensionWithIdentifier:)
OSSExtensionHubDelegate
```

**Entitlements Required:**
```xml
<key>com.apple.developer.system-extension.install</key>
<true/>
```

### 5.4 Dependencies

| Dependency | Purpose | Manager |
|------------|---------|---------|
| PlistParser (custom) | Parse/write plist files | Bundled |
| ShellExecutor (custom) | Execute launchctl commands | Bundled |

*Note: Minimize external dependencies. Use native frameworks.*

### 5.5 Entitlements

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "...">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <false/>
    <key>com.apple.developer.system-extension.install</key>
    <true/>
    <key>com.apple.developer.system-extension.install</key>
    <true/>
    <key>com.apple.security.temporary-exception.files.absolute-path.read-write</key>
    <array>
        <string>/Library/LaunchDaemons/</string>
        <string>/Library/LaunchAgents/</string>
    </array>
</dict>
</plist>
```

---

## 6. Testing Strategy

### 6.1 Unit Tests
- PlistParser accuracy
- Model initialization
- ViewModel state changes
- Service method mocking

### 6.2 Integration Tests
- File system operations (create/remove agents)
- launchctl command execution
- Login item API integration (requires signed app)

### 6.3 UI Tests
- Navigation flow
- List population
- Error state display

---

## 7. Build & Distribution

### 7.1 Signing Requirements

- Apple Developer account required
- System Extension entitlement (requires paid membership)
- Hardened Runtime enabled
- Notarization required for distribution

### 7.2 Distribution Options

| Method | Description |
|--------|-------------|
| Direct Distribution | DMG with notarized app |
| Homebrew Cask | For power users |
| App Store | Not recommended (System Extensions have restrictions) |

---

## 8. Phases & Milestones

### Phase 1: Foundation (Week 1-2)
- [ ] Project setup with XcodeGen
- [ ] Basic window structure
- [ ] Sidebar navigation
- [ ] Models and basic ViewModels

### Phase 2: Login Items (Week 2-3)
- [ ] LoginItemsService implementation
- [ ] LoginItemsView and rows
- [ ] Enable/disable functionality
- [ ] Search/filter

### Phase 3: Launch Agents (Week 3-4)
- [ ] LaunchAgentsService implementation
- [ ] LaunchAgentsView and rows
- [ ] Add/remove agents
- [ ] Load/unload via launchctl

### Phase 4: Launch Daemons (Week 4-5)
- [ ] LaunchDaemonsService with admin privileges
- [ ] LaunchDaemonsView and rows
- [ ] Add/remove daemons
- [ ] Authorization UI

### Phase 5: System Extensions (Week 5-6)
- [ ] SystemExtensionsService implementation
- [ ] SystemExtensionsView and rows
- [ ] Activation/deactivation requests
- [ ] Error handling for extension operations

### Phase 6: Polish (Week 6-7)
- [ ] Inspector/detail views
- [ ] Plist editor view
- [ ] Error handling refinement
- [ ] UI polish and animations

### Phase 7: Testing & Release (Week 7-8)
- [ ] Unit and integration tests
- [ ] Build configuration
- [ ] Code signing setup
- [ ] Documentation

---

## 9. Potential Challenges

| Challenge | Mitigation |
|-----------|------------|
| System Extension entitlement approval | Start enrollment early, Apple Developer Program required |
| Admin privilege escalation | Use SMJobBless or AuthorizationServices properly |
| Deprecated APIs (LSSharedFileList) | Graceful fallback, document limitations |
| App Sandbox restrictions | Disable sandbox or request specific exceptions |
| launchctl permission issues | Clear user communication, guide to System Preferences |

---

## 10. Future Enhancements

- [ ] Cron jobs management
- [ ] Startup items history
- [ ] Scheduled task management
- [ ] Cloud configuration sync
- [ ] Profiles/mobile device management support

---

## 11. References

- [ServiceManagement Framework](https://developer.apple.com/documentation/servicemanagement)
- [System Extensions Framework](https://developer.apple.com/documentation/systemextensions)
- [Launch Agents Documentation](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPLaunchServices/Articles/LaunchAgents.html)
- [LSSharedFileList Reference](https://developer.apple.com/library/archive/documentation/CoreFoundation/Reference/LSSharedFileListRef/)
- [XcodeGen Documentation](https://github.com/yonaskolb/XcodeGen)