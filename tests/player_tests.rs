use nodoka::player::{Scanner, VlcPlayer};

fn skip_if_vlc_unavailable() -> Option<VlcPlayer> {
    VlcPlayer::new().ok()
}

fn skip_scanner_if_vlc_unavailable() -> Option<Scanner> {
    Scanner::new().ok()
}

#[test]
fn test_player_creation_integration() {
    if let Some(player) = skip_if_vlc_unavailable() {
        assert_eq!(player.get_volume(), 100);
        assert!((player.get_rate() - 1.0).abs() < f32::EPSILON);
        assert!(!player.is_playing());
    }
}

#[test]
fn test_scanner_creation_integration() {
    if skip_scanner_if_vlc_unavailable().is_some() {
        // Scanner created successfully
    }
}

#[test]
fn test_player_volume_persistence() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        assert!(player.set_volume(50).is_ok());
        assert_eq!(player.get_volume(), 50);

        assert!(player.set_volume(150).is_ok());
        assert_eq!(player.get_volume(), 150);
    }
}

#[test]
fn test_player_rate_persistence() {
    if let Some(mut player) = skip_if_vlc_unavailable() {
        assert!(player.set_rate(1.5).is_ok());
        assert!((player.get_rate() - 1.5).abs() < f32::EPSILON);

        assert!(player.set_rate(0.75).is_ok());
        assert!((player.get_rate() - 0.75).abs() < f32::EPSILON);
    }
}

#[test]
fn test_player_state_transitions() {
    if let Some(player) = skip_if_vlc_unavailable() {
        assert!(player.stop().is_ok());
    }
}

#[test]
fn test_player_time_operations() {
    if let Some(player) = skip_if_vlc_unavailable() {
        let time = player.get_time();
        assert!(time.is_ok());

        assert!(player.set_time(1000).is_ok());
    }
}

#[test]
fn test_player_length_without_media() {
    if let Some(player) = skip_if_vlc_unavailable() {
        let length = player.get_length();
        assert!(length.is_ok());
        if let Ok(l) = length {
            assert_eq!(l, 0);
        }
    }
}

#[test]
fn test_multiple_players() {
    if VlcPlayer::new().is_ok() {
        let player1 = VlcPlayer::new();
        let player2 = VlcPlayer::new();

        assert!(player1.is_ok());
        assert!(player2.is_ok());
    }
}
