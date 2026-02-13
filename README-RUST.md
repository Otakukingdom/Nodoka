# Nodoka Audiobook Reader - Rust Port

This is a Rust port of the Nodoka Audiobook Reader, converting from the original C++/Qt implementation to Rust with the iced GUI framework.

## Project Status

**Current Progress:** Core infrastructure completed (Steps 1-6, 13, 16 from implementation plan)

### Completed Components

- ✅ **Step 1**: Cargo workspace with strict linting configuration
- ✅ **Step 2**: Database layer with rusqlite (connection, schema, queries)
- ✅ **Step 3**: Core data models (Audiobook, AudiobookFile, Directory, MediaProperty)
- ✅ **Step 4**: VLC player wrapper with Rust bindings
- ✅ **Step 5**: Settings management layer
- ✅ **Step 6**: Iced application state and message types
- ✅ **Step 13**: Directory scanning tasks (async)
- ✅ **Step 16**: Main application entry point with logging

### Project Structure

```
src/
├── db/              # Database layer (rusqlite)
│   ├── connection.rs
│   ├── schema.rs
│   └── queries.rs
├── models/          # Domain models
│   ├── audiobook.rs
│   ├── audiobook_file.rs
│   ├── directory.rs
│   └── media_property.rs
├── player/          # VLC player integration
│   ├── concrete_player.rs
│   ├── scan_player.rs
│   └── events.rs
├── settings/        # Settings management
│   └── storage.rs
├── tasks/           # Async background tasks
│   ├── scan_directory.rs
│   ├── checksum.rs
│   └── player_scan.rs
├── ui/              # Iced UI framework
│   ├── message.rs
│   ├── state.rs
│   ├── styles.rs
│   └── components/
├── error.rs         # Error types
├── lib.rs
└── main.rs
```

## Dependencies

### Required

- **Rust 1.82+**: Modern Rust toolchain
- **VLC 3.x**: Required for media playback
  - macOS: Install VLC.app to `/Applications/VLC.app`
  - Windows: Install to `C:/Program Files/VideoLAN/VLC`
  - Linux: Install via package manager (`vlc`, `libvlc-dev`)

### Key Crates

- `iced 0.12`: GUI framework (Elm architecture)
- `vlc-rs 0.3`: Safe Rust bindings for libVLC
- `rusqlite 0.31`: SQLite database
- `tokio 1.35`: Async runtime
- `chrono 0.4`: Date/time handling
- `walkdir 2.4`: Directory traversal
- `sha2 0.10`: Checksums

## Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check

# Run lints
cargo clippy
```

## Linting Configuration

The project uses **strict linting rules** as per acceptance criteria:

- All clippy lints enabled at `deny` level
- No `unwrap()` or `expect()` calls allowed
- No `panic!()` macros
- No unsafe code
- No dead code
- All errors must be handled explicitly with `Result`

## Database Schema

The application uses SQLite with the following tables:

- `metadata`: Key-value settings storage
- `directories`: Tracked audiobook directories
- `audiobooks`: Audiobook metadata and progress
- `audiobook_file`: Individual audio file tracking

## Remaining Work

### UI Components (Steps 7-11)
- Main window layout
- Player controls component
- Audiobook list view
- File list view
- Settings dialog

### Application Logic (Step 12)
- Iced update function
- Event handlers
- Message routing

### Proxy Layer (Step 15)
- Audiobook proxy with caching
- File proxy
- Proxy manager

### Additional Tasks (Steps 14, 17-25)
- Media property scanning with VLC
- Cross-platform build configurations
- Installer packaging (MSI, DMG, DEB)
- Integration tests
- Manual testing

## Architecture

### Iced Application Pattern

The application follows the Elm architecture via iced:

1. **State**: `NodokaState` holds all application state
2. **Messages**: `Message` enum for all user/system events
3. **Update**: Pure function that transforms state based on messages
4. **View**: Pure function that renders UI from state

### Async Operations

Background tasks use Tokio for async operations:
- Directory scanning
- Media file parsing
- Checksum calculation
- Database operations

### Error Handling

All errors use the `NodokaError` enum with proper context:
- Database errors
- VLC errors
- IO errors
- Media parsing errors

## Color Scheme

Original Qt application colors preserved:
- Top bar: `#FEDB53` (yellow)
- Player controls: `#414141` (dark gray)
- Selected items: `#555152` (gray)
- Text: `#515151` (dark gray)

## Development Notes

### VLC Integration

The `vlc-rs` crate provides safe bindings but has some API differences from the C VLC API:
- Methods return `Option` instead of `Result`
- Some APIs require importing trait extensions (e.g., `MediaPlayerAudioEx`)
- Media duration requires parsing after loading

### Database Migrations

DateTime values are stored as RFC3339 strings and parsed on retrieval. This maintains compatibility with the original C++ SQLite schema.

### Single Instance Guard

Uses a lock file in the user's data directory to prevent multiple instances, replacing the original Qt `RunGuard`.

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## License

MIT License (same as original project)

## Credits

Original C++/Qt implementation by Mistlight Oriroris
Rust port: 2025
