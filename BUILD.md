# Building System Extension Manager

This document describes how to build the System Extension Manager macOS application.

## Prerequisites

- **macOS 14.0+** (Sonoma or later)
- **Xcode 15.0+** with command line tools
- **XcodeGen** installed (`brew install xcodegen`)

## Quick Build

```bash
# Generate Xcode project
xcodegen generate

# Build the project
xcodebuild -project SystemExtensionManager.xcodeproj -scheme SystemExtensionManager -configuration Debug build

# Run the app
open ./build/SystemExtensionManager.app
```

## Build Commands

### Generate Xcode Project

```bash
xcodegen generate
```

This creates `SystemExtensionManager.xcodeproj` from `project.yml`.

### Build Debug Configuration

```bash
xcodebuild -project SystemExtensionManager.xcodeproj -scheme SystemExtensionManager -configuration Debug build
```

### Build Release Configuration

```bash
xcodebuild -project SystemExtensionManager.xcodeproj -scheme SystemExtensionManager -configuration Release build
```

### Clean Build

```bash
xcodebuild -project SystemExtensionManager.xcodeproj -scheme SystemExtensionManager clean
```

## Build Output

The built application is located at:

```
./build/SystemExtensionManager.app
```

## Running the App

After building, you can run the app with:

```bash
open ./build/SystemExtensionManager.app
```

Or from Xcode:

```bash
open SystemExtensionManager.xcodeproj
```

Then press **Cmd+R** to run.

## Troubleshooting

### "xcodegen: command not found"

Install XcodeGen:

```bash
brew install xcodegen
```

### Build fails with entitlements error

For development builds, you may need to sign with your personal Apple Developer certificate:

```bash
xcodebuild -project SystemExtensionManager.xcodeproj -scheme SystemExtensionManager -configuration Debug CODE_SIGN_IDENTITY="-" CODE_SIGN_STYLE=Manual build
```

### System Extension permissions

Some features require admin privileges or System Extension entitlements. Full functionality requires:
- An Apple Developer Program membership
- Proper entitlements signing
- User approval for System Extensions in System Preferences

## Project Structure

```
.
├── project.yml          # XcodeGen configuration
├── Info.plist          # App bundle configuration
├── SystemExtensionManager.entitlements
├── Assets.xcassets/    # App icons and images
├── Sources/
│   ├── App/            # App entry point
│   ├── Models/         # Data models
│   ├── Services/       # Business logic
│   ├── ViewModels/     # UI state management
│   ├── Views/          # SwiftUI views
│   ├── Utilities/      # Helper utilities
│   └── Resources/      # Assets and resources
└── build/              # Build output (created after build)
```

## Architecture

The app follows the MVVM pattern:

- **Models**: Data structures for Login Items, Launch Agents, Launch Daemons, System Extensions
- **Services**: Business logic for interacting with macOS APIs and shell commands
- **ViewModels**: UI state management with `@Published` properties
- **Views**: SwiftUI interface with navigation sidebar and detail panel