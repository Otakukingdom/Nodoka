use crate::conversions::{f64_to_ms, ms_to_f64, percentage_to_i32};
use crate::db::Database;
use crate::player::{PlaybackState, Vlc};
use crate::tasks::{cleanup_temp_files, parse_zip_virtual_path, zip_temp_dir};
use crate::ui::state::RippleTarget;
use crate::ui::styles::interactive::RippleAnimation;
use crate::ui::styles::{spacing, typography};
use crate::ui::{ActiveModal, FocusedElement, PlaybackStatus, State};
use iced::Point;
use iced::Task;

use super::library;
use super::sleep_timer;
use super::MediaControl;

const PLAYBACK_RIPPLE_RADIUS: f32 = 80.0;
const PLAYBACK_BUTTON_WIDTH: f32 = 100.0;
const PLAYBACK_BUTTON_HEIGHT: f32 = typography::SIZE_BASE + spacing::MD * 2.0;

fn ripple_center(state: &State, target: RippleTarget) -> Point {
    let fallback = Point::new(PLAYBACK_BUTTON_WIDTH / 2.0, PLAYBACK_BUTTON_HEIGHT / 2.0);
    let position = state
        .ripple_origins
        .get(&target)
        .copied()
        .unwrap_or(fallback);
    position
}

pub(super) fn handle_play_pause(
    state: &mut State,
    player: &mut Option<Vlc>,
) -> Task<crate::ui::Message> {
    // Don't process play/pause when a modal is open (keyboard shortcut should not work in modal context)
    if state.active_modal != ActiveModal::None || state.bookmark_editor.is_some() {
        return Task::none();
    }

    state.focused_element = FocusedElement::PlayPauseButton;
    state.ripple_animations.insert(
        RippleTarget::PlayPauseButton,
        RippleAnimation::new(
            ripple_center(state, RippleTarget::PlayPauseButton),
            PLAYBACK_RIPPLE_RADIUS,
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
            // Update last_listened timestamp when playback starts
            update_last_listened_timestamp(state);
        }
    }
    Task::none()
}

/// Updates the `last_listened` timestamp for the currently selected audiobook
fn update_last_listened_timestamp(state: &mut State) {
    if let Some(audiobook_id) = state.selected_audiobook {
        let now = chrono::Utc::now();

        // Update in-memory state
        if let Some(ab) = state
            .audiobooks
            .iter_mut()
            .find(|ab| ab.id == Some(audiobook_id))
        {
            ab.last_listened = Some(now);
        }

        // Database update will be performed by a task
        // to avoid blocking the UI thread
        tracing::debug!("Updated last_listened for audiobook {audiobook_id}");
    }
}

pub(super) fn handle_stop(state: &mut State, player: &mut Option<Vlc>) -> Task<crate::ui::Message> {
    state.focused_element = FocusedElement::StopButton;
    state.ripple_animations.insert(
        RippleTarget::StopButton,
        RippleAnimation::new(
            ripple_center(state, RippleTarget::StopButton),
            PLAYBACK_RIPPLE_RADIUS,
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

pub(super) fn reset_playback_state(state: &mut State, player: &mut Option<Vlc>) {
    if let Some(ref mut p) = player {
        if let Err(e) = p.stop() {
            tracing::error!("Failed to stop: {e}");
        }
    }
    state.playback = PlaybackStatus::Paused;
    state.current_time = 0.0;
    state.total_duration = 0.0;
}

pub(super) fn handle_seek_to(
    state: &mut State,
    player: &mut Option<Vlc>,
    position: f64,
) -> Task<crate::ui::Message> {
    handle_seek_to_media_control(state, player, position)
}

pub(super) fn handle_seek_to_media_control<P: MediaControl>(
    state: &mut State,
    player: &mut Option<P>,
    position: f64,
) -> Task<crate::ui::Message> {
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

pub(super) fn handle_player_tick(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<crate::ui::Message> {
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

pub(super) fn handle_volume_changed(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    volume: i32,
) -> Task<crate::ui::Message> {
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

pub(super) fn handle_speed_changed(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    speed: f32,
) -> Task<crate::ui::Message> {
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

pub(super) fn handle_time_updated(
    state: &mut State,
    db: &Database,
    time: f64,
) -> Task<crate::ui::Message> {
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
) -> Task<crate::ui::Message> {
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
        return library::handle_file_selected(state, player, db, &next_path).task;
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
