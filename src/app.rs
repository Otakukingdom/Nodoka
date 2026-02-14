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
use iced::{Application, Command, Element, Settings, Subscription, Theme};
use rusqlite::Connection;
use std::time::Duration;

const DEFAULT_WINDOW_WIDTH: f32 = 1200.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 900.0;
const MIN_WINDOW_WIDTH: f32 = 800.0;
const MIN_WINDOW_HEIGHT: f32 = 600.0;

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

/// Main application state for the Nodoka audiobook reader.
///
/// This struct implements the [`iced::Application`] trait and manages
/// the UI state, VLC player instance, and database connection.
///
/// The application runs in an event loop where:
/// 1. User interactions generate [`Message`] events
/// 2. Messages are processed by [`update`] to modify state
/// 3. UI is re-rendered via [`view`](crate::ui::main_window::view)
pub struct App {
    state: State,
    player: Option<Vlc>,
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

/// Initialization flags passed to [`App`] on startup.
///
/// Contains the database connection and any other startup configuration.
/// Passed to [`Application::new()`] during application initialization.
pub struct Flags {
    pub db: Database,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut state = State::default();

        // Load directories from database
        match crate::db::queries::get_all_directories(flags.db.connection()) {
            Ok(directories) => {
                state.directories = directories;
            }
            Err(e) => {
                tracing::error!("Failed to load directories: {e}");
            }
        }

        // Load all audiobooks
        match crate::db::queries::get_all_audiobooks(flags.db.connection()) {
            Ok(audiobooks) => {
                state.audiobooks = audiobooks;
            }
            Err(e) => {
                tracing::error!("Failed to load audiobooks: {e}");
            }
        }

        // Load settings
        if let Ok(Some(volume_str)) =
            crate::db::queries::get_metadata(flags.db.connection(), "volume")
        {
            if let Ok(volume) = volume_str.parse::<i32>() {
                state.volume = volume;
            }
        }

        if let Ok(Some(speed_str)) =
            crate::db::queries::get_metadata(flags.db.connection(), "speed")
        {
            if let Ok(speed) = speed_str.parse::<f32>() {
                state.speed = speed;
            }
        }

        // Load current audiobook
        if let Ok(Some(id_str)) =
            crate::db::queries::get_metadata(flags.db.connection(), "current_audiobook_id")
        {
            if let Ok(id) = id_str.parse::<i64>() {
                state.selected_audiobook = Some(id);

                // Load files for current audiobook
                if let Ok(files) =
                    crate::db::queries::get_audiobook_files(flags.db.connection(), id)
                {
                    state.current_files = files;
                }

                if let Ok(bookmarks) =
                    crate::db::queries::get_bookmarks_for_audiobook(flags.db.connection(), id)
                {
                    state.bookmarks = bookmarks;
                }
            }
        }

        // Initialize player
        let player = match Vlc::new() {
            Ok(mut p) => {
                if let Err(e) = p.set_volume(state.volume) {
                    tracing::error!("Failed to set initial volume: {e}");
                }
                if let Err(e) = p.set_rate(state.speed) {
                    tracing::error!("Failed to set initial speed: {e}");
                }
                Some(p)
            }
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

        let app = Self {
            state,
            player,
            db: flags.db,
        };

        (
            app,
            Command::perform(async {}, |()| Message::InitialLoadComplete),
        )
    }

    fn title(&self) -> String {
        String::from("Nodoka Audiobook Reader")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update::update(&mut self.state, message, &mut self.player, &self.db)
    }

    fn view(&self) -> Element<Self::Message> {
        main_window::view(&self.state)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subs: Vec<Subscription<Message>> = Vec::new();

        if self.state.selected_file.is_some() {
            subs.push(iced::time::every(Duration::from_secs(1)).map(|_| Message::PlayerTick));
        }

        subs.push(iced::keyboard::on_key_press(map_key_press));

        subs.push(iced::event::listen_with(|event, _status| match event {
            iced::Event::Window(_id, iced::window::Event::Moved { x, y }) => {
                Some(Message::WindowMoved(x, y))
            }
            iced::Event::Window(_id, iced::window::Event::Resized { width, height }) => {
                Some(Message::WindowResized(width, height))
            }
            _ => None,
        }));

        Subscription::batch(subs)
    }

    fn theme(&self) -> Self::Theme {
        crate::ui::nodoka_theme()
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
        _ => return None,
    };

    crate::ui::shortcuts::message_for_key_chord(shortcut_key, modifiers)
}

/// Runs the Nodoka application
///
/// # Errors
///
/// Returns an error if the application fails to start or encounters a runtime error
pub fn run(db: Database) -> iced::Result {
    // Load embedded fonts
    let fonts = vec![
        include_bytes!("../assets/fonts/Roboto-Regular.ttf")
            .as_slice()
            .into(),
        include_bytes!("../assets/fonts/Roboto-Bold.ttf")
            .as_slice()
            .into(),
        include_bytes!("../assets/fonts/Roboto-Medium.ttf")
            .as_slice()
            .into(),
    ];

    // Load application icon
    let icon_data = include_bytes!("../assets/icons/Entypo_d83d(0)_256.png");
    let icon = image::load_from_memory(icon_data).ok().and_then(|img| {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
    });

    App::run(Settings {
        window: window_settings_from_storage(db.connection(), icon),
        flags: Flags { db },
        id: None,
        fonts,
        default_font: iced::Font::DEFAULT,
        default_text_size: iced::Pixels::from(16),
        antialiasing: false,
    })
}
