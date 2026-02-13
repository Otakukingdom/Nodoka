use crate::db::Database;
use crate::player::ConcretePlayer;
use crate::ui::{Message, NodokaState};
use iced::Command;
use std::path::Path;

pub fn update(
    state: &mut NodokaState,
    message: Message,
    player: &mut Option<ConcretePlayer>,
    db: &Database,
) -> Command<Message> {
    match message {
        // Player control messages
        Message::PlayPause => handle_play_pause(state, player),
        Message::Stop => handle_stop(state, player),
        Message::SeekTo(position) => handle_seek_to(state, player, position),
        Message::VolumeChanged(volume) => handle_volume_changed(state, player, db, volume),
        Message::SpeedChanged(speed) => handle_speed_changed(state, player, db, speed),
        Message::PlayerTimeUpdated(time) => handle_time_updated(state, db, time),

        // Selection messages
        Message::AudiobookSelected(id) => handle_audiobook_selected(state, db, id),
        Message::FileSelected(path) => handle_file_selected(state, player, &path),

        // Directory management messages
        Message::DirectoryAdd => handle_directory_add(),
        Message::DirectoryAdded(path) => handle_directory_added(state, db, &path),
        Message::DirectoryRemove(path) => handle_directory_remove(state, db, &path),
        Message::DirectoryRescan(_path) => handle_directory_rescan(),

        // Settings messages
        Message::OpenSettings => handle_open_settings(state),
        Message::CloseSettings => handle_close_settings(state),

        // Scan messages
        Message::ScanComplete(new_audiobooks) => handle_scan_complete(state, new_audiobooks),
        Message::ScanError(error) => handle_scan_error(&error),

        // Lifecycle messages
        Message::InitialLoadComplete => handle_initial_load_complete(state),

        // Catch-all for unhandled messages
        _ => Command::none(),
    }
}

fn handle_play_pause(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
) -> Command<Message> {
    if let Some(ref mut p) = player {
        if state.is_playing {
            if let Err(e) = p.pause() {
                tracing::error!("Failed to pause: {e}");
            }
        } else if let Err(e) = p.play() {
            tracing::error!("Failed to play: {e}");
        }
        state.is_playing = !state.is_playing;
    }
    Command::none()
}

fn handle_stop(state: &mut NodokaState, player: &mut Option<ConcretePlayer>) -> Command<Message> {
    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop: {e}");
        }
        state.is_playing = false;
        state.current_time = 0.0;
    }
    Command::none()
}

fn handle_seek_to(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
    position: f64,
) -> Command<Message> {
    if let Some(ref mut p) = player {
        let position_ms = position.round() as i64;
        if let Err(e) = p.set_time(position_ms) {
            tracing::error!("Failed to seek: {e}");
        } else {
            state.current_time = position;
        }
    }
    Command::none()
}

fn handle_volume_changed(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
    db: &Database,
    volume: i32,
) -> Command<Message> {
    if let Some(ref mut p) = player {
        if let Err(e) = p.set_volume(volume) {
            tracing::error!("Failed to set volume: {e}");
        }
        state.volume = volume;

        if let Err(e) =
            crate::db::queries::set_metadata(db.connection(), "volume", &volume.to_string())
        {
            tracing::error!("Failed to save volume setting: {e}");
        }
    }
    Command::none()
}

fn handle_speed_changed(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
    db: &Database,
    speed: f32,
) -> Command<Message> {
    if let Some(ref mut p) = player {
        if let Err(e) = p.set_rate(speed) {
            tracing::error!("Failed to set speed: {e}");
        }
        state.speed = speed;

        if let Err(e) =
            crate::db::queries::set_metadata(db.connection(), "speed", &speed.to_string())
        {
            tracing::error!("Failed to save speed setting: {e}");
        }
    }
    Command::none()
}

fn handle_audiobook_selected(state: &mut NodokaState, db: &Database, id: i64) -> Command<Message> {
    state.selected_audiobook = Some(id);

    match crate::db::queries::get_audiobook_files(db.connection(), id) {
        Ok(files) => {
            state.current_files = files;
        }
        Err(e) => {
            tracing::error!("Failed to load audiobook files: {e}");
        }
    }

    if let Err(e) =
        crate::db::queries::set_metadata(db.connection(), "current_audiobook_id", &id.to_string())
    {
        tracing::error!("Failed to save current audiobook: {e}");
    }

    Command::none()
}

fn handle_file_selected(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
    path: &str,
) -> Command<Message> {
    state.selected_file = Some(path.to_string());

    if let Some(ref mut p) = player {
        if let Err(e) = p.load_media(Path::new(&path)) {
            tracing::error!("Failed to load media: {e}");
        } else {
            if let Ok(duration) = p.get_length() {
                // VLC returns i64 milliseconds, convert to f64 for UI state
                // Precision loss only occurs for durations > 285 million years
                state.total_duration = duration as f64;
            }

            if let Err(e) = p.play() {
                tracing::error!("Failed to auto-play: {e}");
            } else {
                state.is_playing = true;
            }
        }
    }

    Command::none()
}

fn handle_directory_add() -> Command<Message> {
    Command::perform(
        async {
            rfd::AsyncFileDialog::new()
                .pick_folder()
                .await
                .map(|handle| handle.path().to_string_lossy().to_string())
        },
        |result| result.map_or(Message::DirectoryAddCancelled, Message::DirectoryAdded),
    )
}

fn handle_directory_added(state: &mut NodokaState, db: &Database, path: &str) -> Command<Message> {
    let directory = crate::models::Directory {
        full_path: path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    if let Err(e) = crate::db::queries::insert_directory(db.connection(), &directory) {
        tracing::error!("Failed to insert directory: {e}");
        return Command::none();
    }

    state.directories.push(directory);
    tracing::info!("Directory added: {path}. Manual rescan recommended.");

    Command::none()
}

fn handle_directory_remove(state: &mut NodokaState, db: &Database, path: &str) -> Command<Message> {
    if let Err(e) = crate::db::queries::delete_directory(db.connection(), path) {
        tracing::error!("Failed to delete directory: {e}");
    } else {
        state.directories.retain(|d| d.full_path != path);
        state.audiobooks.retain(|a| a.directory != path);
    }
    Command::none()
}

fn handle_directory_rescan() -> Command<Message> {
    tracing::info!("Directory rescan requested but not implemented in this version");
    Command::none()
}

fn handle_open_settings(state: &mut NodokaState) -> Command<Message> {
    state.settings_open = true;
    Command::none()
}

fn handle_close_settings(state: &mut NodokaState) -> Command<Message> {
    state.settings_open = false;
    Command::none()
}

fn handle_scan_complete(
    state: &mut NodokaState,
    new_audiobooks: Vec<crate::models::Audiobook>,
) -> Command<Message> {
    for audiobook in new_audiobooks {
        let exists = state
            .audiobooks
            .iter()
            .any(|a| a.full_path == audiobook.full_path);

        if !exists {
            state.audiobooks.push(audiobook);
        }
    }
    Command::none()
}

fn handle_scan_error(error: &str) -> Command<Message> {
    tracing::error!("Scan error: {error}");
    Command::none()
}

fn handle_initial_load_complete(state: &mut NodokaState) -> Command<Message> {
    state.is_loading = false;
    Command::none()
}

fn handle_time_updated(state: &mut NodokaState, db: &Database, time: f64) -> Command<Message> {
    state.current_time = time;

    if let Some(ref file_path) = state.selected_file {
        if state.total_duration > 0.0 {
            let percentage = (time * 100.0) / state.total_duration;
            let completeness = percentage.round().clamp(0.0, 100.0) as i32;

            if let Err(e) = crate::db::queries::update_file_progress(
                db.connection(),
                file_path,
                time,
                completeness,
            ) {
                tracing::error!("Failed to update file progress: {e}");
            }
        }
    }

    Command::none()
}
