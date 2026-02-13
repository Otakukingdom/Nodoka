use super::events::{PlaybackEvent, PlaybackState};
use super::media_duration;
use super::vlc_env;
use crate::conversions::ms_to_f64;
use crate::error::{Error, Result};
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;
use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx, State as VlcState};

pub struct Vlc {
    instance: Instance,
    player: MediaPlayer,
    event_sender: broadcast::Sender<PlaybackEvent>,
    current_file: Arc<Mutex<Option<String>>>,
    volume: AtomicI32,
    speed: Arc<Mutex<f32>>,
}

impl Vlc {
    /// Creates a new concrete player
    ///
    /// # Errors
    ///
    /// Returns an error if VLC instance or media player cannot be created
    pub fn new() -> Result<Self> {
        let instance = vlc_env::create_vlc_instance().ok_or_else(|| {
            let plugin_path_info = std::env::var("VLC_PLUGIN_PATH").map_or_else(
                |_| "VLC_PLUGIN_PATH not set".to_string(),
                |p| format!("VLC_PLUGIN_PATH={p}"),
            );

            tracing::error!(
                "Failed to create VLC instance. Environment: {}. \
                 This usually means VLC is not installed or cannot find its plugins.",
                plugin_path_info
            );

            Error::Vlc(format!(
                "Failed to create VLC instance. {plugin_path_info}\n\
                 Please ensure VLC media player is installed:\n\
                 - macOS: brew install --cask vlc\n\
                 - Linux: sudo apt install vlc libvlc-dev (Ubuntu/Debian)\n\
                 - Windows: Download from https://www.videolan.org/vlc/\n\
                 For more details, see the VLC error documentation in the error module."
            ))
        })?;

        let player = MediaPlayer::new(&instance)
            .ok_or_else(|| Error::Vlc("Failed to create media player".to_string()))?;

        let (event_sender, _) = broadcast::channel(16);

        Ok(Self {
            instance,
            player,
            event_sender,
            current_file: Arc::new(Mutex::new(None)),
            volume: AtomicI32::new(100),
            speed: Arc::new(Mutex::new(1.0)),
        })
    }

    /// Subscribes to playback events emitted by this player.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<PlaybackEvent> {
        self.event_sender.subscribe()
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
        self.send_event(PlaybackEvent::StateChanged(PlaybackState::Playing));
        Ok(())
    }

    /// Pauses playback
    ///
    /// # Errors
    ///
    /// Returns an error if pause fails
    pub fn pause(&self) -> Result<()> {
        self.player.pause();
        self.send_event(PlaybackEvent::StateChanged(PlaybackState::Paused));
        Ok(())
    }

    /// Stops playback
    ///
    /// # Errors
    ///
    /// Returns an error if stop fails
    pub fn stop(&self) -> Result<()> {
        self.player.stop();
        self.send_event(PlaybackEvent::StateChanged(PlaybackState::Stopped));
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
            |media| media_duration::parse_duration_with_timeout(&media, Duration::from_secs(2)),
        )
    }

    /// Gets the current player state
    #[must_use]
    pub fn get_state(&self) -> PlaybackState {
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
    fn send_event(&self, event: PlaybackEvent) {
        let _ = self.event_sender.send(event);
    }
}

const fn convert_vlc_state(state: VlcState) -> PlaybackState {
    match state {
        VlcState::NothingSpecial => PlaybackState::NothingSpecial,
        VlcState::Opening => PlaybackState::Opening,
        VlcState::Buffering => PlaybackState::Buffering,
        VlcState::Playing => PlaybackState::Playing,
        VlcState::Paused => PlaybackState::Paused,
        VlcState::Stopped => PlaybackState::Stopped,
        VlcState::Ended => PlaybackState::Ended,
        VlcState::Error => PlaybackState::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn skip_if_vlc_unavailable() -> Option<Vlc> {
        Vlc::new().ok()
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
                    PlaybackState::NothingSpecial | PlaybackState::Stopped | PlaybackState::Opening
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
            // VLC allows loading nonexistent files (lazy validation)
            // Error occurs during playback, not during load
            let result = player.load_media(&nonexistent_path);
            assert!(
                result.is_ok(),
                "VLC allows loading nonexistent media (validation happens during playback)"
            );
        }
    }

    #[test]
    fn test_vlc_state_conversion() {
        assert_eq!(
            convert_vlc_state(VlcState::NothingSpecial),
            PlaybackState::NothingSpecial
        );
        assert_eq!(convert_vlc_state(VlcState::Opening), PlaybackState::Opening);
        assert_eq!(
            convert_vlc_state(VlcState::Buffering),
            PlaybackState::Buffering
        );
        assert_eq!(convert_vlc_state(VlcState::Playing), PlaybackState::Playing);
        assert_eq!(convert_vlc_state(VlcState::Paused), PlaybackState::Paused);
        assert_eq!(convert_vlc_state(VlcState::Stopped), PlaybackState::Stopped);
        assert_eq!(convert_vlc_state(VlcState::Ended), PlaybackState::Ended);
        assert_eq!(convert_vlc_state(VlcState::Error), PlaybackState::Error);
    }

    #[test]
    fn test_stop_without_media() {
        if let Some(player) = skip_if_vlc_unavailable() {
            assert!(player.stop().is_ok());
        }
    }
}
