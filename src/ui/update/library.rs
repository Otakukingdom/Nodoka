use crate::conversions::ms_to_f64;
use crate::db::Database;
use crate::tasks::{materialize_zip_virtual_path, parse_zip_virtual_path};
use crate::ui::{FocusedElement, PlaybackStatus, State};
use iced::Task;
use std::path::PathBuf;

use super::playback;
use super::MediaControl;

pub(super) struct FileSelectionOutcome {
    pub(super) task: Task<crate::ui::Message>,
    pub(super) file_loaded: bool,
}

pub(super) fn handle_audiobook_selected(
    state: &mut State,
    player: &mut Option<crate::player::Vlc>,
    db: &Database,
    id: i64,
) -> Task<crate::ui::Message> {
    state.focused_element = FocusedElement::AudiobookList;

    let old_selection = state.selected_audiobook;
    let is_new_selection = old_selection != Some(id);

    // Load files first, only update selection if successful
    let files = match crate::db::queries::get_audiobook_files(db.connection(), id) {
        Ok(files) => files,
        Err(e) => {
            tracing::error!("Failed to load audiobook files: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(
                    "Failed to load audiobook files".to_string(),
                ));
            return Task::none();
        }
    };

    let bookmarks = match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), id) {
        Ok(bookmarks) => bookmarks,
        Err(e) => {
            tracing::error!("Failed to load bookmarks: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(
                    "Failed to load bookmarks".to_string(),
                ));
            return Task::none();
        }
    };

    // Only now update state after successful loading.
    //
    // Keep audiobook selection and dependent UI state atomic: if loading fails, do not clear
    // or reset the previous selection (avoids inconsistent UI state).
    if is_new_selection {
        if let Some(old_id) = old_selection {
            if let Some(old_ab) = state.audiobooks.iter().find(|a| a.id == Some(old_id)) {
                crate::ui::media_paths::cleanup_zip_temp_for_path(&old_ab.full_path);
            }
        }

        playback::reset_playback_state(state, player);
        state.selected_file = None;
        state.bookmark_editor = None;
    }

    state.selected_audiobook = Some(id);
    state.current_files = files;
    state.bookmarks = bookmarks;
    let now_ms = crate::ui::state::current_time_ms();
    state.file_list_animation_start_ms = Some(now_ms);
    state.bookmark_list_animation_start_ms = Some(now_ms);

    // Only save metadata after successful state update
    if let Err(e) =
        crate::db::queries::set_metadata(db.connection(), "current_audiobook_id", &id.to_string())
    {
        tracing::error!("Failed to save current audiobook: {e}");
    }

    if state
        .selected_audiobook
        .and_then(|id| state.audiobooks.iter().find(|a| a.id == Some(id)))
        .and_then(|ab| ab.id)
        .is_some_and(|id| state.cover_thumbnails.contains_key(&id))
    {
        return Task::none();
    }

    let Some(selected_id) = state.selected_audiobook else {
        return Task::none();
    };

    let Some(ab) = state.audiobooks.iter().find(|a| a.id == Some(selected_id)) else {
        return Task::none();
    };

    generate_cover_thumbnail_command(selected_id, ab.full_path.clone())
}

fn persist_selected_file(state: &mut State, db: &Database, path: &str) {
    state.selected_file = Some(path.to_string());

    if let Some(id) = state.selected_audiobook {
        if let Err(e) =
            crate::db::queries::update_audiobook_selected_file(db.connection(), id, Some(path))
        {
            tracing::error!("Failed to save selected file: {e}");
        }
        return;
    }

    if let Ok(Some(file)) =
        crate::db::queries::get_audiobook_file_by_path(db.connection(), path)
    {
        if let Err(e) = crate::db::queries::update_audiobook_selected_file(
            db.connection(),
            file.audiobook_id,
            Some(path),
        ) {
            tracing::error!("Failed to save selected file: {e}");
        }
    }
}

fn restore_progress_and_duration<P: MediaControl>(
    state: &mut State,
    player: &P,
    db: &Database,
    path: &str,
) {
    if let Ok(Some(file)) =
        crate::db::queries::get_audiobook_file_by_path(db.connection(), path)
    {
        if let Some(length_ms) = file.length_of_file {
            if let Ok(duration_f64) = ms_to_f64(length_ms) {
                state.total_duration = duration_f64;
            }
        }

        if let Some(seek_position) = file.seek_position {
            if let Err(e) = player.set_time(seek_position) {
                tracing::error!("Failed to restore seek position: {e}");
            } else if let Ok(position_f64) = ms_to_f64(seek_position) {
                state.current_time = position_f64;
            }
        } else {
            state.current_time = 0.0;
        }
    }

    if let Ok(duration) = player.get_length() {
        if duration > 0 {
            if let Ok(duration_f64) = ms_to_f64(duration) {
                state.total_duration = duration_f64;
            }
        }
    }
}

pub(super) fn handle_file_selected<P: MediaControl>(
    state: &mut State,
    player: &mut Option<P>,
    db: &Database,
    path: &str,
) -> FileSelectionOutcome {
    state.focused_element = FocusedElement::FileList;

    if let Some(p) = player.as_mut() {
        let playback_path = if path.starts_with("zip://") {
            if parse_zip_virtual_path(path).is_none() {
                tracing::warn!("Rejecting malformed ZIP virtual path: {path}");
                state
                    .notification_queue
                    .push(crate::ui::state::Notification::error(
                        "Invalid ZIP virtual path".to_string(),
                    ));
                return FileSelectionOutcome {
                    task: Task::none(),
                    file_loaded: false,
                };
            }

            match materialize_zip_virtual_path(path) {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to extract ZIP entry for playback: {e}");
                    state
                        .notification_queue
                        .push(crate::ui::state::Notification::error(format!(
                            "Failed to extract audio file: {e}"
                        )));
                    return FileSelectionOutcome {
                        task: Task::none(),
                        file_loaded: false,
                    };
                }
            }
        } else {
            std::path::PathBuf::from(path)
        };

        if let Err(e) = p.load_media(&playback_path) {
            tracing::error!("Failed to load media: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(format!(
                    "Failed to load audio file: {e}"
                )));
            return FileSelectionOutcome {
                task: Task::none(),
                file_loaded: false,
            };
        }

        persist_selected_file(state, db, path);
        restore_progress_and_duration(state, p, db, path);

        if let Err(e) = p.play() {
            tracing::error!("Failed to auto-play: {e}");
            state
                .notification_queue
                .push(crate::ui::state::Notification::error(format!(
                    "Failed to start playback: {e}"
                )));
            state.playback = PlaybackStatus::Paused;
        } else {
            state.playback = PlaybackStatus::Playing;
        }
        return FileSelectionOutcome {
            task: Task::none(),
            file_loaded: true,
        };
    }

    persist_selected_file(state, db, path);
    FileSelectionOutcome {
        task: Task::none(),
        file_loaded: true,
    }
}

pub(super) fn handle_cover_thumbnail_generated(
    state: &mut State,
    audiobook_id: i64,
    path: Option<PathBuf>,
) -> Task<crate::ui::Message> {
    if let Some(path) = path {
        state.cover_thumbnails.insert(audiobook_id, path);
    }
    Task::none()
}

pub(super) fn handle_initial_load_complete(state: &mut State) -> Task<crate::ui::Message> {
    state.load_state = crate::ui::LoadState::Ready;

    let cmds: Vec<Task<crate::ui::Message>> = state
        .audiobooks
        .iter()
        .take(32)
        .filter_map(|ab| {
            let id = ab.id?;
            if state.cover_thumbnails.contains_key(&id) {
                return None;
            }
            Some(generate_cover_thumbnail_command(id, ab.full_path.clone()))
        })
        .collect();

    Task::batch(cmds)
}

pub(super) fn generate_cover_thumbnail_command(
    audiobook_id: i64,
    full_path: String,
) -> Task<crate::ui::Message> {
    Task::perform(
        async move {
            let path = PathBuf::from(full_path);
            let res = tokio::task::spawn_blocking(move || {
                crate::cover_cache::ensure_cover_thumbnail(audiobook_id, &path)
            })
            .await;

            let out = match res {
                Ok(Ok(path)) => path,
                Ok(Err(e)) => {
                    tracing::debug!("Cover thumbnail generation failed for {audiobook_id}: {e}");
                    None
                }
                Err(e) => {
                    tracing::debug!(
                        "Cover thumbnail generation task failed for {audiobook_id}: {e}"
                    );
                    None
                }
            };

            (audiobook_id, out)
        },
        |(id, path)| crate::ui::Message::CoverThumbnailGenerated(id, path),
    )
}
