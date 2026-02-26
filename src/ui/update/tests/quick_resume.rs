use crate::db::{self, Database};
use crate::models::{Audiobook, AudiobookFile};
use crate::ui::State;

#[test]
fn test_play_last_listened_selects_resume_file_when_selected_file_missing(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut older = Audiobook::new(
        "/dir".to_string(),
        "Older".to_string(),
        "/dir/older".to_string(),
        0,
    );
    let older_id = crate::db::queries::insert_audiobook(db.connection(), &older)?;
    older.id = Some(older_id);
    older.last_listened = Some(chrono::Utc::now() - chrono::Duration::minutes(10));

    let mut newer = Audiobook::new(
        "/dir".to_string(),
        "Newer".to_string(),
        "/dir/newer".to_string(),
        1,
    );
    let newer_id = crate::db::queries::insert_audiobook(db.connection(), &newer)?;
    newer.id = Some(newer_id);
    newer.last_listened = Some(chrono::Utc::now() - chrono::Duration::minutes(1));
    newer.selected_file = None;

    let ch1 = "/dir/newer/ch1.mp3";
    let mut f1 = AudiobookFile::new(newer_id, "ch1".to_string(), ch1.to_string(), 0);
    f1.length_of_file = Some(60_000);
    f1.seek_position = Some(60_000);
    f1.completeness = 100;
    crate::db::queries::insert_audiobook_file(db.connection(), &f1)?;

    let ch2 = "/dir/newer/ch2.mp3";
    let mut f2 = AudiobookFile::new(newer_id, "ch2".to_string(), ch2.to_string(), 1);
    f2.length_of_file = Some(300_000);
    f2.seek_position = Some(120_000);
    f2.completeness = 40;
    crate::db::queries::insert_audiobook_file(db.connection(), &f2)?;

    let ch3 = "/dir/newer/ch3.mp3";
    let mut f3 = AudiobookFile::new(newer_id, "ch3".to_string(), ch3.to_string(), 2);
    f3.length_of_file = Some(300_000);
    f3.seek_position = None;
    f3.completeness = 0;
    crate::db::queries::insert_audiobook_file(db.connection(), &f3)?;

    let mut state = State {
        audiobooks: vec![older, newer],
        ..Default::default()
    };
    let mut player = None;

    let _ = super::super::handle_play_last_listened(&mut state, &mut player, &db);

    assert_eq!(state.selected_audiobook, Some(newer_id));
    assert_eq!(
        state.selected_file.as_deref(),
        Some(ch2),
        "quick resume should pick the most plausible in-progress file"
    );
    Ok(())
}
