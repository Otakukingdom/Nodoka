use crate::error::{Error, Result};
use crate::ui::update::MediaControl;
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;

pub(super) mod bookmarks;
pub(super) mod directory_ops;
pub(super) mod file_selection;
pub(super) mod focus;
pub(super) mod player_settings;
pub(super) mod sleep_timer;
pub(super) mod smoke;
pub(super) mod time_updates;

#[derive(Default)]
pub(super) struct FailingLoadPlayer;

impl MediaControl for FailingLoadPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        Err(Error::Vlc("load failed".to_string()))
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub(super) struct CountingFailingLoadPlayer {
    pub(super) set_time_calls: Rc<Cell<u32>>,
}

impl MediaControl for CountingFailingLoadPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        Err(Error::Vlc("load failed".to_string()))
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        self.set_time_calls.set(self.set_time_calls.get() + 1);
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub(super) struct RecordingPlayer {
    pub(super) load_calls: Rc<Cell<u32>>,
    pub(super) play_calls: Rc<Cell<u32>>,
}

impl MediaControl for RecordingPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        self.load_calls.set(self.load_calls.get() + 1);
        Ok(())
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        self.play_calls.set(self.play_calls.get() + 1);
        Ok(())
    }
}

pub(super) struct FailingPlayPlayer;

impl MediaControl for FailingPlayPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        Err(Error::Vlc("play failed".to_string()))
    }
}

#[derive(Clone)]
pub(super) struct RecordingSeekPlayer {
    pub(super) last_set_time_ms: Rc<Cell<i64>>,
    pub(super) length_ms: i64,
}

impl MediaControl for RecordingSeekPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn set_time(&self, time_ms: i64) -> Result<()> {
        self.last_set_time_ms.set(time_ms);
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(self.length_ms)
    }

    fn play(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub(super) struct CountingLoadPlayer {
    pub(super) load_calls: Rc<Cell<u32>>,
}

impl MediaControl for CountingLoadPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        self.load_calls.set(self.load_calls.get() + 1);
        Ok(())
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        Ok(())
    }
}
