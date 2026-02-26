use crate::db::Database;
use crate::player::Vlc;
use crate::tasks::{convert_to_audiobooks, scan_directory, DiscoveredAudiobook, DiscoveredFile};
use crate::ui::state::{Notification, NotificationType};
use crate::ui::{Message, ScanState, State};
use iced::Task;

fn is_legacy_sha256(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_fs_fingerprint(value: &str) -> bool {
    value.starts_with("fs:v1:")
}

pub(super) fn handle_directory_add() -> Task<Message> {
    Task::perform(
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
) -> Task<Message> {
    let stored_path = normalize_directory_path_for_storage(path);
    let compare_key = normalize_directory_path_for_compare(&stored_path);

    if state
        .directories
        .iter()
        .any(|d| normalize_directory_path_for_compare(&d.full_path) == compare_key)
    {
        tracing::info!("Directory already added: {stored_path}. Skipping.");
        return Task::none();
    }

    let directory = crate::models::Directory {
        full_path: stored_path.clone(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    if let Err(e) = crate::db::queries::insert_directory(db.connection(), &directory) {
        tracing::error!("Failed to insert directory: {e}");
        return Task::none();
    }

    state.directories.push(directory);
    tracing::info!("Directory added: {stored_path}. Starting scan.");

    // Set scanning state
    state.scan_state = ScanState::Scanning {
        directory: Some(stored_path.clone()),
    };

    start_directory_scan(stored_path)
}

pub(super) fn handle_directory_remove(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    path: &str,
) -> Task<Message> {
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
        let before_len = state.audiobooks.len();
        state.audiobooks.retain(|a| a.directory != path);
        if state.audiobooks.len() != before_len {
            state.audiobook_list_animation_start_ms = Some(crate::ui::state::current_time_ms());
        }
        if should_clear_selection {
            super::playback::reset_playback_state(state, player);
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
    Task::none()
}

pub(super) fn handle_directory_rescan(
    state: &mut State,
    db: &Database,
    path: &str,
) -> Task<Message> {
    tracing::info!("Directory rescan requested: {path}");

    // Set scanning state
    state.scan_state = ScanState::Scanning {
        directory: Some(path.to_string()),
    };

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
) -> Task<Message> {
    let discovered_count = discovered.len();

    // Clear scanning state and any previous errors
    state.scan_state = ScanState::Idle;

    let audiobooks = convert_to_audiobooks(discovered.clone(), directory);

    let mut cmds: Vec<Task<Message>> = Vec::new();

    for (idx, disc) in discovered.into_iter().enumerate() {
        if let Some(cmd) =
            process_discovered_audiobook(state, db, directory, &audiobooks, idx, disc)
        {
            cmds.push(cmd);
        }
    }

    finalize_directory_scan(state, db, directory);

    state.notification_queue.push(Notification::info_with_title(
        "Scan Complete".to_string(),
        format!(
            "Found {discovered_count} audiobook{} in {directory}.",
            if discovered_count == 1 { "" } else { "s" }
        ),
    ));

    Task::batch(cmds)
}

pub(super) fn handle_scan_error(state: &mut State, error: &str) -> Task<Message> {
    state.scan_state = ScanState::Idle;
    state
        .notification_queue
        .retain(|notification| notification.kind != NotificationType::Error);
    state
        .notification_queue
        .push(Notification::error_with_title(
            "Scan Failed".to_string(),
            format!("Failed to scan directory: {error}"),
        ));

    tracing::error!("Scan error: {error}");
    Task::none()
}

fn process_discovered_audiobook(
    state: &mut State,
    db: &Database,
    directory: &str,
    audiobooks: &[crate::models::Audiobook],
    idx: usize,
    disc: DiscoveredAudiobook,
) -> Option<Task<Message>> {
    let disc_path = disc.path.display().to_string();
    let mut resolved_audiobook = audiobooks.get(idx).cloned().unwrap_or_else(|| {
        crate::models::Audiobook::new(
            directory.to_string(),
            disc.name.clone(),
            disc_path.clone(),
            i32::try_from(idx).unwrap_or(i32::MAX),
        )
    });

    let audiobook_id = resolve_audiobook_id(db, &disc_path, &mut resolved_audiobook)?;

    let cmd = update_state_audiobook_id(state, &resolved_audiobook, audiobook_id);
    process_audiobook_files(db, audiobook_id, disc.files);

    cmd
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
) -> Option<Task<Message>> {
    let mut inserted_new = false;
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
        inserted_new = true;
    }

    if inserted_new {
        state.audiobook_list_animation_start_ms = Some(crate::ui::state::current_time_ms());
    }

    if state.cover_thumbnails.contains_key(&audiobook_id) {
        return None;
    }

    // Generate cover thumbnails asynchronously to keep directory scans non-blocking.
    Some(super::library::generate_cover_thumbnail_command(
        audiobook_id,
        resolved_audiobook.full_path.clone(),
    ))
}

fn normalize_directory_path_for_storage(path: &str) -> String {
    let raw = std::path::PathBuf::from(path);
    let canonical = std::fs::canonicalize(&raw).unwrap_or(raw);
    trim_trailing_separators(&canonical.to_string_lossy())
}

fn normalize_directory_path_for_compare(path: &str) -> String {
    let stored = normalize_directory_path_for_storage(path);
    #[cfg(windows)]
    {
        return stored.to_ascii_lowercase();
    }
    #[cfg(not(windows))]
    {
        stored
    }
}

fn trim_trailing_separators(path: &str) -> String {
    let mut out = path;
    while out.len() > 1 && (out.ends_with('/') || out.ends_with('\\')) {
        out = &out[..out.len() - 1];
    }
    out.to_string()
}

fn process_audiobook_files(db: &Database, audiobook_id: i64, files: Vec<DiscoveredFile>) {
    let mut files = files;
    files.sort_by(|a, b| natord::compare(&a.sort_key, &b.sort_key));

    for (pos, file) in files.into_iter().enumerate() {
        process_single_file(db, audiobook_id, pos, file);
    }

    // Calculate and update total duration after processing all files
    update_audiobook_total_duration(db, audiobook_id);
}

fn process_single_file(db: &Database, audiobook_id: i64, pos: usize, discovered: DiscoveredFile) {
    let name = discovered.name;
    let full_path = discovered.full_path;
    let mut file = crate::models::AudiobookFile::new(
        audiobook_id,
        name,
        full_path.clone(),
        i32::try_from(pos).unwrap_or(i32::MAX),
    );
    file.checksum = discovered.checksum;

    if let Ok(Some(existing_file)) =
        crate::db::queries::get_audiobook_file_by_path(db.connection(), &full_path)
    {
        file.length_of_file = existing_file.length_of_file;

        // Preserve progress when migrating from legacy SHA-256 checksums to cheap filesystem
        // fingerprints.
        let changed = match (existing_file.checksum.as_deref(), file.checksum.as_deref()) {
            (Some(old), Some(new)) if is_legacy_sha256(old) && is_fs_fingerprint(new) => false,
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

/// Calculates and updates the total duration for an audiobook
///
/// Sums all file durations and stores in the audiobook's `total_duration_ms` field.
/// This enables accurate remaining time calculations in the UI.
fn update_audiobook_total_duration(db: &Database, audiobook_id: i64) {
    // Get all files for this audiobook
    let files = match crate::db::queries::get_audiobook_files(db.connection(), audiobook_id) {
        Ok(files) => files,
        Err(e) => {
            tracing::error!(
                "Failed to get files for audiobook {audiobook_id} while calculating duration: {e}"
            );
            return;
        }
    };

    // Sum up all file durations (skip files without duration)
    let total_duration_ms: i64 = files
        .iter()
        .filter_map(|f| f.length_of_file)
        .sum();

    // Only store if we have at least one file with duration
    let duration_to_store = if total_duration_ms > 0 {
        Some(total_duration_ms)
    } else {
        None
    };

    // Update the audiobook's total duration
    if let Err(e) = crate::db::queries::update_audiobook_total_duration(
        db.connection(),
        audiobook_id,
        duration_to_store,
    ) {
        tracing::error!("Failed to update total duration for audiobook {audiobook_id}: {e}");
    }
}

fn start_directory_scan(path: String) -> Task<Message> {
    Task::perform(
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

#[cfg(test)]
#[allow(clippy::unwrap_used)] // Test assertions use unwrap for clarity
mod tests {
    use super::*;

    #[test]
    fn test_update_audiobook_total_duration_calculates_sum() {
        let db = crate::db::Database::new_in_memory().unwrap();
        crate::db::initialize(db.connection()).unwrap();

        // Create an audiobook
        let audiobook = crate::models::Audiobook::new(
            "/test".to_string(),
            "Test Book".to_string(),
            "/test/book".to_string(),
            0,
        );
        let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook).unwrap();

        // Insert some files with durations
        let file1 = crate::models::AudiobookFile {
            audiobook_id,
            name: "Chapter 1".to_string(),
            full_path: "/test/book/chapter1.mp3".to_string(),
            length_of_file: Some(3_600_000), // 1 hour
            seek_position: None,
            checksum: None,
            position: 0,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        crate::db::queries::insert_audiobook_file(db.connection(), &file1).unwrap();

        let file2 = crate::models::AudiobookFile {
            audiobook_id,
            name: "Chapter 2".to_string(),
            full_path: "/test/book/chapter2.mp3".to_string(),
            length_of_file: Some(7_200_000), // 2 hours
            seek_position: None,
            checksum: None,
            position: 1,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        crate::db::queries::insert_audiobook_file(db.connection(), &file2).unwrap();

        // Calculate total duration
        update_audiobook_total_duration(&db, audiobook_id);

        // Verify total duration was stored correctly
        let updated = crate::db::queries::get_audiobook_by_id(db.connection(), audiobook_id)
            .unwrap()
            .unwrap();
        assert_eq!(updated.total_duration_ms, Some(10_800_000)); // 3 hours total
    }

    #[test]
    fn test_update_audiobook_total_duration_handles_missing_durations() {
        let db = crate::db::Database::new_in_memory().unwrap();
        crate::db::initialize(db.connection()).unwrap();

        let audiobook = crate::models::Audiobook::new(
            "/test".to_string(),
            "Test Book".to_string(),
            "/test/book".to_string(),
            0,
        );
        let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook).unwrap();

        // Insert file without duration
        let file = crate::models::AudiobookFile {
            audiobook_id,
            name: "Chapter 1".to_string(),
            full_path: "/test/book/chapter1.mp3".to_string(),
            length_of_file: None,
            seek_position: None,
            checksum: None,
            position: 0,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        crate::db::queries::insert_audiobook_file(db.connection(), &file).unwrap();

        update_audiobook_total_duration(&db, audiobook_id);

        // Should be None when no files have durations
        let updated = crate::db::queries::get_audiobook_by_id(db.connection(), audiobook_id)
            .unwrap()
            .unwrap();
        assert_eq!(updated.total_duration_ms, None);
    }
}
