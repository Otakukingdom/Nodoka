//! Screenshot capture orchestration using iced's rendering system.

use crate::scenarios::Scenario;
use iced::{window, Element, Settings, Task, Theme};
use nodoka::db::Database;
use nodoka::ui::main_window;
use nodoka::ui::state::ThemeMode;
use nodoka::ui::State;
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

const RENDER_SETTLE_DELAY_MS: u64 = 200;

fn view_fn(state: &ScreenshotApp) -> Element<'_, AppMessage> {
    state.view()
}

fn theme_fn(state: &ScreenshotApp) -> Theme {
    state.theme()
}

fn subscription_fn(state: &ScreenshotApp) -> iced::Subscription<AppMessage> {
    state.subscription()
}

/// Captures all screenshot scenarios to the output directory.
///
/// # Errors
///
/// Returns an error if:
/// - Database initialization fails
/// - Directory creation fails
/// - The iced application encounters a runtime error
pub fn capture_all_screenshots(output_dir: &Path) -> Result<CaptureManifest, CaptureError> {
    fs::create_dir_all(output_dir.join("light"))?;
    fs::create_dir_all(output_dir.join("dark"))?;

    let db = Rc::new(
        Database::new_in_memory()
            .map_err(|e| CaptureError::Database(format!("Failed to create database: {e}")))?,
    );

    nodoka::db::initialize(db.connection())
        .map_err(|e| CaptureError::Database(format!("Failed to initialize schema: {e}")))?;

    let mut scenarios = Vec::new();
    for scenario in Scenario::all() {
        for theme in &[ThemeMode::Light, ThemeMode::Dark] {
            scenarios.push((*scenario, *theme));
        }
    }

    let captures: Rc<RefCell<Vec<CaptureInfo>>> = Rc::new(RefCell::new(Vec::new()));
    let captures_for_app = captures.clone();
    let output_path = output_dir.to_path_buf();

    let window_settings = window::Settings {
        size: iced::Size::new(1280.0, 800.0),
        visible: true,
        ..Default::default()
    };

    let iced_settings = Settings {
        default_text_size: iced::Pixels(14.0),
        ..Settings::default()
    };

    let result = iced::application(
        move || {
            ScreenshotApp::new(
                output_path.clone(),
                db.clone(),
                scenarios.clone(),
                captures_for_app.clone(),
            )
        },
        |state: &mut ScreenshotApp, message| state.update(message),
        view_fn,
    )
    .settings(iced_settings)
    .window(window_settings)
    .theme(theme_fn)
    .subscription(subscription_fn)
    .run();

    if let Err(e) = result {
        return Err(CaptureError::Screenshot(format!("Application error: {e}")));
    }

    let captures_vec = captures.borrow().clone();

    let manifest = CaptureManifest {
        timestamp: chrono::Utc::now().to_rfc3339(),
        scenarios: captures_vec,
    };

    Ok(manifest)
}

/// Screenshot capture application state.
struct ScreenshotApp {
    state: State,
    db: Rc<Database>,
    scenarios: Vec<(Scenario, ThemeMode)>,
    current_index: usize,
    output_dir: PathBuf,
    captures: Rc<RefCell<Vec<CaptureInfo>>>,
    window_id: Option<window::Id>,
}

/// Messages for the screenshot application.
#[derive(Debug, Clone)]
enum AppMessage {
    /// The main window has been created and its ID is now available.
    WindowOpened(window::Id),
    /// Advance to the next scenario (or finish if all are done).
    NextScenario,
    /// The render-settle delay has elapsed; capture the screenshot now.
    ReadyToCapture,
    /// A screenshot has been captured (RGBA bytes, width, height) or failed.
    ScreenshotCaptured(Result<(Vec<u8>, u32, u32), String>),
    /// Ignored message (used to discard mapped UI messages).
    NoOp,
}

impl ScreenshotApp {
    fn new(
        output_dir: PathBuf,
        db: Rc<Database>,
        scenarios: Vec<(Scenario, ThemeMode)>,
        captures: Rc<RefCell<Vec<CaptureInfo>>>,
    ) -> (Self, Task<AppMessage>) {
        let app = Self {
            state: State::default(),
            db,
            scenarios,
            current_index: 0,
            output_dir,
            captures,
            window_id: None,
        };

        // Wait for WindowOpened event via subscription before starting captures.
        (app, Task::none())
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::WindowOpened(id) => {
                self.window_id = Some(id);
                Task::done(AppMessage::NextScenario)
            }
            AppMessage::NextScenario => self.handle_next_scenario(),
            AppMessage::ReadyToCapture => self.handle_ready_to_capture(),
            AppMessage::ScreenshotCaptured(result) => self.handle_screenshot_captured(&result),
            AppMessage::NoOp => Task::none(),
        }
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        if self.window_id.is_none() {
            iced::event::listen_with(|event, _status, id| {
                if matches!(
                    event,
                    iced::Event::Window(iced::window::Event::Opened { .. })
                ) {
                    Some(AppMessage::WindowOpened(id))
                } else {
                    None
                }
            })
        } else {
            iced::Subscription::none()
        }
    }

    fn handle_next_scenario(&mut self) -> Task<AppMessage> {
        let Some(&(scenario, theme)) = self.scenarios.get(self.current_index) else {
            // All scenarios captured — close the window to end the event loop.
            return self.window_id.map_or_else(Task::none, window::close);
        };

        self.state = State::default();
        scenario.apply_to_state(&mut self.state, &self.db);
        self.state.theme = theme;
        self.state.window_width = 1280;
        self.state.window_height = 800;
        self.state.prefers_reduced_motion = true;

        Task::perform(
            async {
                tokio::time::sleep(tokio::time::Duration::from_millis(RENDER_SETTLE_DELAY_MS))
                    .await;
            },
            |()| AppMessage::ReadyToCapture,
        )
    }

    fn handle_ready_to_capture(&self) -> Task<AppMessage> {
        let Some(id) = self.window_id else {
            eprintln!("✗ No window ID available for screenshot capture");
            return Task::done(AppMessage::NextScenario);
        };

        window::screenshot(id).map(|screenshot| {
            let width = screenshot.size.width;
            let height = screenshot.size.height;
            let rgba_bytes = screenshot.rgba.to_vec();
            AppMessage::ScreenshotCaptured(Ok((rgba_bytes, width, height)))
        })
    }

    fn handle_screenshot_captured(
        &mut self,
        result: &Result<(Vec<u8>, u32, u32), String>,
    ) -> Task<AppMessage> {
        let Some(&(scenario, theme)) = self.scenarios.get(self.current_index) else {
            return Task::none();
        };

        let theme_dir = match theme {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };

        match result {
            Ok((rgba_bytes, width, height)) => {
                let output_path = self.output_dir.join(theme_dir).join(scenario.filename());

                if let Err(e) = save_rgba_as_png(rgba_bytes, *width, *height, &output_path) {
                    eprintln!("✗ Failed to save {}: {e}", scenario.filename());
                } else {
                    println!("✓ Captured: {} ({})", scenario.filename(), theme_dir);

                    self.captures.borrow_mut().push(CaptureInfo {
                        scenario: scenario.filename().to_string(),
                        theme: theme_dir.to_string(),
                        path: output_path.display().to_string(),
                    });
                }
            }
            Err(e) => {
                eprintln!(
                    "✗ Screenshot capture error for {}: {e}",
                    scenario.filename()
                );
            }
        }

        self.current_index += 1;
        Task::done(AppMessage::NextScenario)
    }

    fn view(&self) -> Element<'_, AppMessage> {
        main_window::view(&self.state).map(|_| AppMessage::NoOp)
    }

    fn theme(&self) -> Theme {
        nodoka::ui::styles::nodoka_theme_for_mode(self.state.theme)
    }
}

/// Saves RGBA bytes as a PNG file.
///
/// # Errors
///
/// Returns an error if image creation or file writing fails.
fn save_rgba_as_png(
    rgba_bytes: &[u8],
    width: u32,
    height: u32,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use image::{ImageBuffer, RgbaImage};

    let img: RgbaImage =
        ImageBuffer::from_raw(width, height, rgba_bytes.to_vec()).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to create image buffer from RGBA bytes",
            )
        })?;

    img.save(path)?;
    Ok(())
}

/// Error types for screenshot capture.
#[derive(Debug)]
pub enum CaptureError {
    /// IO error during file operations.
    Io(std::io::Error),
    /// Database initialization or query error.
    Database(String),
    /// Screenshot capture or iced application error.
    Screenshot(String),
}

impl From<std::io::Error> for CaptureError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for CaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Database(s) => write!(f, "Database error: {s}"),
            Self::Screenshot(s) => write!(f, "Screenshot error: {s}"),
        }
    }
}

impl std::error::Error for CaptureError {}

/// Capture manifest documenting what was captured and when.
#[derive(Debug, serde::Serialize)]
pub struct CaptureManifest {
    /// ISO 8601 timestamp of capture.
    pub timestamp: String,
    /// List of captured scenarios.
    pub scenarios: Vec<CaptureInfo>,
}

impl CaptureManifest {
    /// Saves the manifest to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization or file writing fails.
    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)?;
        Ok(())
    }
}

/// Information about a captured screenshot.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CaptureInfo {
    /// Scenario filename.
    pub scenario: String,
    /// Theme name (light or dark).
    pub theme: String,
    /// Output file path.
    pub path: String,
}
