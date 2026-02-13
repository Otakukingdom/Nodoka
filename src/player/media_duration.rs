use crate::error::{Error, Result};
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(5);

pub(super) fn parse_duration_with_timeout(media: &vlc::Media, timeout: Duration) -> Result<i64> {
    media.parse_async();
    wait_for_duration(
        || {
            if media.is_parsed() {
                if let Some(duration) = media.duration() {
                    return Ok(Some(duration));
                }

                return Err(Error::MediaParse(
                    "Duration not available after media was parsed".to_string(),
                ));
            }

            Ok(None)
        },
        timeout,
    )
}

fn wait_for_duration(
    mut read_duration: impl FnMut() -> Result<Option<i64>>,
    timeout: Duration,
) -> Result<i64> {
    let started = Instant::now();

    loop {
        if let Some(duration) = read_duration()? {
            return Ok(duration);
        }

        if started.elapsed() >= timeout {
            return Err(Error::MediaParse(format!(
                "Duration not available after waiting {timeout:?}"
            )));
        }

        let remaining = timeout
            .checked_sub(started.elapsed())
            .unwrap_or_else(|| Duration::from_millis(0));
        std::thread::sleep(std::cmp::min(POLL_INTERVAL, remaining));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wait_for_duration_returns_when_available() {
        let mut calls = 0_u32;
        let result = wait_for_duration(
            || {
                calls += 1;
                Ok(if calls < 3 { None } else { Some(123) })
            },
            Duration::from_millis(50),
        );

        match result {
            Ok(duration) => assert_eq!(duration, 123),
            Err(e) => {
                let ok = false;
                assert!(ok, "Expected Ok duration, got: {e:?}");
            }
        }
        assert!(calls >= 3);
    }

    #[test]
    fn test_wait_for_duration_times_out() {
        let result = wait_for_duration(|| Ok(None), Duration::from_millis(20));
        assert!(matches!(result, Err(Error::MediaParse(_))));
    }
}
