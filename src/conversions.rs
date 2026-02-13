//! Safe numeric conversion utilities for VLC/iced interoperability.
//!
//! VLC uses `i64` for time values in milliseconds, while iced UI components
//! (sliders, etc.) use `f64`. This module provides safe, explicit conversion
//! functions that satisfy strict linting while maintaining correctness.
//!
//! # Numeric Cast Safety
//!
//! These conversions are necessary for VLC/iced interoperability.
//! They avoid lossy `as` casts by using fallible conversions and explicit
//! validation.

use crate::error::{Error, Result};

const MAX_SAFE_F64_INT: f64 = 9_007_199_254_740_992.0; // 2^53

fn u64_to_f64_exact(value: u64) -> f64 {
    if value == 0 {
        return 0.0;
    }

    let leading = value.leading_zeros();
    let k = u64::from(63_u32.saturating_sub(leading));
    let exponent = (k + 1023) << 52;

    let mantissa = if k <= 52 {
        let leading_bit = 1_u64 << k;
        (value - leading_bit) << (52 - k)
    } else {
        0
    };

    f64::from_bits(exponent | mantissa)
}

fn f64_to_u64_exact(value: f64) -> Option<u64> {
    if value == 0.0 {
        return Some(0);
    }

    let bits = value.to_bits();
    let sign = bits >> 63;
    if sign != 0 {
        return None;
    }

    let exponent_raw_u64 = (bits >> 52) & 0x7ff;
    let exponent_raw = i32::try_from(exponent_raw_u64).ok()?;
    if exponent_raw == 0 || exponent_raw == 0x7ff {
        return None;
    }

    let exponent = exponent_raw - 1023;
    if exponent < 0 {
        return None;
    }

    let mantissa = bits & ((1_u64 << 52) - 1);
    let significand = (1_u64 << 52) | mantissa;

    if exponent <= 52 {
        let shift = u32::try_from(52 - exponent).ok()?;
        let mask = (1_u64 << shift).saturating_sub(1);
        if significand & mask != 0 {
            return None;
        }
        Some(significand >> shift)
    } else {
        let shift = u32::try_from(exponent - 52).ok()?;
        significand.checked_shl(shift)
    }
}

/// Converts VLC time (i64 milliseconds) to UI time (f64 milliseconds).
///
/// # Safety Guarantees
///
/// - **Input range**: `-2^53` to `2^53` milliseconds (validated)
/// - **Output range**: `-9.007e15` to `9.007e15` milliseconds
/// - **Precision**: f64 can represent integers up to 2^53 exactly (no precision loss)
/// - **Practical limit**: Audiobooks typically < 100 hours (360M ms) << 2^53
///
/// The i64 to f64 cast is safe because we validate the input is within the range
/// where f64 maintains exact integer precision (±2^53). This is a mathematical
/// guarantee, not an approximation.
///
/// # Errors
///
/// Returns [`Error::InvalidDuration`] if the time value exceeds the safe
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
pub fn ms_to_f64(ms: i64) -> Result<f64> {
    // f64 maintains exact integer precision up to 2^53 (9,007,199,254,740,992)
    // This is approximately 285 million years in milliseconds, far beyond any
    // practical audiobook duration (typically < 100 hours = 360,000,000 ms)
    const MAX_SAFE_INT_U64: u64 = 1_u64 << 53;

    let abs = ms.unsigned_abs();
    if abs > MAX_SAFE_INT_U64 {
        return Err(Error::InvalidDuration);
    }

    let magnitude = u64_to_f64_exact(abs);
    if ms.is_negative() {
        Ok(-magnitude)
    } else {
        Ok(magnitude)
    }
}

/// Converts UI time (f64 milliseconds) to VLC time (i64 milliseconds).
///
/// # Safety Guarantees
///
/// - **Input validation**: Rejects negative values (invalid time)
/// - **Range validation**: Checks against `i64::MAX` before casting
/// - **Rounding**: Eliminates fractional parts to prevent truncation
/// - **Output range**: 0 to `i64::MAX` milliseconds (valid VLC time)
///
/// # Errors
///
/// Returns [`Error::InvalidPosition`] if the value is negative or
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
        return Err(Error::InvalidPosition);
    }

    if !value.is_finite() {
        return Err(Error::InvalidPosition);
    }

    let rounded = value.round();
    if !(0.0..=MAX_SAFE_F64_INT).contains(&rounded) {
        return Err(Error::InvalidPosition);
    }

    let Some(ms) = f64_to_u64_exact(rounded) else {
        return Err(Error::InvalidPosition);
    };

    i64::try_from(ms).map_err(|_| Error::InvalidPosition)
}

/// Converts a percentage (f64) to an integer percentage (i32).
///
/// # Safety Guarantees
///
/// - **Clamping**: Input is clamped to [0.0, 100.0] range
/// - **Rounding**: Fractional parts eliminated before casting
/// - **Output range**: Always 0 to 100 (well within i32 range)
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
    if !percentage.is_finite() {
        return 0;
    }

    let clamped = percentage.clamp(0.0, 100.0);

    let rounded = clamped.round();
    let Some(value) = f64_to_u64_exact(rounded) else {
        return 0;
    };

    i32::try_from(value).unwrap_or(0)
}

/// Calculates completion percentage from seek position and total length.
///
/// # Errors
///
/// Returns [`Error::InvalidDuration`] if length is zero or negative.
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
        return Err(Error::InvalidDuration);
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
