//! Helper functions for progress calculation and time formatting.

use crate::models::AudiobookFile;

/// Extracts a readable chapter name from a file name
///
/// Removes common audiobook prefixes, file extensions, and cleans up formatting
#[must_use]
pub fn extract_chapter_name(filename: &str) -> &str {
    // Remove file extension
    let without_ext = filename.rsplit_once('.').map_or(filename, |(name, _)| name);

    // Return the cleaned name
    without_ext
}

/// Calculates a progress summary showing chapters and remaining time
///
/// Returns a string like "Chapter 5 of 12 · 3h 45m remaining"
#[must_use]
pub fn calculate_progress_summary(
    files: &[AudiobookFile],
    selected_file: Option<&String>,
    total_duration_ms: Option<i64>,
) -> String {
    if files.is_empty() {
        return String::new();
    }

    let total_files = files.len();

    // Find current file index
    let current_index = selected_file
        .and_then(|path| files.iter().position(|f| &f.full_path == path))
        .map(|i| i + 1); // Convert to 1-based index

    let mut parts = Vec::new();

    // Add chapter information if available
    if let Some(current) = current_index {
        parts.push(format!("Chapter {current} of {total_files}"));
    }

    if let Some(remaining_ms) = compute_remaining_duration_ms(files, total_duration_ms) {
        if remaining_ms > 0 {
            parts.push(format_compact_remaining_hm(remaining_ms));
        }
    }

    parts.join(" · ")
}

fn compute_remaining_duration_ms(
    files: &[AudiobookFile],
    total_duration_ms: Option<i64>,
) -> Option<i64> {
    if files.is_empty() {
        return None;
    }

    let mut sum_remaining: i64 = 0;
    let mut all_have_lengths = true;

    for file in files {
        match file.length_of_file {
            Some(length_ms) if length_ms > 0 => {
                let seek_ms = file.seek_position.unwrap_or(0);
                sum_remaining = sum_remaining.saturating_add((length_ms - seek_ms).max(0));
            }
            _ => {
                all_have_lengths = false;
            }
        }
    }

    if all_have_lengths {
        return Some(sum_remaining);
    }

    let total_ms = total_duration_ms.filter(|v| *v > 0)?;

    let played_ms: i64 = files.iter().map(|f| f.seek_position.unwrap_or(0)).sum();

    Some((total_ms - played_ms).max(0))
}

fn format_compact_remaining_hm(remaining_ms: i64) -> String {
    let remaining_ms = remaining_ms.max(0);

    let minutes_total = (remaining_ms.saturating_add(59_999) / 60_000).max(0);
    let hours = minutes_total / 60;
    let minutes = minutes_total % 60;

    if hours > 0 {
        format!("{hours}h {minutes}m remaining")
    } else {
        format!("{minutes}m remaining")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_chapter_name_removes_extension() {
        assert_eq!(extract_chapter_name("chapter1.mp3"), "chapter1");
        assert_eq!(extract_chapter_name("chapter_two.m4b"), "chapter_two");
        assert_eq!(
            extract_chapter_name("03 The Adventure.flac"),
            "03 The Adventure"
        );
    }

    #[test]
    fn test_extract_chapter_name_handles_no_extension() {
        assert_eq!(extract_chapter_name("chapter1"), "chapter1");
    }

    #[test]
    fn test_calculate_progress_summary_empty_files() {
        let summary = calculate_progress_summary(&[], None, None);
        assert_eq!(summary, "");
    }

    #[test]
    fn test_calculate_progress_summary_includes_remaining_time_across_files() {
        let audiobook_id = 1;

        let mut f1 = AudiobookFile::new(
            audiobook_id,
            "ch1.mp3".to_string(),
            "/dir/book/ch1.mp3".to_string(),
            0,
        );
        f1.length_of_file = Some(60 * 60 * 1000);
        f1.seek_position = Some(60 * 60 * 1000);
        f1.completeness = 100;

        let mut f2 = AudiobookFile::new(
            audiobook_id,
            "ch2.mp3".to_string(),
            "/dir/book/ch2.mp3".to_string(),
            1,
        );
        f2.length_of_file = Some(2 * 60 * 60 * 1000);
        f2.seek_position = Some(15 * 60 * 1000);
        f2.completeness = 12;

        let mut f3 = AudiobookFile::new(
            audiobook_id,
            "ch3.mp3".to_string(),
            "/dir/book/ch3.mp3".to_string(),
            2,
        );
        f3.length_of_file = Some(60 * 60 * 1000);
        f3.seek_position = None;
        f3.completeness = 0;

        let files = vec![f1, f2, f3];
        let total_ms = Some(4 * 60 * 60 * 1000);

        let summary =
            calculate_progress_summary(&files, Some(&"/dir/book/ch2.mp3".to_string()), total_ms);

        assert!(
            summary.contains("Chapter 2 of 3"),
            "summary should include chapter position"
        );
        assert!(
            summary.contains("2h 45m remaining"),
            "summary should include remaining time across files; got: {summary}"
        );
    }

    #[test]
    fn test_format_compact_remaining_hm_hours_only() {
        assert_eq!(format_compact_remaining_hm(3_600_000), "1h 0m remaining");
        assert_eq!(format_compact_remaining_hm(7_200_000), "2h 0m remaining");
    }

    #[test]
    fn test_format_compact_remaining_hm_minutes_only() {
        assert_eq!(format_compact_remaining_hm(60_000), "1m remaining");
        assert_eq!(format_compact_remaining_hm(180_000), "3m remaining");
    }

    #[test]
    fn test_format_compact_remaining_hm_hours_and_minutes() {
        assert_eq!(format_compact_remaining_hm(3_660_000), "1h 1m remaining");
        assert_eq!(format_compact_remaining_hm(7_260_000), "2h 1m remaining");
    }

    #[test]
    fn test_compute_remaining_duration_ms_all_files_have_lengths() {
        let mut f1 = AudiobookFile::new(1, "ch1.mp3".to_string(), "/path/ch1.mp3".to_string(), 0);
        f1.length_of_file = Some(60_000);
        f1.seek_position = Some(30_000);

        let mut f2 = AudiobookFile::new(1, "ch2.mp3".to_string(), "/path/ch2.mp3".to_string(), 1);
        f2.length_of_file = Some(60_000);
        f2.seek_position = Some(0);

        let files = vec![f1, f2];
        let remaining = compute_remaining_duration_ms(&files, None);
        assert_eq!(remaining, Some(90_000)); // 30s from ch1 + 60s from ch2
    }

    #[test]
    fn test_compute_remaining_duration_ms_uses_total_when_missing_lengths() {
        let mut f1 = AudiobookFile::new(1, "ch1.mp3".to_string(), "/path/ch1.mp3".to_string(), 0);
        f1.length_of_file = None;
        f1.seek_position = Some(30_000);

        let mut f2 = AudiobookFile::new(1, "ch2.mp3".to_string(), "/path/ch2.mp3".to_string(), 1);
        f2.length_of_file = None;
        f2.seek_position = Some(0);

        let files = vec![f1, f2];
        let total_ms = Some(180_000);
        let remaining = compute_remaining_duration_ms(&files, total_ms);
        assert_eq!(remaining, Some(150_000)); // 180s total - 30s played
    }
}
