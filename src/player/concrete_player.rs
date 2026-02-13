use super::events::{PlayerEvent, PlayerState};
use crate::error::{NodokaError, Result};
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx, State};

pub struct ConcretePlayer {
    instance: Instance,
    player: MediaPlayer,
    event_sender: mpsc::UnboundedSender<PlayerEvent>,
    current_file: Arc<Mutex<Option<String>>>,
    volume: AtomicI32,
    speed: Arc<Mutex<f32>>,
}

impl ConcretePlayer {
    /// Creates a new concrete player
    ///
    /// # Errors
    ///
    /// Returns an error if VLC instance or media player cannot be created
    pub fn new() -> Result<Self> {
        let instance = Instance::new()
            .ok_or_else(|| NodokaError::Vlc("Failed to create VLC instance".to_string()))?;
        let player = MediaPlayer::new(&instance)
            .ok_or_else(|| NodokaError::Vlc("Failed to create media player".to_string()))?;

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
            .ok_or_else(|| NodokaError::Vlc("Failed to load media".to_string()))?;
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
            .map_err(|()| NodokaError::Vlc("Failed to play".to_string()))?;
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
            .map_err(|()| NodokaError::Vlc("Failed to set volume".to_string()))?;
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
            .map_err(|()| NodokaError::Vlc("Failed to set rate".to_string()))?;
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
    /// # Precision
    ///
    /// VLC internally uses i64 for time values. This function converts to f64
    /// for UI consistency. For practical media durations (< 285 million years),
    /// the conversion is exact within f64's 53-bit mantissa precision.
    #[must_use]
    pub fn get_time(&self) -> f64 {
        let time_ms = self.player.get_time().unwrap_or(0);
        time_ms as f64
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
