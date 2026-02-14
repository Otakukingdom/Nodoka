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
    fn test_space_with_modifiers_returns_none() {
        assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::CTRL).is_none());
        assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::SHIFT).is_none());
        assert!(message_for_key_chord(ShortcutKey::Space, Modifiers::ALT).is_none());
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

    #[test]
    fn test_b_without_modifiers_returns_none() {
        assert!(message_for_key_chord(ShortcutKey::B, Modifiers::default()).is_none());
    }

    #[test]
    fn test_b_with_wrong_modifiers_returns_none() {
        assert!(message_for_key_chord(ShortcutKey::B, Modifiers::SHIFT).is_none());
        assert!(message_for_key_chord(ShortcutKey::B, Modifiers::ALT).is_none());
    }

    #[test]
    fn test_b_with_extra_modifiers_returns_none() {
        // Shift + Ctrl + B should not work
        let mods = Modifiers::CTRL | Modifiers::SHIFT;
        assert!(message_for_key_chord(ShortcutKey::B, mods).is_none());

        // Alt + Ctrl + B should not work
        let mods = Modifiers::CTRL | Modifiers::ALT;
        assert!(message_for_key_chord(ShortcutKey::B, mods).is_none());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos_b_with_ctrl_also_works() {
        // On macOS, Ctrl+B should also work for compatibility
        assert!(matches!(
            message_for_key_chord(ShortcutKey::B, Modifiers::CTRL),
            Some(Message::CreateBookmark)
        ));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_non_macos_b_with_logo_returns_none() {
        // On non-macOS, Logo+B should not work
        assert!(message_for_key_chord(ShortcutKey::B, Modifiers::LOGO).is_none());
    }

    #[test]
    fn test_is_bookmark_modifier_rejects_shift() {
        #[cfg(target_os = "macos")]
        let mods = Modifiers::LOGO | Modifiers::SHIFT;
        #[cfg(not(target_os = "macos"))]
        let mods = Modifiers::CTRL | Modifiers::SHIFT;

        assert!(!is_bookmark_modifier(mods));
    }

    #[test]
    fn test_is_bookmark_modifier_rejects_alt() {
        #[cfg(target_os = "macos")]
        let mods = Modifiers::LOGO | Modifiers::ALT;
        #[cfg(not(target_os = "macos"))]
        let mods = Modifiers::CTRL | Modifiers::ALT;

        assert!(!is_bookmark_modifier(mods));
    }

    #[test]
    fn test_shortcut_key_equality() {
        assert_eq!(ShortcutKey::Space, ShortcutKey::Space);
        assert_eq!(ShortcutKey::B, ShortcutKey::B);
        assert_ne!(ShortcutKey::Space, ShortcutKey::B);
    }

    #[test]
    fn test_shortcut_key_debug() {
        // Verify ShortcutKey implements Debug for better error messages
        let _debug = format!("{:?}", ShortcutKey::Space);
        let _debug = format!("{:?}", ShortcutKey::B);
    }

    #[test]
    fn test_shortcut_key_clone() {
        let key = ShortcutKey::Space;
        let cloned = key;
        assert_eq!(key, cloned);
    }
}
