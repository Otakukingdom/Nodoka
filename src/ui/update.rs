use crate::conversions::{f64_to_ms, ms_to_f64, percentage_to_i32};
use crate::db::Database;
use crate::error::Result;
use crate::models::Bookmark;
use crate::player::{PlaybackState, Vlc};
use crate::tasks::{
    cleanup_temp_files, materialize_zip_virtual_path, parse_zip_virtual_path, zip_temp_dir,
};
use crate::ui::{Message, State};
use iced::Command;
use std::path::Path;

mod directories;

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

        // Shortcut actions
        Message::CreateBookmark => handle_create_bookmark(state, db),

        // Bookmarks UI
        Message::BookmarkEdit(id) => handle_bookmark_edit(state, id),
        Message::BookmarkDelete(id) => handle_bookmark_delete(state, db, id),
        Message::BookmarkJump(id) => handle_bookmark_jump(state, player, db, id),
        Message::BookmarkEditorLabelChanged(value) => {
            handle_bookmark_editor_label_changed(state, &value)
        }
        Message::BookmarkEditorNoteChanged(value) => {
            handle_bookmark_editor_note_changed(state, &value)
        }
        Message::BookmarkEditorSave => handle_bookmark_editor_save(state, db),
        Message::BookmarkEditorCancel => handle_bookmark_editor_cancel(state),

        // Selection messages
        Message::AudiobookSelected(id) => handle_audiobook_selected(state, player, db, id),
        Message::FileSelected(path) => handle_file_selected(state, player, db, &path),

        // Directory management messages
        Message::DirectoryAdd => directories::handle_directory_add(),
        Message::DirectoryAdded(path) => directories::handle_directory_added(state, db, &path),
        Message::DirectoryRemove(path) => {
            directories::handle_directory_remove(state, player, db, &path)
        }
        Message::DirectoryRescan(path) => directories::handle_directory_rescan(state, db, &path),

        // Settings messages
        Message::OpenSettings => handle_open_settings(state),
        Message::CloseSettings => handle_close_settings(state),

        // Scan messages
        Message::ScanComplete(directory, discovered) => {
            directories::handle_scan_complete(state, db, &directory, discovered)
        }
        Message::ScanError(error) => directories::handle_scan_error(&error),

        // Lifecycle messages
        Message::InitialLoadComplete => handle_initial_load_complete(state),

        // Catch-all for unhandled messages
        _ => Command::none(),
    }
}

fn handle_create_bookmark(state: &mut State, db: &Database) -> Command<Message> {
    let (Some(audiobook_id), Some(file_path)) =
        (state.selected_audiobook, state.selected_file.clone())
    else {
        return Command::none();
    };

    let position_ms = match f64_to_ms(state.current_time) {
        Ok(ms) => ms,
        Err(e) => {
            tracing::error!("Failed to convert bookmark position: {e}");
            return Command::none();
        }
    };

    let bookmark = Bookmark::new(
        audiobook_id,
        file_path.clone(),
        position_ms,
        String::from("Bookmark"),
    );

    let created_id = match crate::db::queries::insert_bookmark(db.connection(), &bookmark) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to create bookmark: {e}");
            return Command::none();
        }
    };

    if let Ok(bookmarks) =
        crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)
    {
        state.bookmarks = bookmarks;
    }

    // Open editor to allow immediate label/note customization.
    state.bookmark_editor = Some(crate::ui::state::BookmarkEditor {
        id: Some(created_id),
        audiobook_id,
        file_path,
        position_ms,
        label: String::from("Bookmark"),
        note: String::new(),
    });

    Command::none()
}

fn handle_bookmark_editor_label_changed(state: &mut State, value: &str) -> Command<Message> {
    if let Some(editor) = state.bookmark_editor.as_mut() {
        editor.label = value.to_string();
    }
    Command::none()
}

fn handle_bookmark_editor_note_changed(state: &mut State, value: &str) -> Command<Message> {
    if let Some(editor) = state.bookmark_editor.as_mut() {
        editor.note = value.to_string();
    }
    Command::none()
}

fn handle_bookmark_editor_cancel(state: &mut State) -> Command<Message> {
    state.bookmark_editor = None;
    Command::none()
}

fn handle_bookmark_editor_save(state: &mut State, db: &Database) -> Command<Message> {
    let Some(editor) = state.bookmark_editor.clone() else {
        return Command::none();
    };

    let label = if editor.label.trim().is_empty() {
        String::from("Bookmark")
    } else {
        editor.label.clone()
    };

    let note = if editor.note.trim().is_empty() {
        None
    } else {
        Some(editor.note.clone())
    };

    if let Some(id) = editor.id {
        let Some(existing) = state.bookmarks.iter().find(|b| b.id == Some(id)).cloned() else {
            tracing::warn!("Bookmark {id} not found in state; cannot edit");
            return Command::none();
        };

        let updated = Bookmark {
            id: Some(id),
            audiobook_id: existing.audiobook_id,
            file_path: existing.file_path,
            position_ms: existing.position_ms,
            label,
            note,
            created_at: existing.created_at,
        };

        if let Err(e) = crate::db::queries::update_bookmark(db.connection(), &updated) {
            tracing::error!("Failed to update bookmark: {e}");
            return Command::none();
        }
    } else {
        let mut bookmark = Bookmark::new(
            editor.audiobook_id,
            editor.file_path,
            editor.position_ms,
            label,
        );
        bookmark.note = note;

        if let Err(e) = crate::db::queries::insert_bookmark(db.connection(), &bookmark) {
            tracing::error!("Failed to create bookmark: {e}");
            return Command::none();
        }
    }

    match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), editor.audiobook_id) {
        Ok(bms) => state.bookmarks = bms,
        Err(e) => tracing::error!("Failed to reload bookmarks: {e}"),
    }

    state.bookmark_editor = None;

    Command::none()
}

fn handle_bookmark_edit(state: &mut State, id: i64) -> Command<Message> {
    let Some(bm) = state.bookmarks.iter().find(|b| b.id == Some(id)).cloned() else {
        tracing::warn!("Bookmark {id} not found for edit");
        return Command::none();
    };

    state.bookmark_editor = Some(crate::ui::state::BookmarkEditor {
        id: Some(id),
        audiobook_id: bm.audiobook_id,
        file_path: bm.file_path,
        position_ms: bm.position_ms,
        label: bm.label,
        note: bm.note.unwrap_or_default(),
    });

    Command::none()
}

fn handle_bookmark_delete(state: &mut State, db: &Database, id: i64) -> Command<Message> {
    let audiobook_id = state.selected_audiobook;
    if let Err(e) = crate::db::queries::delete_bookmark(db.connection(), id) {
        tracing::error!("Failed to delete bookmark: {e}");
        return Command::none();
    }

    if let Some(audiobook_id) = audiobook_id {
        match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id) {
            Ok(bms) => state.bookmarks = bms,
            Err(e) => tracing::error!("Failed to reload bookmarks: {e}"),
        }
    } else {
        state.bookmarks.clear();
    }

    Command::none()
}

fn handle_bookmark_jump(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    id: i64,
) -> Command<Message> {
    let Some(bm) = state.bookmarks.iter().find(|b| b.id == Some(id)).cloned() else {
        tracing::warn!("Bookmark {id} not found for jump");
        return Command::none();
    };

    let cmd = handle_file_selected(state, player, db, &bm.file_path);

    if let Some(ref p) = player {
        if let Err(e) = p.set_time(bm.position_ms) {
            tracing::error!("Failed to seek to bookmark: {e}");
        } else if let Ok(pos) = ms_to_f64(bm.position_ms) {
            state.current_time = pos;
        }
    }

    cmd
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

    if let Some(selected) = state.selected_file.as_deref() {
        if let Some((zip_path, _entry)) = parse_zip_virtual_path(selected) {
            if let Ok(dir) = zip_temp_dir(&zip_path) {
                if let Err(e) = cleanup_temp_files(&dir) {
                    tracing::warn!("Failed to cleanup ZIP temp dir {}: {e}", dir.display());
                }
            }
        }
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
    let old_selection = state.selected_audiobook;
    let is_new_selection = old_selection != Some(id);
    if is_new_selection {
        if let Some(old_id) = old_selection {
            if let Some(old_ab) = state.audiobooks.iter().find(|a| a.id == Some(old_id)) {
                super::media_paths::cleanup_zip_temp_for_path(&old_ab.full_path);
            }
        }
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

    match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), id) {
        Ok(bookmarks) => {
            state.bookmarks = bookmarks;
        }
        Err(e) => {
            tracing::error!("Failed to load bookmarks: {e}");
        }
    }
    state.bookmark_editor = None;

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
            let playback_path = if parse_zip_virtual_path(path).is_some() {
                match materialize_zip_virtual_path(path) {
                    Ok(p) => p,
                    Err(e) => {
                        tracing::error!("Failed to extract ZIP entry for playback: {e}");
                        return Command::none();
                    }
                }
            } else {
                std::path::PathBuf::from(path)
            };

            if let Err(e) = p.load_media(&playback_path) {
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

fn handle_open_settings(state: &mut State) -> Command<Message> {
    state.settings_open = true;
    Command::none()
}

fn handle_close_settings(state: &mut State) -> Command<Message> {
    state.settings_open = false;
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
    fn test_handle_file_selected_does_not_change_selection_or_db_on_load_failure(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        db::initialize(db.connection())?;

        let mut audiobook = Audiobook::new(
            "/dir".to_string(),
            "Test".to_string(),
            "/dir/book".to_string(),
            0,
        );
        let old_path = "/dir/book/old.mp3";
        audiobook.selected_file = Some(old_path.to_string());
        let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

        let old_file = AudiobookFile::new(audiobook_id, "old".to_string(), old_path.to_string(), 0);
        crate::db::queries::insert_audiobook_file(db.connection(), &old_file)?;

        let new_path = "/dir/book/new.mp3";
        let new_file = AudiobookFile::new(audiobook_id, "new".to_string(), new_path.to_string(), 1);
        crate::db::queries::insert_audiobook_file(db.connection(), &new_file)?;

        let mut state = State {
            selected_audiobook: Some(audiobook_id),
            selected_file: Some(old_path.to_string()),
            ..Default::default()
        };

        let mut player = Some(FailingLoadPlayer);

        let _cmd = handle_file_selected(&mut state, &mut player, &db, new_path);

        assert_eq!(
            state.selected_file.as_deref(),
            Some(old_path),
            "selection should remain unchanged when load fails"
        );

        let saved = crate::db::queries::get_audiobook_by_id(db.connection(), audiobook_id)?
            .ok_or_else(|| Error::AudiobookNotFound(audiobook_id))?;
        assert_eq!(
            saved.selected_file.as_deref(),
            Some(old_path),
            "db selected_file should not change when load fails"
        );
        Ok(())
    }

    #[test]
    fn test_create_bookmark_opens_editor() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        db::initialize(db.connection())?;

        let audiobook = Audiobook::new(
            "/dir".to_string(),
            "Test".to_string(),
            "/dir/book".to_string(),
            0,
        );
        let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

        let file_path = "/dir/book/ch1.mp3";
        let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
        crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

        let mut state = State {
            selected_audiobook: Some(audiobook_id),
            selected_file: Some(file_path.to_string()),
            current_time: 1500.0,
            ..Default::default()
        };

        let mut player: Option<Vlc> = None;
        let _cmd = update(&mut state, Message::CreateBookmark, &mut player, &db);

        let editor = state.bookmark_editor.as_ref().ok_or("expected editor")?;
        assert_eq!(editor.audiobook_id, audiobook_id);
        assert_eq!(editor.file_path, file_path);
        assert_eq!(editor.position_ms, 1500);
        assert_eq!(editor.label, "Bookmark");
        assert!(editor.note.is_empty());
        Ok(())
    }

    #[test]
    fn test_bookmark_editor_save_inserts_and_closes(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        db::initialize(db.connection())?;

        let audiobook = Audiobook::new(
            "/dir".to_string(),
            "Test".to_string(),
            "/dir/book".to_string(),
            0,
        );
        let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

        let file_path = "/dir/book/ch1.mp3";
        let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
        crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

        let mut state = State {
            selected_audiobook: Some(audiobook_id),
            selected_file: Some(file_path.to_string()),
            current_time: 2000.0,
            ..Default::default()
        };
        let mut player: Option<Vlc> = None;

        let _ = update(&mut state, Message::CreateBookmark, &mut player, &db);
        let _ = update(
            &mut state,
            Message::BookmarkEditorLabelChanged("Chapter 1".to_string()),
            &mut player,
            &db,
        );
        let _ = update(
            &mut state,
            Message::BookmarkEditorNoteChanged("note".to_string()),
            &mut player,
            &db,
        );
        let _ = update(&mut state, Message::BookmarkEditorSave, &mut player, &db);

        assert!(state.bookmark_editor.is_none());

        let saved = crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
        let first = saved.first().ok_or("no bookmark")?;
        assert_eq!(first.label, "Chapter 1");
        assert_eq!(first.note.as_deref(), Some("note"));
        Ok(())
    }
}
