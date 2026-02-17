use crate::conversions::{f64_to_ms, ms_to_f64, percentage_to_i32};
use crate::db::Database;
use crate::error::Result;
use crate::player::{PlaybackState, Vlc};
use crate::tasks::{
    cleanup_temp_files, materialize_zip_virtual_path, parse_zip_virtual_path, zip_temp_dir,
};
use crate::ui::{FocusedElement, LoadState, Message, PlaybackStatus, ScanState, State};
use iced::Task;
use std::path::{Path, PathBuf};

mod bookmarks;
mod directories;
mod navigation;
mod sleep_timer;

#[cfg(test)]
mod tests;

pub(crate) trait MediaControl {
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
) -> Task<Message> {
    match message {
        // Player control messages
        Message::PlayPause => handle_play_pause(state, player),
        Message::Stop => handle_stop(state, player),
        Message::SeekTo(position) => handle_seek_to(state, player, position),
        Message::VolumeChanged(volume) => handle_volume_changed(state, player, db, volume),
        Message::SpeedChanged(speed) => handle_speed_changed(state, player, db, speed),
        Message::PlayerTimeUpdated(time) => handle_time_updated(state, db, time),
        Message::PlayerTick => handle_player_tick(state, player, db),
        Message::UiTick => handle_ui_tick(state),

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
        Message::SleepTimerCustomMinutesChanged(value) => {
            sleep_timer::handle_sleep_timer_custom_minutes_changed(state, &value)
        }
        Message::SleepTimerCustomSubmit => sleep_timer::handle_sleep_timer_custom_submit(state),

        // Shortcut actions
        Message::CreateBookmark => bookmarks::handle_create_bookmark(state, db),

        // Keyboard navigation
        Message::SeekForward(seconds) => navigation::handle_seek_forward(state, player, seconds),
        Message::SeekBackward(seconds) => navigation::handle_seek_backward(state, player, seconds),
        Message::NextFile => navigation::handle_next_file(state, player, db),
        Message::PreviousFile => navigation::handle_previous_file(state, player, db),
        Message::CloseModal => navigation::handle_close_modal(state),
        Message::FocusNext => navigation::handle_focus_next(state),
        Message::FocusPrevious => navigation::handle_focus_previous(state),

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
        Message::FileSelected(path) => handle_file_selected(state, player, db, &path).task,

        // Directory management messages
        Message::DirectoryAdd => {
            if state.operation_in_progress {
                Task::none()
            } else {
                state.operation_in_progress = true;
                directories::handle_directory_add()
            }
        }
        Message::DirectoryAdded(path) => {
            let cmd = directories::handle_directory_added(state, db, &path);
            state.operation_in_progress = matches!(state.scan_state, ScanState::Scanning { .. });
            cmd
        }
        Message::DirectoryAddCancelled => {
            state.operation_in_progress = false;
            Task::none()
        }
        Message::None => Task::none(),
        Message::DirectoryRemove(path) => {
            directories::handle_directory_remove(state, player, db, &path)
        }
        Message::DirectoryRescan(path) => {
            if state.operation_in_progress {
                Task::none()
            } else {
                state.operation_in_progress = true;
                directories::handle_directory_rescan(state, db, &path)
            }
        }

        // Settings messages
        Message::OpenSettings => handle_open_settings(state),
        Message::CloseSettings => handle_close_settings(state),

        // Help overlay messages
        Message::ShowKeyboardHelp => handle_show_keyboard_help(state),
        Message::HideKeyboardHelp => handle_hide_keyboard_help(state),

        // Error handling messages
        Message::DismissNotification(notification_id) => {
            handle_dismiss_notification(state, notification_id)
        }

        // Tooltip hover messages
        Message::TooltipHoverStart(target) => handle_tooltip_hover_start(state, target),
        Message::TooltipHoverEnd => handle_tooltip_hover_end(state),

        // Scan messages
        Message::ScanComplete(directory, discovered) => {
            state.operation_in_progress = false;
            directories::handle_scan_complete(state, db, &directory, discovered)
        }
        Message::ScanError(error) => {
            state.operation_in_progress = false;
            directories::handle_scan_error(state, &error)
        }

        // Cover thumbnails
        Message::CoverThumbnailGenerated(audiobook_id, path) => {
            handle_cover_thumbnail_generated(state, audiobook_id, path)
        }

        // Lifecycle messages
        Message::InitialLoadComplete => handle_initial_load_complete(state),

        // Window events
        Message::WindowMoved(x, y) => handle_window_moved(db, x, y),
        Message::WindowResized(width, height) => handle_window_resized(db, width, height),
    }
}

fn handle_ui_tick(state: &mut State) -> Task<Message> {
    let now = crate::ui::state::current_time_ms();
    let delta_ms = if state.ui_time_ms == 0 {
        0
    } else {
        now.saturating_sub(state.ui_time_ms)
    };
    state.ui_time_ms = now;
    update_modal_animation(state, delta_ms);
    update_ripples(state, delta_ms);
    state
        .tooltip_state
        .update(now, state.prefers_reduced_motion);
    prune_notifications(state);
    Task::none()
}

fn handle_window_moved(db: &Database, x: i32, y: i32) -> Task<Message> {
    let settings = crate::settings::Settings::new(db.connection());
    if let Err(e) = settings.set_window_position(x, y) {
        tracing::warn!("Failed to persist window position: {e}");
    }
    Task::none()
}

fn handle_window_resized(db: &Database, width: u32, height: u32) -> Task<Message> {
    let settings = crate::settings::Settings::new(db.connection());

    let width_i32 = match i32::try_from(width) {
        Ok(v) => v,
        Err(_e) => {
            tracing::warn!("Window resize width out of range: {width}");
            return Task::none();
        }
    };
    let height_i32 = match i32::try_from(height) {
        Ok(v) => v,
        Err(_e) => {
            tracing::warn!("Window resize height out of range: {height}");
            return Task::none();
        }
    };

    if let Err(e) = settings.set_window_size(width_i32, height_i32) {
        tracing::warn!("Failed to persist window size: {e}");
    }

    Task::none()
}

fn handle_play_pause(state: &mut State, player: &mut Option<Vlc>) -> Task<Message> {
    // Don't process play/pause when a modal is open (keyboard shortcut should not work in modal context)
    if state.active_modal != crate::ui::ActiveModal::None || state.bookmark_editor.is_some() {
        return Task::none();
    }

    state.focused_element = FocusedElement::PlayPauseButton;
    state.ripple_animations.insert(
        crate::ui::state::RippleTarget::PlayPauseButton,
        crate::ui::styles::interactive::RippleAnimation::new(
            iced::Point::new(0.0, 0.0),
            80.0,
            state.prefers_reduced_motion,
        ),
    );

    if let Some(ref mut p) = player {
        if state.playback == PlaybackStatus::Playing {
            if let Err(e) = p.pause() {
                tracing::error!("Failed to pause: {e}");
            } else {
                state.playback = PlaybackStatus::Paused;
            }
        } else if let Err(e) = p.play() {
            tracing::error!("Failed to play: {e}");
        } else {
            state.playback = PlaybackStatus::Playing;
        }
    }
    Task::none()
}

fn handle_stop(state: &mut State, player: &mut Option<Vlc>) -> Task<Message> {
    state.focused_element = FocusedElement::StopButton;
    state.ripple_animations.insert(
        crate::ui::state::RippleTarget::StopButton,
        crate::ui::styles::interactive::RippleAnimation::new(
            iced::Point::new(0.0, 0.0),
            80.0,
            state.prefers_reduced_motion,
        ),
    );

    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop: {e}");
        }
    }

    // Reset UI playback state even when VLC isn't available.
    state.playback = PlaybackStatus::Paused;
    state.current_time = 0.0;

    if let Some(selected) = state.selected_file.as_deref() {
        if let Some((zip_path, _entry)) = parse_zip_virtual_path(selected) {
            if let Ok(dir) = zip_temp_dir(&zip_path) {
                if let Err(e) = cleanup_temp_files(&dir) {
                    tracing::warn!("Failed to cleanup ZIP temp dir {}: {e}", dir.display());
                }
            }
        }
    }

    Task::none()
}

fn reset_playback_state(state: &mut State, player: &mut Option<Vlc>) {
    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop: {e}");
        }
    }
    state.playback = PlaybackStatus::Paused;
    state.current_time = 0.0;
    state.total_duration = 0.0;
}

fn handle_seek_to(state: &mut State, player: &mut Option<Vlc>, position: f64) -> Task<Message> {
    handle_seek_to_media_control(state, player, position)
}

fn handle_seek_to_media_control<P: MediaControl>(
    state: &mut State,
    player: &mut Option<P>,
    position: f64,
) -> Task<Message> {
    state.focused_element = FocusedElement::ProgressSlider;

    let clamped = if state.total_duration > 0.0 {
        position.clamp(0.0, state.total_duration)
    } else {
        position.max(0.0)
    };

    if let Some(ref mut p) = player {
        match f64_to_ms(clamped) {
            Ok(position_ms) => {
                if let Err(e) = p.set_time(position_ms) {
                    tracing::error!("Failed to seek: {e}");
                } else {
                    state.current_time = clamped;
                }
            }
            Err(e) => {
                tracing::error!("Invalid seek position: {e}");
            }
        }
    }
    Task::none()
}

fn handle_player_tick(state: &mut State, player: &mut Option<Vlc>, db: &Database) -> Task<Message> {
    let timer_consumed_tick = sleep_timer::handle_sleep_timer_tick(state, player);
    if timer_consumed_tick {
        return Task::none();
    }

    if state.selected_file.is_none() {
        return Task::none();
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
                    return Task::none();
                }
            };

            (time, p.get_state())
        }
        None => return Task::none(),
    };

    // Synchronize playback status with actual player state to prevent drift
    let should_be_playing = matches!(
        player_state,
        PlaybackState::Playing | PlaybackState::Buffering
    );
    let is_playing = state.playback == PlaybackStatus::Playing;
    if is_playing != should_be_playing {
        tracing::debug!(
            "Synchronizing playback: {} -> {}",
            is_playing,
            should_be_playing
        );
        state.playback = if should_be_playing {
            PlaybackStatus::Playing
        } else {
            PlaybackStatus::Paused
        };
    }

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

            state.playback = PlaybackStatus::Paused;
            state.current_time = state.total_duration;
            state.sleep_timer = None;
            state.sleep_timer_base_volume = None;
            return Task::none();
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
) -> Task<Message> {
    state.focused_element = FocusedElement::VolumeSlider;
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
    Task::none()
}

fn sanitize_speed(speed: f32) -> f32 {
    if !speed.is_finite() {
        return 1.0;
    }

    // Quantize to match the UI slider step (0.05x) while preserving preset values like 0.75x.
    let clamped = speed.clamp(0.5, 2.0);
    let step = (clamped * 20.0).round();
    (step / 20.0).clamp(0.5, 2.0)
}

fn handle_speed_changed(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    speed: f32,
) -> Task<Message> {
    state.focused_element = FocusedElement::SpeedSlider;
    let speed = sanitize_speed(speed);

    state.speed = speed;
    if let Err(e) = crate::db::queries::set_metadata(db.connection(), "speed", &speed.to_string()) {
        tracing::error!("Failed to save speed setting: {e}");
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.set_rate(speed) {
            tracing::error!("Failed to set speed: {e}");
        }
    }
    Task::none()
}

fn handle_audiobook_selected(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    id: i64,
) -> Task<Message> {
    state.focused_element = FocusedElement::AudiobookList;

    let old_selection = state.selected_audiobook;
    let is_new_selection = old_selection != Some(id);

    // Load files first, only update selection if successful
    let files = match crate::db::queries::get_audiobook_files(db.connection(), id) {
        Ok(files) => files,
        Err(e) => {
            tracing::error!("Failed to load audiobook files: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(
                    "Failed to load audiobook files".to_string(),
                ));
            return Task::none();
        }
    };

    let bookmarks = match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), id) {
        Ok(bookmarks) => bookmarks,
        Err(e) => {
            tracing::error!("Failed to load bookmarks: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(
                    "Failed to load bookmarks".to_string(),
                ));
            return Task::none();
        }
    };

    // Only now update state after successful loading.
    //
    // Keep audiobook selection and dependent UI state atomic: if loading fails, do not clear
    // or reset the previous selection (avoids inconsistent UI state).
    if is_new_selection {
        if let Some(old_id) = old_selection {
            if let Some(old_ab) = state.audiobooks.iter().find(|a| a.id == Some(old_id)) {
                super::media_paths::cleanup_zip_temp_for_path(&old_ab.full_path);
            }
        }

        reset_playback_state(state, player);
        state.selected_file = None;
        state.bookmark_editor = None;
    }

    state.selected_audiobook = Some(id);
    state.current_files = files;
    state.bookmarks = bookmarks;
    let now_ms = crate::ui::state::current_time_ms();
    state.file_list_animation_start_ms = Some(now_ms);
    state.bookmark_list_animation_start_ms = Some(now_ms);

    // Only save metadata after successful state update
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
        return Task::none();
    }

    let Some(selected_id) = state.selected_audiobook else {
        return Task::none();
    };

    let Some(ab) = state.audiobooks.iter().find(|a| a.id == Some(selected_id)) else {
        return Task::none();
    };

    generate_cover_thumbnail_command(selected_id, ab.full_path.clone())
}

pub(crate) struct FileSelectionOutcome {
    pub(crate) task: Task<Message>,
    pub(crate) file_loaded: bool,
}

pub(crate) fn handle_file_selected<P: MediaControl>(
    state: &mut State,
    player: &mut Option<P>,
    db: &Database,
    path: &str,
) -> FileSelectionOutcome {
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

    state.focused_element = FocusedElement::FileList;

    if let Some(p) = player.as_mut() {
        let playback_path = if path.starts_with("zip://") {
            if parse_zip_virtual_path(path).is_none() {
                tracing::warn!("Rejecting malformed ZIP virtual path: {path}");
                state
                    .notification_queue
                    .push(crate::ui::state::Notification::error(
                        "Invalid ZIP virtual path".to_string(),
                    ));
                return FileSelectionOutcome {
                    task: Task::none(),
                    file_loaded: false,
                };
            }

            match materialize_zip_virtual_path(path) {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to extract ZIP entry for playback: {e}");
                    state
                        .notification_queue
                        .push(crate::ui::state::Notification::error(format!(
                            "Failed to extract audio file: {e}"
                        )));
                    return FileSelectionOutcome {
                        task: Task::none(),
                        file_loaded: false,
                    };
                }
            }
        } else {
            std::path::PathBuf::from(path)
        };

        if let Err(e) = p.load_media(&playback_path) {
            tracing::error!("Failed to load media: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(format!(
                    "Failed to load audio file: {e}"
                )));
            return FileSelectionOutcome {
                task: Task::none(),
                file_loaded: false,
            };
        }

        persist_selected_file(state, db, path);
        restore_progress_and_duration(state, p, db, path);

        if let Err(e) = p.play() {
            tracing::error!("Failed to auto-play: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(format!(
                    "Failed to start playback: {e}"
                )));
            state.playback = PlaybackStatus::Paused;
        } else {
            state.playback = PlaybackStatus::Playing;
        }
        return FileSelectionOutcome {
            task: Task::none(),
            file_loaded: true,
        };
    } else {
        persist_selected_file(state, db, path);
        return FileSelectionOutcome {
            task: Task::none(),
            file_loaded: true,
        };
    }
}

// Cannot be const fn because Task::none() is not const
fn handle_open_settings(state: &mut State) -> Task<Message> {
    // Close any other open modals to maintain single modal invariant
    state.bookmark_editor = None;
    state.active_modal = crate::ui::ActiveModal::Settings;
    state.focused_element = FocusedElement::SettingsCloseButton;
    open_modal(state);
    Task::none()
}

// Cannot be const fn because Task::none() is not const
fn handle_close_settings(state: &mut State) -> Task<Message> {
    request_modal_close(state);
    Task::none()
}

fn handle_show_keyboard_help(state: &mut State) -> Task<Message> {
    // Close any other open modals to maintain single modal invariant
    state.bookmark_editor = None;
    state.active_modal = crate::ui::ActiveModal::KeyboardHelp;
    state.focused_element = FocusedElement::KeyboardHelpCloseButton;
    open_modal(state);
    Task::none()
}

fn handle_hide_keyboard_help(state: &mut State) -> Task<Message> {
    request_modal_close(state);
    Task::none()
}

fn handle_dismiss_notification(state: &mut State, notification_id: u64) -> Task<Message> {
    state
        .notification_queue
        .retain(|notification| notification.id != notification_id);
    Task::none()
}

fn handle_initial_load_complete(state: &mut State) -> Task<Message> {
    state.load_state = LoadState::Ready;

    let cmds: Vec<Task<Message>> = state
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

    Task::batch(cmds)
}

fn prune_notifications(state: &mut State) {
    state.notification_queue.retain(|n| !n.is_expired());
    let max_count = crate::ui::state::MAX_NOTIFICATIONS;
    if state.notification_queue.len() > max_count {
        let excess = state.notification_queue.len() - max_count;
        state.notification_queue.drain(0..excess);
    }
}

fn handle_tooltip_hover_start(
    state: &mut State,
    target: crate::ui::tooltip::TooltipTarget,
) -> Task<Message> {
    let tooltip = target.tooltip();
    let now = if state.ui_time_ms == 0 {
        crate::ui::state::current_time_ms()
    } else {
        state.ui_time_ms
    };
    state.tooltip_state.start_hover(tooltip, now);
    Task::none()
}

fn handle_tooltip_hover_end(state: &mut State) -> Task<Message> {
    state.tooltip_state.end_hover();
    Task::none()
}

pub(super) fn open_modal(state: &mut State) {
    state.modal_pending_close = false;
    if state.prefers_reduced_motion {
        state.modal_state = crate::ui::styles::animations::modal::ModalState::Open;
    } else {
        state.modal_state = crate::ui::styles::animations::modal::ModalState::Opening {
            elapsed_ms: 0,
        };
    }
}

pub(super) fn request_modal_close(state: &mut State) {
    if state.bookmark_editor.is_some() && state.active_modal != crate::ui::ActiveModal::None {
        state.bookmark_editor = None;
        state.modal_pending_close = false;
        state.modal_state = crate::ui::styles::animations::modal::ModalState::Open;
        state.focused_element = FocusedElement::None;
        return;
    }

    if state.prefers_reduced_motion {
        if state.bookmark_editor.is_some() {
            state.bookmark_editor = None;
        } else {
            state.active_modal = crate::ui::ActiveModal::None;
        }
        state.modal_state = crate::ui::styles::animations::modal::ModalState::Closed;
        state.focused_element = FocusedElement::None;
        state.modal_pending_close = false;
    } else {
        state.modal_pending_close = true;
        state.modal_state.close();
    }
}

fn update_modal_animation(state: &mut State, delta_ms: u64) {
    state.modal_state.update(delta_ms);
    if state.modal_pending_close
        && matches!(state.modal_state, crate::ui::styles::animations::modal::ModalState::Closed)
    {
        if state.bookmark_editor.is_some() {
            state.bookmark_editor = None;
        } else {
            state.active_modal = crate::ui::ActiveModal::None;
        }
        state.focused_element = FocusedElement::None;
        state.modal_pending_close = false;
    }
}

fn update_ripples(state: &mut State, delta_ms: u64) {
    state
        .ripple_animations
        .iter_mut()
        .for_each(|(_, ripple)| ripple.update(delta_ms));
    state.ripple_animations.retain(|_, ripple| !ripple.is_complete());
}

fn handle_cover_thumbnail_generated(
    state: &mut State,
    audiobook_id: i64,
    path: Option<PathBuf>,
) -> Task<Message> {
    if let Some(path) = path {
        state.cover_thumbnails.insert(audiobook_id, path);
    }
    Task::none()
}

fn generate_cover_thumbnail_command(audiobook_id: i64, full_path: String) -> Task<Message> {
    Task::perform(
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

fn handle_time_updated(state: &mut State, db: &Database, time: f64) -> Task<Message> {
    if !time.is_finite() {
        tracing::warn!("Ignoring non-finite time update: {time}");
        return Task::none();
    }

    // Clamp current_time to maintain state invariants.
    // - never negative
    // - never exceeds total_duration when known
    let mut sanitized = time.max(0.0);
    if state.total_duration > 0.0 {
        sanitized = sanitized.min(state.total_duration);
    }
    state.current_time = sanitized;

    if let Some(ref file_path) = state.selected_file {
        let file_path_owned = file_path.clone();

        if state.total_duration > 0.0 {
            let percentage = (state.current_time * 100.0) / state.total_duration;
            let completeness = percentage_to_i32(percentage);

            if let Err(e) = crate::db::queries::update_file_progress(
                db.connection(),
                &file_path_owned,
                state.current_time,
                completeness,
            ) {
                tracing::error!("Failed to update file progress: {e}");
            } else {
                update_current_file_progress(
                    state,
                    &file_path_owned,
                    state.current_time,
                    completeness,
                );
                update_audiobook_completeness_after_file_change(state, db, &file_path_owned);
            }
        }
    }

    Task::none()
}

fn should_auto_advance(state: &State, player_state: PlaybackState, time: f64) -> bool {
    if state.selected_file.is_none() {
        return false;
    }

    if player_state == PlaybackState::Ended {
        return true;
    }

    if state.playback != PlaybackStatus::Playing || state.total_duration <= 0.0 {
        return false;
    }

    let end_threshold_ms = 500.0;
    time + end_threshold_ms >= state.total_duration
}

fn advance_to_next_file(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    let Some(current_path) = state.selected_file.clone() else {
        return Task::none();
    };

    mark_current_file_complete(state, db, &current_path);

    let next_path = state
        .current_files
        .iter()
        .position(|file| file.full_path == current_path)
        .and_then(|idx| state.current_files.get(idx + 1))
        .map(|file| file.full_path.clone());

    if let Some(next_path) = next_path {
        return handle_file_selected(state, player, db, &next_path).task;
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop after final file: {e}");
        }
    }
    state.playback = PlaybackStatus::Paused;
    state.current_time = state.total_duration;
    Task::none()
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
