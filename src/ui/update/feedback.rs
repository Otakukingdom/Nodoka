use crate::ui::state::RippleTarget;
use crate::ui::tooltip::TooltipTarget;
use crate::ui::{Message, State};
use iced::{Point, Task};

pub(super) fn handle_dismiss_notification(
    state: &mut State,
    notification_id: u64,
) -> Task<Message> {
    state
        .notification_queue
        .retain(|notification| notification.id != notification_id);
    Task::none()
}

pub(super) fn prune_notifications(state: &mut State) {
    state.notification_queue.retain(|n| !n.is_expired());
    let max_count = crate::ui::state::MAX_NOTIFICATIONS;
    if state.notification_queue.len() > max_count {
        let excess = state.notification_queue.len() - max_count;
        state.notification_queue.drain(0..excess);
    }
}

pub(super) fn handle_tooltip_hover_start(
    state: &mut State,
    target: TooltipTarget,
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

pub(super) fn handle_tooltip_hover_end(state: &mut State) -> Task<Message> {
    state.tooltip_state.end_hover();
    Task::none()
}

pub(super) fn update_ripples(state: &mut State, delta_ms: u64) {
    state
        .ripple_animations
        .iter_mut()
        .for_each(|(_, ripple)| ripple.update(delta_ms));
    state
        .ripple_animations
        .retain(|_, ripple| !ripple.is_complete());
}

pub(super) fn handle_ripple_pointer_moved(
    state: &mut State,
    target: RippleTarget,
    point: Point,
) -> Task<Message> {
    state.ripple_origins.insert(target, point);
    Task::none()
}
