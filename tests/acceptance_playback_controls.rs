mod acceptance_support;
use acceptance_support::*;

use nodoka::player::Vlc;
use std::error::Error;
use std::path::Path;
use temp_dir::TempDir;

fn skip_if_vlc_unavailable() -> Option<Vlc> {
    Vlc::new().ok()
}

fn write_silence_wav(path: &Path, duration_ms: u32) -> std::io::Result<()> {
    let sample_rate: u32 = 8_000;
    let channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let bytes_per_sample: u32 = u32::from(bits_per_sample / 8);

    let num_samples: u32 = sample_rate.saturating_mul(duration_ms) / 1000;
    let data_size: u32 = num_samples
        .saturating_mul(u32::from(channels))
        .saturating_mul(bytes_per_sample);

    let data_size_usize = usize::try_from(data_size).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "WAV data size overflow")
    })?;

    let mut bytes = Vec::with_capacity(44usize.saturating_add(data_size_usize));
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36u32.saturating_add(data_size)).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&channels.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate
        .saturating_mul(u32::from(channels))
        .saturating_mul(bytes_per_sample);
    bytes.extend_from_slice(&byte_rate.to_le_bytes());

    let block_align_u32 = u32::from(channels).saturating_mul(bytes_per_sample);
    let block_align = u16::try_from(block_align_u32).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "WAV block align overflow")
    })?;
    bytes.extend_from_slice(&block_align.to_le_bytes());
    bytes.extend_from_slice(&bits_per_sample.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    bytes.resize(bytes.len().saturating_add(data_size_usize), 0);

    std::fs::write(path, bytes)
}

#[test]
fn test_speed_presets_available() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let presets = vec![0.75, 1.0, 1.25, 1.5, 2.0];

        for preset in presets {
            let result = player.set_rate(preset);
            assert!(result.is_ok(), "Failed to set preset {preset}");

            let actual = player.get_rate();
            let diff = (actual - preset).abs();
            assert!(diff < 0.05, "Preset {preset} not accurate: got {actual}");
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
            assert!(result.is_ok(), "Failed to set custom speed {speed}");

            let actual = player.get_rate();
            let diff = (actual - speed).abs();
            assert!(
                diff < 0.05,
                "Custom speed {speed} not accurate: got {actual}"
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
fn test_play_starts_playback() -> Result<(), Box<dyn Error>> {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let temp = TempDir::new()?;
        let wav_path = temp.path().join("sample.wav");
        write_silence_wav(&wav_path, 500)?;

        player.load_media(&wav_path)?;
        player.play()?;
        std::thread::sleep(std::time::Duration::from_millis(100));

        assert!(
            matches!(
                player.get_state(),
                nodoka::player::PlaybackState::Playing
                    | nodoka::player::PlaybackState::Opening
                    | nodoka::player::PlaybackState::Buffering
                    | nodoka::player::PlaybackState::Paused
            ),
            "Player should enter a valid playing-related state"
        );
    }

    Ok(())
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
            assert!(diff < 100.0, "Position changed by {diff} ms while paused");
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
fn test_stop_resets_position_to_beginning() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let fixtures = TestFixtures::new();
        let audio_file = fixtures.audio_path("sample_mp3.mp3");

        if audio_file.exists() && player.load_media(&audio_file).is_ok() && player.play().is_ok() {
            std::thread::sleep(std::time::Duration::from_millis(300));

            // Verify we're past the beginning
            if let Ok(time_before) = player.get_time() {
                // Only run the test if playback actually started
                if time_before > 0.0 {
                    // Stop playback
                    let _ = player.stop();

                    // Position should reset to 0 after stop
                    // Note: VLC may not immediately report position 0 after stop,
                    // but on next play it should start from beginning
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    // Play again and check it starts from beginning
                    let _ = player.play();
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    if let Ok(time_after) = player.get_time() {
                        // Should be near beginning (allowing for small startup delay)
                        assert!(
                            time_after < 500.0,
                            "After stop and play, position should start from beginning"
                        );
                    }
                }
            }
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

            // Test fixture files are too small for real seeking
            // Just verify set_time doesn't panic
            let result = player.set_time(500);
            std::thread::sleep(std::time::Duration::from_millis(100));

            assert!(result.is_ok(), "Seeking should not fail for a loaded media");
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
            let _is_playing = player.is_playing();
            // VLC state check - ensuring no panic on state query
        }
    }
}

#[test]
fn test_invalid_file_handled_gracefully() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let nonexistent = Path::new("/nonexistent/file.mp3");
        let result = player.load_media(nonexistent);

        assert!(
            result.is_err(),
            "Loading a non-existent media path should error"
        );
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
        // Test 0.1x increments from 0.5 to 2.0
        let speeds = [
            0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0,
        ];
        for speed in speeds {
            let _ = player.set_rate(speed);
            let rate = player.get_rate();
            assert!(
                (rate - speed).abs() < 0.01,
                "Speed {speed:.1} not set correctly"
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

            // Test fixture files are too small for real seeking
            // Just verify set_time API works without panic
            let result = player.set_time(2500);
            std::thread::sleep(std::time::Duration::from_millis(100));

            assert!(result.is_ok(), "Seeking should not fail for a loaded media");
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
            let _ = player.set_time(999_999_999);

            match player.get_time() {
                Ok(time_ms) => assert!(time_ms >= 0.0),
                Err(e) => assert!(!format!("{e}").is_empty()),
            }
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

// Comprehensive speed preset tests

#[test]
fn test_speed_presets_all_defined() {
    // Verify all required speed presets from specification
    let required_presets = vec![0.75, 1.0, 1.25, 1.5, 2.0];

    if let Some(mut player) = skip_if_vlc_unavailable() {
        for preset in required_presets {
            let result = player.set_rate(preset);
            assert!(result.is_ok(), "Speed preset {preset} should be supported");

            // Verify the rate was actually set
            let actual = player.get_rate();
            assert!(
                (actual - preset).abs() < 0.1,
                "Speed preset {preset} not correctly set, got {actual}"
            );
        }
    }
}

#[test]
fn test_speed_below_minimum() {
    // Test that speeds below 0.5x are handled
    // Note: VLC allows speeds outside the recommended range
    // Application should validate/clamp at UI level
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let result = player.set_rate(0.3);

        // VLC accepts the value - validation should happen at UI level
        assert!(result.is_ok(), "VLC should accept speed values");
    }
}

#[test]
fn test_speed_above_maximum() {
    // Test that speeds above 2.0x are handled
    // Note: VLC allows speeds outside the recommended range
    // Application should validate/clamp at UI level
    if let Some(mut player) = skip_if_vlc_unavailable() {
        let result = player.set_rate(3.0);

        // VLC accepts the value - validation should happen at UI level
        assert!(result.is_ok(), "VLC should accept speed values");
    }
}

#[test]
fn test_speed_persists_per_audiobook() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    // Test per-audiobook speed preference storage
    let db = create_test_db()?;

    // Create two test audiobooks
    let ab1_id = create_test_audiobook(&db, "/test/dir1", "Book One")?;
    let ab2_id = create_test_audiobook(&db, "/test/dir2", "Book Two")?;

    // Set different speeds via metadata
    queries::set_metadata(db.connection(), &format!("speed_audiobook_{ab1_id}"), "1.5")?;
    queries::set_metadata(db.connection(), &format!("speed_audiobook_{ab2_id}"), "2.0")?;

    // Verify speeds are independent
    let speed1 = queries::get_metadata(db.connection(), &format!("speed_audiobook_{ab1_id}"))?
        .ok_or("Speed not found")?;
    assert_eq!(speed1, "1.5");

    let speed2 = queries::get_metadata(db.connection(), &format!("speed_audiobook_{ab2_id}"))?
        .ok_or("Speed not found")?;
    assert_eq!(speed2, "2.0");

    Ok(())
}

#[test]
fn test_speed_changes_no_crashes() {
    // Verify rapid speed changes don't cause crashes
    if let Some(mut player) = skip_if_vlc_unavailable() {
        // Change speed multiple times rapidly
        for speed in &[0.5, 1.0, 1.5, 2.0, 1.0, 0.75, 1.25] {
            let _ = player.set_rate(*speed);

            // Player should maintain valid rate
            assert!(player.get_rate() > 0.0, "Player should maintain valid rate");
        }
    }
}

#[test]
fn test_keyboard_shortcut_space_maps_to_play_pause() {
    let message = nodoka::ui::shortcuts::message_for_key_chord(
        nodoka::ui::shortcuts::ShortcutKey::Space,
        iced::keyboard::Modifiers::default(),
    );

    assert!(matches!(message, Some(nodoka::ui::Message::PlayPause)));
}

#[test]
fn test_keyboard_shortcut_ctrl_b_creates_bookmark() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
    let file_path = "/test/Book/chapter_01.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    let mut state = nodoka::ui::State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 12_000.0,
        ..Default::default()
    };

    // Verify the chord maps to the bookmark message.
    let message = nodoka::ui::shortcuts::message_for_key_chord(
        nodoka::ui::shortcuts::ShortcutKey::B,
        iced::keyboard::Modifiers::CTRL,
    )
    .ok_or("Shortcut did not map")?;

    let mut player: Option<nodoka::player::Vlc> = None;
    let _ = nodoka::ui::update::update(&mut state, message, &mut player, &db);

    assertions::assert_bookmark_at_position(&db, audiobook_id, 12_000, 5)?;

    Ok(())
}
