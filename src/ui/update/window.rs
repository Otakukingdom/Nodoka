use crate::db::Database;
use crate::ui::Message;
use iced::Task;

pub(super) fn handle_window_moved(db: &Database, x: i32, y: i32) -> Task<Message> {
    let settings = crate::settings::Settings::new(db.connection());
    if let Err(e) = settings.set_window_position(x, y) {
        tracing::warn!("Failed to persist window position: {e}");
    }
    Task::none()
}

pub(super) fn handle_window_resized(
    db: &Database,
    width: u32,
    height: u32,
) -> Task<Message> {
    let settings = crate::settings::Settings::new(db.connection());

    let width_i32 = match i32::try_from(width) {
        Ok(v) => v,
        Err(_e) => {
            tracing::warn!("Window resize width out of range: {width}");
            return Task::none();
        }
    };
    let height_i32 = match i32::try_from(height) {
        Ok(v) => v,
        Err(_e) => {
            tracing::warn!("Window resize height out of range: {height}");
            return Task::none();
        }
    };

    if let Err(e) = settings.set_window_size(width_i32, height_i32) {
        tracing::warn!("Failed to persist window size: {e}");
    }

    Task::none()
}
