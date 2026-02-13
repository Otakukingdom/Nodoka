# Smoke Test Checklist for Nodoka v0.2.0

This document provides a comprehensive checklist for manually testing Nodoka installers on all three target platforms. These tests must be performed on clean systems after installers are built.

## Prerequisites

- Clean virtual machines or test systems for each platform:
  - **macOS**: macOS 12 Monterey or later
  - **Linux**: Ubuntu 22.04 LTS or Debian 11+
  - **Windows**: Windows 10 or Windows 11
- VLC 3.x installed on each test system
- Sample audiobook files in multiple formats: MP3, M4A, M4B, OGG, FLAC
- Sample multi-file audiobook (e.g., multiple MP3 chapters in a directory)

## Test Execution

Perform ALL 6 test scenarios on ALL 3 platforms (18 total test combinations).

---

## Platform: macOS

**Installer**: `Nodoka-0.2.0.dmg`

### Scenario 1: Installation Verification

- [ ] Download DMG from GitHub release
- [ ] Verify SHA256 checksum matches `SHA256SUMS.txt`
- [ ] Double-click DMG to mount
- [ ] Drag Nodoka.app to Applications folder
- [ ] Eject DMG
- [ ] Verify Nodoka.app appears in Applications folder
- [ ] macOS Gatekeeper allows execution (may require right-click > Open first time)
- [ ] No security warnings after first launch approval

**Expected**: Clean installation with no errors

### Scenario 2: First Launch

- [ ] Launch Nodoka from Applications
- [ ] Application window opens successfully (no crash)
- [ ] No error dialogs displayed
- [ ] Database initializes at `~/Library/Application Support/Nodoka/nodoka.db`
- [ ] UI shows empty library with "Add Directory" prompt
- [ ] Attempt to launch second instance - verify single instance guard prevents duplicate

**Expected**: Clean first launch, database created, single instance enforced

### Scenario 3: Directory Management

- [ ] Click "Settings" or "Add Directory" button
- [ ] File picker dialog opens
- [ ] Select directory containing audiobooks
- [ ] Directory scanning completes without errors
- [ ] Progress indicator shows scanning activity
- [ ] Audiobooks appear in library list after scan
- [ ] Verify correct title/author detection
- [ ] Add second directory - verify both directories' books appear

**Expected**: Directory scanning works, books populate library

### Scenario 4: Audio Playback

- [ ] Select audiobook from library list
- [ ] Click Play button
- [ ] **VERIFY ACTUAL AUDIO OUTPUT** through speakers/headphones
- [ ] Click Pause button - audio stops
- [ ] Click Play again - audio resumes
- [ ] Adjust volume slider (0-100%) - verify volume changes
- [ ] Adjust speed slider (0.5x-2.0x) - verify playback speed changes
- [ ] Drag seek slider - verify audio jumps to new position
- [ ] Position display updates during playback (mm:ss / total)

**Expected**: Full audio playback control with actual sound output

### Scenario 5: Progress Persistence

- [ ] Play audiobook to approximately 50% completion
- [ ] Note exact position (e.g., "12:34 / 25:00")
- [ ] Close application using Cmd+Q or window close button
- [ ] Verify database still exists at `~/Library/Application Support/Nodoka/nodoka.db`
- [ ] Reopen Nodoka
- [ ] Select same audiobook
- [ ] Verify position restored to ~50% (within a few seconds)
- [ ] Click Play - verify audio resumes from saved position

**Expected**: Progress saves and restores accurately across app restarts

### Scenario 6: Multi-File Audiobooks

- [ ] Add directory with multi-file audiobook (e.g., Chapter01.mp3, Chapter02.mp3, etc.)
- [ ] Select audiobook from library
- [ ] Verify file list shows all chapters in correct order
- [ ] Play first chapter
- [ ] Click second chapter in list - verify playback switches
- [ ] Play first chapter to end - verify auto-advance to second chapter
- [ ] Verify progress tracks correctly across files

**Expected**: Multi-file audiobooks handled correctly with chapter navigation

### Scenario 7: Audio Format Support

Test playback with one file of each format:

- [ ] MP3 file plays correctly
- [ ] M4A file plays correctly
- [ ] M4B file plays correctly
- [ ] OGG file plays correctly
- [ ] FLAC file plays correctly

**Expected**: All five formats play with audio output

### macOS-Specific Checks

- [ ] Test on Intel Mac (x86_64) if available
- [ ] Test on Apple Silicon Mac (ARM64) if available
- [ ] Verify app bundle is code-signed (optional)
- [ ] Check memory usage in Activity Monitor (~50-100 MB expected)
- [ ] Verify VLC libraries loaded: `otool -L /Applications/Nodoka.app/Contents/MacOS/nodoka | grep vlc`

---

## Platform: Linux (Ubuntu/Debian)

**Installer**: `nodoka_0.2.0_amd64.deb`

### Scenario 1: Installation Verification

- [ ] Download DEB from GitHub release
- [ ] Verify SHA256 checksum: `sha256sum nodoka_0.2.0_amd64.deb`
- [ ] Install package: `sudo dpkg -i nodoka_0.2.0_amd64.deb`
- [ ] If dependency errors, run: `sudo apt-get install -f`
- [ ] Verify binary installed: `which nodoka` (should show `/usr/bin/nodoka`)
- [ ] Verify desktop file: `ls /usr/share/applications/nodoka.desktop`
- [ ] Verify icon: `ls /usr/share/icons/hicolor/256x256/apps/nodoka.png`
- [ ] Verify VLC dependency satisfied: `ldd /usr/bin/nodoka | grep vlc`

**Expected**: Clean installation with all files in correct locations

### Scenario 2: First Launch

- [ ] Launch from terminal: `nodoka`
- [ ] OR launch from application menu
- [ ] Application window opens successfully (no crash)
- [ ] No error dialogs displayed
- [ ] Database initializes at `~/.local/share/nodoka/nodoka.db` or `~/.nodoka/nodoka.db`
- [ ] UI shows empty library with "Add Directory" prompt
- [ ] Attempt to launch second instance - verify single instance guard prevents duplicate

**Expected**: Clean first launch, database created, single instance enforced

### Scenario 3: Directory Management

- [ ] Click "Settings" or "Add Directory" button
- [ ] File picker dialog opens (GTK or native)
- [ ] Select directory containing audiobooks
- [ ] Directory scanning completes without errors
- [ ] Progress indicator shows scanning activity
- [ ] Audiobooks appear in library list after scan
- [ ] Verify correct title/author detection
- [ ] Add second directory - verify both directories' books appear

**Expected**: Directory scanning works, books populate library

### Scenario 4: Audio Playback

- [ ] Select audiobook from library list
- [ ] Click Play button
- [ ] **VERIFY ACTUAL AUDIO OUTPUT** through PulseAudio/PipeWire
- [ ] Click Pause button - audio stops
- [ ] Click Play again - audio resumes
- [ ] Adjust volume slider (0-100%) - verify volume changes
- [ ] Adjust speed slider (0.5x-2.0x) - verify playback speed changes
- [ ] Drag seek slider - verify audio jumps to new position
- [ ] Position display updates during playback (mm:ss / total)

**Expected**: Full audio playback control with actual sound output

### Scenario 5: Progress Persistence

- [ ] Play audiobook to approximately 50% completion
- [ ] Note exact position (e.g., "12:34 / 25:00")
- [ ] Close application using window close button or Ctrl+Q
- [ ] Verify database still exists
- [ ] Reopen Nodoka
- [ ] Select same audiobook
- [ ] Verify position restored to ~50% (within a few seconds)
- [ ] Click Play - verify audio resumes from saved position

**Expected**: Progress saves and restores accurately across app restarts

### Scenario 6: Multi-File Audiobooks

- [ ] Add directory with multi-file audiobook
- [ ] Select audiobook from library
- [ ] Verify file list shows all chapters in correct order
- [ ] Play first chapter
- [ ] Click second chapter in list - verify playback switches
- [ ] Play first chapter to end - verify auto-advance to second chapter
- [ ] Verify progress tracks correctly across files

**Expected**: Multi-file audiobooks handled correctly with chapter navigation

### Scenario 7: Audio Format Support

Test playback with one file of each format:

- [ ] MP3 file plays correctly
- [ ] M4A file plays correctly
- [ ] M4B file plays correctly
- [ ] OGG file plays correctly
- [ ] FLAC file plays correctly

**Expected**: All five formats play with audio output

### Linux-Specific Checks

- [ ] Verify desktop file integration: Application appears in launcher
- [ ] Verify icon appears in application menu
- [ ] Test audio output with PulseAudio: `pactl list sinks` shows active playback
- [ ] Test audio output with PipeWire (if available)
- [ ] Check memory usage: `ps aux | grep nodoka` (~50-100 MB expected)
- [ ] Test uninstall: `sudo dpkg -r nodoka` removes all files cleanly

---

## Platform: Windows

**Installer**: `nodoka-0.2.0-x64.msi`

### Scenario 1: Installation Verification

- [ ] Download MSI from GitHub release
- [ ] Verify SHA256 checksum: `certutil -hashfile nodoka-0.2.0-x64.msi SHA256`
- [ ] Double-click MSI installer
- [ ] UAC prompt appears - approve elevation
- [ ] Installation wizard completes without errors
- [ ] Verify binary installed: `C:\Program Files\Nodoka\nodoka.exe` exists
- [ ] Verify Start Menu shortcut created
- [ ] Windows Defender does not flag binary as malware
- [ ] Verify VLC dependency: `where vlc` or check VLC installation

**Expected**: Clean installation with no security warnings

### Scenario 2: First Launch

- [ ] Launch from Start Menu shortcut
- [ ] OR launch from `C:\Program Files\Nodoka\nodoka.exe`
- [ ] Application window opens successfully (no crash)
- [ ] No error dialogs displayed
- [ ] Database initializes at `%APPDATA%\Nodoka\nodoka.db`
- [ ] UI shows empty library with "Add Directory" prompt
- [ ] Attempt to launch second instance - verify single instance guard prevents duplicate

**Expected**: Clean first launch, database created, single instance enforced

### Scenario 3: Directory Management

- [ ] Click "Settings" or "Add Directory" button
- [ ] Windows file picker dialog opens
- [ ] Select directory containing audiobooks
- [ ] Directory scanning completes without errors
- [ ] Progress indicator shows scanning activity
- [ ] Audiobooks appear in library list after scan
- [ ] Verify correct title/author detection
- [ ] Add second directory - verify both directories' books appear

**Expected**: Directory scanning works, books populate library

### Scenario 4: Audio Playback

- [ ] Select audiobook from library list
- [ ] Click Play button
- [ ] **VERIFY ACTUAL AUDIO OUTPUT** through Windows audio system
- [ ] Click Pause button - audio stops
- [ ] Click Play again - audio resumes
- [ ] Adjust volume slider (0-100%) - verify volume changes
- [ ] Adjust speed slider (0.5x-2.0x) - verify playback speed changes
- [ ] Drag seek slider - verify audio jumps to new position
- [ ] Position display updates during playback (mm:ss / total)

**Expected**: Full audio playback control with actual sound output

### Scenario 5: Progress Persistence

- [ ] Play audiobook to approximately 50% completion
- [ ] Note exact position (e.g., "12:34 / 25:00")
- [ ] Close application using window close button or Alt+F4
- [ ] Verify database still exists at `%APPDATA%\Nodoka\nodoka.db`
- [ ] Reopen Nodoka
- [ ] Select same audiobook
- [ ] Verify position restored to ~50% (within a few seconds)
- [ ] Click Play - verify audio resumes from saved position

**Expected**: Progress saves and restores accurately across app restarts

### Scenario 6: Multi-File Audiobooks

- [ ] Add directory with multi-file audiobook
- [ ] Select audiobook from library
- [ ] Verify file list shows all chapters in correct order
- [ ] Play first chapter
- [ ] Click second chapter in list - verify playback switches
- [ ] Play first chapter to end - verify auto-advance to second chapter
- [ ] Verify progress tracks correctly across files

**Expected**: Multi-file audiobooks handled correctly with chapter navigation

### Scenario 7: Audio Format Support

Test playback with one file of each format:

- [ ] MP3 file plays correctly
- [ ] M4A file plays correctly
- [ ] M4B file plays correctly
- [ ] OGG file plays correctly
- [ ] FLAC file plays correctly

**Expected**: All five formats play with audio output

### Windows-Specific Checks

- [ ] Verify Start Menu shortcut works
- [ ] Verify Desktop shortcut (if created) works
- [ ] Test uninstall via Control Panel > Programs and Features
- [ ] Verify uninstall removes all program files
- [ ] Verify uninstall preserves user data in %APPDATA%\Nodoka
- [ ] Check Task Manager for resource usage (~50-100 MB expected)
- [ ] Verify no firewall prompts (application doesn't use network)

---

## Test Results Summary

After completing all tests, fill out this summary:

### macOS Results
- [ ] All 7 scenarios passed
- Issues found: _______________________________________________

### Linux Results
- [ ] All 7 scenarios passed
- Issues found: _______________________________________________

### Windows Results
- [ ] All 7 scenarios passed
- Issues found: _______________________________________________

### Overall Assessment
- [ ] **PASS**: All tests passed on all platforms - ready for release
- [ ] **CONDITIONAL PASS**: Minor issues found but release can proceed
- [ ] **FAIL**: Critical issues found - release blocked

### Critical Issues (must fix before release)
1. _______________________________________________
2. _______________________________________________

### Non-Critical Issues (can be addressed in future release)
1. _______________________________________________
2. _______________________________________________

---

## Notes

- **Actual audio output verification**: Do not rely on UI state alone. You must HEAR audio through speakers/headphones to confirm playback works.
- **Clean test systems**: Tests should be performed on systems without development tools or dependencies pre-installed.
- **VLC requirement**: VLC 3.x must be installed before running Nodoka. Document any VLC-related errors clearly.
- **Multi-platform testing**: All three platforms must pass before release. Do not skip any platform.
- **Document everything**: If a test fails, document exact error messages, steps to reproduce, and system details.

## Tester Signature

- **Tester Name**: _______________________________
- **Date Tested**: _______________________________
- **macOS Version**: _______________________________
- **Linux Version**: _______________________________
- **Windows Version**: _______________________________
- **Overall Result**: [ ] PASS  [ ] CONDITIONAL PASS  [ ] FAIL
