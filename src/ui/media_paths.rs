use crate::tasks::{cleanup_temp_files, parse_zip_virtual_path, zip_temp_dir};
use std::path::Path;

pub(super) fn path_is_within_directory(file_path: &str, directory_root: &str) -> bool {
    let root = Path::new(directory_root);
    if let Some((zip_path, _entry)) = parse_zip_virtual_path(file_path) {
        return zip_path.starts_with(root);
    }
    Path::new(file_path).starts_with(root)
}

pub(super) fn cleanup_zip_temp_for_path(path: &str) {
    let zip_path = Path::new(path);
    if !crate::tasks::is_zip_archive(zip_path) {
        return;
    }

    match zip_temp_dir(zip_path).and_then(|dir| cleanup_temp_files(&dir).map(|()| dir)) {
        Ok(dir) => {
            tracing::debug!("Cleaned up ZIP temp dir {}", dir.display());
        }
        Err(e) => {
            tracing::warn!("Failed to cleanup ZIP temp dir for {path}: {e}");
        }
    }
}
