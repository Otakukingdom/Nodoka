use crate::db::Database;
use crate::player::Vlc;
use crate::tasks::{convert_to_audiobooks, scan_directory, DiscoveredAudiobook};
use crate::ui::{Message, State};
use iced::Command;

pub(super) fn handle_directory_add() -> Command<Message> {
    Command::perform(
        async {
            rfd::AsyncFileDialog::new()
                .pick_folder()
                .await
                .map(|handle| handle.path().to_string_lossy().to_string())
        },
        |result| result.map_or(Message::DirectoryAddCancelled, Message::DirectoryAdded),
    )
}

pub(super) fn handle_directory_added(
    state: &mut State,
    db: &Database,
    path: &str,
) -> Command<Message> {
    if state.directories.iter().any(|d| d.full_path == path) {
        tracing::info!("Directory already added: {path}. Skipping.");
        return Command::none();
    }

    let directory = crate::models::Directory {
        full_path: path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    if let Err(e) = crate::db::queries::insert_directory(db.connection(), &directory) {
        tracing::error!("Failed to insert directory: {e}");
        return Command::none();
    }

    state.directories.push(directory);
    tracing::info!("Directory added: {path}. Starting scan.");

    start_directory_scan(path.to_string())
}

pub(super) fn handle_directory_remove(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    path: &str,
) -> Command<Message> {
    let selected_audiobook_in_directory = state
        .selected_audiobook
        .and_then(|id| state.audiobooks.iter().find(|a| a.id == Some(id)))
        .is_some_and(|audiobook| audiobook.directory == path);

    let selected_file_in_directory = state
        .selected_file
        .as_ref()
        .is_some_and(|file_path| crate::ui::media_paths::path_is_within_directory(file_path, path));

    let current_files_in_directory = state
        .current_files
        .iter()
        .any(|file| crate::ui::media_paths::path_is_within_directory(&file.full_path, path));

    let should_clear_selection =
        selected_audiobook_in_directory || selected_file_in_directory || current_files_in_directory;

    if let Err(e) = crate::db::queries::delete_directory(db.connection(), path) {
        tracing::error!("Failed to delete directory: {e}");
    } else {
        state.directories.retain(|d| d.full_path != path);
        state.audiobooks.retain(|a| a.directory != path);
        if should_clear_selection {
            super::reset_playback_state(state, player);
            state.selected_audiobook = None;
            state.selected_file = None;
            state.current_files.clear();
            state.bookmarks.clear();
            state.bookmark_editor = None;
            if let Err(e) =
                crate::db::queries::delete_metadata(db.connection(), "current_audiobook_id")
            {
                tracing::error!("Failed to clear current audiobook metadata: {e}");
            }
        }
    }
    Command::none()
}

pub(super) fn handle_directory_rescan(
    _state: &mut State,
    db: &Database,
    path: &str,
) -> Command<Message> {
    tracing::info!("Directory rescan requested: {path}");
    if let Ok(audiobooks) = crate::db::queries::get_audiobooks_by_directory(db.connection(), path) {
        for audiobook in audiobooks {
            if let Some(id) = audiobook.id {
                if let Err(e) =
                    crate::db::queries::mark_audiobook_files_missing(db.connection(), id)
                {
                    tracing::error!("Failed to mark audiobook files missing: {e}");
                }
            }
        }
    }
    start_directory_scan(path.to_string())
}

pub(super) fn handle_scan_complete(
    state: &mut State,
    db: &Database,
    directory: &str,
    discovered: Vec<DiscoveredAudiobook>,
) -> Command<Message> {
    let audiobooks = convert_to_audiobooks(discovered.clone(), directory);

    update_state_with_discovered_audiobooks(state, &audiobooks);

    for (idx, disc) in discovered.into_iter().enumerate() {
        process_discovered_audiobook(state, db, directory, &audiobooks, idx, disc);
    }

    finalize_directory_scan(state, db, directory);

    Command::none()
}

pub(super) fn handle_scan_error(error: &str) -> Command<Message> {
    tracing::error!("Scan error: {error}");
    Command::none()
}

fn update_state_with_discovered_audiobooks(
    state: &mut State,
    audiobooks: &[crate::models::Audiobook],
) {
    for audiobook in audiobooks {
        let exists = state
            .audiobooks
            .iter()
            .any(|a| a.full_path == audiobook.full_path);

        if !exists {
            state.audiobooks.push(audiobook.clone());
        }
    }
}

fn process_discovered_audiobook(
    state: &mut State,
    db: &Database,
    directory: &str,
    audiobooks: &[crate::models::Audiobook],
    idx: usize,
    disc: DiscoveredAudiobook,
) {
    let disc_path = disc.path.display().to_string();
    let mut resolved_audiobook = audiobooks.get(idx).cloned().unwrap_or_else(|| {
        crate::models::Audiobook::new(
            directory.to_string(),
            disc.name.clone(),
            disc_path.clone(),
            i32::try_from(idx).unwrap_or(i32::MAX),
        )
    });

    let Some(audiobook_id) = resolve_audiobook_id(db, &disc_path, &mut resolved_audiobook) else {
        return;
    };

    update_state_audiobook_id(state, &resolved_audiobook, audiobook_id);
    process_audiobook_files(db, audiobook_id, disc.files, disc.checksums);
}

fn resolve_audiobook_id(
    db: &Database,
    disc_path: &str,
    resolved_audiobook: &mut crate::models::Audiobook,
) -> Option<i64> {
    let existing = match crate::db::queries::get_audiobook_by_path(db.connection(), disc_path) {
        Ok(existing) => existing,
        Err(e) => {
            tracing::error!("Failed to lookup audiobook by path {disc_path}: {e}");
            return None;
        }
    };

    let audiobook_id = if let Some(existing_ab) = existing {
        if let Some(id) = existing_ab.id {
            resolved_audiobook.id = Some(id);
            id
        } else if let Ok(id) =
            crate::db::queries::insert_audiobook(db.connection(), resolved_audiobook)
        {
            resolved_audiobook.id = Some(id);
            id
        } else {
            return None;
        }
    } else {
        match crate::db::queries::insert_audiobook(db.connection(), resolved_audiobook) {
            Ok(id) => {
                resolved_audiobook.id = Some(id);
                id
            }
            Err(e) => {
                tracing::error!("Failed to insert audiobook: {e}");
                return None;
            }
        }
    };

    Some(audiobook_id)
}

fn update_state_audiobook_id(
    state: &mut State,
    resolved_audiobook: &crate::models::Audiobook,
    audiobook_id: i64,
) {
    if let Some(entry) = state
        .audiobooks
        .iter_mut()
        .find(|a| a.full_path == resolved_audiobook.full_path)
    {
        entry.id = Some(audiobook_id);
    } else {
        let mut ab = resolved_audiobook.clone();
        ab.id = Some(audiobook_id);
        state.audiobooks.push(ab);
    }

    match crate::cover_cache::ensure_cover_thumbnail(
        audiobook_id,
        std::path::Path::new(&resolved_audiobook.full_path),
    ) {
        Ok(Some(path)) => {
            state.cover_thumbnails.insert(audiobook_id, path);
        }
        Ok(None) => {}
        Err(e) => {
            tracing::warn!("Failed to prepare cover thumbnail for audiobook {audiobook_id}: {e}");
        }
    }
}

fn process_audiobook_files(
    db: &Database,
    audiobook_id: i64,
    files: Vec<std::path::PathBuf>,
    checksums: Vec<Option<String>>,
) {
    let paired_len = files.len().min(checksums.len());
    let mut paired: Vec<(std::path::PathBuf, Option<String>)> = files
        .into_iter()
        .take(paired_len)
        .zip(checksums.into_iter().take(paired_len))
        .collect();

    paired.sort_by(|(a, _), (b, _)| {
        let a_name = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let b_name = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        natord::compare(a_name, b_name)
    });

    for (pos, (file_path, checksum)) in paired.into_iter().enumerate() {
        process_single_file(db, audiobook_id, pos, &file_path, checksum);
    }
}

fn process_single_file(
    db: &Database,
    audiobook_id: i64,
    pos: usize,
    file_path: &std::path::Path,
    checksum: Option<String>,
) {
    let name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();
    let full_path = file_path.display().to_string();
    let mut file = crate::models::AudiobookFile::new(
        audiobook_id,
        name,
        full_path.clone(),
        i32::try_from(pos).unwrap_or(i32::MAX),
    );
    file.checksum = checksum;

    if let Ok(Some(existing_file)) =
        crate::db::queries::get_audiobook_file_by_path(db.connection(), &full_path)
    {
        file.length_of_file = existing_file.length_of_file;

        let changed = match (existing_file.checksum.as_deref(), file.checksum.as_deref()) {
            (Some(old), Some(new)) => old != new,
            _ => false,
        };

        if file.checksum.is_none() {
            file.checksum.clone_from(&existing_file.checksum);
        }

        if changed {
            file.seek_position = None;
            file.completeness = 0;
        } else {
            file.seek_position = existing_file.seek_position;
            file.completeness = existing_file.completeness;
        }
        file.file_exists = true;
    }

    if let Err(e) = crate::db::queries::insert_audiobook_file(db.connection(), &file) {
        tracing::error!("Failed to insert audiobook file: {e}");
    }
}

fn finalize_directory_scan(state: &mut State, db: &Database, directory: &str) {
    if let Err(e) = crate::db::queries::update_directory_last_scanned(db.connection(), directory) {
        tracing::error!("Failed to update directory scan timestamp: {e}");
    }

    if let Some(dir) = state
        .directories
        .iter_mut()
        .find(|d| d.full_path == directory)
    {
        dir.last_scanned = Some(chrono::Utc::now());
    }
}

fn start_directory_scan(path: String) -> Command<Message> {
    Command::perform(
        async move {
            scan_directory(path.clone().into())
                .await
                .map(|discovered| (path, discovered))
        },
        |result| match result {
            Ok((path, discovered)) => Message::ScanComplete(path, discovered),
            Err(error) => Message::ScanError(error.to_string()),
        },
    )
}
