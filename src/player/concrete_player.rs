use super::events::{PlayerEvent, PlayerState};
use crate::conversions::ms_to_f64;
use crate::error::{Error, Result};
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx, State};

pub struct VlcPlayer {
    instance: Instance,
    player: MediaPlayer,
    event_sender: mpsc::UnboundedSender<PlayerEvent>,
    current_file: Arc<Mutex<Option<String>>>,
    volume: AtomicI32,
    speed: Arc<Mutex<f32>>,
}

impl VlcPlayer {
    /// Creates a new concrete player
    ///
    /// # Errors
    ///
    /// Returns an error if VLC instance or media player cannot be created
    pub fn new() -> Result<Self> {
        let instance = Instance::new()
            .ok_or_else(|| Error::Vlc("Failed to create VLC instance".to_string()))?;
        let player = MediaPlayer::new(&instance)
            .ok_or_else(|| Error::Vlc("Failed to create media player".to_string()))?;

        let (event_sender, _) = mpsc::unbounded_channel();

        Ok(Self {
            instance,
            player,
            event_sender,
            current_file: Arc::new(Mutex::new(None)),
            volume: AtomicI32::new(100),
            speed: Arc::new(Mutex::new(1.0)),
        })
    }

    /// Loads media from a file path
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be loaded
    pub fn load_media(&mut self, path: &Path) -> Result<()> {
        let media = Media::new_path(&self.instance, path)
            .ok_or_else(|| Error::Vlc("Failed to load media".to_string()))?;
        self.player.set_media(&media);

        if let Ok(mut current) = self.current_file.lock() {
            *current = Some(path.display().to_string());
        }

        Ok(())
    }

    /// Starts or resumes playback
    ///
    /// # Errors
    ///
    /// Returns an error if playback cannot be started
    pub fn play(&self) -> Result<()> {
        self.player
            .play()
            .map_err(|()| Error::Vlc("Failed to play".to_string()))?;
        self.send_event(PlayerEvent::StateChanged(PlayerState::Playing));
        Ok(())
    }

    /// Pauses playback
    ///
    /// # Errors
    ///
    /// Returns an error if pause fails
    pub fn pause(&self) -> Result<()> {
        self.player.pause();
        self.send_event(PlayerEvent::StateChanged(PlayerState::Paused));
        Ok(())
    }

    /// Stops playback
    ///
    /// # Errors
    ///
    /// Returns an error if stop fails
    pub fn stop(&self) -> Result<()> {
        self.player.stop();
        self.send_event(PlayerEvent::StateChanged(PlayerState::Stopped));
        Ok(())
    }

    /// Sets the playback volume
    ///
    /// # Errors
    ///
    /// Returns an error if the volume cannot be set
    pub fn set_volume(&mut self, volume: i32) -> Result<()> {
        self.player
            .set_volume(volume)
            .map_err(|()| Error::Vlc("Failed to set volume".to_string()))?;
        self.volume.store(volume, Ordering::SeqCst);
        Ok(())
    }

    /// Sets the playback rate (speed)
    ///
    /// # Errors
    ///
    /// Returns an error if the rate cannot be set
    pub fn set_rate(&mut self, rate: f32) -> Result<()> {
        self.player
            .set_rate(rate)
            .map_err(|()| Error::Vlc("Failed to set rate".to_string()))?;
        if let Ok(mut spd) = self.speed.lock() {
            *spd = rate;
        }
        Ok(())
    }

    /// Seeks to a specific time position in milliseconds
    ///
    /// # Errors
    ///
    /// Returns an error if seeking fails
    pub fn set_time(&self, time_ms: i64) -> Result<()> {
        self.player.set_time(time_ms);
        Ok(())
    }

    /// Gets the current playback time in milliseconds as f64
    ///
    /// # Errors
    ///
    /// Returns an error if the time value exceeds safe f64 precision range
    pub fn get_time(&self) -> Result<f64> {
        let time_ms = self.player.get_time().unwrap_or(0);
        ms_to_f64(time_ms)
    }

    /// Gets the total duration in milliseconds
    ///
    /// # Errors
    ///
    /// Returns an error if the length cannot be retrieved
    pub fn get_length(&self) -> Result<i64> {
        self.player.get_media().map_or_else(
            || Ok(0),
            |media| {
                // Parse media to get duration
                media.parse();

                // Duration is in milliseconds
                let duration = media.duration().unwrap_or(0);
                Ok(duration)
            },
        )
    }

    /// Gets the current player state
    #[must_use]
    pub fn get_state(&self) -> PlayerState {
        convert_vlc_state(self.player.state())
    }

    /// Checks if the player is currently playing
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.player.is_playing()
    }

    /// Gets the current volume
    #[must_use]
    pub fn get_volume(&self) -> i32 {
        self.volume.load(Ordering::SeqCst)
    }

    /// Gets the current playback rate
    #[must_use]
    pub fn get_rate(&self) -> f32 {
        self.speed.lock().map_or(1.0, |s| *s)
    }

    /// Sends a player event
    fn send_event(&self, event: PlayerEvent) {
        if self.event_sender.send(event).is_err() {
            tracing::warn!("Failed to send player event: receiver dropped");
        }
    }
}

const fn convert_vlc_state(state: State) -> PlayerState {
    match state {
        State::NothingSpecial => PlayerState::NothingSpecial,
        State::Opening => PlayerState::Opening,
        State::Buffering => PlayerState::Buffering,
        State::Playing => PlayerState::Playing,
        State::Paused => PlayerState::Paused,
        State::Stopped => PlayerState::Stopped,
        State::Ended => PlayerState::Ended,
        State::Error => PlayerState::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn skip_if_vlc_unavailable() -> Option<VlcPlayer> {
        VlcPlayer::new().ok()
    }

    #[test]
    fn test_player_creation() {
        if let Some(player) = skip_if_vlc_unavailable() {
            assert_eq!(player.get_volume(), 100);
            assert!((player.get_rate() - 1.0).abs() < f32::EPSILON);
            assert!(!player.is_playing());
        }
    }

    #[test]
    fn test_initial_state() {
        if let Some(player) = skip_if_vlc_unavailable() {
            let state = player.get_state();
            assert!(
                matches!(
                    state,
                    PlayerState::NothingSpecial | PlayerState::Stopped | PlayerState::Opening
                ),
                "Initial state should be NothingSpecial, Stopped, or Opening"
            );
        }
    }

    #[test]
    fn test_volume_setting() {
        if let Some(mut player) = skip_if_vlc_unavailable() {
            assert!(player.set_volume(75).is_ok());
            assert_eq!(player.get_volume(), 75);

            assert!(player.set_volume(0).is_ok());
            assert_eq!(player.get_volume(), 0);

            assert!(player.set_volume(200).is_ok());
            assert_eq!(player.get_volume(), 200);
        }
    }

    #[test]
    fn test_rate_setting() {
        if let Some(mut player) = skip_if_vlc_unavailable() {
            assert!(player.set_rate(1.5).is_ok());
            assert!((player.get_rate() - 1.5).abs() < f32::EPSILON);

            assert!(player.set_rate(0.5).is_ok());
            assert!((player.get_rate() - 0.5).abs() < f32::EPSILON);

            assert!(player.set_rate(2.0).is_ok());
            assert!((player.get_rate() - 2.0).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_time_retrieval_without_media() {
        if let Some(player) = skip_if_vlc_unavailable() {
            let time = player.get_time();
            assert!(time.is_ok());
            if let Ok(t) = time {
                assert!(t.abs() < f64::EPSILON);
            }
        }
    }

    #[test]
    fn test_length_without_media() {
        if let Some(player) = skip_if_vlc_unavailable() {
            let length = player.get_length();
            assert!(length.is_ok());
            if let Ok(l) = length {
                assert_eq!(l, 0);
            }
        }
    }

    #[test]
    fn test_set_time_without_media() {
        if let Some(player) = skip_if_vlc_unavailable() {
            assert!(player.set_time(5000).is_ok());
        }
    }

    #[test]
    fn test_load_nonexistent_media() {
        if let Some(mut player) = skip_if_vlc_unavailable() {
            let nonexistent_path = PathBuf::from("/nonexistent/file/path.mp3");
            let result = player.load_media(&nonexistent_path);
            assert!(
                result.is_err(),
                "Loading nonexistent media should fail or return error state"
            );
        }
    }

    #[test]
    fn test_vlc_state_conversion() {
        assert_eq!(
            convert_vlc_state(State::NothingSpecial),
            PlayerState::NothingSpecial
        );
        assert_eq!(convert_vlc_state(State::Opening), PlayerState::Opening);
        assert_eq!(convert_vlc_state(State::Buffering), PlayerState::Buffering);
        assert_eq!(convert_vlc_state(State::Playing), PlayerState::Playing);
        assert_eq!(convert_vlc_state(State::Paused), PlayerState::Paused);
        assert_eq!(convert_vlc_state(State::Stopped), PlayerState::Stopped);
        assert_eq!(convert_vlc_state(State::Ended), PlayerState::Ended);
        assert_eq!(convert_vlc_state(State::Error), PlayerState::Error);
    }

    #[test]
    fn test_stop_without_media() {
        if let Some(player) = skip_if_vlc_unavailable() {
            assert!(player.stop().is_ok());
        }
    }
}
