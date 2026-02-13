# Contributing to Nodoka

Thank you for considering contributing to Nodoka! This document outlines the process and standards for contributing.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/nodoka.git`
3. Create a branch from `main`: `git checkout -b feature/your-feature-name`
4. Make your changes following the code standards below
5. Submit a pull request

**Note**: Version 0.2.0 represents the complete Rust rewrite baseline. All contributions should build upon this codebase.

## Code Standards (STRICT)

This project enforces **exceptionally strict** linting rules:

### Forbidden Patterns
- ❌ `unwrap()` - Use proper error handling with `?` or `match`
- ❌ `expect()` - Same as unwrap, use Result types
- ❌ `panic!()` - Handle errors gracefully
- ❌ `#[allow(...)]` - Do not suppress warnings (except in Cargo.toml for framework limitations)
- ❌ `unsafe` - Use safe Rust patterns
- ❌ Dead code - Remove unused functions and imports

### Required Practices
- ✅ All errors returned as `Result<T, NodokaError>`
- ✅ Doc comments on all public APIs with `/// Errors` and `/// Panics` sections
- ✅ Clippy passes with `-D warnings` flag
- ✅ All tests pass: `cargo test`
- ✅ Code formatted: `cargo fmt`

## Testing Requirements

- Add tests for new functionality
- Maintain 100% test pass rate
- Use temp-dir crate for integration tests requiring file system access
- Run the full test suite before submitting: `cargo test --all`

## Pull Request Process

1. Ensure your code passes all checks:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   ```
2. Update documentation if adding public APIs
3. Add entry to CHANGELOG.md under [Unreleased]
4. Reference any related issues in PR description
5. Wait for CI/CD to pass (GitHub Actions)
6. Request review from maintainers

## Areas for Contribution

- 🐛 Bug fixes
- 📚 Documentation improvements
- ✨ New features (discuss in issue first)
- 🧪 Additional tests
- 🌐 Translations (when i18n support is added)
- ♿ Accessibility improvements

## Development Setup

See README.md for build instructions and dependencies.

## Linting Configuration

The project uses strict linting enforced in Cargo.toml:

```toml
[lints.clippy]
all = { level = "deny", priority = -1 }
unwrap_used = { level = "deny", priority = 0 }
expect_used = { level = "deny", priority = 0 }
panic = { level = "deny", priority = 0 }
```

Strategic allows are permitted only in Cargo.toml for framework compatibility:
- `module_name_repetitions` - For Rust naming conventions with nested modules
- `cast_possible_truncation` - For intentional numeric casts (e.g., f64 to i64 for timestamps)
- `cast_precision_loss` - For VLC API conversions (i64 to f64)

**Zero allows are permitted in source code (src/).**

## Error Handling

All fallible operations must return `Result<T, NodokaError>`:

```rust
// ❌ Bad
fn read_file() -> String {
    std::fs::read_to_string("file.txt").unwrap()
}

// ✅ Good
fn read_file() -> Result<String> {
    Ok(std::fs::read_to_string("file.txt")?)
}
```

## Documentation

Add doc comments to all public items:

```rust
/// Calculates the SHA-256 checksum of a file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or if I/O fails during hashing.
///
/// # Examples
///
/// ```
/// let checksum = calculate_checksum(Path::new("audio.mp3")).await?;
/// assert_eq!(checksum.len(), 64); // SHA-256 is 64 hex characters
/// ```
pub async fn calculate_checksum(path: &Path) -> Result<String, std::io::Error> {
    // implementation
}
```

## Testing

Write tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;

    #[test]
    fn test_my_feature() -> Result<()> {
        let temp = TempDir::new()?;
        // test implementation
        Ok(())
    }
}
```

## Commit Messages

Write clear, concise commit messages:

- Use present tense ("Add feature" not "Added feature")
- Capitalize first letter
- No period at the end
- Reference issues when applicable (#123)

Examples:
- `Add volume control to player interface`
- `Fix database connection leak on error`
- `Refactor audiobook scanning for better performance`
- `Update README with troubleshooting section`

## Questions?

Open an issue with the "question" label or start a discussion.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
