use crate::db::Database;
use crate::player::{ConcretePlayer, PlayerState};
use crate::tasks::{convert_to_audiobooks, scan_directory, DiscoveredAudiobook};
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
        Message::PlayerTick => handle_player_tick(state, player, db),

        // Selection messages
        Message::AudiobookSelected(id) => handle_audiobook_selected(state, db, id),
        Message::FileSelected(path) => handle_file_selected(state, player, db, &path),

        // Directory management messages
        Message::DirectoryAdd => handle_directory_add(),
        Message::DirectoryAdded(path) => handle_directory_added(state, db, &path),
        Message::DirectoryRemove(path) => handle_directory_remove(state, db, &path),
        Message::DirectoryRescan(path) => handle_directory_rescan(state, db, &path),

        // Settings messages
        Message::OpenSettings => handle_open_settings(state),
        Message::CloseSettings => handle_close_settings(state),

        // Scan messages
        Message::ScanComplete(directory, discovered) => {
            handle_scan_complete(state, db, &directory, discovered)
        }
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
        // Convert f64 to i64 for VLC API
        // Framework requirement: iced slider uses f64, VLC requires i64 milliseconds
        // Safe: Audiobook durations are typically <100 hours (<360,000,000 ms),
        // well within i64 range (±9.2×10^18). Rounding ensures no fractional loss.
        let position_ms = position.round() as i64;
        if let Err(e) = p.set_time(position_ms) {
            tracing::error!("Failed to seek: {e}");
        } else {
            state.current_time = position;
        }
    }
    Command::none()
}

fn handle_player_tick(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
    db: &Database,
) -> Command<Message> {
    if state.selected_file.is_none() {
        return Command::none();
    }

    let (time, player_state) = match player.as_ref() {
        Some(p) => (p.get_time(), p.get_state()),
        None => return Command::none(),
    };

    let command = handle_time_updated(state, db, time);

    if should_auto_advance(state, player_state, time) {
        return advance_to_next_file(state, player, db);
    }

    command
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
    db: &Database,
    path: &str,
) -> Command<Message> {
    state.selected_file = Some(path.to_string());

    if let Some(id) = state.selected_audiobook {
        if let Err(e) = crate::db::queries::update_audiobook_selected_file(
            db.connection(),
            id,
            Some(path),
        ) {
            tracing::error!("Failed to save selected file: {e}");
        }
    } else if let Ok(Some(file)) = crate::db::queries::get_audiobook_file_by_path(
        db.connection(),
        path,
    ) {
        if let Err(e) = crate::db::queries::update_audiobook_selected_file(
            db.connection(),
            file.audiobook_id,
            Some(path),
        ) {
            tracing::error!("Failed to save selected file: {e}");
        }
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.load_media(Path::new(&path)) {
            tracing::error!("Failed to load media: {e}");
        } else {
            if let Ok(Some(file)) = crate::db::queries::get_audiobook_file_by_path(
                db.connection(),
                path,
            ) {
                if let Some(length_ms) = file.length_of_file {
                    state.total_duration = length_ms as f64;
                }

                if let Some(seek_position) = file.seek_position {
                    if let Err(e) = p.set_time(seek_position) {
                        tracing::error!("Failed to restore seek position: {e}");
                    } else {
                        state.current_time = seek_position as f64;
                    }
                } else {
                    state.current_time = 0.0;
                }
            }

            if let Ok(duration) = p.get_length() {
                // Convert i64 to f64 for iced slider widget
                // Framework requirement: iced slider API requires f64 values
                // Safe: i64→f64 cast may lose precision for values >2^53,
                // but audiobook durations are typically <100 hours (<360,000,000 ms),
                // well within f64's precise integer range (2^53 ≈ 9×10^15)
                if duration > 0 {
                    state.total_duration = duration as f64;
                }
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
    tracing::info!("Directory added: {path}. Starting scan.");

    start_directory_scan(path.to_string())
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

fn handle_directory_rescan(
    _state: &mut NodokaState,
    db: &Database,
    path: &str,
) -> Command<Message> {
    tracing::info!("Directory rescan requested: {path}");
    if let Ok(audiobooks) = crate::db::queries::get_audiobooks_by_directory(db.connection(), path)
    {
        for audiobook in audiobooks {
            if let Some(id) = audiobook.id {
                if let Err(e) =
                    crate::db::queries::mark_audiobook_files_missing(db.connection(), id)
                {
                    tracing::error!("Failed to mark audiobook files missing: {e}");
                }
            }
        }
    }
    start_directory_scan(path.to_string())
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
    db: &Database,
    directory: &str,
    discovered: Vec<DiscoveredAudiobook>,
) -> Command<Message> {
    let audiobooks = convert_to_audiobooks(discovered.clone(), directory);

    for audiobook in &audiobooks {
        let exists = state
            .audiobooks
            .iter()
            .any(|a| a.full_path == audiobook.full_path);

        if !exists {
            state.audiobooks.push(audiobook.clone());
        }
    }

    for (idx, disc) in discovered.into_iter().enumerate() {
        let disc_path = disc.path.display().to_string();
        let mut resolved_audiobook = audiobooks
            .get(idx)
            .cloned()
            .unwrap_or_else(|| crate::models::Audiobook::new(
                directory.to_string(),
                disc.name.clone(),
                disc_path.clone(),
                i32::try_from(idx).unwrap_or(i32::MAX),
            ));
        let existing = crate::db::queries::get_audiobook_by_path(db.connection(), &disc_path)
            .unwrap_or(None);

        let audiobook_id = if let Some(existing_ab) = existing {
            if let Some(id) = existing_ab.id {
                resolved_audiobook.id = Some(id);
                id
            } else if let Ok(id) = crate::db::queries::insert_audiobook(
                db.connection(),
                &resolved_audiobook,
            ) {
                resolved_audiobook.id = Some(id);
                id
            } else {
                0
            }
        } else {
            match crate::db::queries::insert_audiobook(db.connection(), &resolved_audiobook) {
                Ok(id) => {
                    resolved_audiobook.id = Some(id);
                    id
                }
                Err(e) => {
                    tracing::error!("Failed to insert audiobook: {e}");
                    0
                }
            }
        };

        if audiobook_id == 0 {
            continue;
        }

        if let Some(entry) = state
            .audiobooks
            .iter_mut()
            .find(|a| a.full_path == resolved_audiobook.full_path)
        {
            entry.id = Some(audiobook_id);
        } else {
            resolved_audiobook.id = Some(audiobook_id);
            state.audiobooks.push(resolved_audiobook);
        }

        let mut files = disc.files;
        files.sort_by(|a, b| {
            a.file_name()
                .and_then(|n| n.to_str())
                .cmp(&b.file_name().and_then(|n| n.to_str()))
        });

        for (pos, file_path) in files.into_iter().enumerate() {
            let name = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let full_path = file_path.display().to_string();
            let mut file = crate::models::AudiobookFile::new(
                audiobook_id,
                name,
                full_path.clone(),
                i32::try_from(pos).unwrap_or(i32::MAX),
            );

            if let Ok(Some(existing_file)) =
                crate::db::queries::get_audiobook_file_by_path(db.connection(), &full_path)
            {
                file.length_of_file = existing_file.length_of_file;
                file.seek_position = existing_file.seek_position;
                file.completeness = existing_file.completeness;
                file.file_exists = true;
            }

            if let Err(e) = crate::db::queries::insert_audiobook_file(db.connection(), &file) {
                tracing::error!("Failed to insert audiobook file: {e}");
            }
        }
    }

    if let Err(e) = crate::db::queries::update_directory_last_scanned(db.connection(), directory) {
        tracing::error!("Failed to update directory scan timestamp: {e}");
    }

    if let Some(dir) = state
        .directories
        .iter_mut()
        .find(|d| d.full_path == directory)
    {
        dir.last_scanned = Some(chrono::Utc::now());
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
            // Convert f64 percentage to i32 for database storage
            // Safe: percentage is clamped to 0.0-100.0 range before conversion,
            // so result is always in valid i32 range (0-100)
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

fn should_auto_advance(state: &NodokaState, player_state: PlayerState, time: f64) -> bool {
    if state.selected_file.is_none() {
        return false;
    }

    if player_state == PlayerState::Ended {
        return true;
    }

    if !state.is_playing || state.total_duration <= 0.0 {
        return false;
    }

    let end_threshold_ms = 500.0;
    time + end_threshold_ms >= state.total_duration
}

fn advance_to_next_file(
    state: &mut NodokaState,
    player: &mut Option<ConcretePlayer>,
    db: &Database,
) -> Command<Message> {
    let Some(current_path) = state.selected_file.clone() else {
        return Command::none();
    };

    mark_current_file_complete(state, db, &current_path);

    let next_path = state
        .current_files
        .iter()
        .position(|file| file.full_path == current_path)
        .and_then(|idx| state.current_files.get(idx + 1))
        .map(|file| file.full_path.clone());

    if let Some(next_path) = next_path {
        return handle_file_selected(state, player, db, &next_path);
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop after final file: {e}");
        }
    }
    state.is_playing = false;
    state.current_time = state.total_duration;
    Command::none()
}

fn mark_current_file_complete(state: &mut NodokaState, db: &Database, file_path: &str) {
    let final_time = if state.total_duration > 0.0 {
        state.total_duration
    } else {
        state.current_time
    };

    if let Err(e) = crate::db::queries::update_file_progress(db.connection(), file_path, final_time, 100)
    {
        tracing::error!("Failed to mark file complete: {e}");
    }

    if let Some(file) = state
        .current_files
        .iter_mut()
        .find(|file| file.full_path == file_path)
    {
        file.completeness = 100;
        file.seek_position = Some(final_time.round() as i64);
    }
}

fn start_directory_scan(path: String) -> Command<Message> {
    Command::perform(
        async move {
            scan_directory(path.clone())
                .await
                .map(|discovered| (path, discovered))
        },
        |result| match result {
            Ok((path, discovered)) => Message::ScanComplete(path, discovered),
            Err(error) => Message::ScanError(error.to_string()),
        },
    )
}
