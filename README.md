# aw-watcher-window-cosmic

> ⚠️ **EXPERIMENTAL** - This project is in early development and may have bugs or missing features. Use at your own risk.

Window watcher for [ActivityWatch](https://activitywatch.net/) on the COSMIC desktop environment.

## Status

This is an experimental window watcher for the COSMIC desktop (Pop!_OS). The original `aw-watcher-window` does not support COSMIC's Wayland compositor, so this project fills that gap.

The eventual goal is to contribute this back to the [ActivityWatch](https://github.com/ActivityWatch) project.

## How it works

Unlike the original aw-watcher-window which uses Xlib (X11 only), this version uses the `zcosmic_toplevel_info_v1` Wayland protocol to track window activity on COSMIC.

The implementation consists of:
- A Rust helper binary that connects to the Wayland compositor and listens for toplevel window events
- A Python wrapper that reads window information from the helper and sends heartbeats to ActivityWatch

## Installation

### Prerequisites

- Rust toolchain (for building the helper binary)
- Python 3.8+
- Poetry (Python package manager)
- ActivityWatch server (`aw-server` or `aw-server-rust`)

### Build & Install

```bash
make install
```

This will:
1. Build the Rust helper binary (`cargo build --release`)
2. Install the Python package with dependencies via Poetry

## Usage

First, make sure the ActivityWatch server is running:

```bash
aw-server
# or
aw-server-rust
```

Then start the window watcher:

```bash
poetry run aw-watcher-window-cosmic
```

Or if installed globally:

```bash
aw-watcher-window-cosmic
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

## Troubleshooting

### "Rust helper binary not found"

Run `make build` to build the Rust helper.

### "zcosmic-toplevel-info-v1 protocol not available"

This means your Wayland compositor doesn't support the COSMIC toplevel protocol. Currently, only COSMIC desktop (cosmic-comp) is supported.

### "WAYLAND_DISPLAY environment variable not set"

You're not running on Wayland. This watcher only works on Wayland compositors.

## Contributing

This project aims to eventually be contributed to the [ActivityWatch](https://github.com/ActivityWatch) organization. Contributions, bug reports, and feature requests are welcome!

## License

MPL-2.0 (Mozilla Public License 2.0) - Same as ActivityWatch.

## Author

Marc Durepos <marc@bemade.org>
