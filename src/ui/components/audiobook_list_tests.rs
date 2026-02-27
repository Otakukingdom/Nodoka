/// Tests for audiobook list component.
const TEST_ANIMATION_ELAPSED_MS: u64 = 1000;
const TEST_PREFERS_REDUCED_MOTION: bool = false;
const TEST_SEARCH_QUERY: &str = "";

fn test_sort_order() -> crate::ui::state::AudiobookSortOrder {
    crate::ui::state::AudiobookSortOrder::default()
}

fn test_filter_status() -> crate::ui::state::AudiobookFilterStatus {
    crate::ui::state::AudiobookFilterStatus::default()
}

fn create_test_audiobook(id: i64, name: &str, completeness: i32) -> Audiobook {
    Audiobook {
        id: Some(id),
        directory: "/test".to_string(),
        name: name.to_string(),
        author: None,
        full_path: format!("/test/{name}"),
        completeness,
        default_order: 0,
        selected_file: None,
        total_duration_ms: None,
        remaining_duration_ms: None,
        created_at: chrono::Utc::now(),
        last_listened: None,
    }
}

const fn test_animation() -> ListItemAnimation {
    ListItemAnimation {
        opacity: 1.0,
        offset_y: 0.0,
    }
}

#[test]
fn test_view_renders_empty_list() {
    let audiobooks: Vec<Audiobook> = vec![];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_view_renders_single_audiobook() {
    let audiobooks = vec![create_test_audiobook(1, "Test Book", 50)];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_view_renders_multiple_audiobooks() {
    let audiobooks = vec![
        create_test_audiobook(1, "Book One", 25),
        create_test_audiobook(2, "Book Two", 75),
        create_test_audiobook(3, "Book Three", 100),
    ];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_view_renders_with_selection() {
    let audiobooks = vec![
        create_test_audiobook(1, "Book One", 25),
        create_test_audiobook(2, "Book Two", 75),
    ];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: Some(2),
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_view_renders_with_cover_thumbnails() {
    let audiobooks = vec![create_test_audiobook(1, "Book One", 50)];
    let mut cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    cover_thumbnails.insert(1, PathBuf::from("/test/cover1.jpg"));
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_build_audiobook_item_not_selected() {
    let audiobook = create_test_audiobook(1, "Test Book", 50);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_selected() {
    let audiobook = create_test_audiobook(1, "Test Book", 50);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, true, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_with_zero_completeness() {
    let audiobook = create_test_audiobook(1, "New Book", 0);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_with_full_completeness() {
    let audiobook = create_test_audiobook(1, "Finished Book", 100);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_with_cover_thumbnail() {
    let audiobook = create_test_audiobook(1, "Book With Cover", 50);
    let mut cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    cover_thumbnails.insert(1, PathBuf::from("/test/cover.jpg"));

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_without_cover_thumbnail() {
    let audiobook = create_test_audiobook(1, "Book Without Cover", 50);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_with_long_name() {
    let long_name = "This is a very long audiobook name that should still render correctly without causing any issues";
    let audiobook = create_test_audiobook(1, long_name, 50);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_with_special_characters() {
    let audiobook = create_test_audiobook(1, "Book: Name (2024) [Edition]", 50);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_view_handles_large_list() {
    let audiobooks: Vec<Audiobook> = (1..=100)
        .map(|i| create_test_audiobook(i, &format!("Book {i}"), (i % 101) as i32))
        .collect();
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_audiobook_without_id() {
    let audiobook = Audiobook {
        id: None,
        directory: "/test".to_string(),
        name: "Book Without ID".to_string(),
        author: None,
        full_path: "/test/book".to_string(),
        completeness: 50,
        default_order: 0,
        selected_file: None,
        total_duration_ms: None,
        remaining_duration_ms: None,
        created_at: chrono::Utc::now(),
        last_listened: None,
    };
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element);
}

#[test]
fn test_build_audiobook_item_uses_selection_parameter() {
    let audiobook = create_test_audiobook(1, "Test Book", 50);
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

    // Verify both selected and unselected states render without panic
    let element_unselected =
        build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
    drop(element_unselected);

    let element_selected =
        build_audiobook_item(&audiobook, true, &cover_thumbnails, test_animation());
    drop(element_selected);

    // Note: Cannot verify visual styling due to iced Element opacity,
    // but we ensure both code paths execute without panic
}

#[test]
fn test_view_properly_passes_selection_state_to_items() {
    let audiobooks = vec![
        create_test_audiobook(1, "Book 1", 25),
        create_test_audiobook(2, "Book 2", 75),
        create_test_audiobook(3, "Book 3", 50),
    ];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    // Test with no selection
    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);

    // Test with book 2 selected
    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: Some(2),
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_skeleton_ui_renders_during_scan() {
    let audiobooks: Vec<Audiobook> = vec![];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Scanning {
        directory: Some("/test/path".to_string()),
    };

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

#[test]
fn test_empty_state_renders_when_idle_and_no_audiobooks() {
    let audiobooks: Vec<Audiobook> = vec![];
    let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
    let scan_state = ScanState::Idle;

    let element = view(&ViewParams {
        audiobooks: &audiobooks,
        selected_id: None,
        cover_thumbnails: &cover_thumbnails,
        scan_state: &scan_state,
        animation_elapsed_ms: TEST_ANIMATION_ELAPSED_MS,
        prefers_reduced_motion: TEST_PREFERS_REDUCED_MOTION,
        search_query: TEST_SEARCH_QUERY,
        sort_order: test_sort_order(),
        filter_status: test_filter_status(),
    });
    drop(element);
}

fn create_test_audiobook_full(
    id: i64,
    name: &str,
    author: Option<&str>,
    completeness: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    last_listened: Option<chrono::DateTime<chrono::Utc>>,
) -> Audiobook {
    Audiobook {
        id: Some(id),
        directory: "/test".to_string(),
        name: name.to_string(),
        author: author.map(std::string::ToString::to_string),
        full_path: format!("/test/{name}"),
        completeness,
        default_order: 0,
        selected_file: None,
        total_duration_ms: None,
        remaining_duration_ms: None,
        created_at,
        last_listened,
    }
}

#[test]
fn test_filter_and_sort_filters_by_search_query_title_case_insensitive() {
    let audiobooks = vec![
        create_test_audiobook(1, "The Odyssey", 0),
        create_test_audiobook(2, "Other Book", 0),
    ];

    let filtered = filter_and_sort_audiobooks(
        &audiobooks,
        "odys",
        crate::ui::state::AudiobookSortOrder::Title,
        crate::ui::state::AudiobookFilterStatus::All,
    );

    let ids: Vec<i64> = filtered.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![1]);
}

#[test]
fn test_filter_and_sort_filters_by_search_query_author_case_insensitive() {
    let audiobooks = vec![
        {
            let mut ab = create_test_audiobook(1, "Book One", 0);
            ab.author = Some("Jane Doe".to_string());
            ab
        },
        create_test_audiobook(2, "Book Two", 0),
    ];

    let filtered = filter_and_sort_audiobooks(
        &audiobooks,
        "DOE",
        crate::ui::state::AudiobookSortOrder::Title,
        crate::ui::state::AudiobookFilterStatus::All,
    );

    let ids: Vec<i64> = filtered.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![1]);
}

#[test]
fn test_filter_and_sort_filters_by_status() {
    let audiobooks = vec![
        create_test_audiobook(1, "Not Started", 0),
        create_test_audiobook(2, "In Progress", 50),
        create_test_audiobook(3, "Complete", 100),
    ];

    let not_started = filter_and_sort_audiobooks(
        &audiobooks,
        "",
        crate::ui::state::AudiobookSortOrder::Title,
        crate::ui::state::AudiobookFilterStatus::NotStarted,
    );
    let ids: Vec<i64> = not_started.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![1]);

    let in_progress = filter_and_sort_audiobooks(
        &audiobooks,
        "",
        crate::ui::state::AudiobookSortOrder::Title,
        crate::ui::state::AudiobookFilterStatus::InProgress,
    );
    let ids: Vec<i64> = in_progress.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![2]);

    let complete = filter_and_sort_audiobooks(
        &audiobooks,
        "",
        crate::ui::state::AudiobookSortOrder::Title,
        crate::ui::state::AudiobookFilterStatus::Complete,
    );
    let ids: Vec<i64> = complete.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![3]);
}

#[test]
fn test_filter_and_sort_sorts_by_progress_desc() {
    let audiobooks = vec![
        create_test_audiobook(1, "Book A", 10),
        create_test_audiobook(2, "Book B", 90),
        create_test_audiobook(3, "Book C", 50),
    ];

    let sorted = filter_and_sort_audiobooks(
        &audiobooks,
        "",
        crate::ui::state::AudiobookSortOrder::Progress,
        crate::ui::state::AudiobookFilterStatus::All,
    );

    let ids: Vec<i64> = sorted.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![2, 3, 1]);
}

#[test]
fn test_filter_and_sort_sorts_by_recently_played_desc_with_none_last() {
    use chrono::TimeZone;

    let t0 = chrono::Utc.timestamp_opt(1_700_000_000, 0).unwrap();
    let t1 = chrono::Utc.timestamp_opt(1_700_000_100, 0).unwrap();

    let audiobooks = vec![
        create_test_audiobook_full(1, "Book A", None, 0, t0, Some(t0)),
        create_test_audiobook_full(2, "Book B", None, 0, t0, Some(t1)),
        create_test_audiobook_full(3, "Book C", None, 0, t0, None),
    ];

    let sorted = filter_and_sort_audiobooks(
        &audiobooks,
        "",
        crate::ui::state::AudiobookSortOrder::RecentlyPlayed,
        crate::ui::state::AudiobookFilterStatus::All,
    );

    let ids: Vec<i64> = sorted.iter().filter_map(|ab| ab.id).collect();
    assert_eq!(ids, vec![2, 1, 3]);
}
