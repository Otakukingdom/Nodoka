/**
 * Async thunks for player-related Tauri commands
 */

import { createAsyncThunk } from '@reduxjs/toolkit';
import {
  setPlayback,
  setCurrentTime,
  setTotalDuration,
  setVolume,
  setSpeed,
} from '../slices/playerSlice';
import { addNotification } from '../slices/appSlice';
import * as TauriCommands from '../../commands/tauri';

export const playPause = createAsyncThunk(
  'player/playPause',
  async (_, { dispatch }) => {
    try {
      const isPlaying = await TauriCommands.playPause();
      dispatch(setPlayback(isPlaying ? 'Playing' : 'Paused'));
      return isPlaying;
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to toggle playback: ${error}`,
        durationMs: Infinity,
        createdAt: Date.now(),
      }));
      throw error;
    }
  }
);

export const stop = createAsyncThunk(
  'player/stop',
  async (_, { dispatch }) => {
    try {
      await TauriCommands.stop();
      dispatch(setPlayback('Stopped'));
      dispatch(setCurrentTime(0));
    } catch (error) {
      console.error('Stop failed:', error);
      throw error;
    }
  }
);

export const seek = createAsyncThunk(
  'player/seek',
  async (positionMs: number, { dispatch }) => {
    try {
      await TauriCommands.seek(positionMs);
      dispatch(setCurrentTime(positionMs));
    } catch (error) {
      console.error('Seek failed:', error);
      throw error;
    }
  }
);

export const seekForward = createAsyncThunk(
  'player/seekForward',
  async (seconds: number, { dispatch, getState }) => {
    const state = getState() as { player: { currentTime: number } };
    const newPosition = state.player.currentTime + (seconds * 1000);
    await dispatch(seek(newPosition));
  }
);

export const seekBackward = createAsyncThunk(
  'player/seekBackward',
  async (seconds: number, { dispatch, getState }) => {
    const state = getState() as { player: { currentTime: number } };
    const newPosition = Math.max(0, state.player.currentTime - (seconds * 1000));
    await dispatch(seek(newPosition));
  }
);

export const updateVolume = createAsyncThunk(
  'player/updateVolume',
  async (volume: number, { dispatch }) => {
    try {
      await TauriCommands.setVolume(volume);
      dispatch(setVolume(volume));
    } catch (error) {
      console.error('Volume change failed:', error);
      throw error;
    }
  }
);

export const updateSpeed = createAsyncThunk(
  'player/updateSpeed',
  async (speed: number, { dispatch }) => {
    try {
      await TauriCommands.setSpeed(speed);
      dispatch(setSpeed(speed));
    } catch (error) {
      console.error('Speed change failed:', error);
      throw error;
    }
  }
);

export const loadFile = createAsyncThunk(
  'player/loadFile',
  async (filePath: string, { dispatch }) => {
    try {
      await TauriCommands.loadFile(filePath);
      dispatch(setPlayback('Playing'));
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to load file: ${error}`,
        durationMs: Infinity,
        createdAt: Date.now(),
      }));
      throw error;
    }
  }
);

export const updatePlayerTime = createAsyncThunk(
  'player/updateTime',
  async (_, { dispatch }) => {
    try {
      const [time, duration, state] = await Promise.all([
        TauriCommands.getCurrentTime(),
        TauriCommands.getDuration(),
        TauriCommands.getPlaybackState(),
      ]);

      dispatch(setCurrentTime(time));
      dispatch(setTotalDuration(duration));

      if (state === 'Playing') {
        dispatch(setPlayback('Playing'));
      } else if (state !== 'Playing') {
        dispatch(setPlayback('Paused'));
      }
    } catch {
      // Player might not be initialized
    }
  }
);
