# Building System Extension Manager

This document describes how to build the System Extension Manager TUI application.

## Prerequisites

- **macOS 12.0+** (Monterey or later)
- **Rust 1.70+** (`brew install rust`)
- **Cargo** (comes with Rust)

## Quick Build

```bash
# Build the project
cargo build

# Run the app
cargo run

# Release build
cargo build --release
./target/release/system-extension-manager
```

## Dependencies

The project uses these crates:
- `ratatui` - Terminal UI rendering
- `crossterm` - Terminal input/output
- `clap` - CLI argument parsing
- `plist` - Property list handling
- `serde` - Serialization
- `anyhow` / `thiserror` - Error handling
- `tracing` - Logging

## Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library exports
├── app.rs               # Main TUI loop
├── error.rs             # Error types
├── models/              # Data models
├── services/            # Business logic
├── state/               # Application state
└── ui/                  # TUI views
```

## Testing

```bash
cargo test
```

## Linting

```bash
cargo fmt --check
cargo clippy -- -D warnings
```