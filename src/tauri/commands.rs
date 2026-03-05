//! Tauri command handlers for IPC communication
//!
//! This module provides all the command handlers that the web frontend
//! can invoke via Tauri's IPC mechanism.

use crate::db::queries;
use crate::models::{Audiobook, AudiobookFile, Bookmark, Directory};
use crate::settings::Settings;
use crate::tauri::state::AppState;
use chrono::Utc;
use std::path::Path;
use tauri::State;

// ============================================================================
// Database Commands - Audiobooks
// ============================================================================

/// Get all audiobooks from the database
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_all_audiobooks(
    state: State<'_, AppState>,
) -> std::result::Result<Vec<Audiobook>, String> {
    let db = state.db.lock();
    queries::get_all_audiobooks(db.connection()).map_err(|e| e.to_string())
}

/// Get a specific audiobook by ID
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_audiobook_by_id(
    id: i64,
    state: State<'_, AppState>,
) -> std::result::Result<Option<Audiobook>, String> {
    let db = state.db.lock();
    queries::get_audiobook_by_id(db.connection(), id).map_err(|e| e.to_string())
}

/// Get all files for a specific audiobook
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_audiobook_files(
    audiobook_id: i64,
    state: State<'_, AppState>,
) -> std::result::Result<Vec<AudiobookFile>, String> {
    let db = state.db.lock();
    queries::get_audiobook_files(db.connection(), audiobook_id).map_err(|e| e.to_string())
}

// ============================================================================
// Database Commands - Bookmarks
// ============================================================================

/// Get all bookmarks for a specific audiobook
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_bookmarks(
    audiobook_id: i64,
    state: State<'_, AppState>,
) -> std::result::Result<Vec<Bookmark>, String> {
    let db = state.db.lock();
    queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)
        .map_err(|e| e.to_string())
}

/// Create a new bookmark
///
/// # Errors
/// Returns an error string if the database insert fails
#[tauri::command]
pub async fn create_bookmark(
    audiobook_id: i64,
    file_path: String,
    position_ms: i64,
    label: String,
    note: Option<String>,
    state: State<'_, AppState>,
) -> std::result::Result<i64, String> {
    let db = state.db.lock();
    let mut bookmark = Bookmark::new(audiobook_id, file_path, position_ms, label);
    bookmark.note = note;
    queries::insert_bookmark(db.connection(), &bookmark).map_err(|e| e.to_string())
}

/// Update an existing bookmark
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn update_bookmark(
    bookmark: Bookmark,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    queries::update_bookmark(db.connection(), &bookmark).map_err(|e| e.to_string())
}

/// Delete a bookmark
///
/// # Errors
/// Returns an error string if the database delete fails
#[tauri::command]
pub async fn delete_bookmark(
    id: i64,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    queries::delete_bookmark(db.connection(), id).map_err(|e| e.to_string())
}

/// Jump to bookmark position
///
/// # Errors
/// Returns an error string if the player or database operations fail
#[tauri::command]
pub async fn jump_to_bookmark(
    id: i64,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();

    // Get the bookmark details - need to query it manually
    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), 0)
        .unwrap_or_default();
    let bookmark = bookmarks.iter().find(|b| b.id == Some(id));

    if let Some(bookmark) = bookmark {
        // Update player state
        {
            let mut player_guard = state.player.lock();
            if let Some(player) = player_guard.as_mut() {
                let _ = player.load_media(Path::new(&bookmark.file_path));
                let _ = player.set_time(bookmark.position_ms);
                let _ = player.play();
            }
        }

        // Update selected file
        let _ = queries::get_metadata(db.connection(), "current_audiobook_id")
            .ok()
            .flatten()
            .and_then(|id_str| id_str.parse::<i64>().ok())
            .map(|audiobook_id| {
                let _ = queries::update_audiobook_selected_file(
                    db.connection(),
                    audiobook_id,
                    Some(&bookmark.file_path),
                );
            });
    }

    Ok(())
}

// ============================================================================
// Database Commands - Directories
// ============================================================================

/// Get all directories
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_directories(
    state: State<'_, AppState>,
) -> std::result::Result<Vec<Directory>, String> {
    let db = state.db.lock();
    queries::get_all_directories(db.connection()).map_err(|e| e.to_string())
}

/// Insert a new directory
///
/// # Errors
/// Returns an error string if the database insert fails
#[tauri::command]
pub async fn insert_directory(
    path: String,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let directory = Directory::new(path.clone());
    queries::insert_directory(db.connection(), &directory).map_err(|e| e.to_string())?;
    drop(db);

    // Start scanning the directory
    let path_clone = path;
    tokio::spawn(async move {
        if let Err(e) = crate::tasks::scan_directory(std::path::PathBuf::from(&path_clone)).await {
            tracing::error!("Failed to scan directory {}: {}", path_clone, e);
        }
    });

    Ok(())
}

/// Remove a directory
///
/// # Errors
/// Returns an error string if the database delete fails
#[tauri::command]
pub async fn remove_directory(
    path: String,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    queries::delete_directory(db.connection(), &path).map_err(|e| e.to_string())
}

/// Validation result for directory accessibility
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirectoryValidation {
    /// Whether the directory exists on disk
    pub exists: bool,
    /// Whether the directory is readable
    pub readable: bool,
    /// Error type if validation failed
    pub error: Option<String>,
}

/// Validate directory accessibility
///
/// Returns validation status for a directory path, checking if it exists and is readable.
///
/// # Errors
///
/// This function does not return errors - all validation results are returned in the
/// `DirectoryValidation` struct, including cases where the path doesn't exist or isn't readable.
#[tauri::command]
pub async fn validate_directory(path: String) -> std::result::Result<DirectoryValidation, String> {
    use std::fs;

    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Ok(DirectoryValidation {
            exists: false,
            readable: false,
            error: Some("NotFound".to_string()),
        });
    }

    if !dir_path.is_dir() {
        return Ok(DirectoryValidation {
            exists: true,
            readable: false,
            error: Some("NotADirectory".to_string()),
        });
    }

    let readable = fs::read_dir(&path)
        .map(|mut d| d.next().map(|r| r.is_ok()).unwrap_or(true))
        .unwrap_or(false);

    Ok(DirectoryValidation {
        exists: true,
        readable,
        error: if readable {
            None
        } else {
            Some("PermissionDenied".to_string())
        },
    })
}

/// Update directory path (for relocated folders)
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn update_directory_path(
    old_path: String,
    new_path: String,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    queries::update_directory_path(db.connection(), &old_path, &new_path)
        .map_err(|e| e.to_string())
}

/// Scan a directory for new audiobooks
///
/// # Errors
/// Returns an error string if scanning or database operations fail
#[tauri::command]
pub async fn scan_directory(
    path: String,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    use crate::tasks::{convert_to_audiobooks, scan_directory};
    use std::path::PathBuf;

    let path_buf = PathBuf::from(&path);
    let discovered = scan_directory(path_buf)
        .await
        .map_err(|e| e.to_string())?;

    let db = state.db.lock();
    let audiobooks = convert_to_audiobooks(discovered.clone(), &path);

    // Process each discovered audiobook
    for (idx, disc) in discovered.into_iter().enumerate() {
        let disc_path = disc.path.display().to_string();
        let resolved_audiobook = audiobooks
            .get(idx)
            .cloned()
            .unwrap_or_else(|| {
                crate::models::Audiobook::new(
                    path.clone(),
                    disc.name.clone(),
                    disc_path.clone(),
                    i32::try_from(idx).unwrap_or(i32::MAX),
                )
            });

        // Resolve or insert audiobook
        let audiobook_id = match crate::db::queries::get_audiobook_by_path(db.connection(), &disc_path) {
            Ok(Some(existing_ab)) => {
                if let Some(id) = existing_ab.id {
                    id
                } else {
                    let id = crate::db::queries::insert_audiobook(db.connection(), &resolved_audiobook)
                        .map_err(|e| e.to_string())?;
                    id
                }
            }
            Err(_) => {
                crate::db::queries::insert_audiobook(db.connection(), &resolved_audiobook)
                    .map_err(|e| e.to_string())?
            }
            Ok(None) => {
                crate::db::queries::insert_audiobook(db.connection(), &resolved_audiobook)
                    .map_err(|e| e.to_string())?
            }
        };

        // Process audiobook files
        process_audiobook_files(&db, audiobook_id, disc.files);
    }

    // Update directory scan timestamp
    let _ = crate::db::queries::update_directory_last_scanned(db.connection(), &path);

    Ok(())
}

/// Process audiobook files during scanning
fn process_audiobook_files(
    db: &crate::db::Database,
    audiobook_id: i64,
    files: Vec<crate::tasks::DiscoveredFile>,
) {
    use natord::compare;

    let mut files = files;
    files.sort_by(|a, b| compare(&a.sort_key, &b.sort_key));

    for (pos, file) in files.into_iter().enumerate() {
        let name = file.name;
        let full_path = file.full_path;
        let mut audiobook_file = crate::models::AudiobookFile::new(
            audiobook_id,
            name,
            full_path.clone(),
            i32::try_from(pos).unwrap_or(i32::MAX),
        );
        audiobook_file.checksum = file.checksum;

        if let Ok(Some(existing_file)) =
            crate::db::queries::get_audiobook_file_by_path(db.connection(), &full_path)
        {
            audiobook_file.length_of_file = existing_file.length_of_file;
            audiobook_file.seek_position = existing_file.seek_position;
            audiobook_file.completeness = existing_file.completeness;
            audiobook_file.file_exists = true;
        }

        if let Err(e) = crate::db::queries::insert_audiobook_file(db.connection(), &audiobook_file) {
            tracing::error!("Failed to insert audiobook file: {e}");
        }
    }
}

// ============================================================================
// Player Commands
// ============================================================================

/// Play/pause toggle
///
/// # Errors
/// Returns an error string if player is not initialized or playback control fails
#[tauri::command]
pub async fn play_pause(state: State<'_, AppState>) -> std::result::Result<bool, String> {
    let mut player_guard = state.player.lock();
    let player = player_guard
        .as_mut()
        .ok_or_else(|| "Player not initialized".to_string())?;

    let is_playing = player.is_playing();
    if is_playing {
        player.pause().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        player.play().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

/// Stop playback
///
/// # Errors
/// Returns an error string if player is not initialized or stop fails
#[tauri::command]
pub async fn stop(state: State<'_, AppState>) -> std::result::Result<(), String> {
    let mut player_guard = state.player.lock();
    let player = player_guard
        .as_mut()
        .ok_or_else(|| "Player not initialized".to_string())?;
    player.stop().map_err(|e| e.to_string())
}

/// Seek to position in milliseconds
///
/// # Errors
/// Returns an error string if player is not initialized or seek fails
#[tauri::command]
pub async fn seek(
    position_ms: i64,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let player_guard = state.player.lock();
    let player = player_guard
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    player.set_time(position_ms).map_err(|e| e.to_string())
}

/// Set volume (0-200)
///
/// # Errors
/// Returns an error string if player is not initialized or volume control fails
#[tauri::command]
pub async fn set_volume(
    volume: i32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let mut player_guard = state.player.lock();
    let player = player_guard
        .as_mut()
        .ok_or_else(|| "Player not initialized".to_string())?;
    player.set_volume(volume).map_err(|e| e.to_string())?;

    // Persist volume preference
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    let _ = settings.set_volume(volume);

    Ok(())
}

/// Set playback speed (0.5-2.0)
///
/// # Errors
/// Returns an error string if player is not initialized or speed control fails
#[tauri::command]
pub async fn set_speed(
    speed: f32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let mut player_guard = state.player.lock();
    let player = player_guard
        .as_mut()
        .ok_or_else(|| "Player not initialized".to_string())?;
    player.set_rate(speed).map_err(|e| e.to_string())?;

    // Persist speed preference
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    let _ = settings.set_speed(speed);

    Ok(())
}

/// Load and play a file
///
/// # Errors
/// Returns an error string if player is not initialized or media loading fails
#[tauri::command]
pub async fn load_file(
    file_path: String,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let mut player_guard = state.player.lock();
    let player = player_guard
        .as_mut()
        .ok_or_else(|| "Player not initialized".to_string())?;
    player
        .load_media(Path::new(&file_path))
        .map_err(|e| e.to_string())?;
    player.play().map_err(|e| e.to_string())?;

    // Update current file metadata
    let db = state.db.lock();
    if let Some(id_str) = queries::get_metadata(db.connection(), "current_audiobook_id")
        .ok()
        .flatten()
    {
        if let Ok(id) = id_str.parse::<i64>() {
            let _ = queries::update_audiobook_selected_file(db.connection(), id, Some(&file_path));
            let _ = queries::update_audiobook_last_listened(db.connection(), id, Utc::now());
        }
    }

    Ok(())
}

/// Get current playback position in milliseconds
///
/// # Errors
/// Returns an error string if player is not initialized or time query fails
#[tauri::command]
pub async fn get_current_time(state: State<'_, AppState>) -> std::result::Result<f64, String> {
    let player_guard = state.player.lock();
    let player = player_guard
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    player.get_time().map_err(|e| e.to_string())
}

/// Get total media duration in milliseconds
///
/// # Errors
/// Returns an error string if player is not initialized or length query fails
#[tauri::command]
pub async fn get_duration(state: State<'_, AppState>) -> std::result::Result<f64, String> {
    let player_guard = state.player.lock();
    let player = player_guard
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    let duration = player.get_length().map_err(|e| e.to_string())?;
    Ok(duration as f64)
}

/// Get current playback state
///
/// # Errors
/// Returns an error string if player is not initialized
#[tauri::command]
pub async fn get_playback_state(state: State<'_, AppState>) -> std::result::Result<String, String> {
    let player_guard = state.player.lock();
    let player = player_guard
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    Ok(format!("{:?}", player.get_state()))
}

// ============================================================================
// Settings Commands
// ============================================================================

/// Get the current theme
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_theme(state: State<'_, AppState>) -> std::result::Result<String, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.get_theme().map_err(|e| e.to_string())
}

/// Set the theme
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_theme(theme: String, state: State<'_, AppState>) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.set_theme(&theme).map_err(|e| e.to_string())
}

/// Get the panel ratio
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_panel_ratio(state: State<'_, AppState>) -> std::result::Result<f32, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.get_panel_ratio().map_err(|e| e.to_string())
}

/// Set the panel ratio
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_panel_ratio(
    ratio: f32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.set_panel_ratio(ratio).map_err(|e| e.to_string())
}

/// Get focus mode setting
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_focus_mode(state: State<'_, AppState>) -> std::result::Result<bool, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.get_focus_mode().map_err(|e| e.to_string())
}

/// Set focus mode
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_focus_mode(
    enabled: bool,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.set_focus_mode(enabled).map_err(|e| e.to_string())
}

/// Get the volume preference
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_volume(state: State<'_, AppState>) -> std::result::Result<i32, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.get_volume().map_err(|e| e.to_string())
}

/// Set the volume preference
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_volume_preference(
    volume: i32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.set_volume(volume).map_err(|e| e.to_string())
}

/// Get the speed preference
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_speed(state: State<'_, AppState>) -> std::result::Result<f32, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.get_speed().map_err(|e| e.to_string())
}

/// Set the speed preference
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_speed_preference(
    speed: f32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());
    settings.set_speed(speed).map_err(|e| e.to_string())
}

/// Get prefers reduced motion setting
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_prefers_reduced_motion(
    state: State<'_, AppState>,
) -> std::result::Result<bool, String> {
    let db = state.db.lock();
    if let Ok(Some(value)) = queries::get_metadata(db.connection(), "prefers_reduced_motion") {
        match value.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Ok(false),
        }
    } else {
        Ok(false)
    }
}

/// Set prefers reduced motion
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_prefers_reduced_motion(
    prefers: bool,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    queries::set_metadata(
        db.connection(),
        "prefers_reduced_motion",
        if prefers { "true" } else { "false" },
    )
    .map_err(|e| e.to_string())
}

/// Get skip forward duration in seconds
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_skip_forward_duration(state: State<'_, AppState>) -> std::result::Result<i32, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());

    settings
        .get_skip_forward_duration()
        .map_err(|e| e.to_string())
        .map(|v| v.unwrap_or(30))
}

/// Set skip forward duration in seconds
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_skip_forward_duration(
    seconds: i32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());

    settings
        .set_skip_forward_duration(seconds)
        .map_err(|e| e.to_string())
}

/// Get skip backward duration in seconds
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_skip_backward_duration(state: State<'_, AppState>) -> std::result::Result<i32, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());

    settings
        .get_skip_backward_duration()
        .map_err(|e| e.to_string())
        .map(|v| v.unwrap_or(15))
}

/// Set skip backward duration in seconds
///
/// # Errors
/// Returns an error string if the database update fails
#[tauri::command]
pub async fn set_skip_backward_duration(
    seconds: i32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());

    settings
        .set_skip_backward_duration(seconds)
        .map_err(|e| e.to_string())
}

/// Get auto-save interval in seconds
///
/// # Errors
/// Returns an error string if the database query fails
#[tauri::command]
pub async fn get_auto_save_interval(state: State<'_, AppState>) -> std::result::Result<i32, String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());

    settings
        .get_auto_save_interval()
        .map_err(|e| e.to_string())
        .map(|v| v.unwrap_or(5))
}

/// Set auto-save interval in seconds
///
/// # Errors
/// Returns an error string if the database update fails or interval is out of range
#[tauri::command]
pub async fn set_auto_save_interval(
    interval: i32,
    state: State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db = state.db.lock();
    let settings = Settings::new(db.connection());

    settings
        .set_auto_save_interval(interval)
        .map_err(|e| e.to_string())
}
