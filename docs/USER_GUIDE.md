# Nodoka User Guide

Welcome to Nodoka Audiobook Reader! This guide will help you get started with managing and playing your audiobook collection.

## Table of Contents

1. [Installation](#installation)
2. [First Launch](#first-launch)
3. [Adding Audiobooks](#adding-audiobooks)
4. [Playing Audiobooks](#playing-audiobooks)
5. [Organizing Your Library](#organizing-your-library)
6. [Keyboard Shortcuts](#keyboard-shortcuts)
7. [Settings](#settings)

## Installation

### Prerequisites

**Important**: Nodoka requires VLC media player to be installed on your system.

- **Windows**: Download VLC from [videolan.org](https://www.videolan.org/vlc/)
- **macOS**: Install via Homebrew: `brew install --cask vlc`
- **Linux**: Install via package manager:
  - Ubuntu/Debian: `sudo apt install vlc`
  - Fedora: `sudo dnf install vlc`

### Installing Nodoka

#### Windows
1. Download `nodoka-0.2.0-x64.msi` from the releases page
2. Double-click the MSI file
3. Follow the installation wizard
4. Launch Nodoka from the Start Menu

#### macOS
1. Download `Nodoka-0.2.0.dmg` from the releases page
2. Open the DMG file
3. Drag Nodoka to your Applications folder
4. Launch from Applications or Spotlight

If you see a "damaged app" warning, run:
```bash
xattr -cr /Applications/Nodoka.app
```

#### Linux
1. Download `nodoka_0.2.0_amd64.deb` from the releases page
2. Install with: `sudo dpkg -i nodoka_0.2.0_amd64.deb`
3. Launch from your application menu or run `nodoka` in terminal

## First Launch

When you first launch Nodoka:

1. The application window will open with an empty library
2. Click the **Settings** button in the top right corner
3. The Settings dialog will open, showing the "Directories" tab
4. You're now ready to add your audiobook directories!

## Adding Audiobooks

### Adding a Directory

1. Click **Settings** → **Directories** tab
2. Click **Add Directory**
3. Navigate to your audiobooks folder
4. Click **Select Folder** or **OK**
5. Nodoka will automatically scan the directory

### Scanning Process

During scanning, Nodoka will:
- Recursively search for audio files (MP3, M4A, M4B, OGG, FLAC)
- Calculate checksums for each file
- Extract media metadata (duration, codec)
- Group files into audiobooks by parent directory
- Display progress in the status bar

**Note**: Large directories (1000+ files) may take several minutes to scan.

### Supported File Formats

Nodoka supports all VLC-compatible audio formats:
- **MP3** (.mp3)
- **M4A/M4B** (.m4a, .m4b) - Apple audiobook format
- **OGG Vorbis** (.ogg)
- **FLAC** (.flac)
- **WAV** (.wav)
- **WMA** (.wma)

## Playing Audiobooks

### Starting Playback

1. Select an audiobook from the list on the left
2. The audiobook details will appear in the center panel
3. Click the **Play** button (▶️)
4. Playback will begin automatically

### Playback Controls

The player controls are located at the bottom of the window:

- **Play/Pause**: Start or pause playback
- **Volume Slider**: Adjust volume from 0% to 100%
- **Speed Control**: Change playback speed from 0.5x to 2.0x
  - Click **-** to decrease speed
  - Click **+** to increase speed
  - Display shows current speed (e.g., "1.0x")
- **Seek Slider**: Jump to any position in the current file
  - Drag the slider to seek
  - Current time is shown on the left
  - Total duration is shown on the right

### Multi-File Audiobooks

For audiobooks with multiple chapters or parts:

1. The file list appears on the right side
2. Click any file to start playing from that chapter
3. Nodoka will automatically advance to the next file when the current one finishes
4. Progress is tracked separately for each file

### Resume Functionality

Nodoka automatically saves your position:
- Your playback position is saved every few seconds
- When you close and reopen Nodoka, your progress is restored
- Each audiobook remembers its own position
- File completion percentage is tracked (shown in file list)

## Organizing Your Library

### Recommended Folder Structure

Nodoka works best when audiobooks are organized like this:

```
Audiobooks/
├── The Great Gatsby/
│   ├── Chapter 01.mp3
│   ├── Chapter 02.mp3
│   └── Chapter 03.mp3
├── 1984/
│   └── 1984 - Complete.m4b
└── Harry Potter and the Philosopher's Stone/
    ├── Part 1.mp3
    ├── Part 2.mp3
    └── Part 3.mp3
```

**Key Points**:
- Each audiobook should be in its own folder
- Folder name becomes the audiobook title
- Files are sorted alphabetically within each audiobook
- Single-file audiobooks work perfectly too

### Managing Directories

To remove a directory:
1. Go to **Settings** → **Directories**
2. Select the directory in the list
3. Click **Remove Directory**
4. Confirm the deletion

**Note**: This only removes the directory from Nodoka's database - it doesn't delete your files!

## Keyboard Shortcuts

Currently, Nodoka is primarily mouse-driven. Keyboard shortcuts may be added in future versions.

**Navigation**:
- **Tab**: Cycle through UI elements
- **Enter**: Activate selected button
- **Arrow Keys**: Navigate lists

## Settings

Access settings by clicking the **Settings** button in the top right.

### Directories Tab

- **Add Directory**: Add a new folder to scan
- **Remove Directory**: Remove a folder from the library
- **Rescan**: Refresh the library (detect new/removed files)

The directory list shows:
- Full path to each tracked directory
- Number of audiobooks found in that directory

### Future Settings

Planned settings for future releases:
- Theme customization
- Playback defaults (speed, volume)
- Auto-scan interval
- File format preferences
- Sleep timer

## Tips and Best Practices

### Performance Tips

1. **Organize Before Adding**: Structure your audiobooks properly before adding to Nodoka
2. **Separate Directories**: Don't add your entire drive - only audiobook folders
3. **Avoid Network Drives**: Local directories scan much faster than network shares
4. **Close During Scan**: Large scans are CPU-intensive; close other apps if needed

### Audio Quality

1. **Use High-Quality Files**: Nodoka plays files as-is; quality depends on your source
2. **Test Playback in VLC**: If a file won't play in Nodoka, try it in VLC first
3. **Convert If Needed**: Use a tool like ffmpeg to convert problematic formats

### Backup Your Progress

Your progress is stored in:
- **Windows**: `%APPDATA%\Otakukingdom\Nodoka\nodoka.db`
- **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka/nodoka.db`
- **Linux**: `~/.local/share/com/Otakukingdom/Nodoka/nodoka.db`

Back up this file to preserve your progress if reinstalling!

## Getting Help

If you encounter issues:

1. Check the [Troubleshooting Guide](TROUBLESHOOTING.md)
2. Search existing [GitHub Issues](https://github.com/your-username/nodoka/issues)
3. Open a new issue with:
   - Your operating system and version
   - VLC version (`vlc --version`)
   - Nodoka version (shown in About)
   - Steps to reproduce the problem
   - Any error messages

## Contributing

Nodoka is open source! See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute.

## License

Nodoka is released under the MIT License. See [LICENSE](../LICENSE) for details.
