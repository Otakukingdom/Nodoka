use crate::db::Database;
use crate::player::ConcretePlayer;
use crate::ui::{main_window, update, Message, NodokaState};
use iced::{Application, Command, Element, Settings, Theme};

pub struct NodokaApp {
    state: NodokaState,
    player: Option<ConcretePlayer>,
    db: Database,
}

pub struct Flags {
    pub db: Database,
}

impl Application for NodokaApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut state = NodokaState::default();

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
            }
        }

        // Initialize player
        let player = match ConcretePlayer::new() {
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

    fn theme(&self) -> Self::Theme {
        crate::ui::nodoka_theme()
    }
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

    NodokaApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 900.0),
            position: iced::window::Position::Centered,
            min_size: Some(iced::Size::new(800.0, 600.0)),
            icon,
            ..Default::default()
        },
        flags: Flags { db },
        id: None,
        fonts,
        default_font: iced::Font::DEFAULT,
        default_text_size: iced::Pixels::from(16),
        antialiasing: false,
    })
}
