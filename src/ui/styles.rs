use iced::Color;

// Color palette based on the original application
pub const TOP_BAR_COLOR: Color = Color::from_rgb(0.996, 0.855, 0.325); // #FEDB53
pub const PLAYER_BG_COLOR: Color = Color::from_rgb(0.255, 0.255, 0.255); // #414141
pub const PLAYER_TEXT_COLOR: Color = Color::from_rgb(0.933, 0.933, 0.933); // #eee
pub const AUDIOBOOK_LIST_BG: Color = Color::from_rgb(0.933, 0.933, 0.933); // #eee
pub const FILE_LIST_BG: Color = Color::from_rgb(1.0, 1.0, 1.0); // white
pub const SELECTED_ITEM_BG: Color = Color::from_rgb(0.333, 0.318, 0.322); // #555152
pub const TEXT_COLOR: Color = Color::from_rgb(0.318, 0.318, 0.318); // #515151

#[must_use]
pub fn format_duration(ms: Option<i64>) -> String {
    match ms {
        Some(duration) if duration > 0 => {
            let total_seconds = duration / 1000;
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;

            if hours > 0 {
                format!("{hours}:{minutes:02}:{seconds:02}")
            } else {
                format!("{minutes}:{seconds:02}")
            }
        }
        _ => String::from("--:--"),
    }
}

#[must_use]
pub fn format_time_ms(ms: i64) -> String {
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}
