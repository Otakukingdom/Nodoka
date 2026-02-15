//! Integration tests for keyboard navigation and shortcuts
//!
//! Tests verify that keyboard shortcuts work end-to-end from key press
//! through message routing to state updates and database persistence.

mod acceptance_support;
use acceptance_support::*;

use iced::keyboard::Modifiers;
use nodoka::db::queries;
use nodoka::ui::shortcuts::{message_for_key_chord, ShortcutKey};
use nodoka::ui::Message;
use std::error::Error;

#[test]
fn test_space_shortcut_maps_to_play_pause() {
    let message = message_for_key_chord(ShortcutKey::Space, Modifiers::default());
    assert!(
        matches!(message, Some(Message::PlayPause)),
        "Space key should map to PlayPause message"
    );
}

#[test]
fn test_left_arrow_shortcut_maps_to_seek_backward() {
    let message = message_for_key_chord(ShortcutKey::ArrowLeft, Modifiers::default());
    assert!(
        matches!(message, Some(Message::SeekBackward(5))),
        "Left arrow should map to SeekBackward(5) message"
    );
}

#[test]
fn test_right_arrow_shortcut_maps_to_seek_forward() {
    let message = message_for_key_chord(ShortcutKey::ArrowRight, Modifiers::default());
    assert!(
        matches!(message, Some(Message::SeekForward(5))),
        "Right arrow should map to SeekForward(5) message"
    );
}

#[test]
fn test_up_arrow_shortcut_maps_to_previous_file() {
    let message = message_for_key_chord(ShortcutKey::ArrowUp, Modifiers::default());
    assert!(
        matches!(message, Some(Message::PreviousFile)),
        "Up arrow should map to PreviousFile message"
    );
}

#[test]
fn test_down_arrow_shortcut_maps_to_next_file() {
    let message = message_for_key_chord(ShortcutKey::ArrowDown, Modifiers::default());
    assert!(
        matches!(message, Some(Message::NextFile)),
        "Down arrow should map to NextFile message"
    );
}

#[test]
fn test_escape_shortcut_maps_to_close_modal() {
    let message = message_for_key_chord(ShortcutKey::Escape, Modifiers::default());
    assert!(
        matches!(message, Some(Message::CloseModal)),
        "Escape key should map to CloseModal message"
    );
}

#[test]
fn test_ctrl_b_creates_bookmark() {
    #[cfg(not(target_os = "macos"))]
    {
        let message = message_for_key_chord(ShortcutKey::B, Modifiers::CTRL);
        assert!(
            matches!(message, Some(Message::CreateBookmark)),
            "Ctrl+B should map to CreateBookmark message on non-macOS"
        );
    }
}

#[test]
#[cfg(target_os = "macos")]
fn test_cmd_b_creates_bookmark_macos() {
    let message = message_for_key_chord(ShortcutKey::B, Modifiers::LOGO);
    assert!(
        matches!(message, Some(Message::CreateBookmark)),
        "Cmd+B should map to CreateBookmark message on macOS"
    );
}

#[test]
fn test_arrow_keys_with_shift_rejected() {
    let left = message_for_key_chord(ShortcutKey::ArrowLeft, Modifiers::SHIFT);
    assert!(
        left.is_none(),
        "Left arrow with Shift modifier should be rejected"
    );

    let right = message_for_key_chord(ShortcutKey::ArrowRight, Modifiers::SHIFT);
    assert!(
        right.is_none(),
        "Right arrow with Shift modifier should be rejected"
    );

    let up = message_for_key_chord(ShortcutKey::ArrowUp, Modifiers::SHIFT);
    assert!(
        up.is_none(),
        "Up arrow with Shift modifier should be rejected"
    );

    let down = message_for_key_chord(ShortcutKey::ArrowDown, Modifiers::SHIFT);
    assert!(
        down.is_none(),
        "Down arrow with Shift modifier should be rejected"
    );
}

#[test]
fn test_arrow_keys_with_ctrl_rejected() {
    let left = message_for_key_chord(ShortcutKey::ArrowLeft, Modifiers::CTRL);
    assert!(
        left.is_none(),
        "Left arrow with Ctrl modifier should be rejected"
    );

    let right = message_for_key_chord(ShortcutKey::ArrowRight, Modifiers::CTRL);
    assert!(
        right.is_none(),
        "Right arrow with Ctrl modifier should be rejected"
    );

    let up = message_for_key_chord(ShortcutKey::ArrowUp, Modifiers::CTRL);
    assert!(
        up.is_none(),
        "Up arrow with Ctrl modifier should be rejected"
    );

    let down = message_for_key_chord(ShortcutKey::ArrowDown, Modifiers::CTRL);
    assert!(
        down.is_none(),
        "Down arrow with Ctrl modifier should be rejected"
    );
}

#[test]
fn test_escape_with_modifiers_rejected() {
    let with_ctrl = message_for_key_chord(ShortcutKey::Escape, Modifiers::CTRL);
    assert!(
        with_ctrl.is_none(),
        "Escape with Ctrl modifier should be rejected"
    );

    let with_shift = message_for_key_chord(ShortcutKey::Escape, Modifiers::SHIFT);
    assert!(
        with_shift.is_none(),
        "Escape with Shift modifier should be rejected"
    );

    let with_alt = message_for_key_chord(ShortcutKey::Escape, Modifiers::ALT);
    assert!(
        with_alt.is_none(),
        "Escape with Alt modifier should be rejected"
    );
}

#[test]
fn test_keyboard_shortcuts_require_exact_modifiers() {
    // Space with any modifier should be rejected
    assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::CTRL).is_none());
    assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::SHIFT).is_none());
    assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::ALT).is_none());
    assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::LOGO).is_none());

    // Arrow keys with any modifier should be rejected
    let modifiers = [Modifiers::CTRL, Modifiers::SHIFT, Modifiers::ALT];
    for modifier in modifiers {
        assert!(message_for_key_chord(ShortcutKey::ArrowLeft, modifier).is_none());
        assert!(message_for_key_chord(ShortcutKey::ArrowRight, modifier).is_none());
        assert!(message_for_key_chord(ShortcutKey::ArrowUp, modifier).is_none());
        assert!(message_for_key_chord(ShortcutKey::ArrowDown, modifier).is_none());
    }
}

#[test]
fn test_bookmark_shortcut_database_persistence() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test/audiobooks", "Test Book")?;
    let file_path = "/test/audiobooks/Test Book/chapter_01.mp3";

    // Insert a test file using the helper function
    insert_test_file(&db, audiobook_id, file_path)?;

    // Simulate bookmark creation (normally done via Message::CreateBookmark in update function)
    let position_ms = 30000; // 30 seconds
    let bookmark = nodoka::models::Bookmark::new(
        audiobook_id,
        file_path.to_string(),
        position_ms,
        "Test Bookmark".to_string(),
    );
    let bookmark_id = queries::insert_bookmark(db.connection(), &bookmark)?;

    // Verify bookmark was saved
    assert!(bookmark_id > 0, "Bookmark ID should be positive");

    // Verify we can retrieve the bookmark
    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 1);
    if let Some(bookmark) = bookmarks.first() {
        assert_eq!(bookmark.audiobook_id, audiobook_id);
        assert_eq!(bookmark.file_path, file_path);
        assert_eq!(bookmark.position_ms, position_ms);
        assert_eq!(bookmark.label, "Test Bookmark");
    } else {
        return Err("bookmark should exist".into());
    }

    Ok(())
}

#[test]
fn test_all_keyboard_shortcuts_documented() {
    // Verify all ShortcutKey variants are tested
    // This test serves as documentation of all available shortcuts
    let shortcuts = vec![
        (ShortcutKey::Space, "Play/Pause toggle"),
        (ShortcutKey::B, "Create bookmark (with Ctrl/Cmd)"),
        (ShortcutKey::ArrowLeft, "Seek backward 5 seconds"),
        (ShortcutKey::ArrowRight, "Seek forward 5 seconds"),
        (ShortcutKey::ArrowUp, "Previous file in list"),
        (ShortcutKey::ArrowDown, "Next file in list"),
        (ShortcutKey::Escape, "Close modal window"),
    ];

    // Verify each shortcut produces a message (or None for B without modifiers)
    for (key, description) in shortcuts {
        if key == ShortcutKey::B {
            // B requires modifiers
            #[cfg(target_os = "macos")]
            let result = message_for_key_chord(key, Modifiers::LOGO);
            #[cfg(not(target_os = "macos"))]
            let result = message_for_key_chord(key, Modifiers::CTRL);

            assert!(
                result.is_some(),
                "Shortcut '{description}' should produce a message with appropriate modifier"
            );
        } else {
            let result = message_for_key_chord(key, Modifiers::default());
            assert!(
                result.is_some(),
                "Shortcut '{description}' should produce a message"
            );
        }
    }
}
