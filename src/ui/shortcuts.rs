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
            if is_bookmark_modifier(modifiers) {
                Some(Message::CreateBookmark)
            } else {
                None
            }
        }
    }
}

fn is_bookmark_modifier(modifiers: Modifiers) -> bool {
    let no_extra = !modifiers.shift() && !modifiers.alt();

    #[cfg(target_os = "macos")]
    {
        // Use Command as the primary modifier on macOS.
        // Also accept Ctrl for compatibility (e.g. external keyboards).
        no_extra && (modifiers.logo() || modifiers.control())
    }

    #[cfg(not(target_os = "macos"))]
    {
        no_extra && modifiers.control() && !modifiers.logo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_without_modifiers_maps_to_play_pause() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::Space, Modifiers::default()),
            Some(Message::PlayPause)
        ));
    }

    #[test]
    fn test_bookmark_shortcut_requires_expected_modifier() {
        #[cfg(target_os = "macos")]
        {
            assert!(matches!(
                message_for_key_chord(ShortcutKey::B, Modifiers::LOGO),
                Some(Message::CreateBookmark)
            ));
        }

        #[cfg(not(target_os = "macos"))]
        {
            assert!(matches!(
                message_for_key_chord(ShortcutKey::B, Modifiers::CTRL),
                Some(Message::CreateBookmark)
            ));
        }
    }
}
