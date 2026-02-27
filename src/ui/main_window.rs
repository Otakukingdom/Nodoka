//! Top-level window layout coordinator for the Nodoka application.
//!
//! The [`view`] function is the single entry point called by the iced runtime
//! for every render cycle.  It composes the library panels, player controls,
//! modal overlays, and notification toasts into one coherent layout.

use crate::ui::components::{
    audiobook_list, bookmarks, continue_listening, file_list, library_toolbar, player_controls,
};
use crate::ui::styles::{
    border_radius, button_styles, colors, focus_indicator, shadows, spacing, typography,
};
use crate::ui::{settings_form, FocusedElement, LoadState, Message, ScanState, State};
use iced::widget::{button, column, container, mouse_area, row, stack, text, Space};
use iced::{Border, Color, Element, Length};

use super::{modal_views, notification_views};

/// Renders the main window with improved layout and visual hierarchy
///
/// Features:
/// - Top bar with application title and settings button
/// - Main content area with audiobook list, file list, and bookmarks
/// - Player controls at the bottom with proper elevation
/// - Modal overlays for settings and bookmark editor with backdrop
/// - Notification toasts in top-right corner (up to 3 concurrent)
/// - Consistent spacing and design system colors throughout
#[must_use]
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let main_content = container(
        column![
            scan_status_banner(state),
            top_bar(state),
            library_panels(state),
            container(player_controls::view(state))
                .padding(spacing::SM)
                .style(|_theme| container::Style {
                    background: Some(colors::BG_ELEVATED.into()),
                    shadow: shadows::shadow_md(),
                    border: Border {
                        radius: border_radius::MD.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        ]
        .spacing(spacing::MD), // MD spacing (16px) between major sections for visual hierarchy
    )
    .padding(spacing::SM);

    let mut content: Element<'a, Message> = main_content.into();

    // Settings modal overlay with backdrop (iced 0.14)
    //
    // Uses the stack widget to layer the main content, a semi-transparent backdrop,
    // and the settings dialog. The backdrop provides visual indication that a modal
    // is active and can be clicked to dismiss the modal (click-outside-to-dismiss pattern).
    if state.active_modal == crate::ui::ActiveModal::Settings {
        let scale = state.modal_state.content_scale();
        let opacity = state.modal_state.content_opacity();
        let settings_dialog =
            modal_views::modal_card(settings_form::build_settings_dialog(state, scale), opacity);
        content = stack![
            content,
            modal_views::modal_backdrop(
                state.modal_state.backdrop_opacity(),
                Message::CloseSettings
            ),
            settings_dialog
        ]
        .into();
    }

    // Bookmark editor modal overlay with backdrop (iced 0.14)
    if let Some(editor) = state.bookmark_editor.as_ref() {
        let scale = state.modal_state.content_scale();
        let opacity = state.modal_state.content_opacity();
        let editor_dialog = modal_views::modal_card(
            bookmarks::editor(editor, state.focused_element, scale),
            opacity,
        );
        content = stack![
            content,
            modal_views::modal_backdrop(
                state.modal_state.backdrop_opacity(),
                Message::BookmarkEditorCancel
            ),
            editor_dialog
        ]
        .into();
    }

    // Keyboard help overlay with backdrop (iced 0.14)
    if state.active_modal == crate::ui::ActiveModal::KeyboardHelp {
        let scale = state.modal_state.content_scale();
        let opacity = state.modal_state.content_opacity();
        let help_dialog = modal_views::modal_card(
            modal_views::keyboard_help_view(state.focused_element, scale),
            opacity,
        );
        content = stack![
            content,
            modal_views::modal_backdrop(
                state.modal_state.backdrop_opacity(),
                Message::HideKeyboardHelp
            ),
            help_dialog
        ]
        .into();
    }

    // Loading state indicator with backdrop (iced 0.14)
    if state.load_state == LoadState::Loading {
        let backdrop = modal_views::modal_backdrop_without_click(0.5);
        let loading_message =
            modal_views::modal_card(text("Loading...").size(typography::SIZE_LG), 1.0);
        content = stack![content, backdrop, loading_message].into();
    }

    // Notification toasts overlay in top-right corner (iced 0.14)
    // Shows up to 3 concurrent notifications with auto-dismiss
    if !state.notification_queue.is_empty() {
        content = stack![content, notification_views::notification_area(state)].into();
    }

    if let Some(tooltip) = state.tooltip_state.visible() {
        content = stack![content, modal_views::tooltip_overlay(tooltip)].into();
    }

    content
}

fn scan_status_banner(state: &State) -> Element<'_, Message> {
    let (prefix, dir_text) = match &state.scan_state {
        ScanState::Idle => return container(text("")).into(),
        ScanState::Scanning { directory } => directory
            .as_deref()
            .map_or(("Scanning...", ""), |dir| ("Scanning: ", dir)),
    };

    container(
        row![
            text(prefix)
                .size(typography::SIZE_SM)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_ON_PRIMARY),
                }),
            text(dir_text)
                .size(typography::SIZE_XS)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                }),
        ]
        .spacing(spacing::SM),
    )
    .padding(spacing::MD)
    .style(|_theme: &iced::Theme| container::Style {
        // Enhanced gradient background
        background: Some(Color::from_rgb(0.0, 0.52, 0.78).into()),
        text_color: Some(colors::TEXT_ON_PRIMARY),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
            width: 1.5,
            radius: border_radius::SM.into(),
        },
        shadow: shadows::shadow_sm(),
        ..Default::default()
    })
    .width(Length::Fill)
    .into()
}

fn top_bar(state: &State) -> Element<'static, Message> {
    let settings_focused = state.focused_element == FocusedElement::SettingsButton;
    let settings_button = mouse_area(
        button(text("Settings").size(typography::SIZE_SM))
            .on_press(Message::OpenSettings)
            .padding(spacing::MD)
            .style(if settings_focused {
                button_styles::secondary_focused
            } else {
                button_styles::secondary
            }),
    )
    .on_enter(Message::TooltipHoverStart(
        crate::ui::tooltip::TooltipTarget::Settings,
    ))
    .on_exit(Message::TooltipHoverEnd);

    container(
        row![
            // Enhanced logo with distinctive treatment
            container(
                text("♪")
                    .size(typography::SIZE_BASE)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::PRIMARY),
                    }),
            )
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(24.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(colors::PRIMARY_GRADIENT_START.into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::SM.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            }),
            column![
                text("Nodoka")
                    .size(typography::SIZE_XXL)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_PRIMARY),
                    }),
                text("Audiobook Reader")
                    .size(typography::SIZE_XS)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_SECONDARY),
                    }),
            ]
            .spacing(0.0),
            Space::new().width(Length::Fill),
            settings_button,
        ]
        .spacing(spacing::MD)
        .align_y(iced::Alignment::Center)
        .padding(spacing::LG),
    )
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(colors::BG_FLOATING.into()),
        border: Border {
            color: colors::BORDER_SUBTLE,
            width: 1.0,
            radius: border_radius::MD.into(),
        },
        shadow: shadows::shadow_md(),
        ..Default::default()
    })
    .into()
}

/// Builds the library column with toolbar, Continue Listening, and audiobook list
fn build_library_column(state: &State) -> Element<'_, Message> {
    let audiobook_elapsed =
        list_animation_elapsed_ms(state.audiobook_list_animation_start_ms, state.ui_time_ms);

    let last_audiobook = state
        .audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    let toolbar = library_toolbar::view(&state.search_query, state.sort_order, state.filter_status);
    let continue_listening = continue_listening::view(last_audiobook, &state.cover_thumbnails);
    let audiobook_list = audiobook_list::view(
        &state.audiobooks,
        state.selected_audiobook,
        &state.cover_thumbnails,
        &state.scan_state,
        audiobook_elapsed,
        state.prefers_reduced_motion,
        &state.search_query,
        state.sort_order,
        state.filter_status,
    );

    column![toolbar, continue_listening, audiobook_list]
        .spacing(spacing::MD)
        .into()
}

/// Builds the detail panel with file list and bookmarks
fn build_detail_panel(state: &State) -> Element<'_, Message> {
    let file_elapsed =
        list_animation_elapsed_ms(state.file_list_animation_start_ms, state.ui_time_ms);
    let bookmark_elapsed =
        list_animation_elapsed_ms(state.bookmark_list_animation_start_ms, state.ui_time_ms);

    let file_list = file_list::view(
        &state.current_files,
        state.selected_file.as_ref(),
        file_elapsed,
        state.prefers_reduced_motion,
    );
    let bookmarks = bookmarks::view(
        &state.bookmarks,
        &state.current_files,
        bookmark_elapsed,
        state.prefers_reduced_motion,
    );

    column![file_list, bookmarks].spacing(spacing::SM).into()
}

/// Creates a draggable divider between panels with enhanced visual treatment.
fn panel_divider(width: f32) -> Element<'static, Message> {
    use iced::widget::mouse_area;

    let divider = container(Space::new())
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(colors::BORDER_SUBTLE.into()),
            ..Default::default()
        });

    mouse_area(divider)
        .on_press(Message::PanelDividerDragStart)
        .into()
}

/// Builds the narrow window layout with vertically stacked panels
fn build_narrow_layout(state: &State) -> Element<'_, Message> {
    let left_panel = build_library_column(state);
    let right_panel = build_detail_panel(state);

    let audiobook_focused = state.focused_element == FocusedElement::AudiobookList;
    let right_focused = matches!(
        state.focused_element,
        FocusedElement::FileList | FocusedElement::BookmarkList
    );

    column![
        container(left_panel)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .padding(spacing::SM)
            .style(focus_indicator(audiobook_focused)),
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(4.0))
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(colors::BORDER_DEFAULT.into()),
                ..Default::default()
            }),
        container(right_panel)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .padding(spacing::SM)
            .style(focus_indicator(right_focused)),
    ]
    .height(Length::Fill)
    .spacing(0)
    .into()
}

/// Builds the wide window layout with side-by-side panels or focus mode
fn build_wide_layout(state: &State) -> Element<'_, Message> {
    let audiobook_focused = state.focused_element == FocusedElement::AudiobookList;
    let right_focused = matches!(
        state.focused_element,
        FocusedElement::FileList | FocusedElement::BookmarkList
    );

    let (left_portion, right_portion, show_left_panel) =
        if state.focus_mode && state.playback == crate::ui::PlaybackStatus::Playing {
            (0_u16, 10_u16, false)
        } else {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let left_portion = (state.panel_ratio * 10.0) as u16;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let right_portion = ((1.0 - state.panel_ratio) * 10.0) as u16;
            (left_portion.max(1), right_portion.max(1), true)
        };

    if show_left_panel {
        let left_panel = build_library_column(state);
        let right_panel = build_detail_panel(state);
        let divider = panel_divider(4.0);

        row![
            container(left_panel)
                .width(Length::FillPortion(left_portion))
                .height(Length::Fill)
                .padding(spacing::SM)
                .style(focus_indicator(audiobook_focused)),
            divider,
            container(right_panel)
                .width(Length::FillPortion(right_portion))
                .height(Length::Fill)
                .padding(spacing::SM)
                .style(focus_indicator(right_focused)),
        ]
        .height(Length::Fill)
        .spacing(0)
        .into()
    } else {
        let right_panel = build_detail_panel(state);
        row![container(right_panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(spacing::SM)
            .style(focus_indicator(right_focused)),]
        .height(Length::Fill)
        .spacing(spacing::SM)
        .into()
    }
}

fn library_panels(state: &State) -> Element<'_, Message> {
    const NARROW_THRESHOLD: u32 = 800;

    if state.window_width < NARROW_THRESHOLD {
        build_narrow_layout(state)
    } else {
        build_wide_layout(state)
    }
}

fn list_animation_elapsed_ms(start_ms: Option<u64>, now_ms: u64) -> u64 {
    start_ms.map_or(0, |start| now_ms.saturating_sub(start))
}

#[cfg(test)]
mod tests {
    //! UI Testing Strategy and Limitations
    //!
    //! ## Testing Approach
    //!
    //! The iced framework uses an opaque `Element` type that doesn't expose internal structure
    //! for assertions. This limits what we can test directly in UI components. Our testing
    //! strategy focuses on:
    //!
    //! 1. **View rendering tests**: Verify that view functions don't panic with various state
    //!    configurations (empty, populated, edge cases)
    //! 2. **Data formatting tests**: Test helper functions that format time, duration, etc.
    //! 3. **State update tests**: Test message handling via the `update()` function to verify
    //!    correct state transitions
    //! 4. **Conditional rendering tests**: Verify that conditional UI elements render without
    //!    panic based on state
    //!
    //! ## What We CAN Test
    //!
    //! - View functions render without panicking
    //! - State updates produce correct values
    //! - Data formatting functions produce correct strings
    //! - Message routing works correctly
    //! - Edge cases don't cause crashes (zero duration, negative values, etc.)
    //!
    //! ## What We CANNOT Test (Framework Limitations)
    //!
    //! - Pixel-perfect rendering (requires full rendering engine)
    //! - Actual event handling (requires event simulation)
    //! - Visual appearance (colors, spacing applied correctly)
    //! - Hover states and transitions
    //! - Focus management
    //! - Keyboard navigation order
    //! - Accessibility attributes (ARIA labels, roles)
    //!
    //! ## Future Improvements
    //!
    //! The following testing enhancements could be added in the future:
    //!
    //! 1. **Visual regression testing**: Screenshot comparison with baseline images
    //!    (requires rendering engine integration)
    //! 2. **Property-based testing**: Use quickcheck/proptest for state transition testing
    //! 3. **Performance testing**: Measure rendering time for large lists (1000+ items)
    //! 4. **Integration tests**: Test full user workflows end-to-end
    //! 5. **Contrast ratio verification**: Automated WCAG compliance checking

    use super::*;

    #[test]
    fn test_view_renders_default_state() {
        let state = State::default();
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_settings_open() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_bookmark_editor() {
        use crate::ui::state::BookmarkEditor;

        let state = State {
            bookmark_editor: Some(BookmarkEditor {
                id: None,
                audiobook_id: 1,
                file_path: "/path/file.mp3".to_string(),
                position_ms: 1000,
                label: "Test".to_string(),
                note: String::new(),
            }),
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_loading_state() {
        let state = State {
            load_state: LoadState::Loading,
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_audiobooks() {
        use crate::models::Audiobook;
        use chrono::Utc;

        let state = State {
            audiobooks: vec![Audiobook {
                id: Some(1),
                directory: "/test".to_string(),
                name: "Test Book".to_string(),
                author: None,
                full_path: "/test/book".to_string(),
                completeness: 50,
                default_order: 0,
                selected_file: None,
                total_duration_ms: None,
                remaining_duration_ms: None,
                created_at: Utc::now(),
                last_listened: None,
            }],
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_files() {
        use crate::models::AudiobookFile;

        let state = State {
            current_files: vec![AudiobookFile {
                audiobook_id: 1,
                name: "chapter1.mp3".to_string(),
                full_path: "/test/chapter1.mp3".to_string(),
                length_of_file: Some(3_600_000),
                seek_position: None,
                checksum: None,
                position: 0,
                completeness: 0,
                file_exists: true,
                created_at: chrono::Utc::now(),
            }],
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_bookmarks() {
        use crate::models::Bookmark;

        let state = State {
            bookmarks: vec![Bookmark {
                id: Some(1),
                audiobook_id: 1,
                file_path: "/test/file.mp3".to_string(),
                position_ms: 1000,
                label: "Test Bookmark".to_string(),
                note: Some(String::new()),
                created_at: chrono::Utc::now(),
            }],
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_playing_state() {
        let state = State {
            playback: crate::ui::PlaybackStatus::Playing,
            selected_file: Some("/test/file.mp3".to_string()),
            current_time: 30000.0,
            total_duration: 3_600_000.0,
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_multiple_modals() {
        use crate::ui::state::BookmarkEditor;

        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            bookmark_editor: Some(BookmarkEditor {
                id: None,
                audiobook_id: 1,
                file_path: "/path/file.mp3".to_string(),
                position_ms: 1000,
                label: "Test".to_string(),
                note: String::new(),
            }),
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_complex_state() {
        use crate::models::{Audiobook, AudiobookFile, Bookmark, Directory};
        use chrono::Utc;

        let state = State {
            audiobooks: vec![
                Audiobook {
                    id: Some(1),
                    directory: "/test".to_string(),
                    name: "Book 1".to_string(),
                    author: None,
                    full_path: "/test/book1".to_string(),
                    completeness: 25,
                    default_order: 0,
                    selected_file: None,
                    total_duration_ms: None,
                    remaining_duration_ms: None,
                    created_at: Utc::now(),
                    last_listened: None,
                },
                Audiobook {
                    id: Some(2),
                    directory: "/test".to_string(),
                    name: "Book 2".to_string(),
                    author: None,
                    full_path: "/test/book2".to_string(),
                    completeness: 50,
                    default_order: 1,
                    selected_file: None,
                    total_duration_ms: None,
                    remaining_duration_ms: None,
                    created_at: Utc::now(),
                    last_listened: None,
                },
                Audiobook {
                    id: Some(2),
                    directory: "/test".to_string(),
                    name: "Book 2".to_string(),
                    author: None,
                    full_path: "/test/book2".to_string(),
                    completeness: 50,
                    default_order: 1,
                    selected_file: None,
                    total_duration_ms: None,
                    remaining_duration_ms: None,
                    created_at: Utc::now(),
                    last_listened: None,
                },
                Audiobook {
                    id: Some(2),
                    directory: "/test".to_string(),
                    name: "Book 2".to_string(),
                    author: None,
                    full_path: "/test/book2".to_string(),
                    completeness: 75,
                    default_order: 0,
                    selected_file: None,
                    total_duration_ms: None,
                    remaining_duration_ms: None,
                    created_at: chrono::Utc::now(),
                    last_listened: None,
                },
            ],
            current_files: vec![AudiobookFile {
                audiobook_id: 1,
                name: "chapter1.mp3".to_string(),
                full_path: "/test/book1/chapter1.mp3".to_string(),
                length_of_file: Some(3_600_000),
                seek_position: Some(3_600_000),
                checksum: None,
                position: 0,
                completeness: 100,
                file_exists: true,
                created_at: chrono::Utc::now(),
            }],
            bookmarks: vec![Bookmark {
                id: Some(1),
                audiobook_id: 1,
                file_path: "/test/book1/chapter1.mp3".to_string(),
                position_ms: 1000,
                label: "Important".to_string(),
                note: Some("Note".to_string()),
                created_at: chrono::Utc::now(),
            }],
            directories: vec![Directory::new("/test".to_string())],
            selected_audiobook: Some(1),
            selected_file: Some("/test/book1/chapter1.mp3".to_string()),
            playback: crate::ui::PlaybackStatus::Playing,
            current_time: 1500.0,
            total_duration: 3_600_000.0,
            volume: 100,
            speed: 1.0,
            ..Default::default()
        };
        let element = view(&state);
        drop(element);
    }
}
