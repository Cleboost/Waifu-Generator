# Waifu Generator

A GTK 4 application developed in Rust for generating waifus.

## Prerequisites

### System Dependencies Installation

On Arch Linux (CachyOS):
```bash
sudo pacman -S gtk4 libadwaita
```

On Ubuntu/Debian:
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

On Fedora:
```bash
sudo dnf install gtk4-devel libadwaita-devel
```

### Rust Installation

If Rust is not yet installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Compilation and Execution

```bash
# Compile the project
cargo build

# Run in debug mode
cargo run

# Compile in release mode
cargo build --release

# Run the optimized version
cargo run --release
```

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── models/                 # Data structures
│   └── mod.rs             # WaifuTags, UserSettings
├── services/              # External services
│   └── mod.rs             # API calls (waifu.pics)
└── ui/                    # User interface
    ├── mod.rs             # Main UI module
    ├── main_window.rs     # Main window
    └── settings_window.rs # Settings window
```

### Modular Organization

- **`models/`** - Data structures and models
- **`services/`** - External services (API, database)
- **`ui/`** - User interface (windows, components)
- **`main.rs`** - Simplified entry point

## Dependencies

- **gtk4** - Rust bindings for GTK 4
- **gio** - Bindings for GIO (GNOME system utilities)
- **glib** - Bindings for GLib (GNOME base library)
- **reqwest** - HTTP client for API calls
- **serde** - Serialization/deserialization
- **tokio** - Async runtime

## Development

The application uses GTK 4 with the official gtk-rs bindings. For more information on GTK development with Rust, see:

- [gtk-rs Documentation](https://gtk-rs.org/)
- [GTK 4 Book with Rust](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [GTK 4 Documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)

## Features

- Modern user interface with GTK 4
- Waifu image generation from waifu.pics API
- Category selection (SFW/NSFW)
- Image navigation (previous/next)
- Image download functionality
- Settings persistence
- Responsive and accessible design