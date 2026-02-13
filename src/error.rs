use thiserror::Error;

/// Error types for the Nodoka audiobook reader application.
///
/// All fallible operations in Nodoka return a [`Result`] with this error type.
/// Each variant includes troubleshooting guidance where applicable.
#[derive(Error, Debug)]
pub enum NodokaError {
    /// Database operation failed.
    ///
    /// ## Common Causes
    ///
    /// - Database file is locked by another Nodoka instance
    /// - Insufficient file permissions
    /// - Database corruption after crash
    /// - Disk full or read-only filesystem
    ///
    /// ## Troubleshooting
    ///
    /// **Database Locked**:
    /// 1. Close all Nodoka windows
    /// 2. Check for background processes:
    ///    - **Windows**: Task Manager → End `nodoka.exe`
    ///    - **macOS**: Activity Monitor → Quit `Nodoka`
    ///    - **Linux**: `killall nodoka`
    /// 3. Delete lock file:
    ///    - **Windows**: `%APPDATA%\Otakukingdom\Nodoka\.nodoka.lock`
    ///    - **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka/.nodoka.lock`
    ///    - **Linux**: `~/.local/share/com/Otakukingdom/Nodoka/.nodoka.lock`
    ///
    /// **Permission Issues**:
    /// ```sh
    /// # macOS/Linux
    /// chmod 600 ~/Library/Application\ Support/com.Otakukingdom.Nodoka/nodoka.db
    /// ```
    ///
    /// **Database Corruption**:
    /// ```sh
    /// # Backup first!
    /// cd ~/Library/Application\ Support/com.Otakukingdom.Nodoka
    /// cp nodoka.db nodoka.db.backup
    ///
    /// # Attempt recovery with sqlite3
    /// sqlite3 nodoka.db "PRAGMA integrity_check;"
    /// sqlite3 nodoka.db ".recover" | sqlite3 recovered.db
    /// ```
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// VLC library initialization or operation failed.
    ///
    /// ## Common Causes
    ///
    /// - VLC is not installed on the system
    /// - VLC version is incompatible (requires 3.x or later)
    /// - VLC library path not in system PATH (Windows)
    /// - Missing VLC plugins directory
    ///
    /// ## Troubleshooting
    ///
    /// **Check VLC Installation**:
    /// ```sh
    /// vlc --version  # Should output version 3.x or later
    /// ```
    ///
    /// **Windows**:
    /// 1. Install VLC from [videolan.org](https://www.videolan.org/vlc/)
    /// 2. Install to default location: `C:\Program Files\VideoLAN\VLC`
    /// 3. If needed, set environment variable:
    ///    ```cmd
    ///    setx VLC_LIB_PATH "C:\Program Files\VideoLAN\VLC"
    ///    ```
    ///
    /// **macOS**:
    /// ```sh
    /// brew install --cask vlc
    ///
    /// # If VLC is installed but not found:
    /// export VLC_LIB_PATH=/Applications/VLC.app/Contents/MacOS/lib
    /// ```
    ///
    /// **Linux**:
    /// ```sh
    /// # Ubuntu/Debian
    /// sudo apt-get update
    /// sudo apt-get install vlc libvlc-dev
    ///
    /// # Fedora
    /// sudo dnf install vlc
    ///
    /// # Arch
    /// sudo pacman -S vlc
    ///
    /// # Find library location if needed:
    /// find /usr -name "libvlc.so*"
    /// export VLC_LIB_PATH=/usr/lib/x86_64-linux-gnu
    /// ```
    #[error("VLC error: {0}")]
    Vlc(String),

    /// File system I/O operation failed.
    ///
    /// ## Common Causes
    ///
    /// - Insufficient permissions to read/write files
    /// - File or directory does not exist
    /// - Disk full or read-only filesystem
    /// - Network drive disconnected
    /// - File locked by another process
    ///
    /// ## Troubleshooting
    ///
    /// - Verify file/directory exists and is accessible
    /// - Check file permissions
    /// - Ensure sufficient disk space
    /// - For network drives, verify connection is stable
    /// - Use local drives instead of network shares for better performance
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Media file metadata extraction failed.
    ///
    /// ## Common Causes
    ///
    /// - Corrupted audio file
    /// - Unsupported or malformed file format
    /// - VLC unable to parse media metadata
    ///
    /// ## Troubleshooting
    ///
    /// 1. Test playback in VLC media player directly
    /// 2. If file doesn't play in VLC, it's corrupted or unsupported
    /// 3. Convert to a standard format:
    ///    ```sh
    ///    ffmpeg -i input.audio -codec:a libmp3lame -b:a 128k output.mp3
    ///    ```
    #[error("Media parsing failed: {0}")]
    MediaParse(String),

    /// Referenced file was not found on the filesystem.
    ///
    /// ## Common Causes
    ///
    /// - File was moved or deleted after being scanned
    /// - Network drive disconnected
    /// - External drive unmounted
    ///
    /// ## Troubleshooting
    ///
    /// - Verify file still exists at the expected path
    /// - Re-scan the directory in Settings to update file paths
    /// - Remove and re-add the directory if files were reorganized
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// Application entered an unexpected state.
    ///
    /// This typically indicates a programming error rather than a user issue.
    /// Please report this as a bug if encountered.
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Project data directory could not be determined.
    ///
    /// ## Common Causes
    ///
    /// - HOME environment variable not set
    /// - Operating system returned unexpected path
    ///
    /// ## Troubleshooting
    ///
    /// Verify environment variables are set correctly for your platform.
    #[error("Project directory not found")]
    ProjectDirNotFound,

    /// Mutex lock acquisition failed.
    ///
    /// This typically indicates a threading issue. If this persists,
    /// please restart Nodoka.
    #[error("Lock error occurred")]
    LockError,

    /// Audiobook with specified ID was not found in database.
    #[error("Audiobook not found: {0}")]
    AudiobookNotFound(i64),

    /// Numeric conversion failed.
    ///
    /// This typically indicates data corruption or an internal error.
    /// Please report as a bug if encountered during normal usage.
    #[error("Conversion error")]
    ConversionError,

    /// Duration value exceeds safe f64 precision range.
    ///
    /// Audio duration is unrealistically large (>285 million years).
    /// This likely indicates corrupted metadata.
    #[error("Invalid duration: value exceeds safe range")]
    InvalidDuration,

    /// Playback position is out of valid range.
    ///
    /// Position must be non-negative and within i64 range.
    #[error("Invalid playback position: value out of range")]
    InvalidPosition,
}

/// Convenience type alias for Results using [`NodokaError`].
pub type Result<T> = std::result::Result<T, NodokaError>;
