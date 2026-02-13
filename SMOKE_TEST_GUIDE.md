# Nodoka 0.2.0 Smoke Test Guide

This document provides step-by-step instructions for conducting smoke tests across all three target platforms (macOS, Linux, Windows) to verify core functionality before release.

## Prerequisites

### Test Environment Setup

**macOS**: Clean macOS 12+ installation or VM with VLC 3.x installed
**Linux**: Clean Ubuntu 22.04 or Debian 11+ installation or VM with VLC 3.x installed
**Windows**: Clean Windows 10/11 installation or VM with VLC 3.x installed

### Test Data

Prepare a test audiobook library with the following:
- Single-file audiobook (MP3 format, ~10-50 MB)
- Multi-file audiobook (3-5 MP3 chapters, ~5 MB each)
- One file in each supported format: M4A, M4B, OGG, FLAC
- Total test data: ~100-200 MB recommended

## Smoke Test Scenarios

### 1. Installation Verification

**Objective**: Verify installer runs without errors and application appears in expected locations.

#### macOS DMG Test
```bash
# Download Nodoka-0.2.0.dmg
# Verify SHA256 checksum
shasum -a 256 Nodoka-0.2.0.dmg

# Mount DMG (double-click or use command)
hdiutil attach Nodoka-0.2.0.dmg

# Drag Nodoka.app to Applications folder
cp -R /Volumes/Nodoka\ Audiobook\ Reader/Nodoka.app /Applications/

# Unmount DMG
hdiutil detach /Volumes/Nodoka\ Audiobook\ Reader

# Verify installation
ls -la /Applications/Nodoka.app
```

**Expected Results**:
- ✅ DMG mounts without errors
- ✅ Nodoka.app appears in Applications
- ✅ App bundle contains MacOS/nodoka binary
- ✅ Info.plist exists with correct version (0.2.0)

#### Linux DEB Test
```bash
# Download nodoka_0.2.0_amd64.deb
# Verify SHA256 checksum
sha256sum nodoka_0.2.0_amd64.deb

# Install package
sudo dpkg -i nodoka_0.2.0_amd64.deb

# Install dependencies if needed
sudo apt-get install -f

# Verify installation
which nodoka
ls -la /usr/bin/nodoka
ls -la /usr/share/applications/nodoka.desktop
ls -la /usr/share/icons/hicolor/256x256/apps/nodoka.png
```

**Expected Results**:
- ✅ Package installs without errors
- ✅ Binary installed at /usr/bin/nodoka
- ✅ Desktop file created
- ✅ Icon installed in hicolor theme
- ✅ VLC dependencies automatically resolved

#### Windows MSI Test
```powershell
# Download nodoka-0.2.0-x64.msi
# Verify SHA256 checksum
certutil -hashfile nodoka-0.2.0-x64.msi SHA256

# Double-click MSI to start installer wizard
# Follow installation prompts

# Verify installation
dir "C:\Program Files\Nodoka\nodoka.exe"
dir "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Nodoka Audiobook Reader"
```

**Expected Results**:
- ✅ MSI wizard opens without UAC issues
- ✅ Installation completes successfully
- ✅ Binary installed at C:\Program Files\Nodoka\
- ✅ Start Menu shortcut created
- ✅ Windows Defender doesn't flag as malware

### 2. First Launch

**Objective**: Verify application window opens without crashes and database initializes correctly.

#### Test Steps (All Platforms)
```bash
# macOS
open -a Nodoka

# Linux
nodoka &

# Windows
Start-Process "C:\Program Files\Nodoka\nodoka.exe"
```

**Expected Results**:
- ✅ Application window appears within 3 seconds
- ✅ No error dialogs or crash reports
- ✅ Database file created:
  - macOS: `~/.nodoka/nodoka.db`
  - Linux: `~/.nodoka/nodoka.db`
  - Windows: `%APPDATA%\Nodoka\nodoka.db`
- ✅ Lock file created: `instance.lock`
- ✅ UI displays empty library state (no audiobooks yet)
- ✅ Settings button visible and clickable

#### Single Instance Guard Test
```bash
# Launch second instance while first is running
# macOS
open -a Nodoka

# Linux
nodoka &

# Windows
Start-Process "C:\Program Files\Nodoka\nodoka.exe"
```

**Expected Results**:
- ✅ Second instance does not open new window
- ✅ First instance remains active
- ✅ No error messages displayed

### 3. Directory Management

**Objective**: Verify directory scanning and audiobook discovery works correctly.

#### Test Steps
1. Click **Settings** button in top toolbar
2. Settings dialog opens with empty directory list
3. Click **Add Directory** button
4. File picker dialog opens
5. Navigate to test audiobook directory
6. Select directory and click **OK/Select**
7. Directory scanning begins (progress indicator appears)
8. Wait for scan to complete

**Expected Results**:
- ✅ Settings dialog opens without errors
- ✅ File picker allows directory selection
- ✅ Scanning completes within reasonable time (30 sec for 100 MB)
- ✅ Audiobooks appear in main library list
- ✅ Metadata extracted correctly (title, file count, duration)
- ✅ Multi-file audiobooks show correct chapter count
- ✅ Single-file audiobooks show 1 file

#### Verify Database Entries
```sql
# Use sqlite3 to inspect database
sqlite3 ~/.nodoka/nodoka.db

SELECT * FROM directories;
SELECT * FROM audiobooks;
SELECT * FROM audiobook_file;
```

**Expected Results**:
- ✅ Directory path stored in `directories` table
- ✅ Audiobooks inserted into `audiobooks` table
- ✅ Files tracked in `audiobook_file` table
- ✅ Checksums calculated for all files

### 4. Audio Playback

**Objective**: Verify VLC integration and playback controls work correctly.

#### Test Steps
1. Select an audiobook from library list (click on it)
2. Audiobook details appear in right panel
3. File list shows all audio files
4. Click **Play** button (▶)
5. Audio playback starts
6. Adjust volume slider (0-100%)
7. Adjust speed slider (0.5x-2.0x)
8. Move seek slider to 50% position
9. Click **Pause** button (⏸)
10. Playback stops

**Expected Results**:
- ✅ Playback starts within 1 second
- ✅ **Actual audio output** from speakers/headphones
- ✅ Volume slider changes audio level (test with system volume monitor)
- ✅ Speed slider changes playback rate (verify voice pitch)
- ✅ Seek slider jumps to correct position (verify timestamp)
- ✅ Pause button stops playback immediately
- ✅ Play/Pause button toggles correctly
- ✅ Current timestamp updates in real-time

#### Verify VLC Runtime Linking
```bash
# macOS
otool -L /Applications/Nodoka.app/Contents/MacOS/nodoka | grep vlc

# Linux
ldd /usr/bin/nodoka | grep vlc

# Windows
dumpbin /dependents "C:\Program Files\Nodoka\nodoka.exe" | findstr vlc
```

**Expected Results**:
- ✅ VLC libraries found and linked correctly
- ✅ No missing library errors

### 5. Progress Persistence

**Objective**: Verify progress is saved and restored across application restarts.

#### Test Steps
1. Start playing an audiobook
2. Let it play to approximately 50% completion
3. Note the current timestamp (e.g., 5:30 / 10:00)
4. Close application normally (click X or Quit)
5. Wait 5 seconds
6. Reopen application
7. Select the same audiobook
8. Check progress bar and timestamp

**Expected Results**:
- ✅ Progress bar shows 50% completion
- ✅ Timestamp restored to saved position (5:30)
- ✅ Clicking Play resumes from saved position
- ✅ Progress updates persist on each playback stop

#### Verify Database Progress Storage
```sql
sqlite3 ~/.nodoka/nodoka.db
SELECT id, title, position, duration FROM audiobooks WHERE position > 0;
```

**Expected Results**:
- ✅ Position value matches playback timestamp
- ✅ Duration value matches total audiobook length
- ✅ Last played timestamp updated

### 6. Multi-File Audiobooks

**Objective**: Verify chapter navigation and auto-advance work correctly.

#### Test Steps
1. Select a multi-file audiobook (3+ chapters)
2. File list displays all chapters in order
3. Click on second file in list
4. Playback switches to selected file
5. Play to end of current file (or seek to 99%)
6. Wait for file to complete
7. Verify auto-advance to next file

**Expected Results**:
- ✅ All chapter files listed in correct order
- ✅ Clicking file in list changes playback
- ✅ Current file highlighted in list
- ✅ Auto-advance to next file at chapter end
- ✅ Progress tracked across all files
- ✅ Total duration shows sum of all files

### 7. Format Support Test

**Objective**: Verify all supported audio formats play correctly.

#### Test Steps
Test playback with one file of each format:
1. MP3 (.mp3)
2. M4A (.m4a)
3. M4B (.m4b - audiobook format)
4. OGG Vorbis (.ogg)
5. FLAC (.flac)

**Expected Results**:
- ✅ All formats detected during scan
- ✅ All formats play with audible output
- ✅ Metadata extracted correctly for each format
- ✅ Seek and volume controls work for all formats

### 8. Platform-Specific Checks

#### macOS Specific
- ✅ Gatekeeper allows execution (no "damaged app" error)
- ✅ Universal binary works on Intel and Apple Silicon
- ✅ App icon appears in Dock
- ✅ Standard macOS window controls work
- ✅ Cmd+Q quits application

#### Linux Specific
- ✅ Desktop file integration works
- ✅ App appears in application launcher
- ✅ Icon renders in launcher and window
- ✅ PulseAudio/PipeWire audio output works
- ✅ Alt+F4 closes application

#### Windows Specific
- ✅ UAC elevation succeeds during install
- ✅ Windows Defender doesn't flag binary
- ✅ Start Menu shortcut works
- ✅ System tray integration (if applicable)
- ✅ Uninstall via Control Panel works

## Test Results Template

Copy this template for documenting test results:

```markdown
## Smoke Test Results - Nodoka 0.2.0

**Tester**: [Your Name]
**Date**: [Test Date]
**Platform**: [macOS 12/13/14, Ubuntu 22.04/24.04, Windows 10/11]
**VLC Version**: [vlc --version output]

### 1. Installation Verification
- [ ] Installer runs without errors
- [ ] Application installed in correct location
- [ ] Desktop integration works
- [ ] VLC dependencies resolved

**Notes**:

### 2. First Launch
- [ ] Window opens successfully
- [ ] No crash or error dialogs
- [ ] Database initialized
- [ ] Single instance guard works

**Notes**:

### 3. Directory Management
- [ ] Settings dialog opens
- [ ] Directory selection works
- [ ] Scanning completes successfully
- [ ] Audiobooks appear in list

**Notes**:

### 4. Audio Playback
- [ ] Play button starts playback
- [ ] Actual audio output verified
- [ ] Volume control works
- [ ] Speed control works
- [ ] Seek control works
- [ ] Pause button works

**Notes**:

### 5. Progress Persistence
- [ ] Progress saved on quit
- [ ] Progress restored on restart
- [ ] Resume playback correct position

**Notes**:

### 6. Multi-File Audiobooks
- [ ] All chapters listed
- [ ] File selection works
- [ ] Auto-advance to next file

**Notes**:

### 7. Format Support
- [ ] MP3 playback works
- [ ] M4A playback works
- [ ] M4B playback works
- [ ] OGG playback works
- [ ] FLAC playback works

**Notes**:

### 8. Platform-Specific
- [ ] Platform integration works
- [ ] Native controls functional
- [ ] No platform-specific crashes

**Notes**:

### Overall Result
- [ ] All tests passed - APPROVED FOR RELEASE
- [ ] Issues found - See notes

**Critical Issues**:

**Non-Critical Issues**:

**Recommendations**:
```

## Automated Smoke Test Script

For quick verification, use this automated test script:

```bash
#!/bin/bash
# automated-smoke-test.sh

set -e

echo "=== Nodoka 0.2.0 Automated Smoke Test ==="

# 1. Verify installation
echo "1. Checking installation..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    test -d /Applications/Nodoka.app && echo "✅ App installed" || exit 1
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    which nodoka && echo "✅ Binary installed" || exit 1
fi

# 2. Verify VLC
echo "2. Checking VLC..."
vlc --version && echo "✅ VLC found" || exit 1

# 3. Test database initialization
echo "3. Testing database..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    DB="$HOME/.nodoka/nodoka.db"
else
    DB="$HOME/.nodoka/nodoka.db"
fi
test -f "$DB" && echo "✅ Database exists" || echo "⚠️  Database not yet created"

# 4. Verify binary execution (dry run)
echo "4. Testing binary..."
# This would require headless mode - skip for now

echo ""
echo "=== Automated checks complete ==="
echo "Manual testing required for UI and playback verification"
```

## Test Data Generator

To generate test audiobook files:

```bash
#!/bin/bash
# generate-test-audiobooks.sh

mkdir -p test-audiobooks
cd test-audiobooks

# Generate silent audio files for testing
# Requires ffmpeg

# Single-file MP3 audiobook
ffmpeg -f lavfi -i anullsrc=r=44100:cl=stereo -t 300 -q:a 5 \
  -metadata title="Test Audiobook Single File" \
  -metadata artist="Test Author" \
  "Single File/audiobook.mp3"

# Multi-file MP3 audiobook
mkdir -p "Multi File"
for i in {1..5}; do
  ffmpeg -f lavfi -i anullsrc=r=44100:cl=stereo -t 60 -q:a 5 \
    -metadata title="Chapter $i" \
    "Multi File/Chapter $(printf %02d $i).mp3"
done

# M4A format
ffmpeg -f lavfi -i anullsrc=r=44100:cl=stereo -t 180 -c:a aac \
  "Formats/test.m4a"

# OGG format
ffmpeg -f lavfi -i anullsrc=r=44100:cl=stereo -t 180 -c:a libvorbis \
  "Formats/test.ogg"

# FLAC format
ffmpeg -f lavfi -i anullsrc=r=44100:cl=stereo -t 180 -c:a flac \
  "Formats/test.flac"

echo "Test audiobook files generated in ./test-audiobooks/"
```

## Success Criteria

All smoke tests must pass on all three platforms before release:

- ✅ Installation works without errors on clean systems
- ✅ Application launches and UI renders correctly
- ✅ Directory scanning finds and catalogs audiobooks
- ✅ Audio playback produces actual sound output
- ✅ All playback controls functional (play, pause, volume, speed, seek)
- ✅ Progress saves and restores across restarts
- ✅ Multi-file audiobooks navigate correctly
- ✅ All supported formats play successfully
- ✅ Platform-specific integration works (menus, icons, shortcuts)
- ✅ No crashes, freezes, or data corruption
- ✅ VLC runtime dependencies resolved automatically

If any critical test fails, the release should be delayed until the issue is resolved.
