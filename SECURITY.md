# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| 0.1.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in Nodoka, please report it by:

1. **DO NOT** open a public GitHub issue
2. Email security concerns to: (add email when available)
3. Include detailed information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and provide a timeline for a fix.

## Security Best Practices

### Dependencies

Nodoka uses the following security measures for dependencies:

- **Stable versions only**: No alpha/beta/rc dependencies in production
- **Minimal dependency tree**: ~26 production dependencies, all from trusted sources
- **Bundled SQLite**: Using rusqlite with bundled feature to avoid system library vulnerabilities
- **Regular updates**: Dependencies reviewed and updated quarterly

### Code Quality

- **No unsafe code**: `#[deny(unsafe_code)]` enforced at compile time
- **No unwrap/expect**: All errors handled with Result types
- **No panic**: Graceful error handling throughout
- **Strict clippy lints**: Enforced at compile time with `-D warnings`

### Runtime Security

- **Single instance guard**: Prevents multiple instances from corrupting database
- **Database file permissions**: User-only read/write on database files
- **No network access**: Application does not make network requests
- **Local data only**: All data stored locally in user's config directory

### VLC Integration

- **Dynamic linking**: VLC loaded from system installation
- **Plugin restrictions**: VLC plugin loading restricted to trusted paths
- **Version requirements**: VLC 3.x required (tested versions)

## Known Limitations

1. **VLC 4.x compatibility**: Not yet tested, use VLC 3.x
2. **Unsigned binaries**: macOS and Windows installers are not code-signed
   - macOS: May require `xattr -cr` to bypass Gatekeeper
   - Windows: May trigger SmartScreen warnings
3. **No sandboxing**: Application runs with full user permissions

## Audit History

- **2026-02-12**: Initial security review for v0.2.0 release
  - Dependency tree reviewed (26 stable dependencies)
  - No known vulnerabilities in dependencies
  - All security best practices implemented

## Security Checklist (for contributors)

Before submitting changes:

- [ ] No new unsafe code introduced
- [ ] All errors handled with Result types
- [ ] No unwrap/expect/panic in source code
- [ ] Clippy passes with `-D warnings`
- [ ] No new dependencies without justification
- [ ] New dependencies are from trusted sources
- [ ] File operations use proper error handling
- [ ] No hardcoded credentials or secrets
- [ ] No logging of sensitive user data

## Future Improvements

- [ ] Implement code signing for macOS and Windows
- [ ] Add automated dependency vulnerability scanning in CI/CD
- [ ] Implement application sandboxing on supported platforms
- [ ] Add integrity verification for database files
- [ ] Implement automatic backup of database
