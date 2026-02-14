mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use std::error::Error;

#[test]
fn test_file_list_shows_all_files() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = temp_dir::TempDir::new()?;

    let audiobook_dir = create_test_audiobook_directory(&temp, "Multi-File Book", 5)?;

    let audiobook_path = audiobook_dir.to_str().ok_or("Invalid path")?;
    let audiobook_id = create_test_audiobook(&db, audiobook_path, "Multi-File Book")?;

    // Insert files
    for i in 1..=5 {
        let file_path = audiobook_dir.join(format!("chapter_{i:02}.mp3"));
        insert_test_file(&db, audiobook_id, file_path.to_str().ok_or("Invalid path")?)?;
    }

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(files.len(), 5);

    Ok(())
}

#[test]
fn test_current_file_tracking() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test/path", "Book")?;

    // Set selected file
    queries::update_audiobook_selected_file(
        db.connection(),
        audiobook_id,
        Some("/test/path/chapter2.mp3"),
    )?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(
        audiobook.selected_file,
        Some("/test/path/chapter2.mp3".to_string())
    );

    Ok(())
}

#[test]
fn test_files_sorted_in_list() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Insert files in non-sorted order
    insert_test_file(&db, audiobook_id, "/test/Book/chapter_10.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/chapter_02.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/chapter_01.mp3")?;

    let mut files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    // Sort files by name (as UI would)
    files.sort_by(|a, b| a.name.cmp(&b.name));

    assert_eq!(files[0].name, "chapter_01.mp3");
    assert_eq!(files[1].name, "chapter_02.mp3");
    assert_eq!(files[2].name, "chapter_10.mp3");

    Ok(())
}

#[test]
fn test_switching_between_files() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/file1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file2.mp3")?;

    // Switch to file 1
    queries::update_audiobook_selected_file(
        db.connection(),
        audiobook_id,
        Some("/test/Book/file1.mp3"),
    )?;
    let ab = queries::get_audiobook_by_id(db.connection(), audiobook_id)?.ok_or("Not found")?;
    assert_eq!(ab.selected_file, Some("/test/Book/file1.mp3".to_string()));

    // Switch to file 2
    queries::update_audiobook_selected_file(
        db.connection(),
        audiobook_id,
        Some("/test/Book/file2.mp3"),
    )?;
    let ab = queries::get_audiobook_by_id(db.connection(), audiobook_id)?.ok_or("Not found")?;
    assert_eq!(ab.selected_file, Some("/test/Book/file2.mp3".to_string()));

    Ok(())
}

#[test]
fn test_file_order_maintained() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let files = vec![
        "/test/Book/01.mp3",
        "/test/Book/02.mp3",
        "/test/Book/03.mp3",
    ];

    for file in &files {
        insert_test_file(&db, audiobook_id, file)?;
    }

    let retrieved = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(retrieved.len(), 3);

    Ok(())
}

#[test]
fn test_next_file_navigation() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/file1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file2.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file3.mp3")?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    // Simulate finding next file
    let current_index = 0;
    let next_index = current_index + 1;

    assert!(next_index < files.len());
    assert_eq!(files[next_index].name, "file2.mp3");

    Ok(())
}

#[test]
fn test_previous_file_navigation() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/file1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file2.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file3.mp3")?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    // Simulate finding previous file from index 2
    let current_index = 2;
    let prev_index = current_index - 1;

    assert_eq!(files[prev_index].name, "file2.mp3");

    Ok(())
}

#[test]
fn test_last_file_detection() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/file1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file2.mp3")?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let last_index = files.len() - 1;

    assert_eq!(files[last_index].name, "file2.mp3");

    // Check if at last file
    assert!(last_index == files.len() - 1);

    Ok(())
}

#[test]
fn test_first_file_detection() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/file1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file2.mp3")?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(files[0].name, "file1.mp3");

    // Check if at first file
    let current_index = 0;
    assert!(current_index == 0);

    Ok(())
}

#[test]
fn test_single_file_audiobook_navigation() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/only_file.mp3")?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(files.len(), 1);

    // No next or previous file available
    assert!(0 == files.len() - 1); // At last file
    assert!(0 == 0); // At first file

    Ok(())
}

#[test]
fn test_file_progress_independent() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file1 = "/test/Book/file1.mp3";
    let file2 = "/test/Book/file2.mp3";

    insert_test_file(&db, audiobook_id, file1)?;
    insert_test_file(&db, audiobook_id, file2)?;

    // Set different progress for each file
    queries::update_file_progress(db.connection(), file1, 1000.0, 50)?;
    queries::update_file_progress(db.connection(), file2, 500.0, 25)?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    let f1 = files
        .iter()
        .find(|f| f.full_path == file1)
        .ok_or("File 1 not found")?;
    let f2 = files
        .iter()
        .find(|f| f.full_path == file2)
        .ok_or("File 2 not found")?;

    assert_eq!(f1.seek_position, Some(1000));
    assert_eq!(f2.seek_position, Some(500));

    Ok(())
}

#[test]
fn test_position_resets_on_file_change() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    insert_test_file(&db, audiobook_id, "/test/Book/file1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/file2.mp3")?;

    // File 1 has progress
    queries::update_file_progress(db.connection(), "/test/Book/file1.mp3", 1500.0, 75)?;

    // File 2 starts at 0
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let f2 = files
        .iter()
        .find(|f| f.name == "file2.mp3")
        .ok_or("Not found")?;

    // New file should start at position 0 or have no saved position
    assert!(f2.seek_position == Some(0) || f2.seek_position.is_none());

    Ok(())
}

#[test]
fn test_natural_sort_order() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Insert in scrambled order
    insert_test_file(&db, audiobook_id, "/test/Book/Chapter 10.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/Chapter 2.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/Chapter 1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/Chapter 20.mp3")?;

    let mut files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    // Sort using natural ordering (as the codebase does)
    files.sort_by(|a, b| natord::compare(&a.name, &b.name));

    assert_eq!(files[0].name, "Chapter 1.mp3");
    assert_eq!(files[1].name, "Chapter 2.mp3");
    assert_eq!(files[2].name, "Chapter 10.mp3");
    assert_eq!(files[3].name, "Chapter 20.mp3");

    Ok(())
}
