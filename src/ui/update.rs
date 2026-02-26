//! Message update handlers for the Nodoka UI.
//!
//! The [`update`] function dispatches each [`Message`] to a focused
//! submodule handler, producing the next [`State`] and an iced [`Task`].

use crate::db::Database;
use crate::error::Result;
use crate::player::Vlc;
use crate::ui::{Message, ScanState, State};
use iced::Task;
use std::path::Path;

mod bookmarks;
mod directories;
mod feedback;
mod library;
mod modal;
mod navigation;
mod playback;
mod sleep_timer;
mod window;

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

/// Processes a [`Message`] and updates [`State`], returning any follow-up [`Task`].
///
/// # Errors
///
/// Individual handlers may produce error notifications via the notification
/// queue rather than returning `Err`; those are not propagated here.
pub fn update(
    state: &mut State,
    message: Message,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    match message {
        Message::UiTick => handle_ui_tick(state),
        Message::None => Task::none(),
        Message::WindowMoved(x, y) => window::handle_window_moved(db, x, y),
        Message::WindowResized(width, height) => window::handle_window_resized(db, width, height),
        Message::InitialLoadComplete => library::handle_initial_load_complete(state),
        Message::CoverThumbnailGenerated(audiobook_id, path) => {
            library::handle_cover_thumbnail_generated(state, audiobook_id, path)
        }

        message @ (Message::PlayPause
        | Message::Stop
        | Message::SeekTo(_)
        | Message::VolumeChanged(_)
        | Message::SpeedChanged(_)
        | Message::PlayerTimeUpdated(_)
        | Message::PlayerTick) => handle_playback_message(state, &message, player, db),

        message @ (Message::SleepTimerSetDurationSeconds(_)
        | Message::SleepTimerSetEndOfChapter
        | Message::SleepTimerExtendSeconds(_)
        | Message::SleepTimerCancel
        | Message::SleepTimerCustomMinutesChanged(_)
        | Message::SleepTimerCustomSubmit) => handle_sleep_timer_message(state, &message, player),

        message @ (Message::SeekForward(_)
        | Message::SeekBackward(_)
        | Message::NextFile
        | Message::PreviousFile
        | Message::CloseModal
        | Message::FocusNext
        | Message::FocusPrevious) => handle_navigation_message(state, &message, player, db),

        message @ (Message::CreateBookmark
        | Message::BookmarkEdit(_)
        | Message::BookmarkDelete(_)
        | Message::BookmarkJump(_)
        | Message::BookmarkEditorLabelChanged(_)
        | Message::BookmarkEditorNoteChanged(_)
        | Message::BookmarkEditorSave
        | Message::BookmarkEditorCancel) => handle_bookmarks_message(state, &message, player, db),

        message @ (Message::AudiobookSelected(_) | Message::FileSelected(_)) => {
            handle_selection_message(state, message, player, db)
        }

        message @ (Message::DirectoryAdd
        | Message::DirectoryAdded(_)
        | Message::DirectoryAddCancelled
        | Message::DirectoryRemove(_)
        | Message::DirectoryRescan(_)
        | Message::ScanComplete(_, _)
        | Message::ScanError(_)) => handle_directory_message(state, message, player, db),

        message @ (Message::OpenSettings
        | Message::CloseSettings
        | Message::ShowKeyboardHelp
        | Message::HideKeyboardHelp) => handle_modal_message(state, &message),

        message @ (Message::DismissNotification(_)
        | Message::TooltipHoverStart(_)
        | Message::TooltipHoverEnd
        | Message::RipplePointerMoved(_, _)) => handle_feedback_message(state, &message),

        message @ (Message::SearchQueryChanged(_)
        | Message::SortOrderChanged(_)
        | Message::FilterStatusChanged(_)
        | Message::ClearFilters) => handle_library_filter_message(state, &message),

        message @ (Message::PanelRatioChanged(_)
        | Message::ToggleFocusMode
        | Message::ThemeChanged(_)) => handle_layout_message(state, &message, db),

        Message::PlayLastListened => handle_play_last_listened(state, player, db),

        Message::FocusSearch => {
            // TODO: Implement search bar focus when text input focus API is available
            Task::none()
        }

        Message::ToggleTheme => {
            use crate::ui::state::ThemeMode;
            let new_theme = match state.theme {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
            };
            handle_layout_message(state, &Message::ThemeChanged(new_theme), db)
        }

        Message::DecreasePanelRatio => {
            let new_ratio = (state.panel_ratio - 0.1).clamp(0.2, 0.8);
            handle_layout_message(state, &Message::PanelRatioChanged(new_ratio), db)
        }

        Message::IncreasePanelRatio => {
            let new_ratio = (state.panel_ratio + 0.1).clamp(0.2, 0.8);
            handle_layout_message(state, &Message::PanelRatioChanged(new_ratio), db)
        }
    }
}

fn handle_playback_message(
    state: &mut State,
    message: &Message,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    match message {
        Message::PlayPause => playback::handle_play_pause(state, player),
        Message::Stop => playback::handle_stop(state, player),
        Message::SeekTo(position) => playback::handle_seek_to(state, player, *position),
        Message::VolumeChanged(volume) => {
            playback::handle_volume_changed(state, player, db, *volume)
        }
        Message::SpeedChanged(speed) => playback::handle_speed_changed(state, player, db, *speed),
        Message::PlayerTimeUpdated(time) => playback::handle_time_updated(state, db, *time),
        Message::PlayerTick => playback::handle_player_tick(state, player, db),
        _ => Task::none(),
    }
}

fn handle_sleep_timer_message(
    state: &mut State,
    message: &Message,
    player: &mut Option<Vlc>,
) -> Task<Message> {
    match message {
        Message::SleepTimerSetDurationSeconds(secs) => {
            sleep_timer::handle_sleep_timer_set_duration(state, *secs)
        }
        Message::SleepTimerSetEndOfChapter => {
            sleep_timer::handle_sleep_timer_set_end_of_chapter(state)
        }
        Message::SleepTimerExtendSeconds(secs) => {
            sleep_timer::handle_sleep_timer_extend(state, *secs)
        }
        Message::SleepTimerCancel => sleep_timer::handle_sleep_timer_cancel(state, player),
        Message::SleepTimerCustomMinutesChanged(value) => {
            sleep_timer::handle_sleep_timer_custom_minutes_changed(state, value)
        }
        Message::SleepTimerCustomSubmit => sleep_timer::handle_sleep_timer_custom_submit(state),
        _ => Task::none(),
    }
}

fn handle_navigation_message(
    state: &mut State,
    message: &Message,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    match message {
        Message::SeekForward(seconds) => navigation::handle_seek_forward(state, player, *seconds),
        Message::SeekBackward(seconds) => navigation::handle_seek_backward(state, player, *seconds),
        Message::NextFile => navigation::handle_next_file(state, player, db),
        Message::PreviousFile => navigation::handle_previous_file(state, player, db),
        Message::CloseModal => navigation::handle_close_modal(state),
        Message::FocusNext => navigation::handle_focus_next(state),
        Message::FocusPrevious => navigation::handle_focus_previous(state),
        _ => Task::none(),
    }
}

fn handle_bookmarks_message(
    state: &mut State,
    message: &Message,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    match message {
        Message::CreateBookmark => bookmarks::handle_create_bookmark(state, db),
        Message::BookmarkEdit(id) => bookmarks::handle_bookmark_edit(state, *id),
        Message::BookmarkDelete(id) => bookmarks::handle_bookmark_delete(state, db, *id),
        Message::BookmarkJump(id) => bookmarks::handle_bookmark_jump(state, player, db, *id),
        Message::BookmarkEditorLabelChanged(value) => {
            bookmarks::handle_bookmark_editor_label_changed(state, value)
        }
        Message::BookmarkEditorNoteChanged(value) => {
            bookmarks::handle_bookmark_editor_note_changed(state, value)
        }
        Message::BookmarkEditorSave => bookmarks::handle_bookmark_editor_save(state, db),
        Message::BookmarkEditorCancel => bookmarks::handle_bookmark_editor_cancel(state),
        _ => Task::none(),
    }
}

fn handle_selection_message(
    state: &mut State,
    message: Message,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    match message {
        Message::AudiobookSelected(id) => library::handle_audiobook_selected(state, player, db, id),
        Message::FileSelected(path) => library::handle_file_selected(state, player, db, &path).task,
        _ => Task::none(),
    }
}

fn handle_directory_message(
    state: &mut State,
    message: Message,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    match message {
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
        Message::ScanComplete(directory, discovered) => {
            state.operation_in_progress = false;
            directories::handle_scan_complete(state, db, &directory, discovered)
        }
        Message::ScanError(error) => {
            state.operation_in_progress = false;
            directories::handle_scan_error(state, &error)
        }
        _ => Task::none(),
    }
}

fn handle_modal_message(state: &mut State, message: &Message) -> Task<Message> {
    match message {
        Message::OpenSettings => modal::handle_open_settings(state),
        Message::CloseSettings => modal::handle_close_settings(state),
        Message::ShowKeyboardHelp => modal::handle_show_keyboard_help(state),
        Message::HideKeyboardHelp => modal::handle_hide_keyboard_help(state),
        _ => Task::none(),
    }
}

fn handle_feedback_message(state: &mut State, message: &Message) -> Task<Message> {
    match message {
        Message::DismissNotification(notification_id) => {
            feedback::handle_dismiss_notification(state, *notification_id)
        }
        Message::TooltipHoverStart(target) => feedback::handle_tooltip_hover_start(state, *target),
        Message::TooltipHoverEnd => feedback::handle_tooltip_hover_end(state),
        Message::RipplePointerMoved(target, point) => {
            feedback::handle_ripple_pointer_moved(state, *target, *point)
        }
        _ => Task::none(),
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
    modal::update_modal_animation(state, delta_ms);
    feedback::update_ripples(state, delta_ms);
    state
        .tooltip_state
        .update(now, state.prefers_reduced_motion);
    feedback::prune_notifications(state);
    Task::none()
}

fn handle_library_filter_message(state: &mut State, message: &Message) -> Task<Message> {
    match message {
        Message::SearchQueryChanged(query) => {
            state.search_query.clone_from(query);
            Task::none()
        }
        Message::SortOrderChanged(order) => {
            state.sort_order = *order;
            Task::none()
        }
        Message::FilterStatusChanged(status) => {
            state.filter_status = *status;
            Task::none()
        }
        Message::ClearFilters => {
            state.search_query = String::new();
            state.sort_order = crate::ui::state::AudiobookSortOrder::default();
            state.filter_status = crate::ui::state::AudiobookFilterStatus::default();
            Task::none()
        }
        _ => Task::none(),
    }
}

fn handle_layout_message(state: &mut State, message: &Message, db: &Database) -> Task<Message> {
    match message {
        Message::PanelRatioChanged(ratio) => {
            state.panel_ratio = ratio.clamp(0.2, 0.8);
            // Persist to database
            if let Err(e) = crate::settings::Settings::new(db.connection()).set_panel_ratio(state.panel_ratio) {
                tracing::error!("Failed to save panel ratio: {e}");
            }
            Task::none()
        }
        Message::ToggleFocusMode => {
            state.focus_mode = !state.focus_mode;
            // Persist to database
            if let Err(e) = crate::settings::Settings::new(db.connection()).set_focus_mode(state.focus_mode) {
                tracing::error!("Failed to save focus mode: {e}");
            }
            Task::none()
        }
        Message::ThemeChanged(theme) => {
            state.theme = *theme;
            // Persist to database
            let theme_str = match theme {
                crate::ui::state::ThemeMode::Light => "light",
                crate::ui::state::ThemeMode::Dark => "dark",
            };
            if let Err(e) = crate::settings::Settings::new(db.connection()).set_theme(theme_str) {
                tracing::error!("Failed to save theme: {e}");
            }
            Task::none()
        }
        _ => Task::none(),
    }
}

fn handle_play_last_listened(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    // Find most recently played audiobook
    if let Some(audiobook) = state
        .audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened)
    {
        if let Some(id) = audiobook.id {
            // Select the audiobook and start playing
            state.selected_audiobook = Some(id);
            return Task::batch(vec![
                library::handle_audiobook_selected(state, player, db, id),
                Task::done(Message::PlayPause),
            ]);
        }
    }
    Task::none()
}
