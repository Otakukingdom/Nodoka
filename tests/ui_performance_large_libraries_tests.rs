//! Performance tests for UI rendering with large libraries
//!
//! Verifies that UI components remain responsive even with large datasets
//! (hundreds or thousands of audiobooks, files, and bookmarks).
//!
//! These tests measure rendering performance to ensure the UI doesn't degrade
//! with larger libraries. Virtualization (rendering only visible items) is
//! a future enhancement as documented in `BUG_ANALYSIS_REPORT.md`.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use nodoka::models::{Audiobook, AudiobookFile, Bookmark};
use nodoka::ui::{main_window, State};
use std::time::Instant;

/// Helper function to create test audiobook with specified ID and completeness
fn create_test_audiobook(id: i64) -> Audiobook {
    Audiobook {
        id: Some(id),
        directory: format!("/test/dir{id}"),
        name: format!("Test Audiobook {id:04}"),
        full_path: format!("/test/dir{id}/audiobook"),
        completeness: i32::try_from(id % 101).unwrap_or(0), // 0-100% range
        default_order: i32::try_from(id).unwrap_or(0),
        selected_file: None,
        created_at: chrono::Utc::now(),
    }
}

/// Helper function to create test file with specified position
fn create_test_file(audiobook_id: i64, position: i64) -> AudiobookFile {
    AudiobookFile {
        audiobook_id,
        name: format!("chapter_{position:03}.mp3"),
        full_path: format!("/test/audiobook/chapter_{position:03}.mp3"),
        length_of_file: Some(3_600_000), // 1 hour
        seek_position: Some(0),
        checksum: None,
        position: i32::try_from(position).unwrap_or(0),
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    }
}

/// Helper function to create test bookmark with specified ID
fn create_test_bookmark(id: i64, audiobook_id: i64) -> Bookmark {
    Bookmark {
        id: Some(id),
        audiobook_id,
        file_path: "/test/file.mp3".to_string(),
        position_ms: id * 1000,
        label: format!("Bookmark {id:04}"),
        note: Some(format!("Note for bookmark {id}")),
        created_at: chrono::Utc::now(),
    }
}

// ============================================================================
// AUDIOBOOK LIST PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_performance_audiobook_list_10_items() {
    let audiobooks: Vec<Audiobook> = (1..=10).map(create_test_audiobook).collect();

    let state = State {
        audiobooks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 50,
        "Audiobook list with 10 items should render in under 50ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_audiobook_list_100_items() {
    let audiobooks: Vec<Audiobook> = (1..=100).map(create_test_audiobook).collect();

    let state = State {
        audiobooks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 100,
        "Audiobook list with 100 items should render in under 100ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_audiobook_list_500_items() {
    let audiobooks: Vec<Audiobook> = (1..=500).map(create_test_audiobook).collect();

    let state = State {
        audiobooks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 300,
        "Audiobook list with 500 items should render in under 300ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_audiobook_list_1000_items() {
    let audiobooks: Vec<Audiobook> = (1..=1000).map(create_test_audiobook).collect();

    let state = State {
        audiobooks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    // Note: Without virtualization, 1000 items may take longer
    // This is documented in BUG_ANALYSIS_REPORT.md as a future enhancement
    assert!(
        elapsed.as_millis() < 500,
        "Audiobook list with 1000 items should render in under 500ms, took {}ms (virtualization is future enhancement)",
        elapsed.as_millis()
    );
}

// ============================================================================
// FILE LIST PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_performance_file_list_10_files() {
    let files: Vec<AudiobookFile> = (1..=10).map(|i| create_test_file(1, i)).collect();

    let state = State {
        current_files: files,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 30,
        "File list with 10 files should render in under 30ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_file_list_50_files() {
    let files: Vec<AudiobookFile> = (1..=50).map(|i| create_test_file(1, i)).collect();

    let state = State {
        current_files: files,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 50,
        "File list with 50 files should render in under 50ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_file_list_100_files() {
    let files: Vec<AudiobookFile> = (1..=100).map(|i| create_test_file(1, i)).collect();

    let state = State {
        current_files: files,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 100,
        "File list with 100 files should render in under 100ms, took {}ms",
        elapsed.as_millis()
    );
}

// ============================================================================
// BOOKMARK LIST PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_performance_bookmark_list_10_bookmarks() {
    let bookmarks: Vec<Bookmark> = (1..=10).map(|i| create_test_bookmark(i, 1)).collect();

    let state = State {
        bookmarks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 30,
        "Bookmark list with 10 bookmarks should render in under 30ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_bookmark_list_100_bookmarks() {
    let bookmarks: Vec<Bookmark> = (1..=100).map(|i| create_test_bookmark(i, 1)).collect();

    let state = State {
        bookmarks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 100,
        "Bookmark list with 100 bookmarks should render in under 100ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_bookmark_list_500_bookmarks() {
    let bookmarks: Vec<Bookmark> = (1..=500).map(|i| create_test_bookmark(i, 1)).collect();

    let state = State {
        bookmarks,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 250,
        "Bookmark list with 500 bookmarks should render in under 250ms, took {}ms",
        elapsed.as_millis()
    );
}

// ============================================================================
// COMBINED PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_performance_complex_state_with_large_data() {
    // Test rendering with all three lists populated with large datasets
    let audiobooks: Vec<Audiobook> = (1..=100).map(create_test_audiobook).collect();
    let files: Vec<AudiobookFile> = (1..=50).map(|i| create_test_file(1, i)).collect();
    let bookmarks: Vec<Bookmark> = (1..=100).map(|i| create_test_bookmark(i, 1)).collect();

    let state = State {
        audiobooks,
        current_files: files,
        bookmarks,
        selected_audiobook: Some(1),
        selected_file: Some("/test/audiobook/chapter_001.mp3".to_string()),
        is_playing: true,
        current_time: 1800.0,
        total_duration: 3600.0,
        volume: 100,
        speed: 1.0,
        ..Default::default()
    };

    let start = Instant::now();
    let element = main_window::view(&state);
    let elapsed = start.elapsed();
    drop(element);

    assert!(
        elapsed.as_millis() < 200,
        "Complex state (100 audiobooks + 50 files + 100 bookmarks) should render in under 200ms, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_performance_switching_between_large_audiobooks() {
    // Test the performance of switching selection between large audiobooks
    let audiobooks: Vec<Audiobook> = (1..=100).map(create_test_audiobook).collect();

    // Test switching from audiobook 1 to audiobook 50
    let state_1 = State {
        audiobooks: audiobooks.clone(),
        selected_audiobook: Some(1),
        ..Default::default()
    };

    let state_2 = State {
        audiobooks,
        selected_audiobook: Some(50),
        ..Default::default()
    };

    let start = Instant::now();
    let element_1 = main_window::view(&state_1);
    drop(element_1);
    let element_2 = main_window::view(&state_2);
    drop(element_2);
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 200,
        "Switching between audiobooks in list of 100 should render both in under 200ms, took {}ms",
        elapsed.as_millis()
    );
}

// ============================================================================
// MEMORY USAGE VERIFICATION TESTS
// ============================================================================

#[test]
fn test_performance_memory_efficiency_large_dataset() {
    // Verify that creating large state doesn't cause excessive memory allocation
    // This test doesn't assert on memory usage directly, but ensures the code
    // doesn't panic or run out of memory with large datasets

    let audiobooks: Vec<Audiobook> = (1..=1000).map(create_test_audiobook).collect();
    let files: Vec<AudiobookFile> = (1..=100).map(|i| create_test_file(1, i)).collect();
    let bookmarks: Vec<Bookmark> = (1..=500).map(|i| create_test_bookmark(i, 1)).collect();

    let state = State {
        audiobooks,
        current_files: files,
        bookmarks,
        ..Default::default()
    };

    // Render multiple times to simulate user interaction
    for _ in 0..10 {
        let element = main_window::view(&state);
        drop(element);
    }

    // If we reach here without panic, memory handling is acceptable
}

// ============================================================================
// RENDERING CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_performance_render_time_consistency() {
    // Verify that rendering time is consistent across multiple renders
    let audiobooks: Vec<Audiobook> = (1..=100).map(create_test_audiobook).collect();

    let state = State {
        audiobooks,
        ..Default::default()
    };

    let mut timings = Vec::new();

    // Perform 5 renders and measure each
    for _ in 0..5 {
        let start = Instant::now();
        let element = main_window::view(&state);
        let elapsed = start.elapsed();
        drop(element);
        timings.push(elapsed.as_millis());
    }

    // Verify all timings are within reasonable range (no extreme outliers)
    let max_timing = timings.iter().max().unwrap();
    let min_timing = timings.iter().min().unwrap();

    assert!(
        max_timing - min_timing < 50,
        "Rendering time should be consistent. Max: {}ms, Min: {}ms, Difference: {}ms",
        max_timing,
        min_timing,
        max_timing - min_timing
    );
}

// ============================================================================
// PERFORMANCE BASELINE DOCUMENTATION
// ============================================================================

/// This test documents expected performance baselines for future reference.
/// It doesn't assert on specific timings but logs them for comparison.
#[test]
fn test_performance_baseline_documentation() {
    let test_cases = vec![
        (10, "10 audiobooks"),
        (50, "50 audiobooks"),
        (100, "100 audiobooks"),
        (250, "250 audiobooks"),
        (500, "500 audiobooks"),
    ];

    println!("\n=== Performance Baseline Results ===");

    for (count, description) in test_cases {
        let audiobooks: Vec<Audiobook> = (1..=count).map(create_test_audiobook).collect();
        let state = State {
            audiobooks,
            ..Default::default()
        };

        let start = Instant::now();
        let element = main_window::view(&state);
        let elapsed = start.elapsed();
        drop(element);

        println!(
            "{}: {}ms ({}µs)",
            description,
            elapsed.as_millis(),
            elapsed.as_micros()
        );
    }

    println!("====================================\n");
}
