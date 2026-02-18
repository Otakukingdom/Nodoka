use crate::ui::components::{audiobook_list, bookmarks, file_list, player_controls};
use crate::ui::styles::{
    border_radius, button_styles, colors, focus_indicator, shadows, spacing, typography,
};
use crate::ui::tooltip::{TooltipAnchor, TooltipPosition};
use crate::ui::{settings_form, FocusedElement, LoadState, Message, ScanState, State};
use iced::widget::{button, column, container, mouse_area, row, stack, text, Space};
use iced::{Border, Color, Element, Length};

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
            modal_card(settings_form::build_settings_dialog(state, scale), opacity);
        content = stack![
            content,
            modal_backdrop(state.modal_state.backdrop_opacity(), Message::CloseSettings),
            settings_dialog
        ]
        .into();
    }

    // Bookmark editor modal overlay with backdrop (iced 0.14)
    if let Some(editor) = state.bookmark_editor.as_ref() {
        let scale = state.modal_state.content_scale();
        let opacity = state.modal_state.content_opacity();
        let editor_dialog = modal_card(
            bookmarks::editor(editor, state.focused_element, scale),
            opacity,
        );
        content = stack![
            content,
            modal_backdrop(
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
        let help_dialog = modal_card(keyboard_help_view(state.focused_element, scale), opacity);
        content = stack![
            content,
            modal_backdrop(
                state.modal_state.backdrop_opacity(),
                Message::HideKeyboardHelp
            ),
            help_dialog
        ]
        .into();
    }

    // Loading state indicator with backdrop (iced 0.14)
    if state.load_state == LoadState::Loading {
        let backdrop = modal_backdrop_without_click(0.5);
        let loading_message = modal_card(text("Loading...").size(typography::SIZE_LG), 1.0);
        content = stack![content, backdrop, loading_message].into();
    }

    // Notification toasts overlay in top-right corner (iced 0.14)
    // Shows up to 3 concurrent notifications with auto-dismiss
    if !state.notification_queue.is_empty() {
        content = stack![content, notification_area(state)].into();
    }

    if let Some(tooltip) = state.tooltip_state.visible() {
        content = stack![content, tooltip_overlay(tooltip)].into();
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
            text("Nodoka Audiobook Reader").size(typography::SIZE_XXL),
            Space::new().width(Length::Fill),
            settings_button,
        ]
        .padding(spacing::LG), // LG padding (24px) for prominent top bar
    )
    .into()
}

fn library_panels(state: &State) -> Element<'_, Message> {
    let audiobook_elapsed =
        list_animation_elapsed_ms(state.audiobook_list_animation_start_ms, state.ui_time_ms);
    let file_elapsed =
        list_animation_elapsed_ms(state.file_list_animation_start_ms, state.ui_time_ms);
    let bookmark_elapsed =
        list_animation_elapsed_ms(state.bookmark_list_animation_start_ms, state.ui_time_ms);

    let audiobook_list_widget = audiobook_list::view(
        &state.audiobooks,
        state.selected_audiobook,
        &state.cover_thumbnails,
        &state.scan_state,
        audiobook_elapsed,
        state.prefers_reduced_motion,
    );
    let file_list_widget = file_list::view(
        &state.current_files,
        state.selected_file.as_ref(),
        file_elapsed,
        state.prefers_reduced_motion,
    );
    let bookmarks_widget = bookmarks::view(
        &state.bookmarks,
        &state.current_files,
        bookmark_elapsed,
        state.prefers_reduced_motion,
    );

    let audiobook_panel_focused = state.focused_element == FocusedElement::AudiobookList;
    let right_panel_focused = matches!(
        state.focused_element,
        FocusedElement::FileList | FocusedElement::BookmarkList
    );

    row![
        container(audiobook_list_widget)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(spacing::SM)
            .style(focus_indicator(audiobook_panel_focused)),
        container(column![file_list_widget, bookmarks_widget].spacing(spacing::SM))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(spacing::SM)
            .style(focus_indicator(right_panel_focused)),
    ]
    .height(Length::Fill)
    .spacing(spacing::SM)
    .into()
}

fn list_animation_elapsed_ms(start_ms: Option<u64>, now_ms: u64) -> u64 {
    start_ms.map_or(0, |start| now_ms.saturating_sub(start))
}

fn modal_backdrop(opacity: f32, on_press: Message) -> Element<'static, Message> {
    button(container(text("")).width(Length::Fill).height(Length::Fill))
        .style(move |_theme: &iced::Theme, _status| button::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, opacity).into()),
            ..Default::default()
        })
        .on_press(on_press)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn modal_backdrop_without_click(opacity: f32) -> Element<'static, Message> {
    container(text(""))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, opacity).into()),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Modal card wrapper with elevated styling
///
/// Renders the keyboard shortcuts help overlay
///
/// Displays all available keyboard shortcuts in a modal dialog.
/// Activated by pressing F1 key. Can be dismissed by pressing Escape
/// or clicking outside the dialog.
fn keyboard_help_view(focused_element: FocusedElement, scale: f32) -> Element<'static, Message> {
    // Platform-specific bookmark shortcut
    #[cfg(target_os = "macos")]
    let bookmark_shortcut = "Cmd+B";
    #[cfg(not(target_os = "macos"))]
    let bookmark_shortcut = "Ctrl+B";

    let shortcuts = column![
        // Header
        text("Keyboard Shortcuts")
            .size(typography::SIZE_XXL)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_PRIMARY),
            }),
        container(text("")).height(Length::Fixed(spacing::LG)),
        // Playback controls group
        shortcut_group(
            "Playback",
            vec![
                ("Space", "Play / Pause"),
                ("← →", "Seek backward / forward 5s"),
            ]
        ),
        // Navigation controls group
        shortcut_group(
            "Navigation",
            vec![
                ("↑ ↓", "Previous / Next file"),
                ("Tab", "Cycle through UI elements"),
            ]
        ),
        // Library controls group
        shortcut_group(
            "Library",
            vec![
                (bookmark_shortcut, "Create bookmark"),
                ("Escape", "Close modal"),
                ("F1", "Show this help"),
            ]
        ),
        container(text("")).height(Length::Fixed(spacing::LG)),
        // Close button
        button(text("Close").size(typography::SIZE_BASE))
            .on_press(Message::HideKeyboardHelp)
            .padding(spacing::MD)
            .style(
                if focused_element == FocusedElement::KeyboardHelpCloseButton {
                    button_styles::secondary_focused
                } else {
                    button_styles::secondary
                }
            )
            .width(Length::Fill)
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(500.0 * scale));

    container(shortcuts).padding(spacing::XL).into()
}

/// Renders a grouped section of keyboard shortcuts with styled keys
fn shortcut_group(
    title: &'static str,
    shortcuts: Vec<(&'static str, &'static str)>,
) -> Element<'static, Message> {
    let mut group = column![text(title)
        .size(typography::SIZE_LG)
        .style(|_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(colors::TEXT_PRIMARY),
        })]
    .spacing(spacing::SM);

    for (key, description) in shortcuts {
        group = group.push(
            row![
                // Styled keyboard key (bordered container)
                container(
                    text(key)
                        .size(typography::SIZE_SM)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(colors::TEXT_PRIMARY),
                        })
                )
                .padding(iced::Padding::from([spacing::XS, spacing::SM]))
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(colors::BG_ELEVATED.into()),
                    border: Border {
                        color: colors::BORDER_DEFAULT,
                        width: 1.0,
                        radius: border_radius::SM.into(),
                    },
                    ..Default::default()
                }),
                Space::new().width(Length::Fixed(spacing::MD)),
                // Description text
                text(description)
                    .size(typography::SIZE_SM)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_SECONDARY),
                    }),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
        );
    }

    container(group)
        .padding(iced::Padding::from([spacing::SM, 0.0]))
        .into()
}

/// Applies design system styling for modal overlays per ui-ux-pro-max:
/// - White background (`BG_SECONDARY`) for contrast against backdrop
/// - LG border radius (12px) for modern appearance
/// - Shadow for depth perception and visual elevation
/// - MD padding (16px) for content spacing
/// - Centered both horizontally and vertically
fn modal_card<'a, E>(content: E, opacity: f32) -> Element<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    use iced::Shadow;
    let background = with_alpha(colors::BG_SECONDARY, opacity);
    let border_color = with_alpha(shadows::MD_BORDER, opacity);
    container(content)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(background.into()),
            border: Border {
                color: border_color,
                width: 2.0,                       // Increased to 2px for better definition
                radius: border_radius::LG.into(), // LG radius (12px) for modern modal appearance
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2 * opacity),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
            ..Default::default()
        })
        .padding(spacing::MD)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

/// Renders notification toasts in top-right corner per ui-ux-pro-max
///
/// Features:
/// - Displays up to 3 concurrent notifications
/// - Auto-dismiss after duration specified in Notification
/// - Fade-in/fade-out animations using `TransitionState`
/// - Semantic colors based on notification type
/// - Non-intrusive positioning in top-right corner
fn notification_area(state: &State) -> Element<'_, Message> {
    // Take up to 3 most recent notifications
    let notifications_to_show = state
        .notification_queue
        .iter()
        .take(3)
        .fold(column![].spacing(spacing::SM), |col, notification| {
            col.push(notification_toast(notification))
        });

    // Position in top-right corner with padding
    let notifications = container(notifications_to_show)
        .width(Length::Fixed(320.0))
        .padding(spacing::MD);

    container(row![Space::new().width(Length::Fill), notifications])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            top: spacing::XL,
            right: spacing::XL,
            bottom: 0.0,
            left: 0.0,
        })
        .style(|_theme: &iced::Theme| container::Style {
            background: None, // Transparent background for overlay
            ..Default::default()
        })
        .into()
}

/// Renders a single notification toast with semantic colors
///
/// Enhanced features (v0.3.0):
/// - Title + message support for two-line notifications
/// - Countdown progress bar for auto-dismiss notifications
/// - Action button support for contextual operations
/// - Dismiss button for manual close
fn notification_toast(notification: &crate::ui::state::Notification) -> Element<'_, Message> {
    use crate::ui::styles::components::notification_styles;
    use iced::Shadow;

    let bg_color = notification_styles::background_color(&notification.kind);
    let icon_text = notification.icon();

    let content_col = notification_content_column(notification);
    let mut main_content = notification_main_content(icon_text, content_col);

    if let (Some(action_label), Some(action_message)) = (
        &notification.action_label,
        notification.action_message.clone(),
    ) {
        main_content = main_content.push(notification_action_button(action_label, action_message));
    }

    main_content = main_content.push(notification_dismiss_button(notification.id));

    let (progress_portion, remaining_portion) = notification.dismiss_progress_portions();
    if progress_portion < 100 {
        main_content = main_content.push(notification_progress_bar(
            progress_portion,
            remaining_portion,
        ));
    }

    container(main_content)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(bg_color.into()),
            text_color: Some(notification_styles::TEXT_COLOR),
            border: Border {
                radius: border_radius::MD.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .into()
}

/// Builds notification content column with optional title and message
fn notification_content_column(
    notification: &crate::ui::state::Notification,
) -> iced::widget::Column<'_, Message> {
    let mut content_col = column![].spacing(spacing::XS);

    if let Some(title) = &notification.title {
        content_col = content_col.push(text(title).size(typography::SIZE_BASE).style(
            |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(crate::ui::styles::components::notification_styles::TEXT_COLOR),
            },
        ));
    }

    content_col = content_col.push(text(&notification.message).size(typography::SIZE_SM).style(
        |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(crate::ui::styles::components::notification_styles::TEXT_COLOR),
        },
    ));

    content_col
}

/// Builds main notification content with icon and text
fn notification_main_content<'a>(
    icon_text: &'static str,
    content_col: iced::widget::Column<'a, Message>,
) -> iced::widget::Column<'a, Message> {
    column![row![
        container(text(icon_text).size(typography::SIZE_LG))
            .width(Length::Fixed(32.0))
            .center_y(Length::Fill)
            .center_x(Length::Fill),
        container(content_col).width(Length::Fill),
    ]
    .spacing(spacing::SM)
    .align_y(iced::Alignment::Start),]
    .spacing(spacing::XS)
}

/// Creates notification action button
fn notification_action_button(action_label: &str, action_message: Message) -> Element<'_, Message> {
    use crate::ui::styles::components::notification_styles;

    let action_btn = button(text(action_label).size(typography::SIZE_SM).style(
        |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(notification_styles::TEXT_COLOR),
        },
    ))
    .on_press(action_message)
    .padding(iced::Padding {
        top: spacing::XS,
        bottom: spacing::XS,
        left: spacing::SM,
        right: spacing::SM,
    })
    .style(move |_theme: &iced::Theme, status| {
        let bg = if matches!(status, iced::widget::button::Status::Hovered) {
            Color::from_rgba(1.0, 1.0, 1.0, 0.2)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
        };

        iced::widget::button::Style {
            background: Some(bg.into()),
            text_color: notification_styles::TEXT_COLOR,
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    container(action_btn)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .into()
}

/// Creates notification dismiss button
fn notification_dismiss_button(notification_id: u64) -> Element<'static, Message> {
    use crate::ui::styles::components::notification_styles;

    let dismiss_btn = button(text("Dismiss").size(typography::SIZE_SM).style(
        |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(notification_styles::TEXT_COLOR),
        },
    ))
    .on_press(Message::DismissNotification(notification_id))
    .padding(iced::Padding {
        top: spacing::XS,
        bottom: spacing::XS,
        left: spacing::SM,
        right: spacing::SM,
    })
    .style(move |_theme: &iced::Theme, status| {
        let bg = if matches!(status, iced::widget::button::Status::Hovered) {
            Color::from_rgba(1.0, 1.0, 1.0, 0.2)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
        };

        iced::widget::button::Style {
            background: Some(bg.into()),
            text_color: notification_styles::TEXT_COLOR,
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    container(dismiss_btn)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .into()
}

/// Creates notification progress bar for auto-dismiss countdown
fn notification_progress_bar(
    progress_portion: u16,
    remaining_portion: u16,
) -> Element<'static, Message> {
    use crate::ui::styles::components::notification_styles;

    let progress_bar = container(Space::new())
        .width(Length::FillPortion(progress_portion))
        .height(Length::Fixed(3.0))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(notification_styles::PROGRESS_COLOR.into()),
            ..Default::default()
        });

    container(row![
        progress_bar,
        Space::new().width(Length::FillPortion(remaining_portion)),
    ])
    .width(Length::Fill)
    .height(Length::Fixed(3.0))
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(notification_styles::PROGRESS_BG.into()),
        ..Default::default()
    })
    .into()
}

fn tooltip_overlay(tooltip: &crate::ui::tooltip::Tooltip) -> Element<'static, Message> {
    use crate::ui::styles::components::tooltip_styles;
    use iced::widget::text::Wrapping;

    let text_element = text(tooltip.text.clone())
        .size(typography::SIZE_SM)
        .width(Length::Fill)
        .wrapping(Wrapping::Word)
        .style(|_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(tooltip_styles::TEXT_COLOR),
        });

    let tooltip_body = container(text_element)
        .padding(iced::Padding {
            top: tooltip_styles::PADDING_VERTICAL,
            bottom: tooltip_styles::PADDING_VERTICAL,
            left: tooltip_styles::PADDING_HORIZONTAL,
            right: tooltip_styles::PADDING_HORIZONTAL,
        })
        .style(tooltip_styles::container_style)
        .width(Length::Shrink)
        .max_width(tooltip_styles::MAX_WIDTH);

    match tooltip.anchor {
        TooltipAnchor::TopBar => {
            let row = match tooltip.position {
                TooltipPosition::Left => row![tooltip_body, Space::new().width(Length::Fill)],
                _ => row![Space::new().width(Length::Fill), tooltip_body],
            };

            container(row)
                .padding(iced::Padding {
                    top: if tooltip.position == TooltipPosition::Bottom {
                        spacing::LG
                    } else {
                        spacing::SM
                    },
                    right: spacing::XL,
                    bottom: 0.0,
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        TooltipAnchor::PlayerControls => {
            let row = match tooltip.position {
                TooltipPosition::Left => row![tooltip_body, Space::new().width(Length::Fill)],
                TooltipPosition::Right => row![Space::new().width(Length::Fill), tooltip_body],
                _ => row![
                    Space::new().width(Length::Fill),
                    tooltip_body,
                    Space::new().width(Length::Fill)
                ],
            };

            container(column![Space::new().height(Length::Fill), row])
                .padding(iced::Padding {
                    top: spacing::XL,
                    right: spacing::XL,
                    bottom: if matches!(
                        tooltip.position,
                        TooltipPosition::Top | TooltipPosition::Left | TooltipPosition::Right
                    ) {
                        spacing::XL + spacing::LG
                    } else {
                        spacing::SM
                    },
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        TooltipAnchor::LibraryLeft => {
            let row = match tooltip.position {
                TooltipPosition::Right => row![Space::new().width(Length::Fill), tooltip_body],
                _ => row![tooltip_body, Space::new().width(Length::Fill)],
            };

            container(row)
                .padding(iced::Padding {
                    top: spacing::XL,
                    right: spacing::XL,
                    bottom: spacing::XL,
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: (color.a * alpha).clamp(0.0, 1.0),
    }
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
