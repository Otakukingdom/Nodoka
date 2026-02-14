use crate::conversions::f64_to_ms;
use crate::models::SleepTimerMode;
use crate::ui::{Message, State};
use iced::widget::{button, column, container, horizontal_space, row, slider, text};
use iced::{Element, Length};

pub fn view(state: &State) -> Element<'static, Message> {
    let current_file_text = state
        .selected_file
        .as_ref()
        .and_then(|f| std::path::Path::new(f).file_name())
        .and_then(|f| f.to_str())
        .unwrap_or("No file selected");

    let play_pause_label = if state.is_playing { "⏸" } else { "▶" };

    let sleep_timer_row = match state.sleep_timer.as_ref().map(|t| t.mode) {
        None => row![
            text("Sleep:"),
            button(text("15m")).on_press(Message::SleepTimerSetDurationSeconds(15 * 60)),
            button(text("30m")).on_press(Message::SleepTimerSetDurationSeconds(30 * 60)),
            button(text("45m")).on_press(Message::SleepTimerSetDurationSeconds(45 * 60)),
            button(text("60m")).on_press(Message::SleepTimerSetDurationSeconds(60 * 60)),
            button(text("End"))
                .on_press(Message::SleepTimerSetEndOfChapter)
                .padding(5),
        ]
        .spacing(10),
        Some(SleepTimerMode::EndOfChapter) => row![
            text("Sleep:"),
            text("End of chapter"),
            horizontal_space(),
            button(text("Cancel")).on_press(Message::SleepTimerCancel),
        ]
        .spacing(10),
        Some(SleepTimerMode::Duration(_)) => {
            let remaining = state
                .sleep_timer
                .as_ref()
                .and_then(crate::models::SleepTimer::remaining_seconds)
                .unwrap_or(0);
            row![
                text("Sleep:"),
                text(format_remaining_seconds(remaining)),
                horizontal_space(),
                button(text("+15m")).on_press(Message::SleepTimerExtendSeconds(15 * 60)),
                button(text("Cancel")).on_press(Message::SleepTimerCancel),
            ]
            .spacing(10)
        }
    };

    column![
        // Currently playing label
        container(text(format!("Now Playing: {current_file_text}")).size(12)).padding(10),
        // Progress slider
        slider(
            0.0..=state.total_duration.max(1.0),
            state.current_time.min(state.total_duration),
            Message::SeekTo
        ),
        row![
            text(format_time(state.current_time)),
            horizontal_space(),
            text(format_time(state.total_duration)),
        ]
        .padding(5),
        // Control buttons and volume
        row![
            // Play/pause button
            button(text(play_pause_label).size(20))
                .on_press(Message::PlayPause)
                .padding(10),
            button(text("⏹").size(20))
                .on_press(Message::Stop)
                .padding(10),
            horizontal_space(),
            // Speed label and value
            text("Speed:"),
            button(text("0.5x"))
                .on_press(Message::SpeedChanged(0.5))
                .padding(5),
            button(text("0.75x"))
                .on_press(Message::SpeedChanged(0.75))
                .padding(5),
            button(text("1.0x"))
                .on_press(Message::SpeedChanged(1.0))
                .padding(5),
            button(text("1.25x"))
                .on_press(Message::SpeedChanged(1.25))
                .padding(5),
            button(text("1.5x"))
                .on_press(Message::SpeedChanged(1.5))
                .padding(5),
            button(text("2.0x"))
                .on_press(Message::SpeedChanged(2.0))
                .padding(5),
            horizontal_space(),
            // Volume controls
            text("Volume:"),
            slider(0..=200, state.volume, Message::VolumeChanged).width(Length::Fixed(150.0)),
            text(format!("{}%", state.volume)),
        ]
        .padding(15)
        .spacing(10),
        container(sleep_timer_row).padding(10)
    ]
    .padding(10)
    .into()
}

fn format_remaining_seconds(secs: i64) -> String {
    let secs = secs.max(0);
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{minutes}:{seconds:02}")
}

fn format_time(ms: f64) -> String {
    // Convert f64 milliseconds to i64 for time calculations
    let ms_i64 = f64_to_ms(ms).unwrap_or(0);
    let seconds = ms_i64 / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
    } else {
        format!("{}:{:02}", minutes, seconds % 60)
    }
}
