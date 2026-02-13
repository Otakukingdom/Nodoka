# Troubleshooting Guide

This guide covers common issues and their solutions when using Nodoka Audiobook Reader.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [VLC-Related Problems](#vlc-related-problems)
3. [Audio Playback Issues](#audio-playback-issues)
4. [Database Errors](#database-errors)
5. [Performance Issues](#performance-issues)
6. [Platform-Specific Issues](#platform-specific-issues)
7. [Getting More Help](#getting-more-help)

## Installation Issues

### Windows: "Windows protected your PC" warning

**Problem**: Windows SmartScreen blocks the installer.

**Solution**:
1. Click "More info"
2. Click "Run anyway"
3. This is a false positive - the installer is safe

**Alternative**: Add an exception in Windows Defender or your antivirus software.

### macOS: "Nodoka is damaged and can't be opened"

**Problem**: Gatekeeper blocks unsigned applications.

**Solution**:
1. Open Terminal
2. Run: `xattr -cr /Applications/Nodoka.app`
3. Try launching Nodoka again

**Alternative**: Right-click Nodoka → Open → Click "Open" in the dialog.

### Linux: Missing desktop icon after installation

**Problem**: DEB package installs but icon doesn't appear in application menu.

**Solution**:
1. Log out and log back in
2. Refresh desktop database: `update-desktop-database ~/.local/share/applications/`
3. Check if `nodoka.desktop` exists in `/usr/share/applications/`

**Manual fix**:
```bash
sudo cp /usr/share/applications/nodoka.desktop ~/.local/share/applications/
gtk-update-icon-cache ~/.icons/hicolor
```

## VLC-Related Problems

### VLC library not found

**Error**: `error while loading shared libraries: libvlc.so.5` or similar

**Windows Solution**:
1. Install VLC from [videolan.org](https://www.videolan.org/vlc/)
2. Install to default location: `C:\Program Files\VideoLAN\VLC`
3. Restart Nodoka

**macOS Solution**:
```bash
brew install --cask vlc
```

**Linux Solution**:
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install vlc libvlc-dev

# Fedora
sudo dnf install vlc

# Arch
sudo pacman -S vlc
```

### Wrong VLC version

**Error**: Nodoka crashes immediately or shows version errors.

**Check VLC version**:
```bash
vlc --version
```

**Required**: VLC 3.0.x or later (VLC 4.x not yet tested)

**Solution**: Uninstall VLC and install version 3.x from official sources.

### VLC installed but Nodoka can't find it

**Windows**:
1. Check VLC is installed: `C:\Program Files\VideoLAN\VLC\vlc.exe` should exist
2. Set environment variable:
   ```cmd
   setx VLC_LIB_PATH "C:\Program Files\VideoLAN\VLC"
   ```
3. Restart Nodoka

**macOS**:
```bash
# If using Homebrew VLC
export VLC_LIB_PATH=/Applications/VLC.app/Contents/MacOS/lib

# Add to ~/.zshrc or ~/.bash_profile for persistence
echo 'export VLC_LIB_PATH=/Applications/VLC.app/Contents/MacOS/lib' >> ~/.zshrc
```

**Linux**:
```bash
# Find VLC library location
find /usr -name "libvlc.so*"

# Set in environment
export VLC_LIB_PATH=/usr/lib/x86_64-linux-gnu
```

## Audio Playback Issues

### No sound during playback

**Problem**: Player shows playing but no audio is heard.

**Checklist**:
1. ✅ System volume is not muted
2. ✅ Nodoka volume slider is not at 0%
3. ✅ File plays correctly in VLC media player
4. ✅ Correct audio output device is selected in system settings

**Linux-specific**:
```bash
# Check if PulseAudio is running
pulseaudio --check
echo $?  # Should output 0

# Restart PulseAudio
pulseaudio --kill
pulseaudio --start

# Check if PipeWire is running (newer systems)
systemctl --user status pipewire
```

**Test in VLC**:
1. Open VLC media player
2. Play the same audio file
3. If it doesn't work in VLC, the file may be corrupted
4. If it works in VLC but not Nodoka, file a bug report

### Audio cuts out or stutters

**Possible Causes**:
- High CPU usage
- Disk I/O bottleneck
- Network drive latency
- Corrupted media file

**Solutions**:
1. Close other applications to free up CPU/RAM
2. Move audiobooks to local drive (not network share)
3. Try playing at 1.0x speed instead of faster speeds
4. Update VLC to latest version
5. Check file integrity by playing in VLC directly

### Seek position is inaccurate

**Problem**: Dragging the seek slider jumps to wrong position.

**Explanation**: Some audio formats (especially VBR MP3) don't support precise seeking. This is a VLC limitation, not a Nodoka bug.

**Workaround**: Convert files to constant bitrate (CBR) format:
```bash
ffmpeg -i input.mp3 -codec:a libmp3lame -b:a 128k output.mp3
```

### Playback speed doesn't work

**Problem**: Changing speed has no effect or causes distortion.

**Check**:
1. Speed range is 0.5x to 2.0x only
2. Clicking outside this range has no effect
3. Some codecs don't support time-stretching well

**Solution**: Stick to 0.75x, 1.0x, 1.25x, 1.5x for best quality.

## Database Errors

### "Database is locked" error

**Cause**: Another instance of Nodoka is running, or a previous instance didn't close properly.

**Solution**:
1. Close all Nodoka windows
2. Check for background processes:
   - **Windows**: Task Manager → End `nodoka.exe`
   - **macOS**: Activity Monitor → Quit `Nodoka`
   - **Linux**: `killall nodoka`
3. Delete lock file:
   - **Windows**: `del "%APPDATA%\Otakukingdom\Nodoka\.nodoka.lock"`
   - **macOS**: `rm ~/Library/Application\ Support/com.Otakukingdom.Nodoka/.nodoka.lock`
   - **Linux**: `rm ~/.local/share/com/Otakukingdom/Nodoka/.nodoka.lock`
4. Restart Nodoka

### "Unable to open database file" error

**Cause**: Permission issues or corrupted database.

**Check Permissions**:
```bash
# macOS
ls -la ~/Library/Application\ Support/com.Otakukingdom.Nodoka/
chmod 600 ~/Library/Application\ Support/com.Otakukingdom.Nodoka/nodoka.db

# Linux
ls -la ~/.local/share/com/Otakukingdom/Nodoka/
chmod 600 ~/.local/share/com/Otakukingdom/Nodoka/nodoka.db
```

**Windows**: Right-click `%APPDATA%\Otakukingdom\Nodoka` → Properties → Security → Grant full control to your user.

**Nuclear Option - Reset Database** (⚠️ Loses all progress):
```bash
# macOS (backup first!)
cp ~/Library/Application\ Support/com.Otakukingdom.Nodoka/nodoka.db \
  ~/Library/Application\ Support/com.Otakukingdom.Nodoka/nodoka.db.backup
rm ~/Library/Application\ Support/com.Otakukingdom.Nodoka/nodoka.db

# Linux (backup first!)
cp ~/.local/share/com/Otakukingdom/Nodoka/nodoka.db \
  ~/.local/share/com/Otakukingdom/Nodoka/nodoka.db.backup
rm ~/.local/share/com/Otakukingdom/Nodoka/nodoka.db

# Restart Nodoka (will create new database)
```

### Database corruption after crash

**Symptoms**: Nodoka won't start, or shows SQL errors.

**Recovery**:
```bash
# macOS
cd ~/Library/Application\ Support/com.Otakukingdom.Nodoka
sqlite3 nodoka.db "PRAGMA integrity_check;"
sqlite3 nodoka.db ".recover" | sqlite3 recovered.db
mv nodoka.db nodoka.db.broken
mv recovered.db nodoka.db

# Linux
cd ~/.local/share/com/Otakukingdom/Nodoka
sqlite3 nodoka.db "PRAGMA integrity_check;"
sqlite3 nodoka.db ".recover" | sqlite3 recovered.db
mv nodoka.db nodoka.db.broken
mv recovered.db nodoka.db
```

**Windows**: Use DB Browser for SQLite to repair the database manually.

## Performance Issues

### Slow initial directory scan

**Expected Behavior**: Large libraries take time:
- 100 books: ~10 seconds
- 500 books: ~30-60 seconds
- 1000+ books: several minutes

**Why It's Slow**:
- SHA-256 checksum calculation for each file
- VLC metadata extraction
- Database writes

**Speed It Up**:
1. Scan directories one at a time instead of all at once
2. Use SSD instead of HDD
3. Close other applications during scan
4. Don't scan network drives

### High memory usage

**Normal Usage**: 80-120 MB during playback

**High Usage** (>300 MB):
1. Very large libraries (10,000+ files) cache metadata
2. Long-running instance may accumulate state
3. Memory leak (please report as bug!)

**Reduce Usage**:
- Restart Nodoka periodically
- Remove unused directories from library
- Close other applications

### UI lag or freezing

**Causes**:
- Database query on large library
- File I/O blocking UI thread
- VLC processing large file

**Solutions**:
1. Wait a few seconds - may be temporary
2. Restart Nodoka if it persists
3. Check CPU usage in Task Manager/Activity Monitor
4. File a bug report if reproducible

## Platform-Specific Issues

### Windows: Antivirus blocks Nodoka

**Problem**: Antivirus flags Nodoka as suspicious.

**Why**: Unsigned executable triggers heuristic detection.

**Solution**:
1. Add exception for `nodoka.exe`
2. Add exception for `%APPDATA%\Otakukingdom\Nodoka` directory
3. Download from official GitHub releases only

### macOS: Application freezes on launch

**Problem**: Nodoka hangs with spinning beach ball.

**Causes**:
1. Gatekeeper verification (first launch only)
2. VLC library loading delay
3. Large database query on startup

**Solution**:
1. Wait 30 seconds - may be one-time verification
2. Check Console.app for crash logs
3. Delete `~/Library/Application Support/com.Otakukingdom.Nodoka/.nodoka.lock` and retry
4. Reset database as last resort

### Linux: Missing system fonts

**Problem**: UI text appears as boxes or default font.

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install fonts-liberation fonts-dejavu

# Fedora
sudo dnf install liberation-fonts dejavu-fonts

# Refresh font cache
fc-cache -fv
```

### Linux: Wayland vs X11 issues

**Problem**: UI elements don't render correctly or input is broken.

**Check Display Server**:
```bash
echo $XDG_SESSION_TYPE
```

**Force X11** (if Wayland causes issues):
```bash
GDK_BACKEND=x11 nodoka
```

## Getting More Help

### Gather Diagnostic Information

Before reporting an issue, collect:

1. **Nodoka version**: Check About dialog or run `nodoka --version`
2. **Operating system**: `uname -a` (Linux/macOS) or `systeminfo` (Windows)
3. **VLC version**: `vlc --version`
4. **Error messages**: Check terminal output or logs
5. **Steps to reproduce**: Exact sequence that triggers the issue

### Log Files

**Enable debug logging**:
```bash
RUST_LOG=debug nodoka
```

**Log locations**:
- **Windows**: `%APPDATA%\Otakukingdom\Nodoka\logs\`
- **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka/logs/`
- **Linux**: `~/.local/share/com/Otakukingdom/Nodoka/logs/`

### Reporting Bugs

1. Search [existing issues](https://github.com/your-username/nodoka/issues)
2. Create a new issue if not found
3. Use the bug report template
4. Include diagnostic information
5. Attach logs if available

### Community Support

- **GitHub Discussions**: Ask questions and share tips
- **Issue Tracker**: Report bugs and request features
- **Contributing**: See [CONTRIBUTING.md](../CONTRIBUTING.md)

## Still Having Problems?

If none of these solutions work:

1. Try a clean install:
   - Uninstall Nodoka
   - Delete data directory:
     - **Windows**: `%APPDATA%\Otakukingdom\Nodoka`
     - **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka`
     - **Linux**: `~/.local/share/com/Otakukingdom/Nodoka`
   - Reinstall from latest release
2. Test on a different machine to isolate the issue
3. File a detailed bug report with all diagnostic info

We're here to help! Open an issue on GitHub with as much detail as possible.
