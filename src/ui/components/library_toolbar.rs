//! Library toolbar with search, sort, and filter controls.

use crate::ui::state::{AudiobookFilterStatus, AudiobookSortOrder};
use crate::ui::styles::{border_radius, button_styles, colors, shadows, spacing, typography};
use crate::ui::Message;
use iced::widget::{button, column, row, text, text_input, Space};
use iced::{Border, Element, Length};

/// Stable widget ID for the library search input.
#[must_use]
pub const fn search_input_id() -> iced::widget::Id {
    iced::widget::Id::new("library_search")
}

/// Renders the library toolbar with search bar and filter chips
#[must_use]
pub fn view(
    search_query: &str,
    sort_order: AudiobookSortOrder,
    filter_status: AudiobookFilterStatus,
) -> Element<'_, Message> {
    let search_bar = build_search_bar(search_query);
    let sort_buttons = build_sort_buttons(sort_order);
    let filter_chips = build_filter_chips(filter_status);
    let clear_button = build_clear_button(search_query, sort_order, filter_status);

    column![
        search_bar,
        row![
            text("Sort:")
                .size(typography::SIZE_SM)
                .style(|theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(text_secondary_color(theme)),
                }),
            sort_buttons,
        ]
        .spacing(spacing::SM)
        .align_y(iced::Alignment::Center),
        row![
            text("Filter:")
                .size(typography::SIZE_SM)
                .style(|theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(text_secondary_color(theme)),
                }),
            filter_chips,
            Space::new().width(Length::Fill),
            clear_button,
        ]
        .spacing(spacing::SM)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(spacing::SM)
    .padding(spacing::MD)
    .into()
}

fn build_search_bar(search_query: &str) -> Element<'_, Message> {
    text_input("Search audiobooks...", search_query)
        .id(search_input_id())
        .on_input(Message::SearchQueryChanged)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(|theme: &iced::Theme, status| search_bar_style(theme, status))
        .into()
}

fn search_bar_style(
    _theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    use iced::widget::text_input;
    match status {
        text_input::Status::Focused { .. } => text_input::Style {
            background: iced::Background::Color(colors::BG_SECONDARY),
            border: Border {
                color: colors::BORDER_FOCUS,
                width: 2.0,
                radius: border_radius::MD.into(),
            },
            icon: colors::TEXT_SECONDARY,
            placeholder: colors::TEXT_DISABLED,
            value: colors::TEXT_PRIMARY,
            selection: colors::SELECTION_BG,
        },
        _ => text_input::Style {
            background: iced::Background::Color(colors::BG_SECONDARY),
            border: Border {
                color: colors::BORDER_DEFAULT,
                width: 1.0,
                radius: border_radius::MD.into(),
            },
            icon: colors::TEXT_SECONDARY,
            placeholder: colors::TEXT_DISABLED,
            value: colors::TEXT_PRIMARY,
            selection: colors::SELECTION_BG,
        },
    }
}

fn build_sort_buttons(sort_order: AudiobookSortOrder) -> Element<'static, Message> {
    row![
        sort_button(
            "Recently Played",
            AudiobookSortOrder::RecentlyPlayed,
            sort_order
        ),
        sort_button("Title", AudiobookSortOrder::Title, sort_order),
        sort_button("Progress", AudiobookSortOrder::Progress, sort_order),
        sort_button("Date Added", AudiobookSortOrder::DateAdded, sort_order),
    ]
    .spacing(spacing::SM)
    .into()
}

fn build_filter_chips(filter_status: AudiobookFilterStatus) -> Element<'static, Message> {
    row![
        filter_chip("All", AudiobookFilterStatus::All, filter_status),
        filter_chip(
            "In Progress",
            AudiobookFilterStatus::InProgress,
            filter_status
        ),
        filter_chip("Complete", AudiobookFilterStatus::Complete, filter_status),
        filter_chip(
            "Not Started",
            AudiobookFilterStatus::NotStarted,
            filter_status
        ),
    ]
    .spacing(spacing::SM)
    .into()
}

fn build_clear_button(
    search_query: &str,
    sort_order: AudiobookSortOrder,
    filter_status: AudiobookFilterStatus,
) -> Element<'_, Message> {
    if search_query.is_empty()
        && sort_order == AudiobookSortOrder::default()
        && filter_status == AudiobookFilterStatus::default()
    {
        Space::new().width(Length::Shrink).into()
    } else {
        button(
            text("Clear Filters")
                .size(typography::SIZE_SM)
                .style(|theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(text_on_primary_color(theme)),
                }),
        )
        .on_press(Message::ClearFilters)
        .padding(spacing::SM)
        .style(button_styles::primary)
        .into()
    }
}

const fn text_secondary_color(_theme: &iced::Theme) -> iced::Color {
    colors::TEXT_SECONDARY
}

const fn text_on_primary_color(_theme: &iced::Theme) -> iced::Color {
    colors::TEXT_ON_PRIMARY
}

fn sort_button(
    label: &'static str,
    order: AudiobookSortOrder,
    current: AudiobookSortOrder,
) -> Element<'static, Message> {
    let is_selected = order == current;

    button(
        text(label)
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(if is_selected {
                    colors::TEXT_ON_PRIMARY
                } else {
                    colors::TEXT_PRIMARY
                }),
            }),
    )
    .on_press(Message::SortOrderChanged(order))
    .padding(iced::Padding::from([spacing::SM, spacing::MD]))
    .style(move |_theme: &iced::Theme, status| {
        use iced::widget::button;
        if is_selected {
            button::Style {
                background: Some(colors::PRIMARY_GRADIENT_START.into()),
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                text_color: colors::TEXT_ON_PRIMARY,
                shadow: shadows::shadow_sm(),
                ..Default::default()
            }
        } else {
            // Secondary style - outlined button with refined treatment
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(colors::BG_HOVER.into()),
                    border: Border {
                        color: colors::PRIMARY,
                        width: 1.0,
                        radius: border_radius::MD.into(),
                    },
                    text_color: colors::PRIMARY,
                    ..Default::default()
                },
                button::Status::Pressed => button::Style {
                    background: Some(colors::PRIMARY_HOVER.into()),
                    border: Border {
                        color: colors::PRIMARY_ACTIVE,
                        width: 1.0,
                        radius: border_radius::MD.into(),
                    },
                    text_color: colors::TEXT_ON_PRIMARY,
                    ..Default::default()
                },
                _ => button::Style {
                    background: Some(iced::Color::TRANSPARENT.into()),
                    border: Border {
                        color: colors::BORDER_DEFAULT,
                        width: 1.0,
                        radius: border_radius::MD.into(),
                    },
                    text_color: colors::TEXT_PRIMARY,
                    ..Default::default()
                },
            }
        }
    })
    .into()
}

fn filter_chip(
    label: &'static str,
    status: AudiobookFilterStatus,
    current: AudiobookFilterStatus,
) -> Element<'static, Message> {
    let is_selected = status == current;

    button(
        text(label)
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(if is_selected {
                    colors::TEXT_ON_PRIMARY
                } else {
                    colors::TEXT_PRIMARY
                }),
            }),
    )
    .on_press(Message::FilterStatusChanged(status))
    .padding(iced::Padding::from([spacing::SM, spacing::MD]))
    .style(move |_theme: &iced::Theme, status_arg| {
        use iced::widget::button;
        if is_selected {
            button::Style {
                background: Some(colors::ACCENT_GRADIENT_START.into()),
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: 1.0,
                    radius: border_radius::LG.into(),
                },
                text_color: colors::TEXT_ON_PRIMARY,
                shadow: shadows::shadow_sm(),
                ..Default::default()
            }
        } else {
            // Chip style - rounded button with subtle background
            match status_arg {
                button::Status::Hovered => button::Style {
                    background: Some(colors::BG_HOVER.into()),
                    border: Border {
                        color: colors::BORDER_DEFAULT,
                        width: 1.0,
                        radius: border_radius::LG.into(),
                    },
                    text_color: colors::TEXT_PRIMARY,
                    ..Default::default()
                },
                button::Status::Pressed => button::Style {
                    background: Some(colors::PRIMARY_HOVER.into()),
                    border: Border {
                        color: colors::PRIMARY_ACTIVE,
                        width: 1.0,
                        radius: border_radius::LG.into(),
                    },
                    text_color: colors::TEXT_ON_PRIMARY,
                    ..Default::default()
                },
                _ => button::Style {
                    background: Some(colors::BG_SECONDARY.into()),
                    border: Border {
                        color: colors::BORDER_DEFAULT,
                        width: 1.0,
                        radius: border_radius::LG.into(),
                    },
                    text_color: colors::TEXT_PRIMARY,
                    ..Default::default()
                },
            }
        }
    })
    .into()
}
