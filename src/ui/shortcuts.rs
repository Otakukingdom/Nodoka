//! Keyboard shortcut mapping.
//!
//! This module provides a small, testable mapping from key chords to
//! [`crate::ui::Message`] values.

use crate::ui::Message;
use iced::keyboard::Modifiers;

/// Keys that Nodoka binds to actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutKey {
    /// Space key.
    Space,
    /// The `B` key.
    B,
}

/// Maps a key + modifiers to a UI [`Message`].
///
/// Returns `None` when the chord is not a Nodoka shortcut.
#[must_use]
pub fn message_for_key_chord(key: ShortcutKey, modifiers: Modifiers) -> Option<Message> {
    match key {
        ShortcutKey::Space => {
            if modifiers == Modifiers::default() {
                Some(Message::PlayPause)
            } else {
                None
            }
        }
        ShortcutKey::B => {
            if modifiers.control() && !modifiers.shift() && !modifiers.alt() && !modifiers.logo() {
                Some(Message::CreateBookmark)
            } else {
                None
            }
        }
    }
}
