# 🚀 START HERE - Quick Release Guide

**Last Updated**: February 12, 2026  
**Status**: ✅ Ready for Release  
**Next Action**: Follow 3-step process below

---

## TL;DR - 3 Steps to Release

### 1️⃣ Trigger Builds (5 minutes)

Go to: https://github.com/otakukingdom/nodoka/releases/new

- Tag: `v0.2.0`
- Title: `Nodoka 0.2.0 - Rust Rewrite`
- Description: Copy from `RELEASE_NOTES_v0.2.0.md`
- Click **"Publish release"**

✅ This automatically builds all installers via GitHub Actions.

---

### 2️⃣ Test Installers (2-3 days)

Download installers from release, test on each platform:

**macOS**:
```bash
# Download Nodoka-0.2.0.dmg
# Verify checksum
shasum -a 256 Nodoka-0.2.0.dmg
# Install and test (see SMOKE_TEST_CHECKLIST.md)
```

**Linux** (Ubuntu/Debian):
```bash
# Download nodoka_0.2.0_amd64.deb
# Verify checksum
sha256sum nodoka_0.2.0_amd64.deb
# Install
sudo dpkg -i nodoka_0.2.0_amd64.deb
# Test (see SMOKE_TEST_CHECKLIST.md)
```

**Windows**:
```powershell
# Download Nodoka-0.2.0.msi
# Verify checksum
certutil -hashfile Nodoka-0.2.0.msi SHA256
# Install and test (see SMOKE_TEST_CHECKLIST.md)
```

⚠️ **CRITICAL**: Must verify actual audio playback on each platform!

---

### 3️⃣ Finalize Release (30 minutes)

If all tests pass:

1. Edit GitHub release
2. Add SHA256 checksums from `SHA256SUMS.txt` to description
3. Add "Tested Platforms" section
4. Save changes
5. Announce release! 🎉

---

## Project Status

### ✅ What's Done

- ✅ Full Rust conversion complete (18/18 tests passing)
- ✅ Strict linting enforced (zero warnings)
- ✅ macOS DMG built (4.2 MB)
- ✅ Linux DEB script ready
- ✅ Windows MSI script ready
- ✅ CI/CD configured for automatic builds
- ✅ Comprehensive documentation written

### ⏳ What's Left

- ⏳ Cross-platform smoke tests (requires human + hardware)
- ⏳ GitHub release publication

---

## 📚 Documentation Guide

Lost? Here's what to read:

| Document | When to Read | Time |
|----------|--------------|------|
| **START_HERE.md** (this file) | Right now! | 5 min |
| **MANUAL_STEPS_REQUIRED.md** | Before triggering release | 15 min |
| **SMOKE_TEST_CHECKLIST.md** | During testing phase | Reference |
| **IMPLEMENTATION_COMPLETED.md** | For detailed status | 30 min |
| **PIPELINE_EXECUTION_SUMMARY.md** | For technical details | 30 min |
| **RELEASE_PREPARATION.md** | For release workflow | 15 min |

---

## ⚠️ Important Checks

Before you start:

- [ ] Do you have access to GitHub repository?
- [ ] Do you have macOS, Linux, and Windows test systems?
- [ ] Do you have VLC 3.x installed on each platform?
- [ ] Do you have sample audiobook files (MP3, M4A, M4B, OGG, FLAC)?
- [ ] Have you read `MANUAL_STEPS_REQUIRED.md`?

If you answered "No" to any: **Read the detailed docs first!**

---

## 🆘 Quick Troubleshooting

### CI/CD Build Fails

**Problem**: GitHub Actions workflow shows red X

**Solution**:
1. Go to Actions tab
2. Click on failed workflow
3. Read error logs
4. Common fixes:
   - VLC not found → check VLC install step in workflow
   - WiX not found → check Chocolatey install step
   - File path error → verify paths in packaging scripts

### Smoke Test Fails

**Problem**: Installer works but app crashes or no audio

**Solution**:
1. Document exact error message
2. Check VLC is installed (`vlc --version`)
3. Check app logs in:
   - macOS: `~/Library/Application Support/Nodoka/`
   - Linux: `~/.local/share/nodoka/`
   - Windows: `%APPDATA%\Nodoka\`
4. If critical bug: **abort release, fix code, retry**

### No Audio Output

**Problem**: UI shows playing but no sound

**Solution**:
1. Test file in VLC directly (if works → app issue, if not → file issue)
2. Check system volume not muted
3. Check VLC plugins: `vlc --list` should show audio output modules
4. Linux: verify PulseAudio/PipeWire is running

---

## 🎯 Success Criteria

Release is ready when:

- ✅ All 3 installers downloaded from GitHub release
- ✅ All 3 checksums verified against SHA256SUMS.txt
- ✅ All 21 smoke tests passed (7 scenarios × 3 platforms)
- ✅ Actual audio output verified on all platforms
- ✅ No critical bugs found

---

## 🚦 Go/No-Go Decision

### ✅ GO for Release if:

- Application launches on all platforms
- Audio playback works with actual sound
- Progress saves and restores correctly
- No crashes during normal use

### 🛑 NO-GO for Release if:

- Application crashes on launch
- No audio output despite UI activity
- Database corruption or data loss
- Installer fails to complete

**When in doubt**: Document the issue and ask for help!

---

## 📞 Need Help?

- **Detailed Instructions**: See `MANUAL_STEPS_REQUIRED.md`
- **Technical Status**: See `IMPLEMENTATION_COMPLETED.md`
- **Test Procedures**: See `SMOKE_TEST_CHECKLIST.md`
- **GitHub Issues**: https://github.com/otakukingdom/nodoka/issues
- **Maintainer**: Mistlight Oriroris

---

## 🎉 After Release

Once release is published:

1. ✅ Test download links work
2. ✅ Verify "Latest release" badge shows v0.2.0
3. ✅ Monitor GitHub Issues for 48 hours
4. ✅ Respond to user feedback
5. ✅ Plan v0.2.1 if needed

**Congratulations on shipping Nodoka v0.2.0!** 🚀

---

**Ready to start?** → Begin with Step 1 above or read `MANUAL_STEPS_REQUIRED.md` for full details.
