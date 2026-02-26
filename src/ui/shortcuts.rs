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
    /// The `F` key.
    F,
    /// The `L` key.
    L,
    /// The `T` key.
    T,
    /// The `R` key.
    R,
    /// Left bracket `[` key.
    LeftBracket,
    /// Right bracket `]` key.
    RightBracket,
    /// Left arrow key.
    ArrowLeft,
    /// Right arrow key.
    ArrowRight,
    /// Up arrow key.
    ArrowUp,
    /// Down arrow key.
    ArrowDown,
    /// Escape key.
    Escape,
    /// F1 function key.
    F1,
    /// Tab key.
    Tab,
}

/// Maps a key + modifiers to a UI [`Message`].
///
/// Returns `None` when the chord is not a Nodoka shortcut.
///
/// # Supported shortcuts
/// - Space: Play/Pause
/// - Ctrl+B (Cmd+B on macOS): Create bookmark
/// - Ctrl+F (Cmd+F on macOS): Focus search bar
/// - Ctrl+L (Cmd+L on macOS): Toggle focus mode
/// - Ctrl+T (Cmd+T on macOS): Toggle theme
/// - Ctrl+R (Cmd+R on macOS): Play last listened
/// - Ctrl+[ (Cmd+[ on macOS): Decrease panel ratio
/// - Ctrl+] (Cmd+] on macOS): Increase panel ratio
/// - Left arrow: Seek backward 5 seconds
/// - Right arrow: Seek forward 5 seconds
/// - Up arrow: Previous file
/// - Down arrow: Next file
/// - Escape: Close modal
/// - F1: Show keyboard help
/// - Tab: Cycle focus forward
/// - Shift+Tab: Cycle focus backward
#[must_use]
#[allow(clippy::too_many_lines)]
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
        ShortcutKey::F => {
            if is_command_modifier(modifiers) {
                Some(Message::FocusSearch)
            } else {
                None
            }
        }
        ShortcutKey::L => {
            if is_command_modifier(modifiers) {
                Some(Message::ToggleFocusMode)
            } else {
                None
            }
        }
        ShortcutKey::T => {
            if is_command_modifier(modifiers) {
                // Toggle theme - we need to know current theme, so this will be handled in update
                Some(Message::ToggleTheme)
            } else {
                None
            }
        }
        ShortcutKey::R => {
            if is_command_modifier(modifiers) {
                Some(Message::PlayLastListened)
            } else {
                None
            }
        }
        ShortcutKey::LeftBracket => {
            if is_command_modifier(modifiers) {
                Some(Message::DecreasePanelRatio)
            } else {
                None
            }
        }
        ShortcutKey::RightBracket => {
            if is_command_modifier(modifiers) {
                Some(Message::IncreasePanelRatio)
            } else {
                None
            }
        }
        ShortcutKey::ArrowLeft => {
            if modifiers == Modifiers::default() {
                Some(Message::SeekBackward(5)) // 5 seconds back
            } else {
                None
            }
        }
        ShortcutKey::ArrowRight => {
            if modifiers == Modifiers::default() {
                Some(Message::SeekForward(5)) // 5 seconds forward
            } else {
                None
            }
        }
        ShortcutKey::ArrowUp => {
            if modifiers == Modifiers::default() {
                Some(Message::PreviousFile)
            } else {
                None
            }
        }
        ShortcutKey::ArrowDown => {
            if modifiers == Modifiers::default() {
                Some(Message::NextFile)
            } else {
                None
            }
        }
        ShortcutKey::Escape => {
            if modifiers == Modifiers::default() {
                Some(Message::CloseModal)
            } else {
                None
            }
        }
        ShortcutKey::F1 => {
            if modifiers == Modifiers::default() {
                Some(Message::ShowKeyboardHelp)
            } else {
                None
            }
        }
        ShortcutKey::Tab => {
            if modifiers == Modifiers::default() {
                Some(Message::FocusNext)
            } else if modifiers == Modifiers::SHIFT {
                Some(Message::FocusPrevious)
            } else {
                None
            }
        }
    }
}

fn is_bookmark_modifier(modifiers: Modifiers) -> bool {
    // Accept Ctrl+B or Cmd+B, but not Ctrl+Shift+B, etc.
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

fn is_command_modifier(modifiers: Modifiers) -> bool {
    // Same as bookmark modifier - accept Ctrl or Cmd without extra modifiers
    is_bookmark_modifier(modifiers)
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

    #[test]
    fn test_arrow_left_seeks_backward() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::ArrowLeft, Modifiers::default()),
            Some(Message::SeekBackward(5))
        ));
    }

    #[test]
    fn test_arrow_right_seeks_forward() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::ArrowRight, Modifiers::default()),
            Some(Message::SeekForward(5))
        ));
    }

    #[test]
    fn test_arrow_up_selects_previous_file() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::ArrowUp, Modifiers::default()),
            Some(Message::PreviousFile)
        ));
    }

    #[test]
    fn test_arrow_down_selects_next_file() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::ArrowDown, Modifiers::default()),
            Some(Message::NextFile)
        ));
    }

    #[test]
    fn test_escape_closes_modal() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::Escape, Modifiers::default()),
            Some(Message::CloseModal)
        ));
    }

    #[test]
    fn test_arrow_keys_with_modifiers_return_none() {
        assert!(message_for_key_chord(ShortcutKey::ArrowLeft, Modifiers::CTRL).is_none());
        assert!(message_for_key_chord(ShortcutKey::ArrowRight, Modifiers::SHIFT).is_none());
        assert!(message_for_key_chord(ShortcutKey::ArrowUp, Modifiers::ALT).is_none());
        assert!(message_for_key_chord(ShortcutKey::ArrowDown, Modifiers::CTRL).is_none());
    }

    #[test]
    fn test_escape_with_modifiers_returns_none() {
        assert!(message_for_key_chord(ShortcutKey::Escape, Modifiers::CTRL).is_none());
        assert!(message_for_key_chord(ShortcutKey::Escape, Modifiers::SHIFT).is_none());
    }

    #[test]
    fn test_f1_shows_keyboard_help() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::F1, Modifiers::default()),
            Some(Message::ShowKeyboardHelp)
        ));
    }

    #[test]
    fn test_f1_with_modifiers_returns_none() {
        assert!(message_for_key_chord(ShortcutKey::F1, Modifiers::CTRL).is_none());
        assert!(message_for_key_chord(ShortcutKey::F1, Modifiers::SHIFT).is_none());
        assert!(message_for_key_chord(ShortcutKey::F1, Modifiers::ALT).is_none());
    }

    #[test]
    fn test_tab_cycles_focus_forward() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::Tab, Modifiers::default()),
            Some(Message::FocusNext)
        ));
    }

    #[test]
    fn test_shift_tab_cycles_focus_backward() {
        assert!(matches!(
            message_for_key_chord(ShortcutKey::Tab, Modifiers::SHIFT),
            Some(Message::FocusPrevious)
        ));
    }

    #[test]
    fn test_tab_with_other_modifiers_returns_none() {
        assert!(message_for_key_chord(ShortcutKey::Tab, Modifiers::CTRL).is_none());
        let mods = Modifiers::SHIFT | Modifiers::CTRL;
        assert!(message_for_key_chord(ShortcutKey::Tab, mods).is_none());
    }
}
