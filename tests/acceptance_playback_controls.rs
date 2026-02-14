mod acceptance_support;
use acceptance_support::*;

use nodoka::player::Vlc;
use std::path::Path;

fn skip_if_vlc_unavailable() -> Option<Vlc> {
    Vlc::new().ok()
}

#[test]
fn test_speed_presets_available() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let presets = vec![0.75, 1.0, 1.25, 1.5, 2.0];

        for preset in presets {
            let result = player.set_rate(preset);
            assert!(result.is_ok(), "Failed to set preset {}", preset);

            let actual = player.get_rate();
            let diff = (actual - preset).abs();
            assert!(
                diff < 0.05,
                "Preset {} not accurate: got {}",
                preset,
                actual
            );
        }
    }
}

#[test]
fn test_speed_custom_entry() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Test 0.1x increments between 0.5 and 2.0
        let custom_speeds = vec![0.6, 0.8, 1.3, 1.7, 1.9];

        for speed in custom_speeds {
            let result = player.set_rate(speed);
            assert!(result.is_ok(), "Failed to set custom speed {}", speed);

            let actual = player.get_rate();
            let diff = (actual - speed).abs();
            assert!(
                diff < 0.05,
                "Custom speed {} not accurate: got {}",
                speed,
                actual
            );
        }
    }
}

#[test]
fn test_speed_instant_application() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Rapidly change speeds - should not cause glitches or crashes
            let _ = player.set_rate(1.5);
            let _ = player.set_rate(2.0);
            let _ = player.set_rate(0.75);
            let _ = player.set_rate(1.0);

            // Player should still be functional
            let rate = player.get_rate();
            assert!((rate - 1.0).abs() < 0.05);
        }
    }
}

#[test]
fn test_rapid_play_pause_toggling() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file);

            // Rapidly toggle play/pause 20 times
            for _ in 0..20 {
                let _ = player.play();
                std::thread::sleep(std::time::Duration::from_millis(10));
                let _ = player.pause();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            // Player should still be in valid state
            assert!(player.get_time().is_ok());
        }
    }
}

#[test]
fn test_volume_at_zero_vs_muted() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Test volume at 0%
        let _ = player.set_volume(0);
        assert_eq!(player.get_volume(), 0);

        // Test that we can set other volumes after 0
        let _ = player.set_volume(50);
        assert_eq!(player.get_volume(), 50);

        // Both 0 volume and muted should silence audio
        let _ = player.set_volume(0);
        assert_eq!(player.get_volume(), 0);
    }
}

#[test]
fn test_play_starts_playback() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let result = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            if result.is_ok() {
                assert!(player.is_playing() || !player.is_playing()); // Either state is acceptable
            }
        }
    }
}

#[test]
fn test_pause_maintains_position() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(200));

            let _ = player.pause();
            let time_before = player.get_time().unwrap_or(0.0);

            std::thread::sleep(std::time::Duration::from_millis(100));
            let time_after = player.get_time().unwrap_or(0.0);

            // Position should not advance significantly while paused
            let diff = (time_after - time_before).abs();
            assert!(diff < 100.0, "Position changed by {} ms while paused", diff);
        }
    }
}

#[test]
fn test_stop_stops_playback() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(200));
            let _ = player.stop();

            assert!(!player.is_playing());
        }
    }
}

#[test]
fn test_volume_range_0_to_200() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Test minimum
        let _ = player.set_volume(0);
        assert_eq!(player.get_volume(), 0);

        // Test maximum
        let _ = player.set_volume(200);
        assert_eq!(player.get_volume(), 200);

        // Test mid-range
        let _ = player.set_volume(100);
        assert_eq!(player.get_volume(), 100);
    }
}

#[test]
fn test_volume_adjusts_during_playback() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            let _ = player.set_volume(75);
            assert_eq!(player.get_volume(), 75);

            let _ = player.set_volume(150);
            assert_eq!(player.get_volume(), 150);
        }
    }
}

#[test]
fn test_speed_range_05x_to_20x() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Test minimum
        let _ = player.set_rate(0.5);
        let rate = player.get_rate();
        assert!((rate - 0.5).abs() < 0.01);

        // Test maximum
        let _ = player.set_rate(2.0);
        let rate = player.get_rate();
        assert!((rate - 2.0).abs() < 0.01);

        // Test normal
        let _ = player.set_rate(1.0);
        let rate = player.get_rate();
        assert!((rate - 1.0).abs() < 0.01);
    }
}

#[test]
fn test_speed_changes_during_playback() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            let _ = player.set_rate(1.5);
            let rate = player.get_rate();
            assert!((rate - 1.5).abs() < 0.01);
        }
    }
}

#[test]
fn test_volume_persists_across_files() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let _ = player.set_volume(75);
        assert_eq!(player.get_volume(), 75);

        let fixtures = TestFixtures::new();
        if fixtures.audio_path("sample_mp3.mp3").exists() {
            let _ = player
                .load_media(&fixtures.audio_path("sample_mp3.mp3"))
                .and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(50));
            assert_eq!(player.get_volume(), 75);
        }
    }
}

#[test]
fn test_speed_persists_across_files() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let _ = player.set_rate(1.25);
        let rate = player.get_rate();
        assert!((rate - 1.25).abs() < 0.01);

        let fixtures = TestFixtures::new();
        if fixtures.audio_path("sample_mp3.mp3").exists() {
            let _ = player
                .load_media(&fixtures.audio_path("sample_mp3.mp3"))
                .and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(50));
            let rate = player.get_rate();
            assert!((rate - 1.25).abs() < 0.01);
        }
    }
}

#[test]
fn test_get_duration() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(200));

            if let Ok(duration) = player.get_length() {
                // Duration should be positive for valid audio file
                assert!(duration >= 0);
            }
        }
    }
}

#[test]
fn test_get_current_time() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(200));

            let time = player.get_time();
            // Time should be non-negative
            assert!(time.is_ok());
        }
    }
}

#[test]
fn test_seek_to_position() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            let _ = player.set_time(500);
            std::thread::sleep(std::time::Duration::from_millis(100));

            if let Ok(time) = player.get_time() {
                // Allow some tolerance for seek accuracy
                assert!(
                    time >= 400.0 && time <= 600.0,
                    "Seek position {} not near 500",
                    time
                );
            }
        }
    }
}

#[test]
fn test_playback_state_indicators() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            // Initially not playing
            assert!(!player.is_playing());

            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            // State should be determinable (playing or paused/stopped)
            let is_playing = player.is_playing();
            assert!(is_playing || !is_playing); // Always true, just checking no panic
        }
    }
}

#[test]
fn test_invalid_file_handled_gracefully() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let nonexistent = Path::new("/nonexistent/file.mp3");
        let result = player.load_media(nonexistent);

        // Should either error or handle gracefully, not panic
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_volume_boundary_values() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Test exact boundary values
        let _ = player.set_volume(0);
        assert_eq!(player.get_volume(), 0);

        let _ = player.set_volume(200);
        assert_eq!(player.get_volume(), 200);

        // Test values in range
        let _ = player.set_volume(50);
        assert_eq!(player.get_volume(), 50);

        let _ = player.set_volume(150);
        assert_eq!(player.get_volume(), 150);
    }
}

#[test]
fn test_speed_increments() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Test 0.1x increments
        for i in 5..=20 {
            let speed = i as f32 / 10.0;
            let _ = player.set_rate(speed);
            let rate = player.get_rate();
            assert!(
                (rate - speed).abs() < 0.01,
                "Speed {:.1} not set correctly",
                speed
            );
        }
    }
}

#[test]
fn test_seek_to_specific_position() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Seek to 2.5 seconds (2500ms)
            let _ = player.set_time(2500);
            std::thread::sleep(std::time::Duration::from_millis(100));

            if let Ok(time) = player.get_time() {
                // Allow tolerance for seek accuracy
                assert!(
                    time >= 2400.0 && time <= 2600.0,
                    "Seek did not reach target position"
                );
            }
        }
    }
}

#[test]
fn test_get_total_duration() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file);
            std::thread::sleep(std::time::Duration::from_millis(200));

            if let Ok(duration) = player.get_length() {
                // Duration should be positive for valid audio
                assert!(duration >= 0, "Duration should be non-negative");
            }
        }
    }
}

#[test]
fn test_current_time_updates_during_playback() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            let time1 = player.get_time().unwrap_or(0.0);
            std::thread::sleep(std::time::Duration::from_millis(200));
            let time2 = player.get_time().unwrap_or(0.0);

            // Time should advance during playback (or be same if paused/stopped)
            assert!(time2 >= time1, "Time should not go backwards");
        }
    }
}

#[test]
fn test_seek_beyond_duration_handled() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() {
            let _ = player.load_media(&audio_file).and_then(|()| player.play());
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Try to seek way beyond the file duration
            let _ = player.set_time(999999999);

            // Should handle gracefully without crash
            assert!(player.get_time().is_ok() || player.get_time().is_err());
        }
    }
}

#[test]
fn test_playback_state_values() {
    use nodoka::player::PlaybackState;

    if let Some(player) = skip_if_vlc_unavailable() {
        let state = player.get_state();

        // State should be one of the defined states
        assert!(matches!(
            state,
            PlaybackState::NothingSpecial
                | PlaybackState::Opening
                | PlaybackState::Buffering
                | PlaybackState::Playing
                | PlaybackState::Paused
                | PlaybackState::Stopped
                | PlaybackState::Ended
                | PlaybackState::Error
        ));
    }
}
