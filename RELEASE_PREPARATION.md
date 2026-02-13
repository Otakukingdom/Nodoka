# Release Preparation Guide for v0.2.0

This document outlines the steps required to prepare and publish the v0.2.0 release with all platform installers.

## Prerequisites Checklist

- [x] All 18 tests passing (`cargo test --all`)
- [x] Zero clippy warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [x] No forbidden patterns in src/ (no unwrap/expect/allow)
- [x] macOS DMG installer built and verified
- [x] Linux DEB packaging script ready
- [x] Windows MSI packaging script ready
- [x] CI/CD pipeline configured for automated builds
- [ ] Cross-platform smoke tests completed (see SMOKE_TEST_CHECKLIST.md)

## Release Workflow

### Option 1: Automated Release via CI/CD (Recommended)

The GitHub Actions workflow automatically builds all installers when a release is created:

1. **Create Git Tag**
   ```bash
   git tag -a v0.2.0 -m "Nodoka 0.2.0 - Rust Rewrite Release"
   git push origin v0.2.0
   ```

2. **Create GitHub Release**
   - Go to: https://github.com/otakukingdom/nodoka/releases/new
   - Tag: `v0.2.0` (select existing tag)
   - Release title: `Nodoka 0.2.0 - Rust Rewrite`
   - Description: Copy content from `RELEASE_NOTES_v0.2.0.md`
   - Mark as "Latest release"
   - Click "Publish release"

3. **Wait for CI/CD to Complete**
   - GitHub Actions will automatically:
     - Build Linux DEB package on `ubuntu-latest`
     - Build Windows MSI installer on `windows-latest`
     - Build macOS DMG installer on `macos-latest`
     - Generate SHA256 checksums for all installers
     - Upload all artifacts to the release

4. **Verify Release Assets**
   - Check that all 4 files are attached to the release:
     - `nodoka_0.2.0_amd64.deb` (Linux)
     - `Nodoka-0.2.0.msi` (Windows)
     - `Nodoka-0.2.0.dmg` (macOS)
     - `SHA256SUMS.txt` (checksums)

5. **Update Release Notes**
   - Edit the GitHub release
   - Add SHA256 checksums from `SHA256SUMS.txt` to the downloads table
   - Save changes

### Option 2: Manual Release (If CI/CD Unavailable)

If GitHub Actions is not available, build installers manually:

1. **Build Linux DEB** (requires Linux/Ubuntu 22.04+)
   ```bash
   cargo build --release --target x86_64-unknown-linux-gnu
   cd packaging/linux
   chmod +x build-deb.sh
   ./build-deb.sh
   # Output: nodoka_0.2.0_amd64.deb
   ```

2. **Build Windows MSI** (requires Windows + WiX Toolset)
   ```powershell
   cargo build --release --target x86_64-pc-windows-msvc
   cd packaging/windows
   candle.exe nodoka.wxs
   light.exe -ext WixUIExtension -out nodoka-0.2.0-x64.msi nodoka.wixobj
   # Output: nodoka-0.2.0-x64.msi
   ```

3. **Build macOS DMG** (requires macOS 12+)
   ```bash
   # Universal binary for Intel + Apple Silicon
   cargo build --release --target x86_64-apple-darwin
   cargo build --release --target aarch64-apple-darwin
   lipo -create \
     target/x86_64-apple-darwin/release/nodoka \
     target/aarch64-apple-darwin/release/nodoka \
     -output target/release/nodoka-universal
   
   cd packaging/macos
   chmod +x create-dmg.sh
   ./create-dmg.sh
   # Output: Nodoka-0.2.0.dmg
   ```

4. **Generate Checksums**
   ```bash
   # Collect all installers in one directory
   mkdir -p release-artifacts
   cp packaging/linux/nodoka_0.2.0_amd64.deb release-artifacts/
   cp packaging/windows/nodoka-0.2.0-x64.msi release-artifacts/
   cp packaging/macos/Nodoka-0.2.0.dmg release-artifacts/
   
   # Generate checksums
   cd release-artifacts
   sha256sum * > SHA256SUMS.txt
   cd ..
   
   echo "SHA256 Checksums:"
   cat release-artifacts/SHA256SUMS.txt
   ```

5. **Create GitHub Release**
   - Tag: `v0.2.0`
   - Upload all files from `release-artifacts/`
   - Copy release notes from `RELEASE_NOTES_v0.2.0.md`

## Smoke Testing

Before finalizing the release, perform comprehensive smoke tests on all platforms:

1. **Download installers** from GitHub release (or use local builds)

2. **Follow SMOKE_TEST_CHECKLIST.md** for each platform:
   - macOS 12+ (Intel and Apple Silicon if possible)
   - Ubuntu 22.04 LTS or Debian 11+
   - Windows 10 or Windows 11

3. **Complete all 7 test scenarios** on each platform:
   - Installation Verification
   - First Launch
   - Directory Management
   - Audio Playback (**must verify actual audio output**)
   - Progress Persistence
   - Multi-File Audiobooks
   - Audio Format Support (MP3, M4A, M4B, OGG, FLAC)

4. **Document results** in SMOKE_TEST_CHECKLIST.md

5. **Critical issues block release** - fix before publishing
   - Application crashes
   - No audio output
   - Data loss/corruption
   - Installation failures

6. **Non-critical issues** can be addressed in patch release
   - UI glitches
   - Minor performance issues
   - Feature requests

## Post-Release Tasks

After publishing the release:

1. **Update README.md**
   - Update download links to point to v0.2.0 release
   - Add SHA256 checksums to Installation section
   - Update release status badge (if applicable)

2. **Update CHANGELOG.md**
   - Move v0.2.0 from "Unreleased" to "Released" with date
   - Add GitHub release link

3. **Announce Release**
   - GitHub Discussions post
   - Social media (if applicable)
   - Update project website (if applicable)

4. **Monitor Issues**
   - Watch for bug reports from users
   - Prioritize critical issues for patch release
   - Plan v0.2.1 if needed

## Verification Commands

Run these commands to verify the release is ready:

```bash
# 1. Verify version in Cargo.toml
grep '^version = "0.2.0"' Cargo.toml

# 2. Run full test suite
cargo test --all

# 3. Run strict linting
cargo clippy --all-targets --all-features -- -D warnings

# 4. Check for forbidden patterns
rg '\.unwrap\(|\.expect\(|#\[allow' src/

# 5. Verify packaging scripts exist
ls -la packaging/linux/build-deb.sh
ls -la packaging/windows/nodoka.wxs
ls -la packaging/macos/create-dmg.sh

# 6. Check git status
git status  # Should be clean on main branch

# 7. Verify latest commit
git log -1 --oneline

# 8. Check remote
git remote -v
```

## Expected File Sizes

Reference file sizes for v0.2.0 installers:

| File | Approximate Size | Platform |
|------|------------------|----------|
| `nodoka_0.2.0_amd64.deb` | ~8 MB | Linux (Ubuntu/Debian) |
| `Nodoka-0.2.0.msi` | ~9 MB | Windows 10/11 x64 |
| `Nodoka-0.2.0.dmg` | ~4 MB | macOS 12+ (Universal) |
| `SHA256SUMS.txt` | <1 KB | All platforms |

If file sizes differ significantly (>50%), investigate potential issues.

## Rollback Procedure

If critical issues are discovered after release:

1. **Mark release as "Pre-release"** on GitHub (uncheck "Latest release")
2. **Create GitHub issue** documenting the critical bug
3. **Fix the issue** in a new branch
4. **Increment version** to v0.2.1
5. **Follow release workflow** again for v0.2.1
6. **Delete v0.2.0 tag and release** (optional, if severely broken)

## Contact Information

For questions or issues during release:
- **Repository**: https://github.com/otakukingdom/nodoka
- **Issues**: https://github.com/otakukingdom/nodoka/issues
- **Maintainer**: Mistlight Oriroris <mistlight@otakukingdom.com>

---

**Status**: Ready for automated release via GitHub Actions CI/CD  
**Last Updated**: February 12, 2026
