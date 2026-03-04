//! README index generator for screenshot output

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

/// Screenshot manifest entry
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotEntry {
    /// Relative path from screenshots directory
    pub path: String,
    /// Category name
    pub category: String,
    /// Filename without extension
    pub name: String,
    /// Theme variant (light/dark) if applicable
    pub theme: Option<String>,
}

/// Screenshot manifest structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// Generation timestamp
    pub generated_at: String,
    /// Total screenshot count
    pub total_count: usize,
    /// Screenshots by category
    pub categories: std::collections::BTreeMap<String, Vec<ScreenshotEntry>>,
}

/// Generate README.md with screenshot index
pub fn generate(output_dir: &Path) -> Result<()> {
    let manifest = build_manifest(output_dir)?;

    // Write manifest.json
    let manifest_path = output_dir.join("manifest.json");
    let manifest_content = serde_json::to_string_pretty(&manifest)
        .context("Failed to serialize manifest")?;
    std::fs::write(&manifest_path, manifest_content)
        .context("Failed to write manifest.json")?;

    // Generate README.md
    let readme_content = generate_readme_content(&manifest);
    let readme_path = output_dir.join("README.md");
    std::fs::write(&readme_path, readme_content)
        .context("Failed to write README.md")?;

    Ok(())
}

/// Build manifest by scanning screenshot directory
fn build_manifest(output_dir: &Path) -> Result<Manifest> {
    let mut categories: std::collections::BTreeMap<String, Vec<ScreenshotEntry>> =
        std::collections::BTreeMap::new();
    let mut total_count = 0;

    for entry in WalkDir::new(output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Only process PNG files
        if path.extension().is_some_and(|ext| ext == "png") {
            let relative_path = path.strip_prefix(output_dir)
                .context("Failed to compute relative path")?
                .to_string_lossy()
                .to_string();

            // Extract category from path
            let category = path
                .parent()
                .and_then(|p| p.strip_prefix(output_dir).ok())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "root".to_string());

            // Extract filename without extension
            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();

            // Detect theme variant from filename
            let theme = if name.ends_with("-light") {
                Some("light".to_string())
            } else if name.ends_with("-dark") {
                Some("dark".to_string())
            } else {
                None
            };

            let entry = ScreenshotEntry {
                path: relative_path,
                category: category.clone(),
                name,
                theme,
            };

            categories.entry(category).or_default().push(entry);
            total_count += 1;
        }
    }

    Ok(Manifest {
        generated_at: chrono_lite_timestamp(),
        total_count,
        categories,
    })
}

/// Generate simple timestamp string without chrono dependency
fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Simple ISO-like format without timezone
    let days = secs / 86400;
    let year = 1970 + (days / 365);
    let remaining_days = days % 365;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    let hour = (secs % 86400) / 3600;
    let minute = (secs % 3600) / 60;
    let second = secs % 60;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    )
}

/// Generate README.md content from manifest
fn generate_readme_content(manifest: &Manifest) -> String {
    let mut content = String::new();

    content.push_str("# Nodoka UI Screenshots\n\n");
    content.push_str(&format!("Generated: {}\n\n", manifest.generated_at));
    content.push_str(&format!("Total screenshots: {}\n\n", manifest.total_count));
    content.push_str("---\n\n");

    // Table of contents
    content.push_str("## Categories\n\n");
    for (category, entries) in &manifest.categories {
        content.push_str(&format!("- [{}](#{}) ({} screenshots)\n",
            category,
            category.replace('/', "-").to_lowercase(),
            entries.len()
        ));
    }
    content.push_str("\n---\n\n");

    // Screenshots by category
    for (category, entries) in &manifest.categories {
        content.push_str(&format!("## {}\n\n", category));

        // Group by base name (without theme suffix)
        let mut grouped: std::collections::BTreeMap<String, Vec<&ScreenshotEntry>> =
            std::collections::BTreeMap::new();

        for entry in entries {
            let base_name = entry.theme.as_ref()
                .map(|t| entry.name.trim_end_matches(&format!("-{}", t)))
                .unwrap_or(&entry.name)
                .to_string();
            grouped.entry(base_name).or_default().push(entry);
        }

        for (base_name, group) in grouped {
            if group.len() == 2 {
                // Both light and dark variants
                content.push_str(&format!("### {}\n\n", base_name));
                for entry in group {
                    let theme = entry.theme.as_deref().unwrap_or("unknown");
                    content.push_str(&format!(
                        "![{} - {}](./{})\n\n",
                        base_name, theme, entry.path
                    ));
                }
            } else {
                // Single variant
                let entry = &group[0];
                content.push_str(&format!("### {}\n\n", entry.name));
                content.push_str(&format!("![{}](./{})\n\n", entry.name, entry.path));
            }
        }
    }

    content.push_str("---\n\n");
    content.push_str("*Generated by `cargo xtask screenshot`*\n");

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_manifest_empty() {
        let temp = TempDir::new().unwrap();
        let manifest = build_manifest(temp.path()).unwrap();

        assert_eq!(manifest.total_count, 0);
        assert!(manifest.categories.is_empty());
    }

    #[test]
    fn test_build_manifest_with_screenshots() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("app-shell");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("empty-library-light.png"), "").unwrap();
        fs::write(category_dir.join("empty-library-dark.png"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();

        assert_eq!(manifest.total_count, 2);
        assert!(manifest.categories.contains_key("app-shell"));
        assert_eq!(manifest.categories["app-shell"].len(), 2);
    }

    #[test]
    fn test_theme_detection_light() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("test");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("state-light.png"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();
        let entry = &manifest.categories["test"][0];

        assert_eq!(entry.theme, Some("light".to_string()));
        assert_eq!(entry.name, "state-light");
    }

    #[test]
    fn test_theme_detection_dark() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("test");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("state-dark.png"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();
        let entry = &manifest.categories["test"][0];

        assert_eq!(entry.theme, Some("dark".to_string()));
        assert_eq!(entry.name, "state-dark");
    }

    #[test]
    fn test_theme_detection_none() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("test");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("notheme.png"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();
        let entry = &manifest.categories["test"][0];

        assert!(entry.theme.is_none());
        assert_eq!(entry.name, "notheme");
    }

    #[test]
    fn test_generate_creates_files() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("test");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("screenshot.png"), "").unwrap();

        generate(temp.path()).unwrap();

        assert!(temp.path().join("manifest.json").exists());
        assert!(temp.path().join("README.md").exists());
    }

    #[test]
    fn test_generate_readme_content() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("app-shell");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("test-light.png"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();
        let content = generate_readme_content(&manifest);

        assert!(content.contains("# Nodoka UI Screenshots"));
        assert!(content.contains("## Categories"));
        assert!(content.contains("app-shell"));
    }

    #[test]
    fn test_manifest_json_is_valid() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("test");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("screenshot.png"), "").unwrap();

        generate(temp.path()).unwrap();

        let manifest_content = fs::read_to_string(temp.path().join("manifest.json")).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&manifest_content).expect("manifest.json should be valid JSON");

        assert!(parsed.is_object());
        assert!(parsed["generated_at"].is_string());
        assert!(parsed["total_count"].is_number());
    }

    #[test]
    fn test_nested_category_path() {
        let temp = TempDir::new().unwrap();
        let nested_dir = temp.path().join("modals").join("settings");
        fs::create_dir_all(&nested_dir).unwrap();
        fs::write(nested_dir.join("dialog.png"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();

        // Category should be the relative path
        assert!(manifest.categories.contains_key("modals/settings"));
        assert_eq!(manifest.total_count, 1);
    }

    #[test]
    fn test_ignores_non_png_files() {
        let temp = TempDir::new().unwrap();
        let category_dir = temp.path().join("test");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("screenshot.png"), "").unwrap();
        fs::write(category_dir.join("readme.txt"), "").unwrap();
        fs::write(category_dir.join("data.json"), "").unwrap();

        let manifest = build_manifest(temp.path()).unwrap();

        assert_eq!(manifest.total_count, 1);
    }
}
