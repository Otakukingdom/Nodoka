use crate::conversions::f64_to_ms;
use crate::ui::{Message, NodokaState};
use iced::widget::{button, column, container, horizontal_space, row, slider, text};
use iced::{Element, Length};

pub fn build_player_controls(state: &NodokaState) -> Element<'static, Message> {
    let current_file_text = state
        .selected_file
        .as_ref()
        .and_then(|f| std::path::Path::new(f).file_name())
        .and_then(|f| f.to_str())
        .unwrap_or("No file selected");

    let play_pause_label = if state.is_playing { "⏸" } else { "▶" };

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
            slider(0..=100, state.volume, Message::VolumeChanged).width(Length::Fixed(150.0)),
            text(format!("{}%", state.volume)),
        ]
        .padding(15)
        .spacing(10)
    ]
    .padding(10)
    .into()
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
