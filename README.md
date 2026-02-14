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

- ✅ **220+ acceptance tests** (all passing)
- ✅ ~95% specification coverage across all 18 feature areas
- ✅ Comprehensive test suite: library management, playback, metadata, UI features, edge cases
- ✅ Strict Rust idioms (no unwrap/expect/panic in production code)
- ✅ Cross-platform compatibility tested (Windows, macOS, Linux)
- ✅ Performance tested with 1000+ audiobook libraries
- ✅ No unsafe code, function-level allows only with inline justification
- ✅ Natural sorting for audiobook files (Chapter 1 before Chapter 10)
- ✅ Hidden file filtering (ignores .DS_Store, .hidden files)
- ✅ Recursive directory scanning (unlimited depth)

### Test Coverage Details

Test coverage by specification category:
- **Category A (Library Management)**: 23 tests - 100% coverage
- **Category B (Playback)**: 53 tests - 100% coverage
- **Category C (User Features)**: 32 tests - 100% coverage
- **Category D (Metadata & Organization)**: 29 tests - 100% coverage
- **Category E (Advanced Playback)**: 20 tests - 85% coverage (Skip Silence not implemented)
- **Category F (Application)**: 41 tests - 95% coverage

See `tests/MANUAL_TESTING.md` for manual testing procedures (keyboard shortcuts, file picker dialogs, UI responsiveness) and `cargo doc --open` for detailed test documentation.

### Testing

Run the comprehensive acceptance test suite:
```bash
# All acceptance tests
cargo test --test 'acceptance_*'

# Specific category
cargo test --test acceptance_playback_controls
cargo test --test acceptance_library_management

# View test documentation
cargo doc --no-deps --open
# Navigate to tests::acceptance_tests module
```

Test coverage:
- **220 total tests** across 18 test files
- Library Management: 53 tests (directory management, scanning, archives)
- Playback: 53 tests (controls, navigation, progress tracking)
- User Features: 32 tests (bookmarks, completion, cover art)
- Metadata & Organization: 29 tests
- Advanced Features: 12 tests (sleep timer)
- Application: 41 tests (settings, errors, lifecycle, cross-platform)

**VLC-Dependent Tests**: Some tests require VLC media player to be installed. Tests gracefully skip if VLC is unavailable, ensuring CI/CD compatibility.

```bash
# Install VLC for full test coverage

# macOS
brew install --cask vlc

# Linux (Ubuntu/Debian)
sudo apt install vlc libvlc-dev

# Windows
# Download from https://www.videolan.org/vlc/
```

See `cargo doc --open` for detailed test documentation and implementation status.

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
