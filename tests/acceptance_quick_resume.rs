//! Acceptance tests for Quick Resume (`PlayLastListened`).

mod acceptance_support;

use acceptance_support::*;
use chrono::Utc;
use nodoka::db::queries;
use nodoka::ui::{Message, State};
use std::error::Error;

#[test]
fn test_play_last_listened_selects_most_recent_and_selects_last_file() -> Result<(), Box<dyn Error>>
{
    let db = create_test_db()?;

    let id_old = create_test_audiobook(&db, "/test", "Old Book")?;
    let id_new = create_test_audiobook(&db, "/test", "New Book")?;

    let old_file = "/test/Old Book/chapter_01.mp3";
    let new_file = "/test/New Book/chapter_01.mp3";
    insert_test_file(&db, id_old, old_file)?;
    insert_test_file(&db, id_new, new_file)?;

    queries::update_audiobook_selected_file(db.connection(), id_new, Some(new_file))?;

    let day_ago = Utc::now() - chrono::Duration::days(1);
    let now = Utc::now();
    queries::update_audiobook_last_listened(db.connection(), id_old, day_ago)?;
    queries::update_audiobook_last_listened(db.connection(), id_new, now)?;

    let mut state = State {
        audiobooks: queries::get_all_audiobooks(db.connection())?,
        ..State::default()
    };

    let mut player: Option<nodoka::player::Vlc> = None;

    let _task = nodoka::ui::update::update(&mut state, Message::PlayLastListened, &mut player, &db);
    assert_eq!(state.selected_audiobook, Some(id_new));

    let loaded = state.current_files.iter().any(|f| f.full_path == new_file);
    assert!(
        loaded,
        "Quick resume should load files for the selected audiobook"
    );

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::FileSelected(new_file.to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.selected_file.as_deref(), Some(new_file));

    let persisted =
        queries::get_audiobook_by_id(db.connection(), id_new)?.ok_or("Audiobook missing")?;
    assert_eq!(persisted.selected_file.as_deref(), Some(new_file));

    Ok(())
}

#[test]
fn test_play_last_listened_noop_when_never_played() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let _id1 = create_test_audiobook(&db, "/test", "Never Played 1")?;
    let _id2 = create_test_audiobook(&db, "/test", "Never Played 2")?;

    let mut state = State {
        audiobooks: queries::get_all_audiobooks(db.connection())?,
        ..State::default()
    };

    let mut player: Option<nodoka::player::Vlc> = None;
    let _task = nodoka::ui::update::update(&mut state, Message::PlayLastListened, &mut player, &db);

    assert_eq!(state.selected_audiobook, None);
    assert!(state.current_files.is_empty());

    Ok(())
}
