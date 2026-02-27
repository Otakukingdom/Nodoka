//! Premium empty state component for distinctive visual design.
//!
//! Provides illustration-quality placeholder graphics with enhanced
//! visual hierarchy, premium button styling, and micro-interactions.

use crate::ui::styles::{
    border_radius, button_styles, colors, shadows, spacing, typography,
};
use crate::ui::Message;
use iced::widget::{button, column, container, text, Space};
use iced::{Border, Color, Element, Length, Shadow, Vector};

/// Empty state context - determines the visual variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmptyStateContext {
    /// Library is empty - user needs to add audiobooks
    Library,
    /// File list is empty - user needs to select an audiobook
    FileList,
    /// Bookmarks list is empty - user hasn't created bookmarks
    Bookmarks,
    /// Search returned no results
    SearchResults,
    /// Generic empty state
    Generic,
}

impl EmptyStateContext {
    const fn icon_text(self) -> &'static str {
        match self {
            Self::Library => "♪",
            Self::FileList => "📁",
            Self::Bookmarks => "🔖",
            Self::SearchResults => "🔍",
            Self::Generic => "○",
        }
    }

    const fn title_color(self) -> Color {
        match self {
            Self::SearchResults => colors::TEXT_SECONDARY,
            _ => colors::TEXT_PRIMARY,
        }
    }
}

/// Renders a premium empty state component with enhanced visual treatment
///
/// Features:
/// - Large decorative icon with gradient treatment
/// - Multi-line messaging with proper typography hierarchy
/// - Optional data visualization hint
/// - Premium button styling with micro-interactions
/// - Context-aware variants for different empty states
#[must_use]
pub fn view(
    context: EmptyStateContext,
    title: &'static str,
    description: &'static str,
    action_label: Option<&'static str>,
    action_message: Option<Message>,
) -> Element<'static, Message> {
    let icon_size = typography::type_scale::DISPLAY_SIZE * 1.5;
    // Enhanced icon with multi-layer gradient (three-color blend)
    // Combines sunset, primary, and accent gradients for sophisticated depth
    // Creates a distinctive premium appearance that sets the design apart
    let icon_gradient = {
        let start = colors::SUNSET_GRADIENT_START;
        let mid = colors::PRIMARY_GRADIENT_START;
        let end = colors::ACCENT_GRADIENT_END;
        Color {
            r: start.r * 0.4 + mid.r * 0.3 + end.r * 0.3,
            g: start.g * 0.4 + mid.g * 0.3 + end.g * 0.3,
            b: start.b * 0.4 + mid.b * 0.3 + end.b * 0.3,
            a: 1.0,
        }
    };
    let icon = container(text(context.icon_text()).size(icon_size))
        .width(Length::Fixed(icon_size * 1.6))
        .height(Length::Fixed(icon_size * 1.6))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(icon_gradient.into()),
            border: Border {
                color: colors::ACCENT_GRADIENT_END,
                width: 2.5,
                radius: (icon_size * 1.6 / 2.0).into(),
            },
            // Multi-layer shadow combining ambient lift with rose-tinted definition
            shadow: Shadow {
                color: Color::from_rgba(0.95, 0.85, 0.90, 0.08),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            text_color: Some(colors::TEXT_ON_PRIMARY),
            ..Default::default()
        });

    let title_heading = text(title)
        .size(typography::type_scale::H1_SIZE)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(context.title_color()),
        });

    let description_text = container(
        text(description)
            .size(typography::type_scale::BODY_SIZE)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_SECONDARY),
            }),
    )
    .width(Length::Fixed(480.0))
    .center_x(Length::Fill);

    let mut content_column = column![icon, Space::new().height(Length::Fixed(spacing::XXL)), title_heading]
        .align_x(iced::Alignment::Center)
        .spacing(spacing::SM);

    content_column = content_column.push(Space::new().height(Length::Fixed(spacing::LG)));
    content_column = content_column.push(description_text);

    if let (Some(label), Some(msg)) = (action_label, action_message) {
        content_column = content_column.push(Space::new().height(Length::Fixed(spacing::XXL)));
        content_column = content_column.push(
            button(
                text(label)
                    .size(typography::type_scale::BUTTON_SIZE)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_ON_PRIMARY),
                    }),
            )
            .on_press(msg)
            .padding(iced::Padding::from([
                spacing::MD,
                spacing::XXL,
            ]))
            .style(button_styles::elevated),
        );
    }

    let inner_card = container(content_column)
        .padding(spacing::XXL)
        .width(Length::Shrink)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(colors::BG_ELEVATED.into()),
            border: Border {
                color: colors::BORDER_PROMINENT,
                width: 1.5,
                radius: border_radius::XL.into(),
            },
            shadow: shadows::shadow_xxl(),
            ..Default::default()
        });

    container(inner_card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(spacing::XXL)
        .into()
}

/// Simplified empty state for backward compatibility
///
/// Use `view()` with `EmptyStateContext` for new implementations.
#[must_use]
pub fn simple(
    icon: Option<&'static str>,
    title: &'static str,
    description: &'static str,
    action_label: Option<&'static str>,
    action_message: Option<Message>,
) -> Element<'static, Message> {
    let context = if icon == Some("♪") {
        EmptyStateContext::Library
    } else if icon == Some("🔖") {
        EmptyStateContext::Bookmarks
    } else {
        EmptyStateContext::Generic
    };

    view(context, title, description, action_label, action_message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_context_returns_valid_icon() {
        assert!(!EmptyStateContext::Library.icon_text().is_empty());
        assert!(!EmptyStateContext::FileList.icon_text().is_empty());
        assert!(!EmptyStateContext::Bookmarks.icon_text().is_empty());
        assert!(!EmptyStateContext::SearchResults.icon_text().is_empty());
        assert!(!EmptyStateContext::Generic.icon_text().is_empty());
    }

    #[test]
    fn test_empty_state_context_returns_valid_color() {
        let library_color = EmptyStateContext::Library.title_color();
        assert!(library_color.r >= 0.0 && library_color.r <= 1.0);

        let search_color = EmptyStateContext::SearchResults.title_color();
        assert!(search_color.r >= 0.0 && search_color.r <= 1.0);
    }

    #[test]
    fn test_empty_state_view_renders() {
        let element = view(
            EmptyStateContext::Library,
            "No audiobooks",
            "Add your first audiobook directory to get started.",
            Some("Add Directory"),
            Some(Message::DirectoryAdd),
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_without_action() {
        let element = view(
            EmptyStateContext::Generic,
            "Test Title",
            "Test description",
            None,
            None,
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_with_action() {
        let element = view(
            EmptyStateContext::Library,
            "No Audiobooks",
            "Add audiobook directories to get started",
            Some("Open Settings"),
            Some(Message::OpenSettings),
        );
        drop(element);
    }

    #[test]
    fn test_simple_empty_state_with_library_context() {
        let element = simple(
            Some("♪"),
            "No Audiobooks",
            "Add directories to get started",
            Some("Add Directory"),
            Some(Message::OpenSettings),
        );
        drop(element);
    }

    #[test]
    fn test_simple_empty_state_without_icon() {
        let element = simple(
            None,
            "No Items",
            "No items available",
            None,
            None,
        );
        drop(element);
    }

    #[test]
    fn test_simple_empty_state_with_bookmark_context() {
        let element = simple(
            Some("🔖"),
            "No Bookmarks",
            "Create bookmarks to mark important positions",
            Some("Add Bookmark"),
            Some(Message::CreateBookmark),
        );
        drop(element);
    }

    #[test]
    fn test_view_with_search_results_context() {
        let element = view(
            EmptyStateContext::SearchResults,
            "No Results Found",
            "Try adjusting your search or filters",
            Some("Clear Filters"),
            Some(Message::ClearFilters),
        );
        drop(element);
    }

    #[test]
    fn test_view_with_file_list_context() {
        let element = view(
            EmptyStateContext::FileList,
            "No Files",
            "Select an audiobook to view its files",
            None,
            None,
        );
        drop(element);
    }
}
