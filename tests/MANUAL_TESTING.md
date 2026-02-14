# Manual Acceptance Testing Guide

This document describes acceptance criteria that require manual verification due to UI interactions, native dialogs, or user experience aspects that cannot be fully automated.

## 1. File Picker Dialog Testing

### Test: User can add directory via file picker dialog
**Steps:**
1. Launch Nodoka application
2. Navigate to Settings → Directories
3. Click "Add Directory" button
4. Verify native file picker dialog appears
5. Select a directory containing audiobooks
6. Verify directory appears in directory list
7. Wait for scan to complete
8. Verify audiobooks from directory appear in library

**Expected Results:**
- Native OS file picker appears (not custom dialog)
- Selected directory path is displayed correctly
- Scanning progress is shown in UI
- All audiobooks discovered and listed

**Platform Notes:**
- Windows: Should use standard Windows file dialog
- macOS: Should use standard macOS file picker
- Linux: Should use GTK/Qt file picker based on desktop environment

### Test: File picker handles special characters
**Steps:**
1. Create test directory with special characters: `My Audio$ Books (Test)`
2. Use file picker to select this directory
3. Verify it appears correctly in directory list

**Expected Results:**
- Path with special characters handled correctly
- No errors or crashes

---

## 2. Keyboard Shortcut Testing

### Test: Space key toggles play/pause
**Steps:**
1. Select an audiobook and start playback
2. Press Space key
3. Verify playback pauses
4. Press Space key again
5. Verify playback resumes

**Expected Results:**
- Space key consistently toggles play/pause state
- No delay or missed key presses
- Works regardless of which UI element has focus

### Test: Keyboard shortcuts for speed adjustment
**Steps:**
1. During playback, press speed up shortcut (e.g., `+` or `]`)
2. Verify speed increases by 0.1x
3. Press speed down shortcut (e.g., `-` or `[`)
4. Verify speed decreases by 0.1x
5. Verify speed is displayed in UI

**Expected Results:**
- Speed changes immediately
- Current speed shown in UI updates
- Audio pitch remains natural at all speeds

### Test: Bookmark creation keyboard shortcut
**Steps:**
1. During playback, press bookmark shortcut (e.g., `Ctrl+B` or `Cmd+B`)
2. Verify bookmark created at current position
3. Bookmark appears in bookmark list

**Expected Results:**
- Bookmark created instantly without dialog
- Timestamp matches current playback position
- Bookmark list updates immediately

**Platform-Specific Shortcuts:**
- Windows/Linux: Use `Ctrl` modifier
- macOS: Use `Cmd` modifier
- Document all shortcuts in application help

---

## 3. UI Responsiveness Testing

### Test: UI remains responsive during scanning
**Steps:**
1. Add directory with 500+ audiobook folders
2. Immediately after clicking "Add Directory", try interacting with UI:
   - Click between audiobooks in list
   - Adjust volume slider
   - Navigate to different tabs
   - Use menu items
3. Verify all UI interactions work during scan

**Expected Results:**
- UI never freezes or becomes unresponsive
- Scanning progress updates in real-time
- Can cancel scan if needed
- Other audiobooks remain playable during scan

### Test: Large library doesn't slow UI
**Steps:**
1. Create library with 1000+ audiobooks
2. Open application
3. Verify startup completes within 3 seconds
4. Test UI interactions:
   - Scrolling through audiobook list
   - Searching for audiobooks
   - Sorting by different criteria
   - Switching between views
5. All interactions should be smooth and responsive

**Expected Results:**
- Startup time < 3 seconds
- Scrolling is smooth (60 FPS)
- Search results appear instantly (< 100ms)
- No lag when sorting
- Memory usage remains reasonable

### Test: Async scanning with progress indicator
**Steps:**
1. Add large directory (1000+ files)
2. Observe scanning progress indicator
3. Verify progress updates smoothly
4. Verify can cancel scan at any time

**Expected Results:**
- Progress bar or spinner visible during scan
- Progress updates at least once per second
- Cancel button functional throughout scan
- Partial results shown as scan progresses

---

## 4. Cross-Platform Verification

### Windows-Specific Tests
**Test Environment:** Windows 10/11

1. **VLC Installation Detection**
   - Test with VLC not installed → Show error with download link
   - Test with VLC 3.x installed → Works correctly
   - Test with VLC 2.x installed → Show version error

2. **File Paths**
   - Test path: `C:\Users\Test User\My Documents\Audiobooks`
   - Test network path: `\\server\audiobooks`
   - Verify paths with spaces work correctly

3. **Database Location**
   - Verify database stored in: `%APPDATA%\Nodoka`
   - Example: `C:\Users\<username>\AppData\Roaming\Nodoka\nodoka.db`

4. **Application Installation**
   - Test .exe installer
   - Verify Start Menu shortcut created
   - Verify uninstaller works correctly

### macOS-Specific Tests
**Test Environment:** macOS 12+

1. **VLC Installation Detection**
   - Test with VLC.app in `/Applications`
   - Test with VLC in `~/Applications`
   - Show clear error if VLC not found

2. **File Paths**
   - Test path: `/Users/test/Audio Books`
   - Verify paths with spaces work
   - Test paths with Unicode characters

3. **Database Location**
   - Verify database stored in: `~/Library/Application Support/Nodoka`
   - Full path: `/Users/<username>/Library/Application Support/Nodoka/nodoka.db`

4. **Application Bundle**
   - Test .app bundle launches correctly
   - Test Gatekeeper handling ("damaged app" warning)
   - Verify code signing (if signed)

### Linux-Specific Tests
**Test Environment:** Ubuntu 22.04+ (or other major distros)

1. **VLC Installation Detection**
   - Test with VLC from package manager
   - Test with Flatpak VLC
   - Verify libvlc detection

2. **File Paths**
   - Test path: `/home/test/Audio Books`
   - Test paths with spaces and Unicode
   - Test mounted drives: `/media/user/AudioDrive`

3. **Database Location**
   - Verify database in: `~/.local/share/nodoka` or `~/.config/nodoka`
   - Follow XDG Base Directory Specification

4. **Installation Methods**
   - Test .deb package installation (Debian/Ubuntu)
   - Test binary execution directly
   - Test AppImage (if provided)

---

## 5. Audio Quality Verification

### Test: Pitch correction maintains natural voice
**Steps:**
1. Play audiobook at 0.5x speed
   - Voice should sound natural, not distorted or deep
2. Play at 1.0x speed
   - Baseline normal playback
3. Play at 1.5x speed
   - Voice should sound natural, not chipmunk-like
4. Play at 2.0x speed
   - Voice should remain clear and natural, not overly high-pitched

**Expected Results:**
- No chipmunk effect at high speeds
- No deep/slow effect at low speeds
- Consistent audio quality across all speeds
- Speech remains intelligible at all speeds

**Technical Note:**
- Pitch correction (time stretching) should be enabled by default
- VLC's audio filter should maintain pitch

### Test: Volume amplification works correctly
**Steps:**
1. Play audiobook with quiet audio
2. Set volume to 100%
   - Should be normal maximum system volume
3. Set volume to 150%
   - Audio should be noticeably louder than 100%
4. Set volume to 200%
   - Audio should be very loud but not distorted

**Expected Results:**
- Volume above 100% provides amplification
- No audio distortion or clipping at 200%
- Volume control is smooth and precise
- Volume changes take effect immediately

### Test: Audio quality across formats
**Steps:**
1. Play MP3 file → Verify clear audio
2. Play M4B file → Verify clear audio
3. Play FLAC file → Verify high-quality audio
4. Play OGG file → Verify clear audio
5. Compare quality between formats

**Expected Results:**
- All formats play with good quality
- No artifacts or distortion
- Format-specific features work (chapters in M4B)

---

## 6. Sleep Timer Verification

### Test: Sleep timer end-of-chapter mode
**Steps:**
1. Start playback in middle of a chapter/file
2. Enable sleep timer with "End of Chapter" mode
3. Verify playback continues until end of current file
4. Verify playback pauses at file boundary, not mid-file
5. Check that next file does not auto-play

**Expected Results:**
- Pauses exactly at file transition
- Does not pause mid-chapter
- Position saved correctly for resume

### Test: Sleep timer fade out
**Steps:**
1. Set sleep timer for 1 minute
2. Listen as timer approaches expiration (last 10-15 seconds)
3. Verify audio gradually fades out over final 5-10 seconds
4. Verify smooth fade, not abrupt stop
5. Verify playback is paused (not stopped) after fade

**Expected Results:**
- Smooth volume reduction
- Fade duration ~5-10 seconds
- No abrupt silence
- Position saved correctly

### Test: Sleep timer cancellation
**Steps:**
1. Set sleep timer for 30 minutes
2. After 5 minutes, cancel the timer
3. Verify timer is cancelled
4. Verify playback continues indefinitely
5. Set new timer after cancellation

**Expected Results:**
- Timer cancels immediately
- No residual timer effects
- Can set new timer after cancellation

---

## 7. Cover Art Display

### Test: Cover art from various sources
**Steps:**
1. **Embedded in M4B:**
   - Play M4B file with embedded cover
   - Verify cover displays in player
   - Verify cover displays in library list

2. **Embedded in MP3:**
   - Play MP3 with ID3 cover art
   - Verify cover extracts and displays

3. **Folder images:**
   - Test `cover.jpg` in folder → Should display
   - Test `folder.jpg` in folder → Should display
   - Test `Cover.JPG` (different case) → Should display

4. **Missing cover:**
   - Play audiobook with no cover art
   - Verify placeholder image shown
   - Placeholder should be clear and professional

**Expected Results:**
- All cover art sources work
- Images cached for performance
- Large images resized appropriately
- Corrupted images fall back to placeholder

---

## 8. Metadata Display

### Test: Metadata extraction and display
**Steps:**
1. Play audiobook with complete metadata
   - Verify title, author, narrator displayed
   - Verify duration calculated correctly
   - Verify year/date shown if available

2. Play audiobook with missing metadata
   - Verify missing fields handled gracefully
   - Verify folder name used as fallback title

3. Test very long metadata strings
   - Verify long titles truncated with ellipsis
   - Verify full text available on hover or details view

**Expected Results:**
- All metadata fields display correctly
- Missing fields don't cause errors
- Long strings truncated appropriately
- Metadata encoding handled correctly (UTF-8)

---

## Testing Checklist

Before release, complete manual testing on each platform:

### Windows Testing
- [ ] File picker dialog works
- [ ] VLC detection and error messages
- [ ] All keyboard shortcuts functional
- [ ] Path handling (spaces, Unicode)
- [ ] Database location correct
- [ ] Network paths work
- [ ] UI responsive with 1000+ books

### macOS Testing
- [ ] File picker dialog works
- [ ] VLC detection and error messages
- [ ] All keyboard shortcuts functional
- [ ] Path handling (spaces, Unicode)
- [ ] Database location correct
- [ ] App bundle launches correctly
- [ ] UI responsive with 1000+ books

### Linux Testing
- [ ] File picker dialog works
- [ ] VLC detection and error messages
- [ ] All keyboard shortcuts functional
- [ ] Path handling (spaces, Unicode)
- [ ] Database location follows XDG spec
- [ ] Binary/package installs correctly
- [ ] UI responsive with 1000+ books

### Audio Quality (All Platforms)
- [ ] Pitch correction at all speeds (0.5x-2.0x)
- [ ] Volume amplification (100%-200%)
- [ ] All audio formats play correctly
- [ ] No distortion or artifacts

### User Experience (All Platforms)
- [ ] Sleep timer end-of-chapter mode works
- [ ] Sleep timer fade out is smooth
- [ ] Cover art displays from all sources
- [ ] Metadata displays correctly
- [ ] Long filenames/metadata handled
- [ ] Special characters in paths work

---

## Reporting Issues

If any manual test fails, report in GitHub issues with:

1. **Test name and section** from this document
2. **Platform and OS version** (e.g., "Windows 11 22H2", "macOS 13.1", "Ubuntu 22.04")
3. **VLC version** (run `vlc --version`)
4. **Steps to reproduce** - exact sequence that causes issue
5. **Expected vs actual behavior** - what should happen vs what happened
6. **Screenshots or screen recording** if applicable
7. **Logs** if available (check application log files)

**Example Good Bug Report:**

```
Title: Sleep timer fade out not working on Linux

Section: 6. Sleep Timer Verification - Test: Sleep timer fade out
Platform: Ubuntu 22.04.1 LTS
VLC Version: 3.0.16

Steps to reproduce:
1. Set sleep timer for 1 minute
2. Wait for timer to approach expiration

Expected: Audio should gradually fade out over 5-10 seconds
Actual: Audio stops abruptly with no fade

Additional info: Tested with MP3 and M4B files, same result
```

---

## Notes for Testers

1. **Test with real audiobooks:** While automated tests use tiny fixture files, manual testing should use full-length audiobook files to catch real-world issues.

2. **Test edge cases:** Actively try to break things - use unusual filenames, very long audiobooks, rapid clicking, etc.

3. **Test on clean systems:** If possible, test on a fresh OS install to catch missing dependencies.

4. **Document workarounds:** If you find issues but also discover workarounds, document both.

5. **Accessibility:** While not explicitly in the test plan, note any accessibility issues (keyboard navigation, screen reader compatibility, etc.).

6. **Performance:** Pay attention to performance issues - UI freezes, slow startup, high memory usage, etc.
