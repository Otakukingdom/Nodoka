use crate::models::{SleepTimer, SleepTimerMode};
use crate::player::Vlc;
use crate::ui::{Message, State};
use iced::Task;

const DEFAULT_SLEEP_TIMER_FADE_SECS: u32 = 30;

pub(super) fn handle_sleep_timer_set_duration(state: &mut State, secs: i64) -> Task<Message> {
    let secs = secs.max(0);
    state.sleep_timer = Some(SleepTimer::new(
        SleepTimerMode::Duration(secs),
        DEFAULT_SLEEP_TIMER_FADE_SECS,
    ));
    state.sleep_timer_base_volume = Some(state.volume);
    Task::none()
}

pub(super) fn handle_sleep_timer_set_end_of_chapter(state: &mut State) -> Task<Message> {
    state.sleep_timer = Some(SleepTimer::new(
        SleepTimerMode::EndOfChapter,
        DEFAULT_SLEEP_TIMER_FADE_SECS,
    ));
    state.sleep_timer_base_volume = Some(state.volume);
    Task::none()
}

pub(super) fn handle_sleep_timer_extend(state: &mut State, secs: i64) -> Task<Message> {
    let Some(timer) = state.sleep_timer.as_ref() else {
        return Task::none();
    };

    let SleepTimerMode::Duration(_) = timer.mode else {
        return Task::none();
    };

    let remaining = timer.remaining_seconds().unwrap_or(0);
    let new_total = (remaining + secs).max(0);

    state.sleep_timer = Some(SleepTimer::new(
        SleepTimerMode::Duration(new_total),
        timer.fade_duration_secs,
    ));
    if state.sleep_timer_base_volume.is_none() {
        state.sleep_timer_base_volume = Some(state.volume);
    }
    Task::none()
}

pub(super) fn handle_sleep_timer_cancel(
    state: &mut State,
    player: &mut Option<Vlc>,
) -> Task<Message> {
    state.sleep_timer = None;
    state.sleep_timer_base_volume = None;

    if let Some(ref mut p) = player {
        if let Err(e) = p.set_volume(state.volume) {
            tracing::error!("Failed to restore volume on sleep timer cancel: {e}");
        }
    }

    Task::none()
}

pub(super) fn handle_sleep_timer_custom_minutes_changed(
    state: &mut State,
    value: &str,
) -> Task<Message> {
    state.sleep_timer_custom_minutes = value.to_string();
    state.sleep_timer_custom_error = None;
    Task::none()
}

pub(super) fn handle_sleep_timer_custom_submit(state: &mut State) -> Task<Message> {
    let trimmed = state.sleep_timer_custom_minutes.trim();
    if trimmed.is_empty() {
        state.sleep_timer_custom_error = Some("Enter minutes".to_string());
        return Task::none();
    }

    let minutes: i64 = match trimmed.parse() {
        Ok(v) => v,
        Err(_e) => {
            state.sleep_timer_custom_error = Some("Minutes must be a whole number".to_string());
            return Task::none();
        }
    };

    if minutes <= 0 {
        state.sleep_timer_custom_error = Some("Minutes must be greater than zero".to_string());
        return Task::none();
    }

    let secs = minutes.saturating_mul(60);
    state.sleep_timer_custom_error = None;
    handle_sleep_timer_set_duration(state, secs)
}

pub(super) fn should_pause_for_end_of_chapter(state: &State, should_auto_advance: bool) -> bool {
    should_auto_advance
        && matches!(
            state.sleep_timer.as_ref().map(|t| t.mode),
            Some(SleepTimerMode::EndOfChapter)
        )
}

pub(super) fn handle_sleep_timer_tick(state: &mut State, player: &mut Option<Vlc>) -> bool {
    let Some(timer) = state.sleep_timer.clone() else {
        return false;
    };

    let SleepTimerMode::Duration(_) = timer.mode else {
        return false;
    };

    let Some(remaining) = timer.remaining_seconds() else {
        return false;
    };

    let fade = i64::from(timer.fade_duration_secs);
    if fade > 0 && remaining <= fade {
        let base = i64::from(state.sleep_timer_base_volume.unwrap_or(state.volume).max(0));
        let scaled_i64 = base.saturating_mul(remaining) / fade;
        // Conversion cannot overflow: base <= 200 (max volume), remaining <= fade,
        // so scaled_i64 = (base * remaining) / fade <= base <= 200, which fits in i32.
        // The unwrap_or(0) is a defensive fallback that should never execute.
        let scaled = i32::try_from(scaled_i64).ok().unwrap_or(0);
        if let Some(ref mut p) = player {
            if let Err(e) = p.set_volume(scaled.clamp(0, 200)) {
                tracing::error!("Failed to apply sleep timer fade volume: {e}");
            }
        }
    }

    if remaining > 0 {
        return false;
    }

    if let Some(ref mut p) = player {
        if let Err(e) = p.pause() {
            tracing::error!("Failed to pause on sleep timer expiry: {e}");
        }
        if let Err(e) = p.set_volume(state.volume) {
            tracing::error!("Failed to restore volume after sleep timer expiry: {e}");
        }
    }

    state.is_playing = false;
    state.sleep_timer = None;
    state.sleep_timer_base_volume = None;
    true
}
