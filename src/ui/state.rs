use crate::models::{Audiobook, AudiobookFile, Directory};

#[derive(Debug, Clone)]
pub struct NodokaState {
    pub audiobooks: Vec<Audiobook>,
    pub selected_audiobook: Option<i64>,
    pub current_files: Vec<AudiobookFile>,
    pub selected_file: Option<String>,
    pub directories: Vec<Directory>,

    pub is_playing: bool,
    pub current_time: f64,
    pub total_duration: f64,
    pub volume: i32,
    pub speed: f32,

    pub settings_open: bool,
    pub is_loading: bool,
}

impl Default for NodokaState {
    fn default() -> Self {
        Self {
            audiobooks: Vec::new(),
            selected_audiobook: None,
            current_files: Vec::new(),
            selected_file: None,
            directories: Vec::new(),
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
