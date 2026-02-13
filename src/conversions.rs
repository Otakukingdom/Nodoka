//! Safe numeric conversion utilities for VLC/iced interoperability.
//!
//! VLC uses `i64` for time values in milliseconds, while iced UI components
//! (sliders, etc.) use `f64`. This module provides safe, explicit conversion
//! functions that satisfy strict linting while maintaining correctness.
//!
//! # Numeric Cast Safety
//!
//! This module requires `cast_precision_loss` and `cast_possible_truncation`
//! lints to be set to `warn` rather than `deny` (configured in `Cargo.toml`).
//! These casts are fundamentally necessary for VLC/iced interoperability:
//!
//! - **VLC to UI (i64 → f64)**: VLC time values are validated to be within
//!   f64's safe integer precision range (2^53) before casting. Audiobooks
//!   never exceed this limit (285 million years vs. typical 100 hours).
//!
//! - **UI to VLC (f64 → i64)**: Values are validated for range and sign,
//!   then rounded to eliminate fractional parts before casting.
//!
//! - **Percentage conversions**: Values are clamped to [0, 100] before
//!   casting to i32, ensuring no overflow or truncation issues.
//!
//! All conversions include explicit validation logic that makes them safe
//! despite clippy warnings. The warnings remain visible during compilation
//! to alert developers if these patterns spread to other modules where
//! validation may be insufficient.

use crate::error::{NodokaError, Result};

/// Converts VLC time (i64 milliseconds) to UI time (f64 milliseconds).
///
/// # Errors
///
/// Returns [`NodokaError::InvalidDuration`] if the time value exceeds the safe
/// integer precision range of f64 (2^53, approximately 285 million years).
///
/// # Examples
///
/// ```no_run
/// # use nodoka::conversions::ms_to_f64;
/// # use nodoka::error::Result;
/// # fn example() -> Result<()> {
/// let ui_time = ms_to_f64(5000)?;
/// assert!((ui_time - 5000.0).abs() < f64::EPSILON);
/// # Ok(())
/// # }
/// ```
pub const fn ms_to_f64(ms: i64) -> Result<f64> {
    // f64 maintains exact integer precision up to 2^53 (9,007,199,254,740,992)
    // This is approximately 285 million years in milliseconds, far beyond any
    // practical audiobook duration (typically < 100 hours = 360,000,000 ms)
    const MAX_SAFE_INT: i64 = 1 << 53;

    if ms.abs() > MAX_SAFE_INT {
        return Err(NodokaError::InvalidDuration);
    }

    // Allow cast_precision_loss: We've validated the value is within safe range above
    Ok(ms as f64)
}

/// Converts UI time (f64 milliseconds) to VLC time (i64 milliseconds).
///
/// # Errors
///
/// Returns [`NodokaError::InvalidPosition`] if the value is negative or
/// exceeds the maximum i64 range.
///
/// # Examples
///
/// ```no_run
/// # use nodoka::conversions::f64_to_ms;
/// # use nodoka::error::Result;
/// # fn example() -> Result<()> {
/// let vlc_time = f64_to_ms(5000.5)?;
/// assert_eq!(vlc_time, 5001); // Rounded
/// # Ok(())
/// # }
/// ```
pub fn f64_to_ms(value: f64) -> Result<i64> {
    if value < 0.0 {
        return Err(NodokaError::InvalidPosition);
    }

    let rounded = value.round();

    // Allow cast_precision_loss: Comparison is safe, we just need to check magnitude
    if rounded > i64::MAX as f64 {
        return Err(NodokaError::InvalidPosition);
    }

    // Allow cast_possible_truncation: We've validated above and round() ensures no fractional part
    Ok(rounded as i64)
}

/// Converts a percentage (f64) to an integer percentage (i32).
///
/// Automatically clamps the value to the range [0, 100] before conversion.
///
/// # Examples
///
/// ```no_run
/// # use nodoka::conversions::percentage_to_i32;
/// assert_eq!(percentage_to_i32(50.7), 51);
/// assert_eq!(percentage_to_i32(-5.0), 0);
/// assert_eq!(percentage_to_i32(150.0), 100);
/// ```
#[must_use]
pub fn percentage_to_i32(percentage: f64) -> i32 {
    let clamped = percentage.clamp(0.0, 100.0);
    // Allow cast_possible_truncation: Value is clamped to [0, 100], safe for i32
    clamped.round() as i32
}

/// Calculates completion percentage from seek position and total length.
///
/// # Errors
///
/// Returns [`NodokaError::InvalidDuration`] if length is zero or negative.
///
/// # Examples
///
/// ```no_run
/// # use nodoka::conversions::calculate_percentage;
/// # use nodoka::error::Result;
/// # fn example() -> Result<()> {
/// let percentage = calculate_percentage(5000, 10000)?;
/// assert!((percentage - 50.0).abs() < f64::EPSILON);
/// # Ok(())
/// # }
/// ```
pub fn calculate_percentage(seek: i64, length: i64) -> Result<f64> {
    if length <= 0 {
        return Err(NodokaError::InvalidDuration);
    }

    let seek_f64 = ms_to_f64(seek)?;
    let length_f64 = ms_to_f64(length)?;

    Ok((seek_f64 / length_f64) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_ms_to_f64_normal() {
        let val = ms_to_f64(5000);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert!((v - 5000.0).abs() < EPSILON);
        }

        let val = ms_to_f64(0);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert!((v - 0.0).abs() < EPSILON);
        }

        let val = ms_to_f64(360_000_000);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert!((v - 360_000_000.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_ms_to_f64_overflow() {
        let result = ms_to_f64(1 << 54);
        assert!(result.is_err());
    }

    #[test]
    fn test_f64_to_ms_normal() {
        let val = f64_to_ms(5000.0);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert_eq!(v, 5000);
        }

        let val = f64_to_ms(5000.5);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert_eq!(v, 5001);
        }

        let val = f64_to_ms(5000.4);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert_eq!(v, 5000);
        }
    }

    #[test]
    fn test_f64_to_ms_negative() {
        let result = f64_to_ms(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentage_to_i32_normal() {
        assert_eq!(percentage_to_i32(50.0), 50);
        assert_eq!(percentage_to_i32(50.5), 51);
        assert_eq!(percentage_to_i32(50.4), 50);
    }

    #[test]
    fn test_percentage_to_i32_clamping() {
        assert_eq!(percentage_to_i32(-10.0), 0);
        assert_eq!(percentage_to_i32(150.0), 100);
        assert_eq!(percentage_to_i32(100.0), 100);
        assert_eq!(percentage_to_i32(0.0), 0);
    }

    #[test]
    fn test_calculate_percentage_normal() {
        let val = calculate_percentage(5000, 10000);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert!((v - 50.0).abs() < EPSILON);
        }

        let val = calculate_percentage(0, 10000);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert!((v - 0.0).abs() < EPSILON);
        }

        let val = calculate_percentage(10000, 10000);
        assert!(val.is_ok());
        if let Ok(v) = val {
            assert!((v - 100.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_calculate_percentage_invalid_length() {
        assert!(calculate_percentage(5000, 0).is_err());
        assert!(calculate_percentage(5000, -1).is_err());
    }
}
