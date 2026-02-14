use crate::conversions::{f64_to_ms, ms_to_f64, percentage_to_i32};
use crate::db::Database;
use crate::error::Result;
use crate::player::{PlaybackState, Vlc};
use crate::tasks::{convert_to_audiobooks, scan_directory, DiscoveredAudiobook};
use crate::ui::{Message, State};
use iced::Command;
use std::path::Path;

trait MediaControl {
    fn load_media(&mut self, path: &Path) -> Result<()>;
    fn set_time(&self, time_ms: i64) -> Result<()>;
    fn get_length(&self) -> Result<i64>;
    fn play(&self) -> Result<()>;
}

impl MediaControl for Vlc {
    fn load_media(&mut self, path: &Path) -> Result<()> {
        Self::load_media(self, path)
    }

    fn set_time(&self, time_ms: i64) -> Result<()> {
        Self::set_time(self, time_ms)
    }

    fn get_length(&self) -> Result<i64> {
        Self::get_length(self)
    }

    fn play(&self) -> Result<()> {
        Self::play(self)
    }
}

pub fn update(
    state: &mut State,
    message: Message,
    player: &mut Option<Vlc>,
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
        Message::AudiobookSelected(id) => handle_audiobook_selected(state, player, db, id),
        Message::FileSelected(path) => handle_file_selected(state, player, db, &path),

        // Directory management messages
        Message::DirectoryAdd => handle_directory_add(),
        Message::DirectoryAdded(path) => handle_directory_added(state, db, &path),
        Message::DirectoryRemove(path) => handle_directory_remove(state, player, db, &path),
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

fn handle_play_pause(state: &mut State, player: &mut Option<Vlc>) -> Command<Message> {
    if let Some(ref mut p) = player {
        if state.is_playing {
            if let Err(e) = p.pause() {
                tracing::error!("Failed to pause: {e}");
            } else {
                state.is_playing = false;
            }
        } else if let Err(e) = p.play() {
            tracing::error!("Failed to play: {e}");
        } else {
            state.is_playing = true;
        }
    }
    Command::none()
}

fn handle_stop(state: &mut State, player: &mut Option<Vlc>) -> Command<Message> {
    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop: {e}");
        }
        state.is_playing = false;
        state.current_time = 0.0;
    }
    Command::none()
}

fn reset_playback_state(state: &mut State, player: &mut Option<Vlc>) {
    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop: {e}");
        }
    }
    state.is_playing = false;
    state.current_time = 0.0;
    state.total_duration = 0.0;
}

fn handle_seek_to(state: &mut State, player: &mut Option<Vlc>, position: f64) -> Command<Message> {
    if let Some(ref mut p) = player {
        match f64_to_ms(position) {
            Ok(position_ms) => {
                if let Err(e) = p.set_time(position_ms) {
                    tracing::error!("Failed to seek: {e}");
                } else {
                    state.current_time = position;
                }
            }
            Err(e) => {
                tracing::error!("Invalid seek position: {e}");
            }
        }
    }
    Command::none()
}

fn handle_player_tick(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Command<Message> {
    if state.selected_file.is_none() {
        return Command::none();
    }

    let (time, player_state) = match player.as_ref() {
        Some(p) => {
            if state.total_duration <= 0.0 {
                if let Ok(duration_ms) = p.get_length() {
                    if duration_ms > 0 {
                        match ms_to_f64(duration_ms) {
                            Ok(duration_f64) => state.total_duration = duration_f64,
                            Err(e) => {
                                tracing::warn!(
                                    "Failed to convert media duration {duration_ms}ms to f64: {e}"
                                );
                            }
                        }
                    }
                }
            }

            let time = match p.get_time() {
                Ok(time) => time,
                Err(e) => {
                    tracing::warn!(
                        "Failed to read player time; skipping progress persist for this tick: {e}"
                    );
                    return Command::none();
                }
            };

            (time, p.get_state())
        }
        None => return Command::none(),
    };

    let command = handle_time_updated(state, db, time);

    if should_auto_advance(state, player_state, time) {
        return advance_to_next_file(state, player, db);
    }

    command
}

fn handle_volume_changed(
    state: &mut State,
    player: &mut Option<Vlc>,
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
    state: &mut State,
    player: &mut Option<Vlc>,
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

fn handle_audiobook_selected(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    id: i64,
) -> Command<Message> {
    let is_new_selection = state.selected_audiobook != Some(id);
    if is_new_selection {
        reset_playback_state(state, player);
        state.selected_file = None;
        state.current_files.clear();
    }

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

fn handle_file_selected<P: MediaControl>(
    state: &mut State,
    player: &mut Option<P>,
    db: &Database,
    path: &str,
) -> Command<Message> {
    fn persist_selected_file(state: &mut State, db: &Database, path: &str) {
        state.selected_file = Some(path.to_string());

        if let Some(id) = state.selected_audiobook {
            if let Err(e) =
                crate::db::queries::update_audiobook_selected_file(db.connection(), id, Some(path))
            {
                tracing::error!("Failed to save selected file: {e}");
            }
            return;
        }

        if let Ok(Some(file)) =
            crate::db::queries::get_audiobook_file_by_path(db.connection(), path)
        {
            if let Err(e) = crate::db::queries::update_audiobook_selected_file(
                db.connection(),
                file.audiobook_id,
                Some(path),
            ) {
                tracing::error!("Failed to save selected file: {e}");
            }
        }
    }

    fn restore_progress_and_duration<P: MediaControl>(
        state: &mut State,
        player: &P,
        db: &Database,
        path: &str,
    ) {
        if let Ok(Some(file)) =
            crate::db::queries::get_audiobook_file_by_path(db.connection(), path)
        {
            if let Some(length_ms) = file.length_of_file {
                if let Ok(duration_f64) = ms_to_f64(length_ms) {
                    state.total_duration = duration_f64;
                }
            }

            if let Some(seek_position) = file.seek_position {
                if let Err(e) = player.set_time(seek_position) {
                    tracing::error!("Failed to restore seek position: {e}");
                } else if let Ok(position_f64) = ms_to_f64(seek_position) {
                    state.current_time = position_f64;
                }
            } else {
                state.current_time = 0.0;
            }
        }

        if let Ok(duration) = player.get_length() {
            if duration > 0 {
                if let Ok(duration_f64) = ms_to_f64(duration) {
                    state.total_duration = duration_f64;
                }
            }
        }
    }

    match player.as_mut() {
        Some(p) => {
            if let Err(e) = p.load_media(Path::new(path)) {
                tracing::error!("Failed to load media: {e}");
                return Command::none();
            }

            persist_selected_file(state, db, path);
            restore_progress_and_duration(state, p, db, path);

            if let Err(e) = p.play() {
                tracing::error!("Failed to auto-play: {e}");
            } else {
                state.is_playing = true;
            }
        }
        None => {
            persist_selected_file(state, db, path);
        }
    }

    Command::none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::error::Error;
    use crate::models::{Audiobook, AudiobookFile};

    #[derive(Default)]
    struct FailingLoadPlayer;

    impl MediaControl for FailingLoadPlayer {
        fn load_media(&mut self, _path: &Path) -> Result<()> {
            Err(Error::Vlc("load failed".to_string()))
        }

        fn set_time(&self, _time_ms: i64) -> Result<()> {
            Ok(())
        }

        fn get_length(&self) -> Result<i64> {
            Ok(0)
        }

        fn play(&self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_handle_file_selected_does_not_change_selection_or_db_on_load_failure() {
        let db = Database::new_in_memory().expect("db in-memory");
        db::initialize(db.connection()).expect("db schema init");

        let mut audiobook = Audiobook::new(
            "/dir".to_string(),
            "Test".to_string(),
            "/dir/book".to_string(),
            0,
        );
        let old_path = "/dir/book/old.mp3";
        audiobook.selected_file = Some(old_path.to_string());
        let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)
            .expect("insert audiobook");

        let old_file = AudiobookFile::new(audiobook_id, "old".to_string(), old_path.to_string(), 0);
        crate::db::queries::insert_audiobook_file(db.connection(), &old_file)
            .expect("insert old file");

        let new_path = "/dir/book/new.mp3";
        let new_file = AudiobookFile::new(audiobook_id, "new".to_string(), new_path.to_string(), 1);
        crate::db::queries::insert_audiobook_file(db.connection(), &new_file)
            .expect("insert new file");

        let mut state = State::default();
        state.selected_audiobook = Some(audiobook_id);
        state.selected_file = Some(old_path.to_string());

        let mut player = Some(FailingLoadPlayer::default());

        let _cmd = handle_file_selected(&mut state, &mut player, &db, new_path);

        assert_eq!(
            state.selected_file.as_deref(),
            Some(old_path),
            "selection should remain unchanged when load fails"
        );

        let saved = crate::db::queries::get_audiobook_by_id(db.connection(), audiobook_id)
            .expect("get audiobook")
            .expect("audiobook exists");
        assert_eq!(
            saved.selected_file.as_deref(),
            Some(old_path),
            "db selected_file should not change when load fails"
        );
    }
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

fn handle_directory_added(state: &mut State, db: &Database, path: &str) -> Command<Message> {
    if state.directories.iter().any(|d| d.full_path == path) {
        tracing::info!("Directory already added: {path}. Skipping.");
        return Command::none();
    }

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

fn handle_directory_remove(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    path: &str,
) -> Command<Message> {
    let selected_audiobook_in_directory = state
        .selected_audiobook
        .and_then(|id| state.audiobooks.iter().find(|a| a.id == Some(id)))
        .is_some_and(|audiobook| audiobook.directory == path);

    let selected_file_in_directory = state
        .selected_file
        .as_ref()
        .is_some_and(|file_path| std::path::Path::new(file_path).starts_with(path));

    let current_files_in_directory = state
        .current_files
        .iter()
        .any(|file| std::path::Path::new(&file.full_path).starts_with(path));

    let should_clear_selection =
        selected_audiobook_in_directory || selected_file_in_directory || current_files_in_directory;

    if let Err(e) = crate::db::queries::delete_directory(db.connection(), path) {
        tracing::error!("Failed to delete directory: {e}");
    } else {
        state.directories.retain(|d| d.full_path != path);
        state.audiobooks.retain(|a| a.directory != path);
        if should_clear_selection {
            reset_playback_state(state, player);
            state.selected_audiobook = None;
            state.selected_file = None;
            state.current_files.clear();
            if let Err(e) =
                crate::db::queries::delete_metadata(db.connection(), "current_audiobook_id")
            {
                tracing::error!("Failed to clear current audiobook metadata: {e}");
            }
        }
    }
    Command::none()
}

fn handle_directory_rescan(_state: &mut State, db: &Database, path: &str) -> Command<Message> {
    tracing::info!("Directory rescan requested: {path}");
    if let Ok(audiobooks) = crate::db::queries::get_audiobooks_by_directory(db.connection(), path) {
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

fn handle_open_settings(state: &mut State) -> Command<Message> {
    state.settings_open = true;
    Command::none()
}

fn handle_close_settings(state: &mut State) -> Command<Message> {
    state.settings_open = false;
    Command::none()
}

fn handle_scan_complete(
    state: &mut State,
    db: &Database,
    directory: &str,
    discovered: Vec<DiscoveredAudiobook>,
) -> Command<Message> {
    let audiobooks = convert_to_audiobooks(discovered.clone(), directory);

    update_state_with_discovered_audiobooks(state, &audiobooks);

    for (idx, disc) in discovered.into_iter().enumerate() {
        process_discovered_audiobook(state, db, directory, &audiobooks, idx, disc);
    }

    finalize_directory_scan(state, db, directory);

    Command::none()
}

fn update_state_with_discovered_audiobooks(
    state: &mut State,
    audiobooks: &[crate::models::Audiobook],
) {
    for audiobook in audiobooks {
        let exists = state
            .audiobooks
            .iter()
            .any(|a| a.full_path == audiobook.full_path);

        if !exists {
            state.audiobooks.push(audiobook.clone());
        }
    }
}

fn process_discovered_audiobook(
    state: &mut State,
    db: &Database,
    directory: &str,
    audiobooks: &[crate::models::Audiobook],
    idx: usize,
    disc: DiscoveredAudiobook,
) {
    let disc_path = disc.path.display().to_string();
    let mut resolved_audiobook = audiobooks.get(idx).cloned().unwrap_or_else(|| {
        crate::models::Audiobook::new(
            directory.to_string(),
            disc.name.clone(),
            disc_path.clone(),
            i32::try_from(idx).unwrap_or(i32::MAX),
        )
    });

    let Some(audiobook_id) = resolve_audiobook_id(db, &disc_path, &mut resolved_audiobook) else {
        return;
    };

    update_state_audiobook_id(state, &resolved_audiobook, audiobook_id);
    process_audiobook_files(db, audiobook_id, disc.files);
}

fn resolve_audiobook_id(
    db: &Database,
    disc_path: &str,
    resolved_audiobook: &mut crate::models::Audiobook,
) -> Option<i64> {
    let existing = match crate::db::queries::get_audiobook_by_path(db.connection(), disc_path) {
        Ok(existing) => existing,
        Err(e) => {
            tracing::error!("Failed to lookup audiobook by path {disc_path}: {e}");
            return None;
        }
    };

    let audiobook_id = if let Some(existing_ab) = existing {
        if let Some(id) = existing_ab.id {
            resolved_audiobook.id = Some(id);
            id
        } else if let Ok(id) =
            crate::db::queries::insert_audiobook(db.connection(), resolved_audiobook)
        {
            resolved_audiobook.id = Some(id);
            id
        } else {
            return None;
        }
    } else {
        match crate::db::queries::insert_audiobook(db.connection(), resolved_audiobook) {
            Ok(id) => {
                resolved_audiobook.id = Some(id);
                id
            }
            Err(e) => {
                tracing::error!("Failed to insert audiobook: {e}");
                return None;
            }
        }
    };

    Some(audiobook_id)
}

fn update_state_audiobook_id(
    state: &mut State,
    resolved_audiobook: &crate::models::Audiobook,
    audiobook_id: i64,
) {
    if let Some(entry) = state
        .audiobooks
        .iter_mut()
        .find(|a| a.full_path == resolved_audiobook.full_path)
    {
        entry.id = Some(audiobook_id);
    } else {
        let mut ab = resolved_audiobook.clone();
        ab.id = Some(audiobook_id);
        state.audiobooks.push(ab);
    }
}

fn process_audiobook_files(db: &Database, audiobook_id: i64, files: Vec<std::path::PathBuf>) {
    let mut sorted_files = files;
    sorted_files.sort_by(|a, b| {
        let a_name = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let b_name = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        natord::compare(a_name, b_name)
    });

    for (pos, file_path) in sorted_files.iter().enumerate() {
        process_single_file(db, audiobook_id, pos, file_path);
    }
}

fn process_single_file(db: &Database, audiobook_id: i64, pos: usize, file_path: &std::path::Path) {
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

fn finalize_directory_scan(state: &mut State, db: &Database, directory: &str) {
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
}

fn handle_scan_error(error: &str) -> Command<Message> {
    tracing::error!("Scan error: {error}");
    Command::none()
}

fn handle_initial_load_complete(state: &mut State) -> Command<Message> {
    state.is_loading = false;
    Command::none()
}

fn handle_time_updated(state: &mut State, db: &Database, time: f64) -> Command<Message> {
    state.current_time = time;

    if let Some(ref file_path) = state.selected_file {
        let file_path_owned = file_path.clone();

        if state.total_duration > 0.0 {
            let percentage = (time * 100.0) / state.total_duration;
            let completeness = percentage_to_i32(percentage);

            if let Err(e) = crate::db::queries::update_file_progress(
                db.connection(),
                &file_path_owned,
                time,
                completeness,
            ) {
                tracing::error!("Failed to update file progress: {e}");
            } else {
                update_current_file_progress(state, &file_path_owned, time, completeness);
                update_audiobook_completeness_after_file_change(state, db, &file_path_owned);
            }
        }
    }

    Command::none()
}

fn should_auto_advance(state: &State, player_state: PlaybackState, time: f64) -> bool {
    if state.selected_file.is_none() {
        return false;
    }

    if player_state == PlaybackState::Ended {
        return true;
    }

    if !state.is_playing || state.total_duration <= 0.0 {
        return false;
    }

    let end_threshold_ms = 500.0;
    time + end_threshold_ms >= state.total_duration
}

fn advance_to_next_file(
    state: &mut State,
    player: &mut Option<Vlc>,
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

fn mark_current_file_complete(state: &mut State, db: &Database, file_path: &str) {
    let final_time = if state.total_duration > 0.0 {
        state.total_duration
    } else {
        state.current_time
    };

    if let Err(e) =
        crate::db::queries::update_file_progress(db.connection(), file_path, final_time, 100)
    {
        tracing::error!("Failed to mark file complete: {e}");
    }

    update_current_file_progress(state, file_path, final_time, 100);
    update_audiobook_completeness_after_file_change(state, db, file_path);
}

fn start_directory_scan(path: String) -> Command<Message> {
    Command::perform(
        async move {
            scan_directory(path.clone().into())
                .await
                .map(|discovered| (path, discovered))
        },
        |result| match result {
            Ok((path, discovered)) => Message::ScanComplete(path, discovered),
            Err(error) => Message::ScanError(error.to_string()),
        },
    )
}

fn update_current_file_progress(
    state: &mut State,
    file_path: &str,
    seek_position: f64,
    completeness: i32,
) {
    if let Some(file) = state
        .current_files
        .iter_mut()
        .find(|file| file.full_path == file_path)
    {
        file.completeness = completeness;
        if let Ok(position_ms) = f64_to_ms(seek_position) {
            file.seek_position = Some(position_ms);
        }
    }
}

fn update_audiobook_completeness_after_file_change(
    state: &mut State,
    db: &Database,
    file_path: &str,
) {
    let audiobook_id = state.selected_audiobook.or_else(|| {
        crate::db::queries::get_audiobook_file_by_path(db.connection(), file_path)
            .ok()
            .flatten()
            .map(|file| file.audiobook_id)
    });

    if let Some(id) = audiobook_id {
        recompute_audiobook_completeness(state, db, id);
    }
}

fn recompute_audiobook_completeness(state: &mut State, db: &Database, audiobook_id: i64) {
    let (total, count) = if state.selected_audiobook == Some(audiobook_id)
        && !state.current_files.is_empty()
    {
        let total: i32 = state
            .current_files
            .iter()
            .map(|file| file.completeness)
            .sum();
        let Ok(count) = i32::try_from(state.current_files.len()) else {
            tracing::error!("Failed to convert file count for audiobook {audiobook_id}");
            return;
        };
        (total, count)
    } else {
        match crate::db::queries::get_audiobook_files(db.connection(), audiobook_id) {
            Ok(files) => {
                if files.is_empty() {
                    return;
                }
                let total: i32 = files.iter().map(|file| file.completeness).sum();
                let Ok(count) = i32::try_from(files.len()) else {
                    tracing::error!("Failed to convert file count for audiobook {audiobook_id}");
                    return;
                };
                (total, count)
            }
            Err(e) => {
                tracing::error!("Failed to load audiobook files for completeness: {e}");
                return;
            }
        }
    };

    if count <= 0 {
        return;
    }

    let avg = total / count;

    if let Some(entry) = state
        .audiobooks
        .iter_mut()
        .find(|audiobook| audiobook.id == Some(audiobook_id))
    {
        entry.completeness = avg;
    }

    if let Err(e) =
        crate::db::queries::update_audiobook_completeness(db.connection(), audiobook_id, avg)
    {
        tracing::error!("Failed to update audiobook completeness: {e}");
    }
}
