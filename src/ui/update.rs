use crate::db::Database;
use crate::player::ConcretePlayer;
use crate::ui::{Message, NodokaState};
use iced::Command;
use std::path::Path;

pub fn update(
    state: &mut NodokaState,
    message: Message,
    player: &mut Option<ConcretePlayer>,
    db: &Database,
) -> Command<Message> {
    match message {
        Message::PlayPause => {
            if let Some(ref mut p) = player {
                if state.is_playing {
                    if let Err(e) = p.pause() {
                        tracing::error!("Failed to pause: {e}");
                    }
                } else if let Err(e) = p.play() {
                    tracing::error!("Failed to play: {e}");
                }
                state.is_playing = !state.is_playing;
            }
            Command::none()
        }

        Message::Stop => {
            if let Some(ref mut p) = player {
                if let Err(e) = p.stop() {
                    tracing::error!("Failed to stop: {e}");
                }
                state.is_playing = false;
                state.current_time = 0;
            }
            Command::none()
        }

        Message::SeekTo(position) => {
            if let Some(ref mut p) = player {
                if let Err(e) = p.set_time(position) {
                    tracing::error!("Failed to seek: {e}");
                } else {
                    state.current_time = position;
                }
            }
            Command::none()
        }

        Message::VolumeChanged(volume) => {
            if let Some(ref mut p) = player {
                if let Err(e) = p.set_volume(volume) {
                    tracing::error!("Failed to set volume: {e}");
                }
                state.volume = volume;
                
                // Save to settings
                if let Err(e) = crate::db::queries::set_metadata(
                    db.connection(),
                    "volume",
                    &volume.to_string(),
                ) {
                    tracing::error!("Failed to save volume setting: {e}");
                }
            }
            Command::none()
        }

        Message::SpeedChanged(speed) => {
            if let Some(ref mut p) = player {
                if let Err(e) = p.set_rate(speed) {
                    tracing::error!("Failed to set speed: {e}");
                }
                state.speed = speed;
                
                // Save to settings
                if let Err(e) = crate::db::queries::set_metadata(
                    db.connection(),
                    "speed",
                    &speed.to_string(),
                ) {
                    tracing::error!("Failed to save speed setting: {e}");
                }
            }
            Command::none()
        }

        Message::AudiobookSelected(id) => {
            state.selected_audiobook = Some(id);
            
            // Load files for this audiobook
            match crate::db::queries::get_audiobook_files(db.connection(), id) {
                Ok(files) => {
                    state.current_files = files;
                }
                Err(e) => {
                    tracing::error!("Failed to load audiobook files: {e}");
                }
            }
            
            // Save selected audiobook
            if let Err(e) = crate::db::queries::set_metadata(
                db.connection(),
                "current_audiobook_id",
                &id.to_string(),
            ) {
                tracing::error!("Failed to save current audiobook: {e}");
            }
            
            Command::none()
        }

        Message::FileSelected(path) => {
            state.selected_file = Some(path.clone());
            
            if let Some(ref mut p) = player {
                if let Err(e) = p.load_media(Path::new(&path)) {
                    tracing::error!("Failed to load media: {e}");
                } else {
                    // Get duration from player
                    if let Ok(duration) = p.get_length() {
                        state.total_duration = duration;
                    }
                    
                    // Auto-play
                    if let Err(e) = p.play() {
                        tracing::error!("Failed to auto-play: {e}");
                    } else {
                        state.is_playing = true;
                    }
                }
            }
            
            Command::none()
        }

        Message::DirectoryAdd => {
            Command::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .pick_folder()
                        .await
                        .map(|handle| handle.path().to_string_lossy().to_string())
                },
                |result| match result {
                    Some(path) => Message::DirectoryAdded(path),
                    None => Message::DirectoryAddCancelled,
                },
            )
        }

        Message::DirectoryAdded(path) => {
            let directory = crate::models::Directory {
                full_path: path.clone(),
                created_at: chrono::Utc::now(),
                last_scanned: None,
            };
            
            if let Err(e) = crate::db::queries::insert_directory(db.connection(), &directory) {
                tracing::error!("Failed to insert directory: {e}");
                return Command::none();
            }
            
            state.directories.push(directory);
            
            // Note: Directory scanning would be triggered here but requires database access
            // For now, we skip the async scan since Database is not Clone
            tracing::info!("Directory added: {path}. Manual rescan recommended.");
            
            Command::none()
        }

        Message::DirectoryRemove(path) => {
            if let Err(e) = crate::db::queries::delete_directory(db.connection(), &path) {
                tracing::error!("Failed to delete directory: {e}");
            } else {
                state.directories.retain(|d| d.full_path != path);
                state.audiobooks.retain(|a| a.directory != path);
            }
            Command::none()
        }

        Message::DirectoryRescan(_path) => {
            // Note: Rescanning would require async access to database
            // For now, we skip this operation since Database is not Clone
            tracing::info!("Directory rescan requested but not implemented in this version");
            Command::none()
        }

        Message::OpenSettings => {
            state.settings_open = true;
            Command::none()
        }

        Message::CloseSettings => {
            state.settings_open = false;
            Command::none()
        }

        Message::ScanComplete(new_audiobooks) => {
            for audiobook in new_audiobooks {
                // Check if audiobook already exists
                let exists = state.audiobooks.iter().any(|a| a.full_path == audiobook.full_path);
                
                if !exists {
                    state.audiobooks.push(audiobook);
                }
            }
            Command::none()
        }

        Message::ScanError(error) => {
            tracing::error!("Scan error: {error}");
            Command::none()
        }

        Message::InitialLoadComplete => {
            state.is_loading = false;
            Command::none()
        }

        Message::PlayerTimeUpdated(time) => {
            state.current_time = time;
            
            // Update progress in database
            if let Some(ref file_path) = state.selected_file {
                if state.total_duration > 0 {
                    let completeness = ((time * 100) / state.total_duration) as i32;
                    
                    if let Err(e) = crate::db::queries::update_file_progress(
                        db.connection(),
                        file_path,
                        time,
                        completeness,
                    ) {
                        tracing::error!("Failed to update file progress: {e}");
                    }
                }
            }
            
            Command::none()
        }

        _ => Command::none(),
    }
}


