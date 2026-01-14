# Usage Guide

> ⚠️ **EXPERIMENTAL** - This project is in early development.

## Building

```bash
make install
```

This will:
1. Build the Rust helper binary (`cargo build --release`)
2. Install the Python package with dependencies via Poetry

## Running

```bash
# Start the ActivityWatch server first (if not already running)
aw-server
# or
aw-server-rust

# In another terminal, start the window watcher
poetry run aw-watcher-window-cosmic
```

## Configuration

The watcher uses the same configuration format as aw-watcher-window. Create a config file at:

```
~/.config/aw-watcher-window-cosmic/aw-watcher-window-cosmic.toml
```

Example:

```toml
[aw-watcher-window-cosmic]
exclude_title = false
exclude_titles = []
poll_time = 1.0
```

## Command-line Options

```
--host HOST          ActivityWatch server host (default: 127.0.0.1)
--port PORT          ActivityWatch server port (default: 5600)
--testing            Enable testing mode
--exclude-title      Exclude all window titles
--exclude-titles     Exclude window titles matching regex patterns
--verbose            Enable verbose logging
--poll-time SECONDS  Polling interval in seconds (default: 1.0)
```

## Requirements

- COSMIC desktop environment (Pop!_OS or other distros running cosmic-comp)
- Python 3.8+
- Rust toolchain (for building the helper binary)
- Poetry (for Python dependency management)
- ActivityWatch server (aw-server or aw-server-rust)

## How it Works

1. The Rust helper connects to cosmic-comp and listens for window events using the `zcosmic_toplevel_info_v1` protocol
2. It outputs the current active window information as JSON to stdout
3. The Python wrapper reads this JSON and sends heartbeats to the ActivityWatch server

## Troubleshooting

### "Rust helper binary not found"

Run `make install` to build the Rust helper.

### "zcosmic-toplevel-info-v1 protocol not available"

This means you're not running on COSMIC desktop. This watcher only works with cosmic-comp.

### "WAYLAND_DISPLAY environment variable not set"

You're not running on Wayland. This watcher only works on COSMIC desktop.
