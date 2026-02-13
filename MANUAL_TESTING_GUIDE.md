# Manual Testing Guide for Nodoka 0.2.0

This guide outlines the manual smoke tests to be performed on all three platforms before releasing v0.2.0.

## Test Environment Setup

### macOS Test Environment
- **OS Version**: macOS 12+ (Monterey or later)
- **VLC Version**: 3.0.x
- **Installer**: Nodoka-0.2.0.dmg
- **Test Machine**: Clean macOS VM or physical Mac

### Linux Test Environment
- **OS Version**: Ubuntu 22.04 LTS or Ubuntu 24.04 LTS
- **VLC Version**: 3.0.x
- **Installer**: nodoka_0.2.0_amd64.deb
- **Test Machine**: Clean Ubuntu VM

### Windows Test Environment
- **OS Version**: Windows 10 or Windows 11
- **VLC Version**: 3.0.x
- **Installer**: nodoka-0.2.0-x64.msi
- **Test Machine**: Clean Windows VM

## Pre-Test Setup

### Prepare Test Audiobooks

Create a test audiobook collection with:

1. **Single-file audiobook** (MP3):
   - File: `test-single.mp3` (any MP3 file, rename if needed)
   - Directory: `TestAudiobooks/Single File Book/`

2. **Multi-file audiobook** (MP3 chapters):
   - Files: `chapter-01.mp3`, `chapter-02.mp3`, `chapter-03.mp3`
   - Directory: `TestAudiobooks/Multi File Book/`

3. **M4B audiobook**:
   - File: `test-audiobook.m4b` (or .m4a)
   - Directory: `TestAudiobooks/M4B Book/`

4. **Mixed formats** (if available):
   - `test.ogg`, `test.flac`, `test.opus`
   - Directory: `TestAudiobooks/Format Tests/`

Place test files in a known location before starting tests.

## Test Scenarios

### Scenario 1: Installation Verification

#### macOS
1. [ ] Download `Nodoka-0.2.0.dmg` from GitHub release
2. [ ] Verify SHA256 checksum matches SHA256SUMS.txt:
   ```bash
   shasum -a 256 Nodoka-0.2.0.dmg
   ```
3. [ ] Double-click DMG to open
4. [ ] Verify Nodoka.app icon appears in DMG window
5. [ ] Drag Nodoka.app to Applications folder
6. [ ] Verify copy completes without errors
7. [ ] Close DMG window
8. [ ] Open Applications folder
9. [ ] Verify Nodoka.app is present (~8 MB)
10. [ ] If Gatekeeper blocks: Run `xattr -cr /Applications/Nodoka.app`

**Expected**: DMG opens cleanly, app copies to Applications, no corruption warnings.

#### Linux
1. [ ] Download `nodoka_0.2.0_amd64.deb` from GitHub release
2. [ ] Verify SHA256 checksum:
   ```bash
   sha256sum nodoka_0.2.0_amd64.deb
   ```
3. [ ] Install package:
   ```bash
   sudo dpkg -i nodoka_0.2.0_amd64.deb
   ```
4. [ ] Check for dependency errors
5. [ ] If missing dependencies:
   ```bash
   sudo apt-get install -f
   ```
6. [ ] Verify installation:
   ```bash
   which nodoka
   dpkg -L nodoka
   ```
7. [ ] Check desktop entry:
   ```bash
   cat /usr/share/applications/nodoka.desktop
   ```
8. [ ] Verify icon exists:
   ```bash
   ls /usr/share/icons/hicolor/256x256/apps/nodoka.png
   ```

**Expected**: Package installs cleanly, all files present, desktop entry valid.

#### Windows
1. [ ] Download `nodoka-0.2.0-x64.msi` from GitHub release
2. [ ] Verify SHA256 checksum:
   ```powershell
   certutil -hashfile nodoka-0.2.0-x64.msi SHA256
   ```
3. [ ] Double-click MSI installer
4. [ ] If UAC prompt appears, click "Yes"
5. [ ] Follow installation wizard
6. [ ] Accept license (if shown)
7. [ ] Choose installation directory (default: C:\Program Files\Nodoka)
8. [ ] Click "Install"
9. [ ] Wait for completion
10. [ ] Click "Finish"
11. [ ] Verify Start Menu shortcut exists:
    - Start → "Nodoka Audiobook Reader"
12. [ ] Verify installation directory:
    ```powershell
    dir "C:\Program Files\Nodoka"
    ```
13. [ ] Check Windows Defender didn't flag the binary

**Expected**: MSI installs without errors, Start Menu shortcut created, binary not flagged.

### Scenario 2: First Launch

#### All Platforms
1. [ ] Launch Nodoka (Applications folder / Start Menu / terminal `nodoka`)
2. [ ] Verify application window opens within 5 seconds
3. [ ] Check window title: "Nodoka Audiobook Reader"
4. [ ] Verify UI loads completely:
   - [ ] Top bar with yellow background (#FEDB53)
   - [ ] Settings button visible in top right
   - [ ] Empty audiobook list (gray background)
   - [ ] Player controls at bottom (dark gray #414141)
   - [ ] Status bar showing "Ready"
5. [ ] Check VLC library loads without errors
6. [ ] Verify no crash or error dialogs appear
7. [ ] Check data directory created:
   - macOS/Linux: `~/.nodoka/`
   - Windows: `%APPDATA%\Nodoka\`
8. [ ] Verify database file created:
   - `~/.nodoka/nodoka.db` (macOS/Linux)
   - `%APPDATA%\Nodoka\nodoka.db` (Windows)
9. [ ] Verify instance lock file created:
   - `~/.nodoka/instance.lock`

**Expected**: App launches successfully, UI renders correctly, database initializes, no errors.

### Scenario 3: Single Instance Guard

1. [ ] With Nodoka already running, attempt to launch second instance
2. [ ] Verify second instance exits immediately
3. [ ] First instance remains running and comes to foreground
4. [ ] No error dialog appears to user

**Expected**: Only one instance runs at a time, seamless user experience.

### Scenario 4: Directory Management

1. [ ] Click **Settings** button (top right)
2. [ ] Verify Settings dialog opens
3. [ ] Verify "Directories" tab is active
4. [ ] Check empty directory list shown
5. [ ] Click **Add Directory** button
6. [ ] File picker dialog opens
7. [ ] Navigate to prepared test audiobooks folder
8. [ ] Select folder: `TestAudiobooks/`
9. [ ] Click "Select Folder" or "OK"
10. [ ] Verify directory added to list
11. [ ] Check scanning starts (status bar shows progress)
12. [ ] Wait for scan to complete
13. [ ] Close Settings dialog

**Expected**: Directory picker works, path added to list, scanning starts automatically.

### Scenario 5: Audiobook Library Scanning

1. [ ] After adding directory, verify status bar shows "Scanning..."
2. [ ] Watch for scan progress updates
3. [ ] Wait for scan to complete (status: "Ready" or "Scan complete")
4. [ ] Check audiobook list (left panel) shows discovered books:
   - [ ] "Single File Book"
   - [ ] "Multi File Book"
   - [ ] "M4B Book"
   - [ ] "Format Tests" (if prepared)
5. [ ] Verify book titles match directory names
6. [ ] Check book list is sorted alphabetically
7. [ ] Verify no duplicate entries
8. [ ] Verify no scanning errors in status bar

**Expected**: All test audiobooks discovered, displayed in list, no errors.

### Scenario 6: Audio Playback

#### Single File Audiobook
1. [ ] Click "Single File Book" in list
2. [ ] Verify file list (right panel) shows `test-single.mp3`
3. [ ] Click **Play** button
4. [ ] Verify playback starts within 2 seconds
5. [ ] **CRITICAL**: Confirm audio is audible from speakers/headphones
6. [ ] Check Play button changes to Pause button
7. [ ] Verify seek slider moves as playback progresses
8. [ ] Check current time updates (e.g., "0:05 / 3:45")
9. [ ] Click **Pause** button
10. [ ] Verify playback stops immediately
11. [ ] Resume playback
12. [ ] Test **Volume** slider:
    - [ ] Drag to 0% → audio muted
    - [ ] Drag to 50% → medium volume
    - [ ] Drag to 100% → full volume
13. [ ] Test **Speed** controls:
    - [ ] Click **-** button → speed decreases (e.g., 0.9x)
    - [ ] Click **+** button → speed increases (e.g., 1.1x)
    - [ ] Verify audio pitch changes accordingly
14. [ ] Test **Seek** slider:
    - [ ] Drag to middle → playback jumps to ~50%
    - [ ] Drag to end → playback near end
    - [ ] Verify time label updates correctly

**Expected**: Playback works, audio audible, all controls functional.

#### Multi-File Audiobook
1. [ ] Click "Multi File Book" in list
2. [ ] Verify file list shows three chapters:
   - [ ] `chapter-01.mp3`
   - [ ] `chapter-02.mp3`
   - [ ] `chapter-03.mp3`
3. [ ] Click `chapter-02.mp3` in file list
4. [ ] Click **Play**
5. [ ] Verify chapter 2 starts playing
6. [ ] Click `chapter-01.mp3` in file list
7. [ ] Verify playback switches to chapter 1
8. [ ] Let chapter 1 play to completion (or seek to end)
9. [ ] Verify auto-advance to chapter 2 works
10. [ ] Verify file list highlights current file

**Expected**: Multi-file navigation works, auto-advance functional.

### Scenario 7: Progress Persistence

1. [ ] Start playing "Single File Book"
2. [ ] Seek to 50% position
3. [ ] Wait 5 seconds (for auto-save)
4. [ ] Note exact time position (e.g., "1:52 / 3:45")
5. [ ] Close Nodoka completely (File → Quit or close window)
6. [ ] Wait 3 seconds
7. [ ] Relaunch Nodoka
8. [ ] Click "Single File Book" in list
9. [ ] Verify progress indicator shows ~50% complete
10. [ ] Click **Play**
11. [ ] Verify playback resumes at saved position (~1:52)
12. [ ] Seek to different position (e.g., 25%)
13. [ ] Close and relaunch again
14. [ ] Verify new position (25%) is restored

**Expected**: Progress saves automatically, restores accurately on restart.

### Scenario 8: File Format Support

Test each supported format (if test files available):

1. [ ] **MP3**: Plays correctly (already tested above)
2. [ ] **M4A/M4B**: Select "M4B Book" → Play → Audio works
3. [ ] **OGG**: Select format test book → Play OGG file → Audio works
4. [ ] **FLAC**: Select format test book → Play FLAC file → Audio works
5. [ ] **OPUS**: Select format test book → Play OPUS file → Audio works

**Expected**: All VLC-supported formats play without errors.

### Scenario 9: Database Operations

1. [ ] Open Settings → Directories
2. [ ] Verify added directory is listed
3. [ ] Select the directory
4. [ ] Click **Remove Directory** button
5. [ ] Confirm removal in dialog (if shown)
6. [ ] Verify directory removed from list
7. [ ] Close Settings dialog
8. [ ] Verify audiobook list is now empty
9. [ ] Re-add the directory (repeat Scenario 4)
10. [ ] Verify books reappear after rescan

**Expected**: Directory removal works, cascade delete cleans up audiobooks.

### Scenario 10: Error Handling

#### VLC Not Found (if testable)
1. [ ] Temporarily rename VLC installation directory
2. [ ] Launch Nodoka
3. [ ] Verify helpful error message appears
4. [ ] Verify app doesn't crash
5. [ ] Restore VLC directory

#### Invalid Audio File
1. [ ] Create a fake MP3: `echo "not an audio file" > TestAudiobooks/Broken/fake.mp3`
2. [ ] Add directory containing fake.mp3
3. [ ] Verify Nodoka doesn't crash during scan
4. [ ] Verify invalid file is either skipped or marked as 0 duration

#### Corrupted Database
1. [ ] Close Nodoka
2. [ ] Corrupt database: `echo "garbage" >> ~/.nodoka/nodoka.db`
3. [ ] Launch Nodoka
4. [ ] Verify graceful error message or database recreation
5. [ ] Verify app doesn't crash

**Expected**: Errors handled gracefully, no crashes, helpful messages.

## Platform-Specific Tests

### macOS Specific
1. [ ] Test on both Intel and Apple Silicon (if possible)
2. [ ] Verify universal binary works on both architectures
3. [ ] Check Gatekeeper acceptance (no "damaged app" error)
4. [ ] Verify icon appears in Dock correctly
5. [ ] Test with macOS 12, 13, 14 (if VMs available)
6. [ ] Check menu bar integration (if implemented)

### Linux Specific
1. [ ] Test on Ubuntu 22.04 and 24.04
2. [ ] Verify desktop integration (icon in launcher)
3. [ ] Test with GNOME desktop environment
4. [ ] Test with KDE Plasma desktop environment (if available)
5. [ ] Verify PulseAudio audio output
6. [ ] Verify PipeWire audio output (Ubuntu 24.04)
7. [ ] Check uninstall: `sudo apt remove nodoka`
8. [ ] Verify clean removal (no leftover files in /usr/)

### Windows Specific
1. [ ] Test on Windows 10 and Windows 11
2. [ ] Verify UAC elevation works during install
3. [ ] Check Windows Defender doesn't flag binary
4. [ ] Verify Start Menu shortcut works
5. [ ] Test uninstall from Control Panel → Programs
6. [ ] Verify clean removal (no leftover files in Program Files)
7. [ ] Check registry keys are removed after uninstall

## Performance Tests

### Startup Time
1. [ ] Close Nodoka completely
2. [ ] Launch and measure time until UI fully responsive
3. [ ] Expected: < 2 seconds on modern hardware

### Scanning Performance
1. [ ] Create test library with 100 files
2. [ ] Add directory and measure scan time
3. [ ] Expected: < 30 seconds for 100 files

### Memory Usage
1. [ ] Launch Nodoka
2. [ ] Check memory usage (Activity Monitor / Task Manager / htop)
3. [ ] Expected idle: ~80-120 MB
4. [ ] Start playback
5. [ ] Check memory usage during playback
6. [ ] Expected playing: ~120-150 MB
7. [ ] Play for 5 minutes
8. [ ] Verify no memory leaks (memory stays stable)

## Regression Checklist

Ensure no regressions from previous testing:

- [ ] All 18 unit/integration tests still pass: `cargo test --all`
- [ ] Clippy still passes with zero warnings: `cargo clippy -- -D warnings`
- [ ] No new `unwrap()` or `expect()` calls introduced
- [ ] Database schema unchanged (v0.2.0 baseline)
- [ ] VLC integration still functional
- [ ] No crashes during normal operation
- [ ] Progress tracking accuracy maintained

## Sign-off

After completing all tests on all platforms:

**macOS Testing**
- [ ] All scenarios passed
- [ ] No critical bugs found
- [ ] Installer works correctly
- Tester: _________________ Date: _________

**Linux Testing**
- [ ] All scenarios passed
- [ ] No critical bugs found
- [ ] Installer works correctly
- Tester: _________________ Date: _________

**Windows Testing**
- [ ] All scenarios passed
- [ ] No critical bugs found
- [ ] Installer works correctly
- Tester: _________________ Date: _________

## Critical Bug Criteria

A bug is **critical** if it:
- Causes application crash
- Prevents installation
- Blocks playback functionality
- Causes data loss (progress not saved)
- Makes UI completely non-responsive
- Causes security vulnerability

**If any critical bugs found, STOP release process and fix before publishing.**

## Bug Reporting Template

```
Platform: [macOS / Linux / Windows] [version]
Nodoka Version: 0.2.0
VLC Version: [output of `vlc --version`]

Scenario: [which test scenario]
Steps to Reproduce:
1. 
2. 
3. 

Expected Behavior:
[what should happen]

Actual Behavior:
[what actually happened]

Error Messages:
[any error dialogs or console output]

Screenshots:
[attach if applicable]
```

## Notes

- Perform tests on clean VMs when possible to simulate first-time user experience
- Document any unexpected behavior, even if not critical
- Test with realistic audiobook files (not just test stubs)
- Allow VLC time to initialize during first playback (may take 2-3 seconds)
- Some formats may require VLC plugins (e.g., OPUS on older Windows)
