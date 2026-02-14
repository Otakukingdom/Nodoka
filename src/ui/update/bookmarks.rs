use crate::conversions::{f64_to_ms, ms_to_f64};
use crate::db::Database;
use crate::models::Bookmark;
use crate::player::Vlc;
use crate::ui::{Message, State};
use iced::Command;

pub(super) fn handle_create_bookmark(state: &mut State, db: &Database) -> Command<Message> {
    let (Some(audiobook_id), Some(file_path)) =
        (state.selected_audiobook, state.selected_file.clone())
    else {
        return Command::none();
    };

    let position_ms = match f64_to_ms(state.current_time) {
        Ok(ms) => ms,
        Err(e) => {
            tracing::error!("Failed to convert bookmark position: {e}");
            return Command::none();
        }
    };

    let bookmark = Bookmark::new(
        audiobook_id,
        file_path.clone(),
        position_ms,
        String::from("Bookmark"),
    );

    let created_id = match crate::db::queries::insert_bookmark(db.connection(), &bookmark) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to create bookmark: {e}");
            return Command::none();
        }
    };

    if let Ok(bookmarks) =
        crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)
    {
        state.bookmarks = bookmarks;
    }

    state.bookmark_editor = Some(crate::ui::state::BookmarkEditor {
        id: Some(created_id),
        audiobook_id,
        file_path,
        position_ms,
        label: String::from("Bookmark"),
        note: String::new(),
    });

    Command::none()
}

pub(super) fn handle_bookmark_editor_label_changed(
    state: &mut State,
    value: &str,
) -> Command<Message> {
    if let Some(editor) = state.bookmark_editor.as_mut() {
        editor.label = value.to_string();
    }
    Command::none()
}

pub(super) fn handle_bookmark_editor_note_changed(
    state: &mut State,
    value: &str,
) -> Command<Message> {
    if let Some(editor) = state.bookmark_editor.as_mut() {
        editor.note = value.to_string();
    }
    Command::none()
}

pub(super) fn handle_bookmark_editor_cancel(state: &mut State) -> Command<Message> {
    state.bookmark_editor = None;
    Command::none()
}

pub(super) fn handle_bookmark_editor_save(state: &mut State, db: &Database) -> Command<Message> {
    let Some(editor) = state.bookmark_editor.clone() else {
        return Command::none();
    };

    let label = if editor.label.trim().is_empty() {
        String::from("Bookmark")
    } else {
        editor.label.clone()
    };

    let note = if editor.note.trim().is_empty() {
        None
    } else {
        Some(editor.note.clone())
    };

    if let Some(id) = editor.id {
        let Some(existing) = state.bookmarks.iter().find(|b| b.id == Some(id)).cloned() else {
            tracing::warn!("Bookmark {id} not found in state; cannot edit");
            return Command::none();
        };

        let updated = Bookmark {
            id: Some(id),
            audiobook_id: existing.audiobook_id,
            file_path: existing.file_path,
            position_ms: existing.position_ms,
            label,
            note,
            created_at: existing.created_at,
        };

        if let Err(e) = crate::db::queries::update_bookmark(db.connection(), &updated) {
            tracing::error!("Failed to update bookmark: {e}");
            return Command::none();
        }
    } else {
        let mut bookmark = Bookmark::new(
            editor.audiobook_id,
            editor.file_path,
            editor.position_ms,
            label,
        );
        bookmark.note = note;

        if let Err(e) = crate::db::queries::insert_bookmark(db.connection(), &bookmark) {
            tracing::error!("Failed to create bookmark: {e}");
            return Command::none();
        }
    }

    match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), editor.audiobook_id) {
        Ok(bms) => state.bookmarks = bms,
        Err(e) => tracing::error!("Failed to reload bookmarks: {e}"),
    }

    state.bookmark_editor = None;
    Command::none()
}

pub(super) fn handle_bookmark_edit(state: &mut State, id: i64) -> Command<Message> {
    let Some(bm) = state.bookmarks.iter().find(|b| b.id == Some(id)).cloned() else {
        tracing::warn!("Bookmark {id} not found for edit");
        return Command::none();
    };

    state.bookmark_editor = Some(crate::ui::state::BookmarkEditor {
        id: Some(id),
        audiobook_id: bm.audiobook_id,
        file_path: bm.file_path,
        position_ms: bm.position_ms,
        label: bm.label,
        note: bm.note.unwrap_or_default(),
    });

    Command::none()
}

pub(super) fn handle_bookmark_delete(
    state: &mut State,
    db: &Database,
    id: i64,
) -> Command<Message> {
    let audiobook_id = state.selected_audiobook;
    if let Err(e) = crate::db::queries::delete_bookmark(db.connection(), id) {
        tracing::error!("Failed to delete bookmark: {e}");
        return Command::none();
    }

    if let Some(audiobook_id) = audiobook_id {
        match crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id) {
            Ok(bms) => state.bookmarks = bms,
            Err(e) => tracing::error!("Failed to reload bookmarks: {e}"),
        }
    } else {
        state.bookmarks.clear();
    }

    Command::none()
}

pub(super) fn handle_bookmark_jump(
    state: &mut State,
    player: &mut Option<Vlc>,
    db: &Database,
    id: i64,
) -> Command<Message> {
    let Some(bm) = state.bookmarks.iter().find(|b| b.id == Some(id)).cloned() else {
        tracing::warn!("Bookmark {id} not found for jump");
        return Command::none();
    };

    let cmd = super::handle_file_selected(state, player, db, &bm.file_path);

    if let Some(ref p) = player {
        if let Err(e) = p.set_time(bm.position_ms) {
            tracing::error!("Failed to seek to bookmark: {e}");
        } else if let Ok(pos) = ms_to_f64(bm.position_ms) {
            state.current_time = pos;
        }
    }

    cmd
}
