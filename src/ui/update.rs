use crate::conversions::{f64_to_ms, ms_to_f64, percentage_to_i32};
use crate::db::Database;
use crate::error::Result;
use crate::player::{PlaybackState, Vlc};
use crate::tasks::{
    cleanup_temp_files, materialize_zip_virtual_path, parse_zip_virtual_path, zip_temp_dir,
};
use crate::ui::{Message, State};
use iced::Command;
use std::path::{Path, PathBuf};

mod bookmarks;
mod directories;
mod sleep_timer;

#[cfg(test)]
mod tests;

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

        // Sleep timer
        Message::SleepTimerSetDurationSeconds(secs) => {
            sleep_timer::handle_sleep_timer_set_duration(state, secs)
        }
        Message::SleepTimerSetEndOfChapter => {
            sleep_timer::handle_sleep_timer_set_end_of_chapter(state)
        }
        Message::SleepTimerExtendSeconds(secs) => {
            sleep_timer::handle_sleep_timer_extend(state, secs)
        }
        Message::SleepTimerCancel => sleep_timer::handle_sleep_timer_cancel(state, player),

        // Shortcut actions
        Message::CreateBookmark => bookmarks::handle_create_bookmark(state, db),

        // Bookmarks UI
        Message::BookmarkEdit(id) => bookmarks::handle_bookmark_edit(state, id),
        Message::BookmarkDelete(id) => bookmarks::handle_bookmark_delete(state, db, id),
        Message::BookmarkJump(id) => bookmarks::handle_bookmark_jump(state, player, db, id),
        Message::BookmarkEditorLabelChanged(value) => {
            bookmarks::handle_bookmark_editor_label_changed(state, &value)
        }
        Message::BookmarkEditorNoteChanged(value) => {
            bookmarks::handle_bookmark_editor_note_changed(state, &value)
        }
        Message::BookmarkEditorSave => bookmarks::handle_bookmark_editor_save(state, db),
        Message::BookmarkEditorCancel => bookmarks::handle_bookmark_editor_cancel(state),

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

        // Cover thumbnails
        Message::CoverThumbnailGenerated(audiobook_id, path) => {
            handle_cover_thumbnail_generated(state, audiobook_id, path)
        }

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
    let timer_consumed_tick = sleep_timer::handle_sleep_timer_tick(state, player);
    if timer_consumed_tick {
        return Command::none();
    }

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
        if sleep_timer::should_pause_for_end_of_chapter(state, true) {
            if let Some(current_path) = state.selected_file.clone() {
                mark_current_file_complete(state, db, &current_path);
            }

            if let Some(ref mut p) = player {
                if let Err(e) = p.set_volume(state.volume) {
                    tracing::error!("Failed to restore volume before end-of-chapter pause: {e}");
                }
                if let Err(e) = p.pause() {
                    tracing::error!("Failed to pause at end-of-chapter: {e}");
                }
            }

            state.is_playing = false;
            state.current_time = state.total_duration;
            state.sleep_timer = None;
            state.sleep_timer_base_volume = None;
            return Command::none();
        }

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
    let volume = volume.clamp(0, 200);

    state.volume = volume;
    if let Err(e) = crate::db::queries::set_metadata(db.connection(), "volume", &volume.to_string())
    {
        tracing::error!("Failed to save volume setting: {e}");
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.set_volume(volume) {
            tracing::error!("Failed to set volume: {e}");
        }
    }
    Command::none()
}

fn sanitize_speed(speed: f32) -> f32 {
    if !speed.is_finite() {
        return 1.0;
    }

    let clamped = speed.clamp(0.5, 2.0);
    let formatted = format!("{clamped:.1}");
    formatted.parse::<f32>().ok().unwrap_or(1.0).clamp(0.5, 2.0)
}

fn handle_speed_changed(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    speed: f32,
) -> Command<Message> {
    let speed = sanitize_speed(speed);

    state.speed = speed;
    if let Err(e) =
        crate::db::queries::set_metadata(db.connection(), "speed", &format!("{speed:.1}"))
    {
        tracing::error!("Failed to save speed setting: {e}");
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.set_rate(speed) {
            tracing::error!("Failed to set speed: {e}");
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

    if state
        .selected_audiobook
        .and_then(|id| state.audiobooks.iter().find(|a| a.id == Some(id)))
        .and_then(|ab| ab.id)
        .is_some_and(|id| state.cover_thumbnails.contains_key(&id))
    {
        return Command::none();
    }

    let Some(selected_id) = state.selected_audiobook else {
        return Command::none();
    };

    let Some(ab) = state.audiobooks.iter().find(|a| a.id == Some(selected_id)) else {
        return Command::none();
    };

    generate_cover_thumbnail_command(selected_id, ab.full_path.clone())
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

    let cmds: Vec<Command<Message>> = state
        .audiobooks
        .iter()
        .take(32)
        .filter_map(|ab| {
            let id = ab.id?;
            if state.cover_thumbnails.contains_key(&id) {
                return None;
            }
            Some(generate_cover_thumbnail_command(id, ab.full_path.clone()))
        })
        .collect();

    Command::batch(cmds)
}

fn handle_cover_thumbnail_generated(
    state: &mut State,
    audiobook_id: i64,
    path: Option<PathBuf>,
) -> Command<Message> {
    if let Some(path) = path {
        state.cover_thumbnails.insert(audiobook_id, path);
    }
    Command::none()
}

fn generate_cover_thumbnail_command(audiobook_id: i64, full_path: String) -> Command<Message> {
    Command::perform(
        async move {
            let path = PathBuf::from(full_path);
            let res = tokio::task::spawn_blocking(move || {
                crate::cover_cache::ensure_cover_thumbnail(audiobook_id, &path)
            })
            .await;

            let out = match res {
                Ok(Ok(path)) => path,
                Ok(Err(e)) => {
                    tracing::debug!("Cover thumbnail generation failed for {audiobook_id}: {e}");
                    None
                }
                Err(e) => {
                    tracing::debug!(
                        "Cover thumbnail generation task failed for {audiobook_id}: {e}"
                    );
                    None
                }
            };

            (audiobook_id, out)
        },
        |(id, path)| Message::CoverThumbnailGenerated(id, path),
    )
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
