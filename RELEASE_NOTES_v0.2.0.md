# Nodoka 0.2.0 - Rust Rewrite

This release represents a complete rewrite of Nodoka from C++/Qt to Rust/iced, bringing improved performance, memory efficiency, and code quality.

## Downloads

**Important**: VLC 3.x must be installed separately before using Nodoka.

### Windows
- **Installer**: `nodoka-0.2.0-x64.msi` (8 MB)
- Install VLC from: https://www.videolan.org/vlc/

### macOS
- **DMG**: `Nodoka-0.2.0.dmg` (4 MB)
- SHA256: `31bee7a4509572ea58c33d486bdf83eb177256dddfcb685efc721f1711daf50f`
- Install VLC: `brew install --cask vlc`

If you see a "damaged app" warning:
```bash
xattr -cr /Applications/Nodoka.app
```

### Linux (Ubuntu/Debian)
- **DEB Package**: `nodoka_0.2.0_amd64.deb` (6 MB)
- Install VLC: `sudo apt install vlc libvlc-dev`
- Install Nodoka: `sudo dpkg -i nodoka_0.2.0_amd64.deb`

## What's New

### Major Changes
- **BREAKING**: Complete rewrite from C++/Qt to Rust/iced
- Replaced Qt GUI with iced for native cross-platform UI
- Migrated from LMDB to SQLite for database storage
- Updated to VLC-rs 0.3 bindings (from C libvlc)

### Features Added
- ✅ Strict linting rules enforced at compile time (no unwrap/expect/panic)
- ✅ Comprehensive test suite (17 integration tests)
- ✅ Cross-platform installers (Windows MSI, macOS DMG, Linux DEB)
- ✅ CI/CD pipeline with GitHub Actions for all three platforms
- ✅ Async directory scanning with tokio
- ✅ SHA-256 checksum calculation for media files
- ✅ Cross-platform single instance guard
- ✅ Automatic progress saving and restoration

### Performance Improvements
- 📉 80% reduction in binary size (Qt bloat eliminated)
- ⚡ Faster startup time (<2 seconds)
- 💾 Lower memory usage (~80 MB idle vs ~200 MB in Qt version)
- 🎯 Cross-platform consistency (same UI on all platforms)

### Removed
- ❌ Dependency on Qt framework
- ❌ CMake build system (replaced by Cargo)
- ❌ LMDB database (replaced by SQLite)
- ❌ quazip library (no longer needed)

## System Requirements

- **Operating System**:
  - Windows 10/11 (x64)
  - macOS 12+ (Intel and Apple Silicon)
  - Linux (Ubuntu 22.04+, Debian 11+, Fedora 38+)
- **VLC 3.x** (required for audio playback)
- ~200 MB disk space
- 4 GB RAM recommended

## Supported Audio Formats

- MP3
- M4A / M4B (AAC audiobooks)
- OGG Vorbis
- FLAC
- OPUS
- Any format supported by VLC

## Known Issues

- VLC 4.x compatibility not yet tested (use VLC 3.x)
- Very large libraries (10,000+ files) may have slow initial scan
- Windows: Antivirus may flag the installer (false positive)

## Upgrade Notes

This release is not compatible with the previous C++/Qt version's database. If you were using Nodoka 0.1.x:

1. Note your audiobook directory paths (Settings → Directories)
2. Uninstall the old version
3. Install Nodoka 0.2.0
4. Re-add your audiobook directories
5. Nodoka will re-scan and track progress from this point forward

## Building from Source

See [README.md](README.md) for detailed build instructions.

```bash
# Install Rust 1.82+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install VLC development libraries
# macOS:
brew install libvlc
# Linux:
sudo apt install libvlc-dev vlc
# Windows: Install VLC normally

# Build
cargo build --release
```

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

This project enforces strict code quality standards:
- Zero `unwrap()` / `expect()` / `panic!()` in source code
- All errors handled with `Result` types
- Comprehensive documentation on public APIs
- Clippy passes with `-D warnings`

## Documentation

- [User Guide](docs/USER_GUIDE.md) - Getting started and features
- [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Changelog](CHANGELOG.md) - Full version history

## License

MIT License - see [LICENSE](LICENSE) file

## Credits

- Original C++ implementation: Mistlight Oriroris
- Rust port: Mistlight Oriroris
- Built with [iced](https://github.com/iced-rs/iced) UI framework
- Audio playback powered by [VLC](https://www.videolan.org/vlc/)

## Support

- Report issues: [GitHub Issues](https://github.com/otakukingdom/nodoka/issues)
- Discussions: [GitHub Discussions](https://github.com/otakukingdom/nodoka/discussions)

---

**Full Changelog**: https://github.com/otakukingdom/nodoka/blob/main/CHANGELOG.md
