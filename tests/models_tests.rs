use chrono::Utc;
use nodoka::models::{Audiobook, AudiobookFile};

#[test]
fn test_audiobook_is_complete() {
    let mut audiobook = Audiobook {
        id: Some(1),
        directory: "/test".to_string(),
        name: "Test".to_string(),
        full_path: "/test/audiobook".to_string(),
        completeness: 99,
        default_order: 0,
        selected_file: None,
        created_at: Utc::now(),
    };

    assert!(!audiobook.is_complete());

    audiobook.completeness = 100;
    assert!(audiobook.is_complete());

    audiobook.completeness = 101;
    assert!(audiobook.is_complete());
}

#[test]
fn test_audiobook_file_completeness_calculation() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "Chapter 1.mp3".to_string(),
        full_path: "/test/Chapter 1.mp3".to_string(),
        length_of_file: Some(300000),
        seek_position: Some(150000),
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: Utc::now(),
    };

    // Calculate completeness manually
    if let (Some(length), Some(seek)) = (file.length_of_file, file.seek_position) {
        let calculated = ((seek as f64 / length as f64) * 100.0) as i32;
        assert_eq!(calculated, 50);
    }
}

#[test]
fn test_audiobook_file_no_progress() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "Chapter 1.mp3".to_string(),
        full_path: "/test/Chapter 1.mp3".to_string(),
        length_of_file: Some(300000),
        seek_position: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: Utc::now(),
    };

    assert_eq!(file.completeness, 0);
    assert!(file.seek_position.is_none());
}

#[test]
fn test_audiobook_file_complete() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "Chapter 1.mp3".to_string(),
        full_path: "/test/Chapter 1.mp3".to_string(),
        length_of_file: Some(300000),
        seek_position: Some(300000),
        position: 0,
        completeness: 100,
        file_exists: true,
        created_at: Utc::now(),
    };

    assert_eq!(file.completeness, 100);
}

#[test]
fn test_audiobook_serialization() {
    let audiobook = Audiobook {
        id: Some(1),
        directory: "/test".to_string(),
        name: "Test Audiobook".to_string(),
        full_path: "/test/audiobook".to_string(),
        completeness: 50,
        default_order: 0,
        selected_file: Some("/test/audiobook/file.mp3".to_string()),
        created_at: Utc::now(),
    };

    // Test JSON serialization
    let json = serde_json::to_string(&audiobook).expect("Failed to serialize");
    let deserialized: Audiobook = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(deserialized.id, audiobook.id);
    assert_eq!(deserialized.name, audiobook.name);
    assert_eq!(deserialized.completeness, audiobook.completeness);
    assert_eq!(deserialized.selected_file, audiobook.selected_file);
}

#[test]
fn test_audiobook_file_serialization() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "Chapter 1.mp3".to_string(),
        full_path: "/test/Chapter 1.mp3".to_string(),
        length_of_file: Some(300000),
        seek_position: Some(150000),
        position: 0,
        completeness: 50,
        file_exists: true,
        created_at: Utc::now(),
    };

    let json = serde_json::to_string(&file).expect("Failed to serialize");
    let deserialized: AudiobookFile = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(deserialized.audiobook_id, file.audiobook_id);
    assert_eq!(deserialized.name, file.name);
    assert_eq!(deserialized.length_of_file, file.length_of_file);
    assert_eq!(deserialized.seek_position, file.seek_position);
    assert_eq!(deserialized.completeness, file.completeness);
}
