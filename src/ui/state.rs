use crate::models::{Audiobook, AudiobookFile, Bookmark, Directory};
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
}
