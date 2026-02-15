use crate::conversions::{f64_to_ms, ms_to_f64};
use crate::db::Database;
use crate::player::Vlc;
use crate::ui::{FocusedElement, Message, State};
use iced::Task;

/// Keyboard navigation and shortcut-driven state transitions.
pub(super) fn handle_seek_forward(
    state: &mut State,
    player: &mut Option<Vlc>,
    seconds: i64,
) -> Task<Message> {
    // Don't process seek when a modal is open (keyboard shortcut should not work in modal context)
    if state.settings_open || state.bookmark_editor.is_some() {
        return Task::none();
    }

    state.focused_element = FocusedElement::ProgressSlider;

    if player.is_none() {
        return Task::none();
    }

    let Ok(current_ms) = f64_to_ms(state.current_time) else {
        return Task::none();
    };

    let seek_ms_unclamped = current_ms.saturating_add(seconds.saturating_mul(1000));
    let seek_ms = if state.total_duration > 0.0 {
        f64_to_ms(state.total_duration)
            .ok()
            .map_or(seek_ms_unclamped, |max| seek_ms_unclamped.min(max))
    } else {
        seek_ms_unclamped
    };
    let Ok(seek_f64) = ms_to_f64(seek_ms) else {
        return Task::none();
    };

    super::handle_seek_to(state, player, seek_f64)
}

pub(super) fn handle_seek_backward(
    state: &mut State,
    player: &mut Option<Vlc>,
    seconds: i64,
) -> Task<Message> {
    // Don't process seek when a modal is open (keyboard shortcut should not work in modal context)
    if state.settings_open || state.bookmark_editor.is_some() {
        return Task::none();
    }

    state.focused_element = FocusedElement::ProgressSlider;

    if player.is_none() {
        return Task::none();
    }

    let Ok(current_ms) = f64_to_ms(state.current_time) else {
        return Task::none();
    };

    let seek_ms = (current_ms - (seconds * 1000)).max(0);
    let Ok(seek_f64) = ms_to_f64(seek_ms) else {
        return Task::none();
    };

    super::handle_seek_to(state, player, seek_f64)
}

pub(super) fn handle_next_file(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    // Don't process file navigation when a modal is open (keyboard shortcut should not work in modal context)
    if state.settings_open || state.bookmark_editor.is_some() {
        return Task::none();
    }

    state.focused_element = FocusedElement::FileList;

    let Some(ref current_path) = state.selected_file else {
        return Task::none();
    };

    let next_path = state
        .current_files
        .iter()
        .position(|file| &file.full_path == current_path)
        .and_then(|idx| state.current_files.get(idx + 1))
        .map(|file| file.full_path.clone());

    next_path.map_or_else(Task::none, |path| {
        super::handle_file_selected(state, player, db, &path)
    })
}

pub(super) fn handle_previous_file(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
) -> Task<Message> {
    // Don't process file navigation when a modal is open (keyboard shortcut should not work in modal context)
    if state.settings_open || state.bookmark_editor.is_some() {
        return Task::none();
    }

    state.focused_element = FocusedElement::FileList;

    let Some(ref current_path) = state.selected_file else {
        return Task::none();
    };

    let prev_path = state
        .current_files
        .iter()
        .position(|file| &file.full_path == current_path)
        .and_then(|idx| {
            if idx > 0 {
                state.current_files.get(idx - 1)
            } else {
                None
            }
        })
        .map(|file| file.full_path.clone());

    prev_path.map_or_else(Task::none, |path| {
        super::handle_file_selected(state, player, db, &path)
    })
}

pub(super) fn handle_close_modal(state: &mut State) -> Task<Message> {
    if state.bookmark_editor.is_some() {
        state.bookmark_editor = None;
    } else if state.settings_open {
        state.settings_open = false;
    }
    state.focused_element = FocusedElement::None;
    Task::none()
}
