# Remaining Tasks for Production Release

**Generated**: February 12, 2026  
**Updated**: February 12, 2026 (Implementation Session)  
**Status**: macOS tasks complete - Linux/Windows platform-specific tasks remain

## Automated Verification Complete ✅

All automated verification and cleanup tasks from the implementation plan have been completed:

- ✅ **Step 1**: Comprehensive audit and acceptance criteria validation
- ✅ **Steps 2-4**: C++ source cleanup (already completed during conversion)
- ✅ **Step 5**: Redundant documentation consolidation  
- ✅ **Step 9**: Code quality analysis and documentation coverage
- ✅ **Step 10**: CI/CD pipeline configuration (.github/workflows/build.yml)
- ✅ **Step 11**: CHANGELOG.md creation
- ✅ **Step 12**: CONTRIBUTING.md creation
- ✅ **Step 14**: User documentation (USER_GUIDE.md, TROUBLESHOOTING.md)
- ✅ **Step 15**: Security audit (dependency review - cargo audit unavailable)
- ✅ **Step 16**: Final pre-release verification checklist

## Completed Tasks (This Session) ✅

### Step 1: Document Current Conversion Status ✅
- Verified zero C++ source files remain in repository
- Confirmed 38 Rust source files present in src/
- Validated all 18 tests passing (7 database + 6 models + 4 tasks + 1 doc)
- Verified zero clippy warnings with -D warnings flag
- Confirmed VLC-rs 0.3 bindings in use
- Verified iced 0.12 UI framework implemented
- Release binary verified: 8.0MB with VLC linking (@rpath/libvlc.dylib)

### Step 5: Generate Release Checksums (Partial) ✅
- Created SHA256SUMS.txt with macOS DMG checksum
- Checksum: 82a8c3d1233dffbf38c82b5ffd4ba9b31f7a0c3498ca913130194962f2a7c9f9
- Updated RELEASE_NOTES_v0.2.0.md with correct SHA256 hash
- Placeholder added for Linux/Windows checksums (pending builds)

### Step 10: Document Lessons Learned ✅
- Created comprehensive docs/LESSONS_LEARNED.md (500+ lines)
- Documented VLC bindings migration (C++ libvlc → vlc-rs)
- Documented GUI framework migration (Qt → iced)
- Documented database migration (LMDB → SQLite)
- Documented error handling patterns (exceptions → Result types)
- Documented async/await patterns (QThread → tokio)
- Documented build system migration (CMake → Cargo)
- Documented packaging strategies for all platforms
- Included performance metrics and profiling techniques
- Added future roadmap recommendations

## Manual Tasks Remaining

The following tasks require manual intervention or platform-specific environments:

### Step 6: Cross-Platform VLC Integration Testing (HIGH PRIORITY)

**Platforms to Test**:
1. **Linux (Ubuntu 22.04/24.04)**
   ```bash
   # Install VLC
   sudo apt-get update && sudo apt-get install -y libvlc-dev vlc
   
   # Build and test
   cargo build --release
   cargo test --all
   ldd target/release/nodoka | grep vlc
   
   # Test playback
   ./target/release/nodoka
   ```

2. **Linux (Debian 11/12)**
   ```bash
   sudo apt-get install libvlc-dev vlc
   cargo build --release
   cargo test --all
   ```

3. **Windows 10/11**
   ```powershell
   # Install VLC 3.x from videolan.org
   # Install Rust from rustup.rs
   
   cargo build --release
   cargo test --all
   
   # Verify DLL loading
   dumpbin /dependents target\release\nodoka.exe
   ```

**Test Cases**:
- Audio playback with MP3, M4A, M4B, OGG, FLAC formats
- Volume control (0-100%)
- Speed adjustment (0.5x-2.0x)
- Seek functionality
- Multi-file audiobook navigation

### Step 7: Build Platform-Specific Installers (CRITICAL)

**macOS DMG**: ✅ COMPLETE
```bash
cd packaging/macos
./create-dmg.sh
hdiutil verify Nodoka-0.2.0.dmg
# Result: 4.0MB DMG created and verified successfully
# Test installation on macOS 12+ (Intel and Apple Silicon) - PENDING MANUAL TEST
```

**Linux DEB**:
```bash
cd packaging/linux
./build-deb.sh
dpkg-deb --info nodoka_0.2.0_amd64.deb
sudo dpkg -i nodoka_0.2.0_amd64.deb
# Verify desktop entry appears in application menu
```

**Windows MSI**:
```powershell
# Install WiX Toolset v3.14+
cd packaging\windows
candle nodoka.wxs
light -ext WixUIExtension nodoka.wixobj
# Test installer wizard
```

**Generate Checksums**:
```bash
sha256sum Nodoka-0.2.0.dmg > SHA256SUMS.txt
sha256sum nodoka_0.2.0_amd64.deb >> SHA256SUMS.txt
sha256sum nodoka-0.2.0-x64.msi >> SHA256SUMS.txt
```

### Step 8: Manual Smoke Testing (HIGH PRIORITY)

Execute comprehensive smoke test checklist on each platform:

**1. First Launch**
- [ ] Application launches without errors
- [ ] Database initializes in correct location (~/.nodoka/ or %APPDATA%\Nodoka)
- [ ] Single instance guard prevents duplicate launches

**2. Directory Management**
- [ ] Settings dialog opens and displays correctly
- [ ] Add directory via file picker
- [ ] Directory scanning completes and shows progress
- [ ] Audiobooks appear in library after scan

**3. Playback**
- [ ] Select audiobook from list
- [ ] Play/pause controls work correctly
- [ ] Volume slider adjusts audio level (0-100%)
- [ ] Speed control changes playback rate (0.5x-2.0x)
- [ ] Seek slider jumps to correct position
- [ ] Progress is saved when closing application

**4. Multi-file Audiobooks**
- [ ] File list displays all chapters
- [ ] Clicking file in list starts playback
- [ ] Auto-advance to next file works

**5. Edge Cases**
- [ ] Empty directory handling
- [ ] Non-audio files are ignored during scan
- [ ] Very long audiobook titles display correctly
- [ ] Resume playback after restart

### Step 13: GitHub Repository Metadata (LOW PRIORITY)

**Repository Settings** (via GitHub web interface):
1. **Description**: "A cross-platform audiobook reader built with Rust and iced. Features automatic progress tracking, VLC-powered playback, and a clean UI."

2. **Topics/Tags**: 
   - rust
   - audiobook
   - iced
   - vlc
   - cross-platform
   - desktop-app
   - audiobook-player
   - audiobook-reader

3. **README Badges** (add to README.md):
   ```markdown
   ![Build Status](https://github.com/USERNAME/nodoka/workflows/Build%20and%20Test/badge.svg)
   ![License](https://img.shields.io/badge/license-MIT-blue.svg)
   ![Rust Version](https://img.shields.io/badge/rust-1.82%2B-orange.svg)
   ![Release](https://img.shields.io/github/v/release/USERNAME/nodoka)
   ```

4. **Issue Templates**: Already created in `.github/ISSUE_TEMPLATE/`
   - ✅ bug_report.md
   - ✅ feature_request.md
   - ✅ question.md

### Step 17: GitHub Release Creation (HIGH PRIORITY)

**Create and Push Tag**:
```bash
git tag -a v0.2.0 -m "Nodoka 0.2.0 - Rust Rewrite Release"
git push origin v0.2.0
```

**Wait for CI/CD** to build installers on all platforms (via GitHub Actions)

**Create GitHub Release** (via GitHub web interface):
1. **Tag**: v0.2.0
2. **Title**: "Nodoka 0.2.0 - Rust Rewrite"
3. **Description**: Copy from CHANGELOG.md or RELEASE_NOTES_v0.2.0.md
4. **Attachments**:
   - Nodoka-0.2.0.dmg (macOS)
   - nodoka_0.2.0_amd64.deb (Linux)
   - nodoka-0.2.0-x64.msi (Windows)
   - SHA256SUMS.txt (checksums)
5. Mark as **Latest Release**

## Verification Commands Reference

For quick reference during manual testing:

```bash
# Full verification suite
cargo clean
cargo build --release
cargo test --all
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
rg '\.unwrap\(|\.expect\(|#\[allow' src/
cargo doc --no-deps

# Platform-specific VLC verification
otool -L target/release/nodoka | grep vlc  # macOS
ldd target/release/nodoka | grep vlc       # Linux  
dumpbin /dependents target\release\nodoka.exe  # Windows

# Binary size check
ls -lh target/release/nodoka     # macOS/Linux
dir target\release\nodoka.exe    # Windows
```

## Next Steps

1. **Immediate**: Test on Linux and Windows platforms (Step 6)
2. **Immediate**: Build all platform installers (Step 7)
3. **Before Release**: Complete smoke testing on all platforms (Step 8)
4. **Before Release**: Create GitHub release with installers (Step 17)
5. **Optional**: Configure repository metadata for discoverability (Step 13)

## Success Criteria

The project is ready for v0.2.0 release when:
- ✅ All automated verification passes (COMPLETE)
- ⚠️ Cross-platform testing completed (Linux, Windows, macOS)
- ⚠️ Platform-specific installers built and tested
- ⚠️ Manual smoke testing completed on all platforms
- ⚠️ GitHub release created with all installers attached

---

**Note**: This document tracks remaining manual tasks after automated verification. All code quality and automated testing requirements are already met.
