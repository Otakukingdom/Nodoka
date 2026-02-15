//! Application entry point and iced integration.
//!
//! This module contains the main [`App`] struct which implements
//! the [`iced::Application`] trait, integrating the UI, player, and database.
//!
//! ## Architecture
//!
//! Nodoka follows the Elm architecture pattern:
//!
//! - **Model**: Application state in [`State`]
//! - **Update**: Message handling in [`update`]
//! - **View**: UI rendering in [`main_window::view()`](crate::ui::main_window::view)
//!
//! ## Usage
//!
//! ```no_run
//! use nodoka::{Database, app};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let db = Database::open()?;
//! nodoka::db::initialize(db.connection())?;
//! app::run(db)?;
//! # Ok(())
//! # }
//! ```

use crate::db::Database;
use crate::player::Vlc;
use crate::ui::{main_window, update, Message, State};
use iced::window;
use iced::{Element, Settings, Subscription, Task, Theme};
use rusqlite::Connection;
use std::cell::RefCell;
use std::time::Duration;

const DEFAULT_WINDOW_WIDTH: f32 = 1200.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 900.0;
const MIN_WINDOW_WIDTH: f32 = 800.0;
const MIN_WINDOW_HEIGHT: f32 = 600.0;

const fn sanitize_volume(volume: i32) -> i32 {
    if volume < 0 {
        0
    } else if volume > 200 {
        200
    } else {
        volume
    }
}

const fn sanitize_speed(speed: f32) -> f32 {
    if !speed.is_finite() {
        return 1.0;
    }
    speed.clamp(0.5, 2.0)
}

fn i32_to_f32_window_size(v: i32) -> Option<f32> {
    let v_u16 = u16::try_from(v).ok()?;
    Some(f32::from(v_u16))
}

fn i32_to_f32_window_position(v: i32) -> Option<f32> {
    let v_i16 = i16::try_from(v).ok()?;
    Some(f32::from(v_i16))
}

/// Resolves initial window settings from persisted metadata.
///
/// This is used by [`run`] to restore window size and position across restarts.
///
/// When persisted geometry is missing or invalid, Nodoka falls back to centered
/// defaults.
#[must_use]
pub fn window_settings_from_storage(
    conn: &Connection,
    icon: Option<iced::window::Icon>,
) -> iced::window::Settings {
    let settings = crate::settings::Settings::new(conn);

    let default_size = iced::Size::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);

    let size = match settings.get_window_size() {
        Ok(Some((w, h))) => match (i32_to_f32_window_size(w), i32_to_f32_window_size(h)) {
            (Some(w_f32), Some(h_f32)) if w_f32 > 0.0 && h_f32 > 0.0 => {
                iced::Size::new(w_f32, h_f32)
            }
            _ => default_size,
        },
        _ => default_size,
    };

    let position = match settings.get_window_position() {
        Ok(Some((x, y))) => match (i32_to_f32_window_position(x), i32_to_f32_window_position(y)) {
            (Some(x_f32), Some(y_f32)) => {
                iced::window::Position::Specific(iced::Point::new(x_f32, y_f32))
            }
            _ => iced::window::Position::Centered,
        },
        _ => iced::window::Position::Centered,
    };

    iced::window::Settings {
        size,
        position,
        min_size: Some(iced::Size::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)),
        icon,
        ..Default::default()
    }
}

/// Main application struct for the Nodoka audiobook reader.
///
/// This struct implements the [`iced::Program`] trait (iced 0.14) and manages
/// the VLC player instance and database connection.
///
/// The application runs in an event loop where:
/// 1. User interactions generate [`Message`] events
/// 2. Messages are processed by [`update`] to modify state
/// 3. UI is re-rendered via [`view`](crate::ui::main_window::view)
///
/// Note: In iced 0.14, State is separate from Program. We use `RefCell` for
/// interior mutability since Program methods take `&self` but need to modify player.
pub struct App {
    player: RefCell<Option<Vlc>>,
    db: Database,
}

impl Drop for App {
    fn drop(&mut self) {
        match crate::tasks::zip_temp_root() {
            Ok(root) => {
                if let Err(e) = crate::tasks::cleanup_temp_files(&root) {
                    tracing::warn!(
                        "Failed to cleanup ZIP temp root on shutdown ({}): {e}",
                        root.display()
                    );
                }
            }
            Err(e) => {
                tracing::warn!("Failed to resolve ZIP temp root on shutdown: {e}");
            }
        }
    }
}

/// Initialization flags passed to the application on startup
pub struct Flags {
    pub db: Database,
}

impl iced::Program for App {
    type State = State;
    type Message = Message;
    type Theme = Theme;
    type Renderer = iced::Renderer;
    type Executor = iced::executor::Default;

    fn name() -> &'static str {
        "nodoka"
    }

    fn settings(&self) -> Settings {
        Settings::default()
    }

    fn window(&self) -> Option<window::Settings> {
        None // Window settings are provided via Settings in run()
    }

    fn boot(&self) -> (Self::State, Task<Self::Message>) {
        let flags = &self.db; // Access db from self
        let mut state = State::default();

        // Load directories from database
        match crate::db::queries::get_all_directories(flags.connection()) {
            Ok(directories) => {
                state.directories = directories;
            }
            Err(e) => {
                tracing::error!("Failed to load directories: {e}");
            }
        }

        // Load all audiobooks
        match crate::db::queries::get_all_audiobooks(flags.connection()) {
            Ok(audiobooks) => {
                state.audiobooks = audiobooks;
            }
            Err(e) => {
                tracing::error!("Failed to load audiobooks: {e}");
            }
        }

        // Load settings
        if let Ok(Some(volume_str)) = crate::db::queries::get_metadata(flags.connection(), "volume")
        {
            if let Ok(volume) = volume_str.parse::<i32>() {
                let sanitized = sanitize_volume(volume);
                state.volume = sanitized;
                if sanitized != volume {
                    if let Err(e) = crate::db::queries::set_metadata(
                        flags.connection(),
                        "volume",
                        &sanitized.to_string(),
                    ) {
                        tracing::warn!("Failed to persist sanitized volume: {e}");
                    }
                }
            }
        }

        if let Ok(Some(speed_str)) = crate::db::queries::get_metadata(flags.connection(), "speed") {
            if let Ok(speed) = speed_str.parse::<f32>() {
                let sanitized = sanitize_speed(speed);
                state.speed = sanitized;
                if (sanitized - speed).abs() > f32::EPSILON {
                    if let Err(e) = crate::db::queries::set_metadata(
                        flags.connection(),
                        "speed",
                        &sanitized.to_string(),
                    ) {
                        tracing::warn!("Failed to persist sanitized speed: {e}");
                    }
                }
            }
        }

        // Load current audiobook if set
        if let Ok(Some(id_str)) =
            crate::db::queries::get_metadata(flags.connection(), "current_audiobook_id")
        {
            if let Ok(id) = id_str.parse::<i64>() {
                state.selected_audiobook = Some(id);

                // Load files for current audiobook
                if let Ok(files) = crate::db::queries::get_audiobook_files(flags.connection(), id) {
                    state.current_files = files;
                }

                // Load bookmarks for current audiobook
                if let Ok(bookmarks) =
                    crate::db::queries::get_bookmarks_for_audiobook(flags.connection(), id)
                {
                    state.bookmarks = bookmarks;
                }
            }
        }

        // Initialize player with loaded settings
        let mut player_ref = self.player.borrow_mut();
        if let Some(player) = player_ref.as_mut() {
            if let Err(e) = player.set_volume(state.volume) {
                tracing::error!("Failed to set initial volume: {e}");
            }
            if let Err(e) = player.set_rate(state.speed) {
                tracing::error!("Failed to set initial speed: {e}");
            }
        }

        (state, Task::done(Message::InitialLoadComplete))
    }

    fn update(&self, state: &mut Self::State, message: Self::Message) -> Task<Self::Message> {
        // Use RefCell to get mutable access to player through immutable self
        let mut player_ref = self.player.borrow_mut();
        update::update(state, message, &mut player_ref, &self.db)
    }

    fn view<'a>(
        &self,
        state: &'a Self::State,
        _window: window::Id,
    ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
        main_window::view(state)
    }

    fn subscription(&self, state: &Self::State) -> Subscription<<Self as iced::Program>::Message> {
        let mut subs: Vec<Subscription<<Self as iced::Program>::Message>> = Vec::new();

        if state.selected_file.is_some() {
            subs.push(iced::time::every(Duration::from_secs(1)).map(|_| Message::PlayerTick));
        }

        subs.push(iced::event::listen_with(
            |event, _status, _id| match event {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    modified_key: _,
                    physical_key: _,
                    location: _,
                    modifiers,
                    text: _,
                    repeat: _,
                }) => map_key_press(key, modifiers),
                iced::Event::Window(iced::window::Event::Moved(point)) => {
                    let x = f32_to_i32(point.x)?;
                    let y = f32_to_i32(point.y)?;
                    Some(Message::WindowMoved(x, y))
                }
                iced::Event::Window(iced::window::Event::Resized(size)) => {
                    let width = f32_to_u32(size.width)?;
                    let height = f32_to_u32(size.height)?;
                    Some(Message::WindowResized(width, height))
                }
                _ => None,
            },
        ));

        Subscription::batch(subs)
    }

    fn theme(&self, _state: &Self::State, _window: window::Id) -> Option<Self::Theme> {
        Some(crate::ui::nodoka_theme())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_boot_sanitizes_persisted_volume_and_speed(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        use iced::Program as _;

        let db = Database::new_in_memory()?;
        db::initialize(db.connection())?;

        crate::db::queries::set_metadata(db.connection(), "volume", "-5")?;
        crate::db::queries::set_metadata(db.connection(), "speed", "NaN")?;

        let app = App {
            player: RefCell::new(None),
            db,
        };

        let (state, _cmd) = app.boot();

        assert_eq!(state.volume, 0, "volume should clamp to 0 on load");
        assert!(state.speed.is_finite(), "speed should be finite on load");
        assert!(
            (state.speed - 1.0).abs() < f32::EPSILON,
            "non-finite speed should default to 1.0"
        );
        Ok(())
    }

    #[test]
    fn test_boot_clamps_out_of_range_speed() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        use iced::Program as _;

        let db = Database::new_in_memory()?;
        db::initialize(db.connection())?;

        crate::db::queries::set_metadata(db.connection(), "speed", "3.5")?;
        crate::db::queries::set_metadata(db.connection(), "volume", "999")?;

        let app = App {
            player: RefCell::new(None),
            db,
        };

        let (state, _cmd) = app.boot();

        assert_eq!(state.volume, 200, "volume should clamp to 200 on load");
        assert!(
            (state.speed - 2.0).abs() < f32::EPSILON,
            "speed should clamp to 2.0 on load"
        );
        Ok(())
    }
}

fn f32_to_i32(value: f32) -> Option<i32> {
    if !value.is_finite() {
        return None;
    }

    let value_f64 = f64::from(value);
    if value_f64 < f64::from(i32::MIN) || value_f64 > f64::from(i32::MAX) {
        return None;
    }

    // Preserve the previous semantics of truncation toward zero.
    let truncated = value.trunc();
    let text = format!("{truncated:.0}");
    text.parse::<i32>().ok()
}

fn f32_to_u32(value: f32) -> Option<u32> {
    if !value.is_finite() {
        return None;
    }

    let value_f64 = f64::from(value);
    if value_f64 < 0.0 || value_f64 > f64::from(u32::MAX) {
        return None;
    }

    let truncated = value.trunc();
    let text = format!("{truncated:.0}");
    text.parse::<u32>().ok()
}

impl App {
    /// Creates a new App instance from initialization flags
    fn new_from_flags(flags: Flags) -> Self {
        // Initialize player
        let player = match Vlc::new() {
            Ok(p) => Some(p),
            Err(e) => {
                tracing::error!("Failed to initialize player: {e}");
                tracing::error!(
                    "The application will start but audio playback will not work. \
                     Please install VLC media player and restart the application."
                );

                // Log platform-specific installation instructions
                #[cfg(target_os = "macos")]
                tracing::info!("macOS: Install VLC with 'brew install --cask vlc'");

                #[cfg(target_os = "linux")]
                tracing::info!(
                    "Linux: Install with 'sudo apt install vlc libvlc-dev' (Ubuntu/Debian)"
                );

                #[cfg(target_os = "windows")]
                tracing::info!("Windows: Download VLC from https://www.videolan.org/vlc/");

                None
            }
        };

        Self {
            player: RefCell::new(player),
            db: flags.db,
        }
    }
}

fn map_key_press(
    key: iced::keyboard::Key,
    modifiers: iced::keyboard::Modifiers,
) -> Option<Message> {
    let shortcut_key = match key {
        iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
            crate::ui::shortcuts::ShortcutKey::Space
        }
        iced::keyboard::Key::Character(ch) if ch.eq_ignore_ascii_case("b") => {
            crate::ui::shortcuts::ShortcutKey::B
        }
        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
            crate::ui::shortcuts::ShortcutKey::ArrowLeft
        }
        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
            crate::ui::shortcuts::ShortcutKey::ArrowRight
        }
        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
            crate::ui::shortcuts::ShortcutKey::ArrowUp
        }
        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
            crate::ui::shortcuts::ShortcutKey::ArrowDown
        }
        iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
            crate::ui::shortcuts::ShortcutKey::Escape
        }
        _ => return None,
    };

    crate::ui::shortcuts::message_for_key_chord(shortcut_key, modifiers)
}

fn view_fn(state: &State) -> Element<'_, Message> {
    main_window::view(state)
}

/// Runs the Nodoka application
///
/// # Errors
///
/// Returns an error if the application fails to start or encounters a runtime error
pub fn run(db: Database) -> iced::Result {
    use iced::Program;
    use std::rc::Rc;

    // Restore persisted window geometry (size/position/min size) before starting the event loop.
    let window_settings = window_settings_from_storage(db.connection(), None);

    // Apply a consistent default text size for the UI.
    let iced_settings = Settings {
        default_text_size: crate::ui::typography::SIZE_BASE.into(),
        ..Settings::default()
    };

    // Create app instance with flags wrapped in Rc for sharing across closures
    let app = Rc::new(App::new_from_flags(Flags { db }));
    let app_boot = app.clone();

    // iced 0.14 application API: create application with boot, update, view closures
    iced::application(
        move || app_boot.boot(), // Boot just returns (State, Task)
        move |state: &mut State, message: Message| app.update(state, message),
        view_fn,
    )
    .settings(iced_settings)
    .window(window_settings)
    .run()
}

#[cfg(test)]
mod conversion_tests {
    use super::{f32_to_i32, f32_to_u32};

    #[test]
    fn test_f32_to_i32_rejects_non_finite_and_out_of_range() {
        assert_eq!(f32_to_i32(f32::NAN), None);
        assert_eq!(f32_to_i32(f32::INFINITY), None);
        assert_eq!(f32_to_i32(f32::NEG_INFINITY), None);
        assert_eq!(f32_to_i32(1.0e20), None);
        assert_eq!(f32_to_i32(-1.0e20), None);

        assert_eq!(f32_to_i32(12.9), Some(12));
        assert_eq!(f32_to_i32(-12.9), Some(-12));
    }

    #[test]
    fn test_f32_to_u32_rejects_negative_non_finite_and_out_of_range() {
        assert_eq!(f32_to_u32(f32::NAN), None);
        assert_eq!(f32_to_u32(f32::INFINITY), None);
        assert_eq!(f32_to_u32(f32::NEG_INFINITY), None);
        assert_eq!(f32_to_u32(-1.0), None);
        assert_eq!(f32_to_u32(1.0e20), None);

        assert_eq!(f32_to_u32(12.9), Some(12));
        assert_eq!(f32_to_u32(0.0), Some(0));
    }
}
