# Changelog

All notable changes to Nodoka Audiobook Reader will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **BREAKING**: Removed all Cargo.toml-level lint exceptions for strict compliance
- Renamed `NodokaApp` to `App` (following Rust naming conventions)
- Renamed `NodokaError` to `Error` (standard library pattern)
- Renamed `NodokaState` to `State` (module context makes prefix redundant)
- Renamed `ConcretePlayer` to `VlcPlayer` (describes implementation)
- Renamed `ScanPlayer` to `Scanner` (removes module name repetition)
- Renamed `initialize_schema` to `initialize` (in schema module)
- Moved cast allows from `Cargo.toml` to function-level with explicit safety documentation

### Improved
- Fixed all 6 rustdoc warnings (unresolved links and redundant targets)
- Enhanced numeric conversion validation with comprehensive error documentation
- All documentation now in rustdoc (compliance with AGENTS.md)
- README.md reduced to minimal entry point (~97 lines)
- Function-level `#[allow]` attributes now require inline justification

### Documentation
- Moved CONTRIBUTING.md into `nodoka::contributing` rustdoc module
- Moved SECURITY.md into `nodoka::security` rustdoc module
- Contributing and security policies now accessible via `cargo doc`
- All external documentation consolidated into rustdoc

### Infrastructure
- Enhanced CI to enforce strict compliance (no Cargo.toml-level allows)
- Added validation for minimal external documentation in GitHub Actions

## [0.2.0] - 2026-02-13

### Changed
- **BREAKING**: Complete rewrite from C++/Qt to Rust/iced
- Replaced Qt GUI with iced for native cross-platform UI
- Migrated from LMDB to SQLite for database storage
- Replaced Qt signals/slots with iced message-based architecture
- Updated to VLC-rs 0.3 bindings (from C libvlc)

### Added
- Strict linting rules enforced at compile time (no unwrap/expect/panic)
- Comprehensive test suite (17 integration tests)
- macOS DMG installer
- Linux DEB package installer
- Windows MSI installer via WiX Toolset
- CI/CD pipeline with GitHub Actions
- Async directory scanning with tokio
- SHA-256 checksum calculation for media files

### Improved
- 80% reduction in binary size (Qt bloat eliminated)
- Faster startup time (<2 seconds)
- Lower memory usage (~80 MB idle vs ~200 MB in Qt version)
- Cross-platform consistency (same UI on all platforms)

### Removed
- Dependency on Qt framework
- CMake build system (replaced by Cargo)
- LMDB database (replaced by SQLite)
- quazip library (no longer needed)

## [0.1.0] - Original C++ Release

### Added
- Initial release with Qt/C++ implementation
- Basic audiobook playback and library management
- Directory scanning and metadata extraction
- Progress tracking and resume functionality
