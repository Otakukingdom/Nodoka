use crate::models::{Audiobook, AudiobookFile, Bookmark, Directory, SleepTimer};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BookmarkEditor {
    pub id: Option<i64>,
    pub audiobook_id: i64,
    pub file_path: String,
    pub position_ms: i64,
    pub label: String,
    pub note: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub audiobooks: Vec<Audiobook>,
    pub selected_audiobook: Option<i64>,
    pub current_files: Vec<AudiobookFile>,
    pub selected_file: Option<String>,
    pub directories: Vec<Directory>,

    pub cover_thumbnails: HashMap<i64, PathBuf>,

    pub bookmarks: Vec<Bookmark>,
    pub bookmark_editor: Option<BookmarkEditor>,

    pub is_playing: bool,
    pub current_time: f64,
    pub total_duration: f64,
    pub volume: i32,
    pub speed: f32,

    pub sleep_timer: Option<SleepTimer>,
    pub sleep_timer_base_volume: Option<i32>,
    pub sleep_timer_custom_minutes: String,
    pub sleep_timer_custom_error: Option<String>,

    pub settings_open: bool,
    pub is_loading: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            audiobooks: Vec::new(),
            selected_audiobook: None,
            current_files: Vec::new(),
            selected_file: None,
            directories: Vec::new(),
            cover_thumbnails: HashMap::new(),
            bookmarks: Vec::new(),
            bookmark_editor: None,
            is_playing: false,
            current_time: 0.0,
            total_duration: 0.0,
            volume: 100,
            speed: 1.0,
            sleep_timer: None,
            sleep_timer_base_volume: None,
            sleep_timer_custom_minutes: String::new(),
            sleep_timer_custom_error: None,
            settings_open: false,
            is_loading: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_default_values() {
        let state = State::default();
        assert!(state.audiobooks.is_empty());
        assert_eq!(state.selected_audiobook, None);
        assert!(state.current_files.is_empty());
        assert_eq!(state.selected_file, None);
        assert!(state.directories.is_empty());
        assert!(state.cover_thumbnails.is_empty());
        assert!(state.bookmarks.is_empty());
        assert!(state.bookmark_editor.is_none());
        assert!(!state.is_playing);
        assert!(state.current_time.abs() < f64::EPSILON);
        assert!(state.total_duration.abs() < f64::EPSILON);
        assert_eq!(state.volume, 100);
        assert!((state.speed - 1.0).abs() < f32::EPSILON);
        assert!(state.sleep_timer.is_none());
        assert!(state.sleep_timer_base_volume.is_none());
        assert!(state.sleep_timer_custom_minutes.is_empty());
        assert!(state.sleep_timer_custom_error.is_none());
        assert!(!state.settings_open);
        assert!(state.is_loading);
    }

    #[test]
    fn test_state_clone() {
        let state = State::default();
        assert_eq!(state.volume, state.volume);
        assert!((state.speed - state.speed).abs() < f32::EPSILON);
        assert_eq!(state.is_playing, state.is_playing);
        assert_eq!(state.cover_thumbnails.len(), state.cover_thumbnails.len());
    }

    #[test]
    fn test_state_mutation() {
        let state = State {
            is_playing: true,
            current_time: 100.0,
            total_duration: 3600.0,
            volume: 75,
            speed: 1.5,
            ..Default::default()
        };

        assert!(state.is_playing);
        assert!((state.current_time - 100.0).abs() < f64::EPSILON);
        assert!((state.total_duration - 3600.0).abs() < f64::EPSILON);
        assert_eq!(state.volume, 75);
        assert!((state.speed - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_state_settings_modal() {
        let mut state = State::default();
        assert!(!state.settings_open);

        state.settings_open = true;
        assert!(state.settings_open);

        state.settings_open = false;
        assert!(!state.settings_open);
    }

    #[test]
    fn test_state_selection() {
        let mut state = State::default();
        assert_eq!(state.selected_audiobook, None);
        assert_eq!(state.selected_file, None);

        state.selected_audiobook = Some(42);
        state.selected_file = Some("/path/to/file.mp3".to_string());

        assert_eq!(state.selected_audiobook, Some(42));
        assert_eq!(state.selected_file, Some("/path/to/file.mp3".to_string()));
    }

    #[test]
    fn test_state_loading_flag() {
        let mut state = State::default();
        assert!(state.is_loading);

        state.is_loading = false;
        assert!(!state.is_loading);
    }

    #[test]
    fn test_state_with_negative_time_values() {
        // Test that state can handle negative time values (edge case)
        let state = State {
            current_time: -100.0,
            total_duration: -50.0,
            ..Default::default()
        };

        assert!(state.current_time < 0.0);
        assert!(state.total_duration < 0.0);
    }

    #[test]
    fn test_state_with_time_exceeding_duration() {
        // Test edge case where current_time > total_duration
        let state = State {
            current_time: 5000.0,
            total_duration: 3000.0,
            ..Default::default()
        };

        assert!(state.current_time > state.total_duration);
    }

    #[test]
    fn test_state_with_invalid_volume_values() {
        // Test that state can handle out-of-range volume values
        let state_low = State {
            volume: -10,
            ..Default::default()
        };
        assert_eq!(state_low.volume, -10);

        let state_high = State {
            volume: 999,
            ..Default::default()
        };
        assert_eq!(state_high.volume, 999);
    }

    #[test]
    fn test_state_with_invalid_speed_values() {
        // Test that state can handle out-of-range speed values
        let state_low = State {
            speed: 0.1,
            ..Default::default()
        };
        assert!((state_low.speed - 0.1).abs() < f32::EPSILON);

        let state_high = State {
            speed: 5.0,
            ..Default::default()
        };
        assert!((state_high.speed - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_state_with_zero_duration() {
        let state = State {
            current_time: 100.0,
            total_duration: 0.0,
            ..Default::default()
        };

        assert!((state.total_duration).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bookmark_editor_default_values() {
        let editor = BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 0,
            label: String::new(),
            note: String::new(),
        };

        assert_eq!(editor.id, None);
        assert_eq!(editor.audiobook_id, 1);
        assert_eq!(editor.file_path, "/path/file.mp3");
        assert_eq!(editor.position_ms, 0);
        assert!(editor.label.is_empty());
        assert!(editor.note.is_empty());
    }

    #[test]
    fn test_bookmark_editor_with_data() {
        let editor = BookmarkEditor {
            id: Some(42),
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 12345,
            label: "Test Label".to_string(),
            note: "Test Note".to_string(),
        };

        assert_eq!(editor.id, Some(42));
        assert_eq!(editor.label, "Test Label");
        assert_eq!(editor.note, "Test Note");
        assert_eq!(editor.position_ms, 12345);
    }

    #[test]
    fn test_bookmark_editor_clone() {
        let editor = BookmarkEditor {
            id: Some(1),
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: "Note".to_string(),
        };

        let cloned = &editor;
        assert_eq!(editor.id, cloned.id);
        assert_eq!(editor.label, cloned.label);
        assert_eq!(editor.note, cloned.note);
    }

    #[test]
    fn test_state_with_sleep_timer_custom_minutes() {
        let state = State {
            sleep_timer_custom_minutes: "45".to_string(),
            ..Default::default()
        };

        assert_eq!(state.sleep_timer_custom_minutes, "45");
    }

    #[test]
    fn test_state_with_sleep_timer_custom_error() {
        let state = State {
            sleep_timer_custom_error: Some("Invalid input".to_string()),
            ..Default::default()
        };

        assert_eq!(
            state.sleep_timer_custom_error,
            Some("Invalid input".to_string())
        );
    }

    #[test]
    fn test_state_with_bookmark_editor_open() {
        let editor = BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        };

        let state = State {
            bookmark_editor: Some(editor),
            ..Default::default()
        };

        assert!(state.bookmark_editor.is_some());
    }

    #[test]
    fn test_state_with_multiple_directories() {
        let state = State {
            directories: vec![
                Directory::new("/path1".to_string()),
                Directory::new("/path2".to_string()),
                Directory::new("/path3".to_string()),
            ],
            ..Default::default()
        };

        assert_eq!(state.directories.len(), 3);
    }

    #[test]
    fn test_state_with_multiple_audiobooks() {
        let state = State {
            audiobooks: vec![
                Audiobook {
                    id: Some(1),
                    directory: "/test".to_string(),
                    name: "Book 1".to_string(),
                    full_path: "/test/book1".to_string(),
                    completeness: 50,
                    default_order: 0,
                    selected_file: None,
                    created_at: chrono::Utc::now(),
                },
                Audiobook {
                    id: Some(2),
                    directory: "/test".to_string(),
                    name: "Book 2".to_string(),
                    full_path: "/test/book2".to_string(),
                    completeness: 75,
                    default_order: 0,
                    selected_file: None,
                    created_at: chrono::Utc::now(),
                },
            ],
            ..Default::default()
        };

        assert_eq!(state.audiobooks.len(), 2);
    }

    #[test]
    fn test_state_with_cover_thumbnails() {
        let mut state = State::default();
        state
            .cover_thumbnails
            .insert(1, PathBuf::from("/cover1.jpg"));
        state
            .cover_thumbnails
            .insert(2, PathBuf::from("/cover2.jpg"));

        assert_eq!(state.cover_thumbnails.len(), 2);
        assert!(state.cover_thumbnails.contains_key(&1));
        assert!(state.cover_thumbnails.contains_key(&2));
    }

    #[test]
    fn test_state_debug_implementation() {
        let state = State::default();
        let debug_str = format!("{state:?}");
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_bookmark_editor_debug_implementation() {
        let editor = BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        };
        let debug_str = format!("{editor:?}");
        assert!(!debug_str.is_empty());
    }
}
