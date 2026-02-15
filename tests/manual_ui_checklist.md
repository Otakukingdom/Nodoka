# Manual UI/UX Testing Checklist

This document provides a manual testing checklist for Nodoka Audiobook Player UI/UX improvements. Since automated UI testing with iced framework has limitations (opaque Element types, no DOM inspection), manual verification is required to ensure visual design, accessibility, and interaction patterns meet requirements.

**Purpose**: Verify UI improvements from the comprehensive UI/UX overhaul including visual feedback, keyboard navigation, modal backdrops, button hierarchy, error messaging, and accessibility features.

**Testing Requirements**: 
- Test on all three platforms: Windows, macOS, Linux
- Test with keyboard-only navigation (no mouse)
- Test with screen reader (VoiceOver on macOS, NVDA on Windows)
- Test with various audiobook library sizes (empty, 1 book, 10+ books)

## Selection States and Visual Feedback

### Test Case 1: Audiobook List Selection
**Steps**:
1. Launch application with multiple audiobooks
2. Click on an audiobook in the list
3. Observe visual feedback

**Expected Result**: 
- Selected audiobook has visible background color (vibrant rose #E11D48)
- Selected item clearly distinguishable from unselected items
- Selection persists when clicking elsewhere (e.g., files, bookmarks)

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 2: File List Selection
**Steps**:
1. Select an audiobook with multiple files
2. Click on a file in the file list
3. Observe visual feedback

**Expected Result**:
- Selected file has visible background color with border
- Currently playing file is clearly indicated
- Selection styling distinct from audiobook selection

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Button Hierarchy and Styling

### Test Case 3: Primary vs Secondary Button Distinction
**Steps**:
1. Open Settings dialog
2. Observe "Add Directory" and "Close" buttons
3. Compare visual appearance

**Expected Result**:
- "Add Directory" (primary) has vibrant rose background (#E11D48)
- "Close" (secondary) has elevated background with border
- Primary action visually more prominent than secondary
- All buttons have consistent border radius (8px)

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 4: Danger Button Styling
**Steps**:
1. Open Settings dialog
2. Find a directory entry
3. Observe "Remove" and "Delete" buttons

**Expected Result**:
- Danger buttons (Remove, Delete) use error color (#E11D48)
- Clearly distinguishable from primary and secondary buttons
- Visual warning of destructive action

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Modal Backdrops and Focus Trap

### Test Case 5: Settings Modal Backdrop
**Steps**:
1. Open Settings dialog
2. Observe backdrop behind modal
3. Click on backdrop (not modal content)

**Expected Result**:
- Semi-transparent dark backdrop (40% opacity black) covers main content
- Main content not interactive while modal open
- Clicking backdrop closes the modal
- Modal centered on screen

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 6: Bookmark Editor Modal Backdrop
**Steps**:
1. Create a bookmark (Ctrl+B / Cmd+B)
2. Observe backdrop behind bookmark editor
3. Click on backdrop

**Expected Result**:
- Semi-transparent backdrop covers main content
- Clicking backdrop closes bookmark editor
- Modal properly centered with elevation effect (border)

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Loading States and User Feedback

### Test Case 7: Directory Scanning Indicator
**Steps**:
1. Add a directory or rescan existing directory
2. Observe loading indicator during scan

**Expected Result**:
- "Scanning: [directory path]" message appears
- Message has info background color (blue #2563EB)
- Message disappears when scan completes
- User knows operation is in progress

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Error Messaging

### Test Case 8: Error Banner Display
**Steps**:
1. Trigger an error (e.g., scan directory without permissions)
2. Observe error banner at top of screen

**Expected Result**:
- Error banner appears at top with warning icon (⚠)
- Banner has error background color (rose #E11D48)
- Error message is clear and actionable
- "Dismiss" button present

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 9: Error Dismissal
**Steps**:
1. Display an error banner
2. Click "Dismiss" button

**Expected Result**:
- Error banner disappears
- Application returns to normal state
- No lingering visual artifacts

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Keyboard Navigation - Playback Controls

### Test Case 10: Space Bar Play/Pause
**Steps**:
1. Select an audiobook file
2. Press Space bar
3. Press Space bar again

**Expected Result**:
- First press: Playback starts
- Second press: Playback pauses
- Works without mouse interaction

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 11: Arrow Key Seeking
**Steps**:
1. Start playing an audiobook
2. Press Left Arrow key (←)
3. Press Right Arrow key (→)

**Expected Result**:
- Left Arrow: Seeks backward 5 seconds
- Right Arrow: Seeks forward 5 seconds
- Time position updates visually
- Smooth seeking without stuttering

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Keyboard Navigation - File Navigation

### Test Case 12: Arrow Keys File Navigation
**Steps**:
1. Select audiobook with multiple files
2. Select a file in the middle of the list
3. Press Down Arrow key (↓)
4. Press Up Arrow key (↑)

**Expected Result**:
- Down Arrow: Next file selected and starts playing
- Up Arrow: Previous file selected and starts playing
- File list selection updates visually
- Wrapping behavior at start/end (stays on current file)

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Keyboard Navigation - Modal Control

### Test Case 13: Escape Key Closes Modals
**Steps**:
1. Open Settings dialog
2. Press Escape key
3. Open Bookmark editor (Ctrl+B)
4. Press Escape key

**Expected Result**:
- First Escape: Settings dialog closes
- Second Escape: Bookmark editor closes
- Application returns to main view
- Keyboard focus returns appropriately

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 14: Bookmark Creation Shortcut
**Steps**:
1. Start playing an audiobook
2. Press Ctrl+B (Windows/Linux) or Cmd+B (macOS)

**Expected Result**:
- Bookmark editor opens
- Position field shows current playback time
- Label field has default "Bookmark" text
- Can immediately type to edit label

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Accessibility - Focus Indicators

### Test Case 15: Tab Navigation Visibility
**Steps**:
1. Launch application
2. Press Tab key repeatedly
3. Observe focus indicator on each element

**Expected Result**:
- Focus moves through all interactive elements
- Focus indicator visible on current element
- Focus order is logical (top-to-bottom, left-to-right)
- No keyboard traps (can Tab out of all components)

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Accessibility - Color Contrast

### Test Case 16: Text Readability
**Steps**:
1. Observe all text elements in the application
2. Check primary text on primary background
3. Check button text on colored backgrounds

**Expected Result**:
- Primary text (#730F2E) on light background (#FFF1F2) has ≥ 4.5:1 contrast
- White text on primary/error buttons (#E11D48) is clearly readable
- Secondary text (#666666) on light background is readable
- No color-only differentiation (icons/labels supplement color)

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Accessibility - Screen Reader Support

### Test Case 17: Screen Reader Announces Elements
**Steps**:
1. Enable screen reader (VoiceOver/NVDA)
2. Navigate through the application with keyboard
3. Listen to announcements

**Expected Result**:
- Buttons announce their labels (not just icons)
- Current state announced (playing/paused, selected item)
- Error messages announced when displayed
- Modal dialogs announce title and purpose

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Sleep Timer Functionality

### Test Case 18: Sleep Timer Fade Behavior
**Steps**:
1. Start playing an audiobook
2. Set sleep timer to 1 minute (custom input)
3. Wait for last 30 seconds
4. Observe volume changes

**Expected Result**:
- Volume remains constant for first 30 seconds
- Volume fades smoothly over last 30 seconds
- Playback pauses automatically when timer expires
- No audio glitches or pops during fade

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Edge Cases and Stress Testing

### Test Case 19: Large Library Performance
**Steps**:
1. Add directory with 50+ audiobooks
2. Scroll through audiobook list
3. Select audiobooks and files
4. Navigate with keyboard

**Expected Result**:
- Smooth scrolling with no lag
- Selection updates immediately
- Keyboard navigation remains responsive
- No visual glitches or rendering issues

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

### Test Case 20: Rapid Keyboard Input Handling
**Steps**:
1. Select an audiobook file
2. Rapidly press Space bar 10 times (play/pause toggle)
3. Rapidly press Left/Right arrow keys alternately
4. Rapidly press Up/Down arrow keys alternately

**Expected Result**:
- No crashes or hangs
- State remains consistent (no race conditions)
- Application responds to each input appropriately
- No visual glitches or UI freezes
- Final state is predictable

**Pass**: ☐ Windows ☐ macOS ☐ Linux

---

## Summary

**Total Test Cases**: 20  
**Platforms Tested**: ☐ Windows ☐ macOS ☐ Linux

**Overall Assessment**:
- ☐ All critical tests pass (Cases 1-14)
- ☐ All accessibility tests pass (Cases 15-17)
- ☐ All sleep timer tests pass (Case 18)
- ☐ All edge case tests pass (Cases 19-20)

**Issues Found**:
(List any issues discovered during testing)

---

**AUTOMATED VERIFICATION COMPLETED (Feb 14, 2026)**:
- ✅ All 809 automated tests pass (69 regression tests covering edge cases)
- ✅ Zero clippy warnings with strict lints
- ✅ Release build succeeds (13MB binary)
- ✅ Rust 1.93.1 and iced 0.14.0 versions verified
- ⚠️ Manual UI testing requires human interaction with GUI (cannot be automated in pipeline)

**Manual Testing Status**: PENDING - Requires human tester to launch application and execute 20 test cases

---

**Tester Name**: ___________________  
**Date**: ___________________  
**Platform Details**: ___________________  
**iced Version**: 0.14.0  
**Nodoka Version**: 0.2.0

---

## Notes

This checklist represents an **exceptional case** for external documentation as defined in AGENTS.md:
- Purpose: Manual testing procedures that cannot be automated
- Justification: iced framework limitations prevent full automated UI testing
- Scope: Testing only - does not duplicate rustdoc content
- Maintenance: Update when new UI features are added

For automated test coverage, see:
- `src/ui/shortcuts.rs` - Keyboard shortcut tests
- `src/ui/update/tests.rs` - Message handler tests
- `src/ui/components/*/tests` - Component rendering tests
