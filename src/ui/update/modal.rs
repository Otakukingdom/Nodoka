use crate::ui::styles::animations::modal::ModalState;
use crate::ui::{ActiveModal, FocusedElement, Message, State};
use iced::Task;

pub(super) fn handle_open_settings(state: &mut State) -> Task<Message> {
    // Close any other open modals to maintain single modal invariant
    state.bookmark_editor = None;
    state.active_modal = ActiveModal::Settings;
    state.focused_element = FocusedElement::SettingsCloseButton;
    open_modal(state);
    Task::none()
}

pub(super) fn handle_close_settings(state: &mut State) -> Task<Message> {
    request_modal_close(state);
    Task::none()
}

pub(super) fn handle_show_keyboard_help(state: &mut State) -> Task<Message> {
    // Close any other open modals to maintain single modal invariant
    state.bookmark_editor = None;
    state.active_modal = ActiveModal::KeyboardHelp;
    state.focused_element = FocusedElement::KeyboardHelpCloseButton;
    open_modal(state);
    Task::none()
}

pub(super) fn handle_hide_keyboard_help(state: &mut State) -> Task<Message> {
    request_modal_close(state);
    Task::none()
}

pub(super) const fn open_modal(state: &mut State) {
    if state.prefers_reduced_motion {
        state.modal_state = ModalState::Open;
    } else {
        state.modal_state = ModalState::Opening { elapsed_ms: 0 };
    }
}

pub(super) fn request_modal_close(state: &mut State) {
    if state.bookmark_editor.is_some() && state.active_modal != ActiveModal::None {
        state.bookmark_editor = None;
        state.modal_state = ModalState::Open;
        state.focused_element = FocusedElement::None;
        return;
    }

    state.bookmark_editor = None;
    state.focused_element = FocusedElement::None;
    if state.prefers_reduced_motion {
        state.active_modal = ActiveModal::None;
        state.modal_state = ModalState::Closed;
    } else {
        state.modal_state.close();
    }
}

pub(super) fn update_modal_animation(state: &mut State, delta_ms: u64) {
    state.modal_state.update(delta_ms);

    if matches!(state.modal_state, ModalState::Closed)
        && (state.bookmark_editor.is_some() || state.active_modal != ActiveModal::None)
    {
        state.bookmark_editor = None;
        state.active_modal = ActiveModal::None;
        state.focused_element = FocusedElement::None;
    }
}
