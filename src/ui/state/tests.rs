use super::*;

#[test]
fn test_state_default_values() {
    let state = State::default();
    assert!(state.audiobooks.is_empty());
    assert_eq!(state.selected_audiobook, None);
    assert!(state.current_files.is_empty());
    assert_eq!(state.selected_file, None);
    assert!(state.directories.is_empty());
    assert!(state.cover_thumbnails.is_empty());
    assert!(state.bookmarks.is_empty());
    assert!(state.bookmark_editor.is_none());
    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!(state.current_time.abs() < f64::EPSILON);
    assert!(state.total_duration.abs() < f64::EPSILON);
    assert_eq!(state.volume, 100);
    assert!((state.speed - 1.0).abs() < f32::EPSILON);
    assert!(state.sleep_timer.is_none());
    assert!(state.sleep_timer_base_volume.is_none());
    assert!(state.sleep_timer_custom_minutes.is_empty());
    assert!(state.sleep_timer_custom_error.is_none());
    assert_eq!(state.active_modal, ActiveModal::None);
    assert_eq!(state.load_state, LoadState::Loading);
    assert_eq!(state.scan_state, ScanState::Idle);
    assert_eq!(state.focused_element, FocusedElement::None);
    assert!(!state.operation_in_progress);
    assert!(!state.prefers_reduced_motion);
    assert!(state.notification_queue.is_empty());
    assert!(state.pulse_animations.is_empty());
    assert!(state.transition_states.is_empty());
    assert_eq!(
        state.modal_state,
        crate::ui::styles::animations::modal::ModalState::Closed
    );
    assert!(!state.modal_pending_close);
    assert_eq!(state.ui_time_ms, 0);
    assert!(state.audiobook_list_animation_start_ms.is_none());
    assert!(state.file_list_animation_start_ms.is_none());
    assert!(state.bookmark_list_animation_start_ms.is_none());
    assert!(state.tooltip_state.visible().is_none());
}

#[test]
fn test_state_clone() {
    let state = State::default();
    assert_eq!(state.volume, state.volume);
    assert!((state.speed - state.speed).abs() < f32::EPSILON);
    assert_eq!(state.playback, state.playback);
    assert_eq!(state.cover_thumbnails.len(), state.cover_thumbnails.len());
}

#[test]
fn test_state_mutation() {
    let state = State {
        playback: PlaybackStatus::Playing,
        current_time: 100.0,
        total_duration: 3600.0,
        volume: 75,
        speed: 1.5,
        ..Default::default()
    };

    assert_eq!(state.playback, PlaybackStatus::Playing);
    assert!((state.current_time - 100.0).abs() < f64::EPSILON);
    assert!((state.total_duration - 3600.0).abs() < f64::EPSILON);
    assert_eq!(state.volume, 75);
    assert!((state.speed - 1.5).abs() < f32::EPSILON);
}

#[test]
fn test_state_active_modal() {
    let mut state = State::default();
    assert_eq!(state.active_modal, ActiveModal::None);

    state.active_modal = ActiveModal::Settings;
    assert_eq!(state.active_modal, ActiveModal::Settings);

    state.active_modal = ActiveModal::BookmarkEditor;
    assert_eq!(state.active_modal, ActiveModal::BookmarkEditor);

    state.active_modal = ActiveModal::KeyboardHelp;
    assert_eq!(state.active_modal, ActiveModal::KeyboardHelp);

    state.active_modal = ActiveModal::None;
    assert_eq!(state.active_modal, ActiveModal::None);
}

#[test]
fn test_state_selection() {
    let mut state = State::default();
    assert_eq!(state.selected_audiobook, None);
    assert_eq!(state.selected_file, None);

    state.selected_audiobook = Some(42);
    state.selected_file = Some("/path/to/file.mp3".to_string());

    assert_eq!(state.selected_audiobook, Some(42));
    assert_eq!(state.selected_file, Some("/path/to/file.mp3".to_string()));
}

#[test]
fn test_state_loading_flag() {
    let mut state = State::default();
    assert_eq!(state.load_state, LoadState::Loading);

    state.load_state = LoadState::Ready;
    assert_eq!(state.load_state, LoadState::Ready);
}

#[test]
fn test_state_with_negative_time_values() {
    // Test that state can handle negative time values (edge case)
    let state = State {
        current_time: -100.0,
        total_duration: -50.0,
        ..Default::default()
    };

    assert!(state.current_time < 0.0);
    assert!(state.total_duration < 0.0);
}

#[test]
fn test_state_with_time_exceeding_duration() {
    // Test edge case where current_time > total_duration
    let state = State {
        current_time: 5000.0,
        total_duration: 3000.0,
        ..Default::default()
    };

    assert!(state.current_time > state.total_duration);
}

#[test]
fn test_state_with_invalid_volume_values() {
    // Test that state can handle out-of-range volume values
    let state_low = State {
        volume: -10,
        ..Default::default()
    };
    assert_eq!(state_low.volume, -10);

    let state_high = State {
        volume: 999,
        ..Default::default()
    };
    assert_eq!(state_high.volume, 999);
}

#[test]
fn test_state_with_invalid_speed_values() {
    // Test that state can handle out-of-range speed values
    let state_low = State {
        speed: 0.1,
        ..Default::default()
    };
    assert!((state_low.speed - 0.1).abs() < f32::EPSILON);

    let state_high = State {
        speed: 5.0,
        ..Default::default()
    };
    assert!((state_high.speed - 5.0).abs() < f32::EPSILON);
}

#[test]
fn test_state_with_zero_duration() {
    let state = State {
        current_time: 100.0,
        total_duration: 0.0,
        ..Default::default()
    };

    assert!((state.total_duration).abs() < f64::EPSILON);
}

#[test]
fn test_bookmark_editor_default_values() {
    let editor = BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/path/file.mp3".to_string(),
        position_ms: 0,
        label: String::new(),
        note: String::new(),
    };

    assert_eq!(editor.id, None);
    assert_eq!(editor.audiobook_id, 1);
    assert_eq!(editor.file_path, "/path/file.mp3");
    assert_eq!(editor.position_ms, 0);
    assert!(editor.label.is_empty());
    assert!(editor.note.is_empty());
}

#[test]
fn test_bookmark_editor_with_data() {
    let editor = BookmarkEditor {
        id: Some(42),
        audiobook_id: 1,
        file_path: "/path/file.mp3".to_string(),
        position_ms: 12345,
        label: "Test Label".to_string(),
        note: "Test Note".to_string(),
    };

    assert_eq!(editor.id, Some(42));
    assert_eq!(editor.label, "Test Label");
    assert_eq!(editor.note, "Test Note");
    assert_eq!(editor.position_ms, 12345);
}

#[test]
fn test_bookmark_editor_clone() {
    let editor = BookmarkEditor {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/path/file.mp3".to_string(),
        position_ms: 1000,
        label: "Test".to_string(),
        note: "Note".to_string(),
    };

    let cloned = &editor;
    assert_eq!(editor.id, cloned.id);
    assert_eq!(editor.label, cloned.label);
    assert_eq!(editor.note, cloned.note);
}

#[test]
fn test_state_with_sleep_timer_custom_minutes() {
    let state = State {
        sleep_timer_custom_minutes: "45".to_string(),
        ..Default::default()
    };

    assert_eq!(state.sleep_timer_custom_minutes, "45");
}

#[test]
fn test_state_with_sleep_timer_custom_error() {
    let state = State {
        sleep_timer_custom_error: Some("Invalid input".to_string()),
        ..Default::default()
    };

    assert_eq!(
        state.sleep_timer_custom_error,
        Some("Invalid input".to_string())
    );
}

#[test]
fn test_state_with_bookmark_editor_open() {
    let editor = BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/path/file.mp3".to_string(),
        position_ms: 1000,
        label: "Test".to_string(),
        note: String::new(),
    };

    let state = State {
        bookmark_editor: Some(editor),
        ..Default::default()
    };

    assert!(state.bookmark_editor.is_some());
}

#[test]
fn test_state_with_multiple_directories() {
    let state = State {
        directories: vec![
            Directory::new("/path1".to_string()),
            Directory::new("/path2".to_string()),
            Directory::new("/path3".to_string()),
        ],
        ..Default::default()
    };

    assert_eq!(state.directories.len(), 3);
}

#[test]
fn test_state_with_multiple_audiobooks() {
    use chrono::Utc;

    let state = State {
        audiobooks: vec![
            Audiobook {
                id: Some(1),
                directory: "/test".to_string(),
                name: "Book 1".to_string(),
                author: None,
                full_path: "/test/book1".to_string(),
                completeness: 50,
                default_order: 0,
                selected_file: None,
                total_duration_ms: None,
                created_at: Utc::now(),
                last_listened: None,
            },
            Audiobook {
                id: Some(2),
                directory: "/test".to_string(),
                name: "Book 2".to_string(),
                author: None,
                full_path: "/test/book2".to_string(),
                completeness: 75,
                default_order: 0,
                selected_file: None,
                total_duration_ms: None,
                created_at: Utc::now(),
                last_listened: None,
            },
        ],
        ..Default::default()
    };

    assert_eq!(state.audiobooks.len(), 2);
}

#[test]
fn test_state_with_cover_thumbnails() {
    let mut state = State::default();
    state
        .cover_thumbnails
        .insert(1, PathBuf::from("/cover1.jpg"));
    state
        .cover_thumbnails
        .insert(2, PathBuf::from("/cover2.jpg"));

    assert_eq!(state.cover_thumbnails.len(), 2);
    assert!(state.cover_thumbnails.contains_key(&1));
    assert!(state.cover_thumbnails.contains_key(&2));
}

#[test]
fn test_state_debug_implementation() {
    let state = State::default();
    let debug_str = format!("{state:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn test_bookmark_editor_debug_implementation() {
    let editor = BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/path/file.mp3".to_string(),
        position_ms: 1000,
        label: "Test".to_string(),
        note: String::new(),
    };
    let debug_str = format!("{editor:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn test_notification_success_type() {
    let notification = Notification::success("Test message".to_string());
    assert_eq!(notification.message, "Test message");
    assert_eq!(notification.kind, NotificationType::Success);
    assert_eq!(notification.duration_ms, 3000);
}

#[test]
fn test_notification_info_type() {
    let notification = Notification::info("Info message".to_string());
    assert_eq!(notification.kind, NotificationType::Info);
    assert_eq!(notification.duration_ms, 3000);
}

#[test]
fn test_notification_warning_type() {
    let notification = Notification::warning("Warning message".to_string());
    assert_eq!(notification.kind, NotificationType::Warning);
    assert_eq!(notification.duration_ms, 5000);
}

#[test]
fn test_notification_error_type() {
    let notification = Notification::error("Error message".to_string());
    assert_eq!(notification.kind, NotificationType::Error);
    assert_eq!(notification.duration_ms, u64::MAX); // No auto-dismiss
}

#[test]
fn test_notification_expiry() {
    let mut notification = Notification::success("Test".to_string());
    // Set created_at to past time to test expiry
    notification.created_at = current_time_ms().saturating_sub(4000);
    assert!(notification.is_expired());

    // Fresh notification should not be expired
    let fresh = Notification::success("Test".to_string());
    assert!(!fresh.is_expired());
}

#[test]
fn test_transition_state_initialization() {
    let transition = TransitionState::new(0.0, 100.0, 1000);
    assert!((transition.from_value).abs() < f32::EPSILON);
    assert!((transition.to_value - 100.0).abs() < f32::EPSILON);
    assert!((transition.progress).abs() < f32::EPSILON);
    assert_eq!(transition.duration_ms, 1000);
}

#[test]
fn test_transition_state_update() {
    let mut transition = TransitionState::new(0.0, 100.0, 1000);

    // Not complete after 500ms
    let complete = transition.update(500);
    assert!(!complete);
    assert!(transition.progress > 0.0);
    assert!(transition.progress < 1.0);

    // Complete after another 500ms
    let complete = transition.update(500);
    assert!(complete);
    assert!((transition.progress - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_transition_state_current_value() {
    let mut transition = TransitionState::new(0.0, 100.0, 1000);

    // At start
    let start_value = transition.current_value();
    assert!((start_value).abs() < 1.0);

    // At midpoint
    transition.update(500);
    let mid_value = transition.current_value();
    assert!(mid_value > 25.0);
    assert!(mid_value < 75.0); // Easing affects exact midpoint

    // At end
    transition.update(500);
    let end_value = transition.current_value();
    assert!((end_value - 100.0).abs() < 1.0);
}

#[test]
fn test_transition_state_is_complete() {
    let mut transition = TransitionState::new(0.0, 100.0, 1000);
    assert!(!transition.is_complete());

    transition.update(1000);
    assert!(transition.is_complete());
}

#[test]
fn test_transition_state_zero_duration() {
    let mut transition = TransitionState::new(0.0, 100.0, 0);
    let complete = transition.update(0);
    assert!(complete);
    assert!((transition.progress - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_state_with_notifications() {
    let mut state = State::default();
    state
        .notification_queue
        .push(Notification::success("Test".to_string()));
    state
        .notification_queue
        .push(Notification::info("Info".to_string()));

    assert_eq!(state.notification_queue.len(), 2);
}

#[test]
fn test_state_with_pulse_animations() {
    use crate::ui::styles::animations::PulseAnimation;

    let mut state = State::default();
    state
        .pulse_animations
        .insert("playing_indicator".to_string(), PulseAnimation::new(1000));

    assert_eq!(state.pulse_animations.len(), 1);
    assert!(state.pulse_animations.contains_key("playing_indicator"));
}

#[test]
fn test_state_with_transition_states() {
    let mut state = State::default();
    state.transition_states.insert(
        "modal_fade".to_string(),
        TransitionState::new(0.0, 1.0, 200),
    );

    assert_eq!(state.transition_states.len(), 1);
    assert!(state.transition_states.contains_key("modal_fade"));
}

#[test]
fn test_state_modal_state_override() {
    let state = State {
        modal_state: crate::ui::styles::animations::modal::ModalState::Open,
        ..Default::default()
    };

    assert_eq!(
        state.modal_state,
        crate::ui::styles::animations::modal::ModalState::Open
    );
}
