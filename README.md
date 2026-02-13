# Nodoka Audiobook Reader

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.82+-blue.svg)](https://www.rust-lang.org)
[![Build Status](https://github.com/otakukingdom/nodoka/workflows/Build%20and%20Test/badge.svg)](https://github.com/otakukingdom/nodoka/actions)

A cross-platform audiobook player built with Rust and iced. Nodoka provides automatic progress tracking, multi-format support, and a clean interface for managing your audiobook collection.

## Features

- 📚 Automatic library scanning and organization
- 🎵 Multi-format support (MP3, M4A, M4B, OGG, FLAC, WAV, WMA)
- 📊 SQLite-based progress tracking
- 🔊 Playback speed control (0.5x-2.0x) and volume adjustment
- 🌐 Cross-platform: Windows, macOS, and Linux

## Documentation

**📖 All documentation is in rustdoc:**

```bash
cargo doc --no-deps --open
```

Documentation includes:
- Installation guide (all platforms)
- Usage instructions
- Architecture overview
- Contributing guidelines (`nodoka::contributing`)
- Security policy (`nodoka::security`)
- API reference

## Quick Start

### Installation

Download pre-built installers from the [Releases](../../releases) page:

- **Windows**: `nodoka-0.2.0-x64.msi`
- **macOS**: `Nodoka-0.2.0.dmg`
- **Linux**: `nodoka_0.2.0_amd64.deb`

**Prerequisites**: VLC media player 3.x or later must be installed. See rustdoc for detailed installation instructions.

### Building from Source

```bash
# Install Rust 1.82+ and VLC development libraries
# (See rustdoc for platform-specific prerequisites)

git clone https://github.com/your-username/nodoka.git
cd nodoka
cargo build --release
./target/release/nodoka
```

### Using the Application

1. Launch Nodoka
2. Go to **Settings** → **Directories** → **Add Directory**
3. Select your audiobooks folder
4. Select an audiobook and click Play

## Project Status

**Version**: 0.2.0  
**Status**: Production Ready

- ✅ 18/18 tests passing
- ✅ Zero clippy warnings (strict mode)
- ✅ Cross-platform installers available
- ✅ No unsafe code, function-level allows only with inline justification

See `cargo doc --open` for detailed technical status and code quality metrics.

## Contributing

Contributions welcome! Please follow the project's strict linting requirements:

- All clippy lints at deny level
- No `unwrap()`, `expect()`, or `panic!()`
- Proper error handling with `Result`
- Doc comments for public APIs

See `cargo doc --open` and navigate to `nodoka::contributing` for detailed guidelines.

## License

MIT License - See rustdoc for full license text and credits.

## Links

- [Documentation](https://docs.rs/nodoka) (when published)
- [Issues](../../issues)
- [Releases](../../releases)
- [Changelog](CHANGELOG.md)
- Security Policy: See `nodoka::security` in rustdoc
