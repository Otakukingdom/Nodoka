use crate::db::{self, Database};
use crate::ui::{Message, State};

#[test]
fn test_operation_in_progress_guards_directory_actions(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(&mut state, Message::DirectoryAdd, &mut player, &db);
    assert!(
        state.operation_in_progress,
        "DirectoryAdd should mark operation as in progress"
    );

    let _ = update(&mut state, Message::DirectoryAddCancelled, &mut player, &db);
    assert!(
        !state.operation_in_progress,
        "DirectoryAddCancelled should clear operation_in_progress"
    );

    let _ = update(
        &mut state,
        Message::DirectoryRescan("/dir".to_string()),
        &mut player,
        &db,
    );
    assert!(
        state.operation_in_progress,
        "DirectoryRescan should mark operation as in progress"
    );

    let _ = update(
        &mut state,
        Message::ScanError("scan failed".to_string()),
        &mut player,
        &db,
    );
    assert!(
        !state.operation_in_progress,
        "ScanError should clear operation_in_progress"
    );
    Ok(())
}
