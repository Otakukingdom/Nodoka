use crate::ui::components::{audiobook_list, bookmarks, file_list, player_controls};
use crate::ui::styles::{border_radius, colors, shadows, spacing, typography};
use crate::ui::{settings_form, LoadState, Message, ScanState, State};
use iced::widget::{button, column, container, row, stack, text, Space};
use iced::{Border, Color, Element, Length};

/// Renders the main window with improved layout and visual hierarchy
///
/// Features:
/// - Top bar with application title and settings button
/// - Main content area with audiobook list, file list, and bookmarks
/// - Player controls at the bottom with proper elevation
/// - Modal overlays for settings and bookmark editor with backdrop
/// - Consistent spacing and design system colors throughout
#[must_use]
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let main_content = container(column![
        error_banner(state),
        scan_status_banner(state),
        top_bar(),
        library_panels(state),
        container(player_controls::view(state)).padding(spacing::SM),
    ])
    .padding(spacing::SM);

    let mut content: Element<'a, Message> = main_content.into();

    // Settings modal overlay with backdrop (iced 0.14)
    //
    // Uses the stack widget to layer the main content, a semi-transparent backdrop,
    // and the settings dialog. The backdrop provides visual indication that a modal
    // is active and can be clicked to dismiss the modal (click-outside-to-dismiss pattern).
    if state.settings_open {
        let settings_dialog = modal_card(settings_form::build_settings_dialog(state));
        content = stack![
            content,
            modal_backdrop(Message::CloseSettings),
            settings_dialog
        ]
        .into();
    }

    // Bookmark editor modal overlay with backdrop (iced 0.14)
    if let Some(editor) = state.bookmark_editor.as_ref() {
        let editor_dialog = modal_card(bookmarks::editor(editor));
        content = stack![
            content,
            modal_backdrop(Message::BookmarkEditorCancel),
            editor_dialog
        ]
        .into();
    }

    // Loading state indicator with backdrop (iced 0.14)
    if state.load_state == LoadState::Loading {
        let backdrop = modal_backdrop_without_click();
        let loading_message = modal_card(text("Loading...").size(typography::SIZE_LG));
        content = stack![content, backdrop, loading_message].into();
    }

    content
}

fn error_banner(state: &State) -> Element<'_, Message> {
    state.error_message.as_ref().map_or_else(
        || container(text("")).height(Length::Shrink).into(),
        |error| {
            container(
                row![
                    text("[ERROR] ").size(typography::SIZE_BASE),
                    text(error).size(typography::SIZE_SM),
                    Space::new().width(Length::Fill),
                    button(text("Dismiss").size(typography::SIZE_XS))
                        .on_press(Message::DismissError)
                        .padding(spacing::XS),
                ]
                .padding(spacing::MD)
                .spacing(spacing::SM),
            )
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(colors::ERROR.into()),
                text_color: Some(colors::TEXT_ON_PRIMARY),
                ..Default::default()
            })
            .width(Length::Fill)
            .into()
        },
    )
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
            text(prefix).size(typography::SIZE_SM),
            text(dir_text).size(typography::SIZE_XS),
        ]
        .spacing(spacing::SM),
    )
    .padding(spacing::MD)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(colors::INFO.into()),
        text_color: Some(colors::TEXT_ON_PRIMARY),
        ..Default::default()
    })
    .width(Length::Fill)
    .into()
}

fn top_bar() -> Element<'static, Message> {
    container(
        row![
            text("Nodoka Audiobook Reader").size(typography::SIZE_XXL),
            Space::new().width(Length::Fill),
            button(text("Settings").size(typography::SIZE_SM))
                .on_press(Message::OpenSettings)
                .padding(spacing::MD),
        ]
        .padding(spacing::MD),
    )
    .into()
}

fn library_panels(state: &State) -> Element<'_, Message> {
    let audiobook_list_widget = audiobook_list::view(
        &state.audiobooks,
        state.selected_audiobook,
        &state.cover_thumbnails,
    );
    let file_list_widget = file_list::view(&state.current_files, state.selected_file.as_ref());
    let bookmarks_widget = bookmarks::view(&state.bookmarks, &state.current_files);

    row![
        container(audiobook_list_widget)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(spacing::SM),
        container(column![file_list_widget, bookmarks_widget].spacing(spacing::SM))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(spacing::SM),
    ]
    .height(Length::Fill)
    .spacing(spacing::SM)
    .into()
}

fn modal_backdrop(on_press: Message) -> Element<'static, Message> {
    button(container(text("")).width(Length::Fill).height(Length::Fill))
        .style(|_theme: &iced::Theme, _status| button::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        })
        .on_press(on_press)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn modal_backdrop_without_click() -> Element<'static, Message> {
    container(text(""))
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn modal_card<'a, E>(content: E) -> Element<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    container(content)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(colors::BG_SECONDARY.into()),
            border: Border {
                color: shadows::MD_BORDER,
                width: 1.0,
                radius: border_radius::LG.into(),
            },
            ..Default::default()
        })
        .padding(spacing::MD)
        .center_x(Length::Fill)
        .into()
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
    //! 4. **Manual accessibility testing**: Checklist for screen reader testing
    //! 5. **Integration tests**: Test full user workflows end-to-end
    //! 6. **Contrast ratio verification**: Automated WCAG compliance checking
    //!
    //! ## Manual Testing Checklist
    //!
    //! Since automated UI testing has limitations, manual verification is required for:
    //!
    //! ### Selection States
    //! - [ ] Click audiobook in list - item has visible selection background color
    //! - [ ] Click file in list - item has visible selection background with border
    //! - [ ] Selected items remain highlighted after clicking elsewhere
    //!
    //! ### Hover States
    //! - [ ] Hover over any button - button shows visual feedback (color change or highlight)
    //! - [ ] Hover transitions are smooth (150-300ms duration)
    //!
    //! ### Button Hierarchy
    //! - [ ] Primary buttons are visually distinct (Play, Save, Add use primary color)
    //! - [ ] Secondary buttons have different style (Cancel, Close have borders)
    //! - [ ] Danger buttons use error color (Delete, Remove are red/error colored)
    //!
    //! ### Modal Backdrops
    //! - [ ] Open settings - modal appears centered with border, elevation, and dimmed background
    //! - [ ] Click outside modal - modal closes (click-outside-to-dismiss pattern)
    //! - [ ] Open bookmark editor - modal appears centered with backdrop
    //!
    //! ### Loading States
    //! - [ ] Add/rescan directory - "Scanning..." message appears during operation
    //! - [ ] Scanning message shows directory path being scanned
    //! - [ ] Scanning message disappears when operation completes
    //!
    //! ### Error Messages
    //! - [ ] Trigger scan error - error banner appears at top with message
    //! - [ ] Click "Dismiss" on error - error banner disappears
    //! - [ ] Error banner has warning icon and proper error color
    //!
    //! ### Keyboard Navigation
    //! - [ ] Press Space - play/pause toggles
    //! - [ ] Press ← while playing - seeks back 5 seconds
    //! - [ ] Press → while playing - seeks forward 5 seconds
    //! - [ ] Press ↓ with multiple files - next file plays
    //! - [ ] Press ↑ with multiple files - previous file plays
    //! - [ ] Press Escape with modal open - modal closes
    //!
    //! ### Accessibility
    //! - [ ] Tab through interface - all interactive elements reachable
    //! - [ ] Focus indicators visible on interactive elements (framework-dependent)
    //! - [ ] Text contrast meets WCAG AA (4.5:1 for body text)
    //! - [ ] All buttons have descriptive labels (not just icons)
    //! - [ ] Error messages are clear and actionable
    //!
    //! ### Resolved in iced 0.14
    //! - Modal backdrops now use stack widget for proper layering and click handling
    //! - Focus indicators available through native button theme focus state
    //! - Native button styling eliminates container-based workarounds

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
            settings_open: true,
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

        let state = State {
            audiobooks: vec![Audiobook {
                id: Some(1),
                directory: "/test".to_string(),
                name: "Test Book".to_string(),
                full_path: "/test/book".to_string(),
                completeness: 50,
                default_order: 0,
                selected_file: None,
                created_at: chrono::Utc::now(),
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
            settings_open: true,
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

        let state = State {
            audiobooks: vec![
                Audiobook {
                    id: Some(1),
                    directory: "/test".to_string(),
                    name: "Book 1".to_string(),
                    full_path: "/test/book1".to_string(),
                    completeness: 25,
                    default_order: 0,
                    selected_file: None,
                    created_at: chrono::Utc::now(),
                },
                Audiobook {
                    id: Some(2),
                    directory: "/test".to_string(),
                    name: "Book 2".to_string(),
                    full_path: "/test/book2".to_string(),
                    completeness: 75,
                    default_order: 0,
                    selected_file: None,
                    created_at: chrono::Utc::now(),
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
