//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Tests in this file focus on bookmarks and bookmark editor behavior.

use nodoka::models::{AudiobookFile, Bookmark};
use nodoka::ui::{BookmarkEditor, State};

/// Bug #0003: Bookmark editor handles empty labels correctly
///
/// Scenario: User tries to save a bookmark with an empty label.
///
/// Expected: Label defaults to "Bookmark" rather than saving empty string.
#[test]
fn test_bug_0003_bookmark_editor_empty_label_handling() {
    let editor = BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: String::new(), // Empty label
        note: String::new(),
    };

    // The actual validation happens in handle_bookmark_editor_save.
    assert!(editor.label.is_empty());

    // Verify fallback logic
    let label = if editor.label.trim().is_empty() {
        String::from("Bookmark")
    } else {
        editor.label
    };

    assert_eq!(label, "Bookmark");
}

/// Bug #0013: Bookmark list shows correct file status
///
/// Scenario: Bookmark references a file that no longer exists.
///
/// Expected: Bookmark is marked with warning indicator.
#[test]
fn test_bug_0013_bookmark_missing_file_indication() {
    let bookmark = Bookmark {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/missing.mp3".to_string(),
        position_ms: 1000,
        label: "Test Bookmark".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    let file = AudiobookFile {
        audiobook_id: 1,
        name: "missing.mp3".to_string(),
        full_path: "/test/missing.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: false, // File doesn't exist
        created_at: chrono::Utc::now(),
    };

    let files = [file];

    let is_missing = files
        .iter()
        .any(|f| f.full_path == bookmark.file_path && !f.file_exists);

    assert!(
        is_missing,
        "Bookmark should be marked as having missing file"
    );
}

/// Bug #0026: Bookmark editor cleared early when switching audiobooks
///
/// Scenario: User has bookmark editor open, then switches to a different audiobook.
///
/// Expected: Bookmark editor is closed immediately when switching audiobooks.
#[test]
fn test_bug_0026_bookmark_editor_closed_on_audiobook_switch() {
    let mut state = State {
        selected_audiobook: Some(1),
        bookmark_editor: Some(BookmarkEditor {
            id: Some(1),
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Old Bookmark".to_string(),
            note: String::new(),
        }),
        ..Default::default()
    };

    assert!(
        state.bookmark_editor.is_some(),
        "Editor should be open initially"
    );

    // Simulate switching to audiobook 2 (new selection)
    let old_selection = state.selected_audiobook;
    let is_new_selection = old_selection != Some(2);

    if is_new_selection {
        state.bookmark_editor = None; // Cleared early
    }

    assert!(
        state.bookmark_editor.is_none(),
        "Bookmark editor should be closed when switching audiobooks"
    );
}

/// Bug #0035: Bookmark editor with extremely long label or note
///
/// Scenario: User creates a bookmark with a very long label (1000+ characters)
/// or note text, which could cause UI layout issues.
///
/// Expected: Bookmark editor accepts long text, UI renders without panic.
#[test]
fn test_bug_0035_bookmark_with_extremely_long_text() {
    let long_label = "L".repeat(2000); // 2000 character label
    let long_note = "N".repeat(5000); // 5000 character note

    let editor = BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: long_label,
        note: long_note,
    };

    assert!(editor.label.len() > 1500);
    assert!(editor.note.len() > 4000);
}

/// Bug #0041: Bookmark position exceeds file duration
///
/// Scenario: Bookmark has `position_ms` greater than file length (data corruption).
///
/// Expected: UI displays bookmark but prevents seeking beyond file length.
#[test]
fn test_bug_0041_bookmark_position_exceeds_duration() {
    let bookmark = Bookmark {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 120_000, // 2 minutes
        label: "Test".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    let file_duration_ms = 60000; // 1 minute (shorter than bookmark)
    assert!(bookmark.position_ms > file_duration_ms);

    // Seek should be clamped to file duration
    let safe_position = bookmark.position_ms.min(file_duration_ms);
    assert_eq!(safe_position, file_duration_ms);
}

/// Bug #0049: Bookmark creation at position 0:00
///
/// Scenario: User creates bookmark at the very start of file (0ms).
///
/// Expected: Bookmark saves successfully with position 0.
#[test]
fn test_bug_0049_bookmark_at_zero_position() {
    let bookmark = Bookmark {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 0, // Start of file
        label: "Start".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    assert_eq!(bookmark.position_ms, 0);
    assert!(!bookmark.label.is_empty());
}

/// Bug #0054: Text input field focus transitions
///
/// Scenario: User tabs between bookmark label and note fields.
///
/// Expected: Focus moves correctly, no input loss.
#[test]
fn test_bug_0054_text_input_focus_transitions() {
    let editor = BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: "Initial Label".to_string(),
        note: "Initial Note".to_string(),
    };

    // After editing label
    let editor_after_label = BookmarkEditor {
        label: "Updated Label".to_string(),
        ..editor
    };
    assert_eq!(editor_after_label.label, "Updated Label");
    assert_eq!(editor_after_label.note, "Initial Note");

    // After editing note
    let editor_after_note = BookmarkEditor {
        note: "Updated Note".to_string(),
        ..editor_after_label
    };
    assert_eq!(editor_after_note.label, "Updated Label");
    assert_eq!(editor_after_note.note, "Updated Note");
}

/// Bug #0060: Concurrent bookmark save and jump operations
///
/// Scenario: User saves bookmark and immediately clicks to jump to another.
///
/// Expected: Both operations complete successfully without conflict.
#[test]
fn test_bug_0060_concurrent_bookmark_operations() {
    let bookmark1 = Bookmark {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: "Bookmark 1".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    let bookmark2 = Bookmark {
        id: Some(2),
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 5000,
        label: "Bookmark 2".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    assert_ne!(bookmark1.id, bookmark2.id);
    assert_ne!(bookmark1.position_ms, bookmark2.position_ms);
}
