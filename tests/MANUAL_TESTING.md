# Manual Testing Procedures for Nodoka

This document provides comprehensive step-by-step procedures for manual verification of acceptance criteria that cannot be fully automated due to UI constraints, audio quality assessment requirements, or platform-specific functionality.

**Document Version:** 1.0  
**Last Updated:** 2026-02-14  
**Test Coverage:** 8 acceptance criteria requiring manual verification

---

## Prerequisites

Before beginning manual testing, ensure you have:

- ✅ Nodoka installed and compiled from latest source
- ✅ Test audiobook collection with various formats (MP3, M4A, M4B, OGG, FLAC, etc.)
- ✅ Audio playback equipment (headphones or speakers)
- ✅ VLC Media Player installed (version 3.x or later)
- ✅ Write access to test directories
- ✅ Approximately 30-45 minutes for full manual test suite

### Test Environment Setup

1. Create a test directory structure:
   ```
   ~/AudiobookTest/
   ├── Audiobooks/
   │   ├── Test Book 1/
   │   │   ├── chapter1.mp3
   │   │   └── chapter2.mp3
   │   └── Test Book 2/
   │       └── single_file.m4b
   └── Test Results/
   ```

2. Prepare test audio files (use `scripts/generate_test_fixtures.sh` or download sample audiobooks)

3. Launch Nodoka application

---

## Section 1: File Picker Dialog Testing

### Test 1.1: User can add directory via file picker dialog

**Specification Reference:** Section 1, Acceptance Criterion 1.1  
**Priority:** High  
**Estimated Time:** 3 minutes

#### Procedure

1. Launch Nodoka application
2. Navigate to **Settings** → **Library** → **Directories**
3. Click the **"Add Directory"** button
4. Verify native OS file picker dialog appears:
   - **Windows:** Windows Explorer folder selection dialog with appropriate styling
   - **macOS:** Finder folder selection dialog with native appearance
   - **Linux:** GTK/Qt native folder picker matching desktop environment
5. Navigate to the test audiobook directory (`~/AudiobookTest/Audiobooks/`)
6. Select the directory
7. Click **"Select"** or **"Choose"** (button text varies by platform)
8. Verify directory appears in the directories list in Settings
9. Observe scanning progress indicator appears in the UI
10. Wait for scan to complete (progress indicator disappears)
11. Navigate to **Library** view
12. Verify audiobooks from selected directory appear in the library list

#### Expected Result

- Native file picker appears with correct platform styling
- Directory selection succeeds without errors
- Scanning progress is visually indicated
- Audiobooks appear in library after scan completes

#### Pass Criteria

- [ ] Native file picker dialog appears
- [ ] Platform-appropriate styling and behavior
- [ ] Directory successfully added to list
- [ ] Scanning progress visible
- [ ] Audiobooks appear in library

#### Troubleshooting

- **Dialog doesn't appear:** Check application permissions (macOS may require Accessibility or Full Disk Access)
- **Scan hangs:** Check file permissions on audiobook directory
- **No audiobooks found:** Verify directory contains valid audio files

---

### Test 1.2: Added directories persist across application restarts

**Specification Reference:** Section 1, Acceptance Criterion 1.3  
**Priority:** High  
**Estimated Time:** 2 minutes

#### Procedure

1. Complete Test 1.1 to add a directory
2. Note the directory path displayed in Settings → Library → Directories
3. Close Nodoka application completely (⌘Q on macOS, Alt+F4 on Windows, etc.)
4. Verify application is fully closed (check system tray, process list)
5. Relaunch Nodoka
6. Navigate to **Settings** → **Library** → **Directories**
7. Verify previously added directory is still present in the list
8. Navigate to **Library** view
9. Verify audiobooks from that directory are still visible

#### Expected Result

- Directory persists after application restart
- Audiobooks remain in library without re-scanning

#### Pass Criteria

- [ ] Directory appears in list after restart
- [ ] Directory path matches original
- [ ] Audiobooks still visible in library

---

## Section 2: Keyboard Shortcuts Testing

### Test 2.1: Space key toggles play/pause

**Specification Reference:** Section 4, Acceptance Criterion 4.17  
**Priority:** High  
**Estimated Time:** 2 minutes

#### Procedure

1. Launch Nodoka
2. Select an audiobook from the library
3. Click on a file to load it
4. Click the **Play** button to start playback
5. Verify audiobook is playing:
   - Waveform or progress bar animating
   - Timer incrementing
   - Audio output heard
6. Press the **Space** key on keyboard
7. Verify playback pauses:
   - Waveform/progress bar stops moving
   - Timer stops incrementing
   - Audio output stops
8. Press **Space** key again
9. Verify playback resumes:
   - Animation resumes
   - Timer continues from previous position
   - Audio output resumes
10. Repeat Space key toggle 3-4 times rapidly
11. Verify consistent toggle behavior with no lag or missed inputs

#### Expected Result

- Space key reliably toggles between play and pause states
- No lag or delay in response
- No audio glitches during toggle

#### Pass Criteria

- [ ] Space key pauses playback when playing
- [ ] Space key resumes playback when paused
- [ ] Consistent behavior across multiple toggles
- [ ] No lag or audio artifacts
- [ ] Works regardless of which UI element has focus

#### Troubleshooting

- **Space key doesn't work:** Check if another UI element (text field) has keyboard focus
- **Inconsistent behavior:** Verify no conflicting keyboard shortcuts from OS or other apps

---

### Test 2.2: Keyboard shortcut creates bookmark

**Specification Reference:** Section 7, Acceptance Criterion 7.11  
**Priority:** Medium  
**Estimated Time:** 3 minutes

#### Procedure

1. Start playing an audiobook
2. Let it play for approximately 10-15 seconds to establish a clear position
3. Note the current timestamp (e.g., "0:12")
4. Press **Ctrl+B** (Windows/Linux) or **⌘B** (macOS)
5. Verify bookmark creation dialog appears
6. Enter bookmark name: **"Test Bookmark via Keyboard"**
7. Optionally add a note: **"Created via keyboard shortcut"**
8. Click **Save** or press **Enter**
9. Open the **Bookmarks** panel/list
10. Verify bookmark appears in the list with:
    - Correct name: "Test Bookmark via Keyboard"
    - Correct timestamp matching when shortcut was pressed
    - Note text (if entered)
11. Click on the bookmark in the list
12. Verify playback jumps to the bookmarked position
13. Verify timestamp matches the bookmark

#### Expected Result

- Keyboard shortcut immediately opens bookmark creation dialog
- Bookmark is saved with current position
- Bookmark is functional and navigable

#### Pass Criteria

- [ ] Ctrl+B/⌘B opens bookmark dialog
- [ ] Dialog pre-fills current position
- [ ] Bookmark saves successfully
- [ ] Bookmark appears in bookmarks list
- [ ] Clicking bookmark navigates to correct position

---

## Section 3: Audio Quality Testing

### Test 3.1: Pitch correction maintains natural voice at all speeds

**Specification Reference:** Section 4, Acceptance Criterion 4.12  
**Priority:** High  
**Estimated Time:** 5 minutes

#### Procedure

1. Select an audiobook with clear human narration (not music)
2. Start playback
3. **Test at 1.0x speed (baseline):**
   - Listen for 10-15 seconds
   - Note the natural voice quality, clarity, and pitch
   - Record subjective quality: "Natural, clear voice"

4. **Test at 0.5x speed (slow):**
   - Change speed to 0.5x using speed control
   - Listen for 10-15 seconds
   - Verify voice sounds natural (not overly deep or robotic)
   - Voice should be slower but maintain normal pitch
   - Check for: clarity, no digital artifacts, no excessive pitch lowering
   - Record observation

5. **Test at 1.5x speed (fast):**
   - Change speed to 1.5x
   - Listen for 10-15 seconds
   - Verify voice sounds natural (not "chipmunk effect")
   - Voice should be faster but maintain normal pitch
   - Check for: clarity, intelligibility, no high-pitched squeaking
   - Record observation

6. **Test at 2.0x speed (maximum):**
   - Change speed to 2.0x
   - Listen for 10-15 seconds
   - Verify voice remains intelligible and natural-sounding
   - Some quality degradation acceptable at extreme speeds
   - Voice should not have severe chipmunk effect
   - Record observation

7. **Test at 0.75x, 1.25x (intermediate speeds):**
   - Test additional speeds for consistency
   - Verify smooth pitch correction across range

#### Expected Result

- Voice maintains natural pitch at all speeds
- No chipmunk effect at high speeds
- No excessive pitch lowering at slow speeds
- Voice remains intelligible and pleasant to listen to

#### Pass Criteria

- [ ] 1.0x speed: Natural voice quality (baseline)
- [ ] 0.5x speed: Slower but natural pitch, clear
- [ ] 1.5x speed: Faster but natural pitch, no chipmunk effect
- [ ] 2.0x speed: Fast but still intelligible, natural pitch
- [ ] Smooth transitions between speeds
- [ ] No audio artifacts or glitches during speed changes

#### Quality Assessment Guide

**Pass:** Voice maintains natural pitch, remains intelligible, minimal artifacts  
**Marginal:** Slight pitch variation but acceptable, remains usable  
**Fail:** Severe chipmunk effect, robotic sound, or unintelligible at any speed

---

### Test 3.2: Volume amplification works correctly to 200%

**Specification Reference:** Section 4, Acceptance Criterion 4.8  
**Priority:** Medium  
**Estimated Time:** 4 minutes

#### Procedure

1. Find an audiobook with relatively quiet recording (or reduce system volume to 50%)
2. Start playback
3. Set Nodoka volume to **50%** using volume slider
   - Note the audio level (establish baseline)
   - Audio should be clearly audible but not loud

4. Gradually increase Nodoka volume to **75%**
   - Verify audio gets noticeably louder
   - No distortion or clipping

5. Increase volume to **100%**
   - Verify audio continues to get louder
   - This is "normal" maximum volume
   - Note audio level

6. Increase volume to **150%** (amplification begins)
   - Verify audio continues to get louder beyond 100%
   - Check for distortion (some is acceptable with amplification)
   - Audio should be significantly louder than 100%

7. Increase volume to **200%** (maximum amplification)
   - Verify audio reaches maximum volume
   - Check for distortion or clipping
   - Some clipping may occur with loud recordings, but should be minimal with quiet recordings

8. Test with different audio sources:
   - Quiet audiobook (amplification most useful)
   - Loud/well-mastered audiobook (may clip at 200%)

#### Expected Result

- Volume smoothly increases from 0% to 200%
- Amplification above 100% makes quiet audio significantly louder
- Minimal distortion with quiet source material
- Some clipping acceptable at 200% with loud sources

#### Pass Criteria

- [ ] Volume increases smoothly from 0% to 100%
- [ ] Volume continues increasing from 100% to 200%
- [ ] Amplification is effective (quieter audiobooks become louder)
- [ ] Distortion is minimal with quiet sources
- [ ] No crashes or audio system errors at maximum amplification

#### Notes

- Amplification above 100% is intended for quiet recordings
- Some distortion/clipping at 200% is expected and acceptable
- Test with both quiet and loud source material

---

### Test 3.3: Sleep timer fade-out is gradual

**Specification Reference:** Section 12, Acceptance Criterion 12.6  
**Priority:** Low  
**Estimated Time:** 3-5 minutes (includes waiting)

#### Procedure

1. Start playing an audiobook
2. Open **Sleep Timer** settings
3. Set timer for **2 minutes** with **10-second fade duration**
4. Activate timer
5. Continue listening to audiobook
6. Approximately 1 minute 50 seconds into timer, prepare to observe fade
7. **Listen carefully during final 10 seconds:**
   - Volume should begin decreasing gradually
   - Fade should be smooth, not abrupt
   - No audio pops or clicks
   - Volume should reach near-silence by end of timer
8. When timer expires:
   - Verify playback pauses automatically
   - Verify final fade to silence is smooth

9. **Test shorter fade:**
   - Set timer for 1 minute with 5-second fade
   - Repeat observation
   - Verify fade is proportionally faster but still smooth

10. **Test longer fade:**
    - Set timer for 2 minutes with 30-second fade
    - Verify gradual, extended fade

#### Expected Result

- Volume decreases gradually over fade duration
- Fade is smooth without abrupt changes
- No audio artifacts (pops, clicks, glitches)
- Playback pauses automatically when timer expires

#### Pass Criteria

- [ ] Fade begins at appropriate time before timer expiration
- [ ] Volume decreases smoothly (no sudden jumps)
- [ ] Fade duration matches configured setting
- [ ] No audio artifacts during fade
- [ ] Playback pauses automatically at end
- [ ] Fade is pleasant and suitable for falling asleep

#### Quality Assessment

**Pass:** Smooth, gradual fade; suitable for sleep usage  
**Marginal:** Fade is slightly abrupt but functional  
**Fail:** Abrupt cutoff, audio glitches, or doesn't pause automatically

---

## Section 4: Settings and UI Navigation

### Test 4.1: Settings dialog is accessible from main window

**Specification Reference:** Section 15, Acceptance Criterion 15.1  
**Priority:** Medium  
**Estimated Time:** 2 minutes

#### Procedure

1. Launch Nodoka application
2. Locate **Settings** menu item or button in main window:
   - **macOS:** Should be in application menu (Nodoka → Preferences) or toolbar
   - **Windows/Linux:** Should be in File menu, Edit menu, or toolbar
3. Click **Settings** / **Preferences**
4. Verify Settings dialog/window opens
5. Verify Settings contains the following sections:
   - [ ] **Library** (directory management)
   - [ ] **Playback** (default speed, volume)
   - [ ] **Interface** (if applicable: theme, appearance)
   - [ ] **Advanced** (auto-save interval, skip duration)
6. Navigate between settings sections
7. Verify all sections are accessible and render correctly
8. Close Settings dialog
9. Re-open Settings to verify persistent state

#### Expected Result

- Settings accessible via clear menu item or button
- Settings dialog opens without errors
- All settings sections present and functional
- Dialog can be closed and re-opened

#### Pass Criteria

- [ ] Settings accessible from main window
- [ ] Clear navigation (menu item or button)
- [ ] All expected settings sections present
- [ ] Settings dialog renders correctly
- [ ] Can close and re-open dialog

---

## Section 5: Visual Indicators and UI Feedback

### Test 5.1: Playback state is visually indicated

**Specification Reference:** Section 4, Acceptance Criterion 4.13  
**Priority:** Medium  
**Estimated Time:** 2 minutes

#### Procedure

1. Load an audiobook
2. **When stopped:**
   - Verify Play button is enabled/highlighted
   - Verify Pause button is disabled/grayed out
   - Verify progress bar shows position at 0:00
   - Record visual state

3. Click **Play**
4. **When playing:**
   - Verify Play button changes appearance (disabled or changes icon)
   - Verify Pause button is enabled/highlighted
   - Verify progress bar animates (moves forward)
   - Verify timer increments
   - Optional: Verify waveform visualization or other playing indicator

5. Click **Pause**
6. **When paused:**
   - Verify Pause button changes appearance
   - Verify Play button is enabled/highlighted again
   - Verify progress bar stops animating
   - Verify timer stops incrementing
   - Verify position is maintained (not reset to 0)

7. Click **Stop**
8. **When stopped:**
   - Verify UI returns to stopped state
   - Verify position resets to 0:00
   - Verify all controls in appropriate stopped state

#### Expected Result

- Clear visual distinction between playing, paused, and stopped states
- User can immediately determine current state from UI
- Button states reflect available actions

#### Pass Criteria

- [ ] Stopped state clearly identifiable
- [ ] Playing state clearly identifiable (animation, button states)
- [ ] Paused state clearly identifiable
- [ ] Button states prevent invalid actions (e.g., can't pause when stopped)
- [ ] Progress animation reflects playback state

---

### Test 5.2: Cover art is displayed correctly

**Specification Reference:** Section 9, Criteria 9.7-9.8  
**Priority:** Low  
**Estimated Time:** 3 minutes

#### Procedure

1. Prepare test audiobooks:
   - One with embedded cover art (M4B or MP3 with ID3 art)
   - One with folder cover art (cover.jpg in audiobook directory)
   - One with no cover art

2. **Test embedded cover art:**
   - Select audiobook with embedded cover art
   - Verify cover art appears in library list view (thumbnail)
   - Verify cover art appears in now-playing area (larger display)
   - Verify image quality is good (not pixelated)

3. **Test folder cover art:**
   - Select audiobook with folder cover art
   - Verify cover art loads from folder image
   - Verify display in both list and now-playing area

4. **Test missing cover art:**
   - Select audiobook with no cover art
   - Verify placeholder image appears (not broken image icon)
   - Verify placeholder is appropriate (book icon, default image, etc.)

5. **Test image formats:**
   - Verify JPG images display correctly
   - Verify PNG images display correctly
   - Verify other formats if available (GIF, WebP)

#### Expected Result

- Cover art displays correctly when available
- Appropriate placeholder when cover art missing
- Cover art appears in all expected UI locations
- Image quality is good

#### Pass Criteria

- [ ] Embedded cover art displays correctly
- [ ] Folder cover art displays correctly
- [ ] Placeholder shown for missing cover art
- [ ] Cover art in library list view
- [ ] Cover art in now-playing view
- [ ] Multiple image formats supported
- [ ] No broken images or rendering errors

---

## Section 6: Cross-Platform Testing

### Test 6.1: Platform-specific functionality verification

**Specification Reference:** Section 18, Criteria 18.1-18.3, 18.7-18.9  
**Priority:** High (but requires access to multiple platforms)  
**Estimated Time:** 15 minutes per platform

#### Prerequisites

- Access to Windows 10/11, macOS 12+, and Linux (Ubuntu 22.04+)
- Build and install Nodoka on each platform

#### Procedure (repeat on each platform)

1. **Application Launch:**
   - [ ] Application launches successfully
   - [ ] No error dialogs on startup
   - [ ] Main window appears with native styling
   - [ ] Window decorations match platform conventions

2. **File Picker Dialog:**
   - [ ] File picker uses native OS dialog
   - [ ] Dialog has platform-appropriate styling
   - [ ] Directory selection works correctly
   - [ ] Paths with spaces work
   - [ ] Paths with Unicode characters work

3. **Keyboard Shortcuts:**
   - [ ] Windows/Linux: Ctrl+B for bookmarks
   - [ ] macOS: ⌘B for bookmarks
   - [ ] Space for play/pause works
   - [ ] Platform-appropriate modifier keys

4. **Audio Playback:**
   - [ ] VLC integration works
   - [ ] Audio plays correctly through system audio
   - [ ] Volume control works
   - [ ] Speed control works

5. **Path Handling:**
   - [ ] Application handles platform-specific path separators
   - [ ] Paths with spaces work correctly
   - [ ] Unicode paths work correctly
   - [ ] (Windows only) UNC paths work (e.g., `\\server\share\audiobooks`)

6. **Database Location:**
   - [ ] Database stored in platform-appropriate location
   - [ ] Windows: `%APPDATA%\Nodoka\`
   - [ ] macOS: `~/Library/Application Support/Nodoka/`
   - [ ] Linux: `~/.config/nodoka/` or `~/.local/share/nodoka/`

7. **Application Behavior:**
   - [ ] Settings persist across restarts
   - [ ] Progress tracking works
   - [ ] All major features functional

#### Expected Result

Application works consistently across all platforms with platform-appropriate adaptations for UI, keyboard shortcuts, and file system conventions.

#### Pass Criteria Per Platform

- [ ] **Windows 10/11:** All tests pass
- [ ] **macOS 12+:** All tests pass
- [ ] **Linux (Ubuntu 22.04+):** All tests pass
- [ ] Platform-specific features adapted correctly
- [ ] No cross-platform compatibility issues

#### Platform-Specific Notes

**Windows:**
- Test with Windows-style paths (`C:\Users\...`)
- Test UNC paths (`\\server\share`) if network available
- Verify Ctrl key usage for shortcuts

**macOS:**
- Test with macOS-style paths (`/Users/...`)
- Verify ⌘ key usage for shortcuts
- Check app menu structure (Nodoka → Preferences)

**Linux:**
- Test on multiple desktop environments if possible (GNOME, KDE, XFCE)
- Verify file picker matches desktop environment
- Check compatibility with different audio backends (PulseAudio, PipeWire)

---

## Test Completion Checklist

Use this checklist to track progress through manual testing:

### Section 1: File Picker Dialogs
- [ ] Test 1.1: Add directory via file picker
- [ ] Test 1.2: Directory persistence across restarts

### Section 2: Keyboard Shortcuts
- [ ] Test 2.1: Space key play/pause toggle
- [ ] Test 2.2: Bookmark creation shortcut

### Section 3: Audio Quality
- [ ] Test 3.1: Pitch correction at all speeds
- [ ] Test 3.2: Volume amplification to 200%
- [ ] Test 3.3: Sleep timer fade-out

### Section 4: Settings and Navigation
- [ ] Test 4.1: Settings dialog accessibility

### Section 5: Visual Indicators
- [ ] Test 5.1: Playback state indication
- [ ] Test 5.2: Cover art display

### Section 6: Cross-Platform (if applicable)
- [ ] Test 6.1: Windows functionality
- [ ] Test 6.1: macOS functionality
- [ ] Test 6.1: Linux functionality

---

## Reporting Issues

If any manual test fails, please document:

1. **Test ID and Name:** (e.g., Test 2.1: Space key play/pause)
2. **Platform:** (Windows/macOS/Linux version)
3. **Steps to Reproduce:** Exact steps that led to failure
4. **Expected Result:** What should have happened
5. **Actual Result:** What actually happened
6. **Screenshots/Video:** If applicable
7. **System Information:**
   - OS version
   - VLC version (`vlc --version`)
   - Nodoka version
   - Audio output device

Create a GitHub issue with this information for tracking and resolution.

---

## Automation Potential

Some of these tests may become automatable in the future:

- **Keyboard shortcuts:** With UI testing frameworks (e.g., Playwright, Tauri testing)
- **Visual indicators:** With screenshot-based testing
- **Audio quality:** With signal processing analysis (FFT, pitch detection)

However, subjective assessments (audio quality, UI aesthetics) will always benefit from human verification.

---

## Appendix: Audio Test Files

For consistent audio quality testing, use these recommended test files:

1. **LibriVox public domain audiobooks:** Clear narration, various voices
   - Download from: https://librivox.org/
   - Recommended: Any chapter from "Pride and Prejudice" or "Sherlock Holmes"

2. **Generate test tones (for technical testing):**
   ```bash
   # Generate 1-minute test file with speech-like characteristics
   ffmpeg -f lavfi -i "sine=frequency=200:duration=60" \
          -f lavfi -i "sine=frequency=400:duration=60" \
          -filter_complex "[0:a][1:a]amix=inputs=2:duration=first" \
          test_audio_speech_sim.mp3
   ```

3. **Quiet audiobook (for volume amplification testing):**
   - Find or create an audiobook with -20dB or lower average volume
   - Or reduce volume of existing file:
     ```bash
     ffmpeg -i input.mp3 -filter:a "volume=0.2" quiet_audiobook.mp3
     ```

---

## Notes

- Manual testing should be performed before each major release
- Document results and attach to release notes
- Cross-platform testing can be performed via CI/CD in addition to manual verification
- Some tests (especially audio quality) are subjective; use best judgment
- Update this document as new manual-only features are added

---

**Document End**
