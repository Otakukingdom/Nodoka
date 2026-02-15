//! UI interaction coverage tests
//!
//! These tests drive the real `nodoka::ui::update::update` handler and assert observable state
//! transitions and persisted metadata. They avoid framework-introspection assertions that iced does
//! not expose via `Element`.

use nodoka::db::{self, Database};
use nodoka::models::{Audiobook, AudiobookFile, SleepTimerMode};
use nodoka::player::Vlc;
use nodoka::ui::{LoadState, Message, PlaybackStatus, ScanState, State};

fn db_in_memory() -> Result<Database, Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;
    Ok(db)
}

#[test]
fn play_pause_does_not_toggle_without_player() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State {
        playback: PlaybackStatus::Paused,
        ..State::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::PlayPause, &mut player, &db);
    assert_eq!(state.playback, PlaybackStatus::Paused);

    Ok(())
}

#[test]
fn speed_changed_clamps_quantizes_and_persists() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::SpeedChanged(0.75), &mut player, &db);
    assert!((state.speed - 0.75).abs() < 0.0001);

    let saved = nodoka::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "0.75");

    Ok(())
}

#[test]
fn volume_changed_clamps_and_persists() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::VolumeChanged(999), &mut player, &db);
    assert_eq!(state.volume, 200);

    let saved = nodoka::db::queries::get_metadata(db.connection(), "volume")?
        .ok_or("missing volume metadata")?;
    assert_eq!(saved, "200");

    Ok(())
}

#[test]
fn open_settings_sets_modal_state() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::OpenSettings, &mut player, &db);
    assert!(state.settings_open);

    let _ = nodoka::ui::update::update(&mut state, Message::CloseSettings, &mut player, &db);
    assert!(!state.settings_open);

    Ok(())
}

#[test]
fn dismiss_error_clears_error_state() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State {
        error_message: Some("oops".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        ..State::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::DismissError, &mut player, &db);
    assert!(state.error_message.is_none());
    assert!(state.error_timestamp.is_none());

    Ok(())
}

#[test]
fn sleep_timer_duration_button_creates_timer() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerSetDurationSeconds(15 * 60),
        &mut player,
        &db,
    );

    let Some(timer) = state.sleep_timer.as_ref() else {
        return Err("expected active timer".into());
    };
    assert!(matches!(timer.mode, SleepTimerMode::Duration(secs) if secs == 15 * 60));

    Ok(())
}

#[test]
fn directory_added_enters_scanning_state() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let temp = temp_dir::TempDir::new()?;
    let dir_path = std::fs::canonicalize(temp.path())?;
    let dir = dir_path.to_string_lossy().into_owned();
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::DirectoryAdded(dir.clone()),
        &mut player,
        &db,
    );

    assert!(matches!(
        &state.scan_state,
        ScanState::Scanning { directory: Some(d) } if d == &dir
    ));

    Ok(())
}

#[test]
fn initial_load_complete_marks_ready() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::InitialLoadComplete, &mut player, &db);
    assert_eq!(state.load_state, LoadState::Ready);

    Ok(())
}

#[test]
fn create_bookmark_opens_editor_for_selected_file() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_in_memory()?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = nodoka::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(
        audiobook_id,
        "ch1.mp3".to_string(),
        file_path.to_string(),
        0,
    );
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1_500.0,
        ..State::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());

    Ok(())
}
