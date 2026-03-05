//! Tauri application entry point
//!
//! Replaces the iced-based UI with a Tauri hybrid architecture.
//! All existing business logic (database, player, scanning) is preserved.

use crate::db::Database;
use crate::player::Vlc;
use crate::tauri::AppState;
use tauri::Manager;

/// Runs the Tauri application
///
/// This replaces the iced application runner. All business logic remains intact.
///
/// # Panics
/// Panics if the Tauri application fails to run. This is intentional as the
/// application cannot recover from a Tauri initialization failure.
pub fn run(db: Database) -> ! {
    // Setup VLC environment early
    crate::player::setup_vlc_environment();

    // Initialize player
    let player = match Vlc::new() {
        Ok(p) => Some(p),
        Err(e) => {
            tracing::error!("Failed to initialize player: {e}");
            tracing::error!(
                "Audio playback will not work. Please install VLC media player."
            );
            None
        }
    };

    // Create shared application state
    let app_state = AppState::new(db, player);

    // Build and run Tauri application
    let result = tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database schema
            let state = app.state::<AppState>();
            let db = state.db.lock();
            if let Err(e) = crate::db::initialize(db.connection()) {
                tracing::error!("Failed to initialize database schema: {e}");
                return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Audiobook commands
            crate::tauri::commands::get_all_audiobooks,
            crate::tauri::commands::get_audiobook_by_id,
            crate::tauri::commands::get_audiobook_files,
            // Bookmark commands
            crate::tauri::commands::get_bookmarks,
            crate::tauri::commands::create_bookmark,
            crate::tauri::commands::update_bookmark,
            crate::tauri::commands::delete_bookmark,
            crate::tauri::commands::jump_to_bookmark,
            // Directory commands
            crate::tauri::commands::get_directories,
            crate::tauri::commands::insert_directory,
            crate::tauri::commands::remove_directory,
            crate::tauri::commands::scan_directory,
            crate::tauri::commands::validate_directory,
            crate::tauri::commands::update_directory_path,
            // Player commands
            crate::tauri::commands::play_pause,
            crate::tauri::commands::stop,
            crate::tauri::commands::seek,
            crate::tauri::commands::set_volume,
            crate::tauri::commands::set_speed,
            crate::tauri::commands::load_file,
            crate::tauri::commands::get_current_time,
            crate::tauri::commands::get_duration,
            crate::tauri::commands::get_playback_state,
            // Settings commands
            crate::tauri::commands::get_theme,
            crate::tauri::commands::set_theme,
            crate::tauri::commands::get_panel_ratio,
            crate::tauri::commands::set_panel_ratio,
            crate::tauri::commands::get_focus_mode,
            crate::tauri::commands::set_focus_mode,
            crate::tauri::commands::get_volume,
            crate::tauri::commands::set_volume_preference,
            crate::tauri::commands::get_speed,
            crate::tauri::commands::set_speed_preference,
            crate::tauri::commands::get_prefers_reduced_motion,
            crate::tauri::commands::set_prefers_reduced_motion,
            crate::tauri::commands::get_skip_forward_duration,
            crate::tauri::commands::set_skip_forward_duration,
            crate::tauri::commands::get_skip_backward_duration,
            crate::tauri::commands::set_skip_backward_duration,
            crate::tauri::commands::get_auto_save_interval,
            crate::tauri::commands::set_auto_save_interval,
        ])
        .run(tauri::generate_context!());

    if let Err(e) = result {
        tracing::error!("error while running tauri application: {e}");
        std::process::exit(1);
    }
    std::process::exit(0);
}
