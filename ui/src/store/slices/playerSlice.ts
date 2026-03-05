/**
 * Redux slice for audio player state
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type { PlaybackStatus, VolumeExpansion } from '../types';

export type SleepTimerMode = 'Duration' | 'EndOfChapter';

interface PlayerState {
  playback: PlaybackStatus;
  currentTime: number;
  totalDuration: number;
  volume: number;
  isMuted: boolean;
  volumeExpanded: VolumeExpansion;
  speed: number;

  // Playback preferences
  skipForwardDurationSeconds: number;
  skipBackwardDurationSeconds: number;
  autoSaveIntervalSeconds: number;

  // Sleep timer state
  sleepTimer: SleepTimerMode | null;
  sleepTimerStartMs: number | null;
  sleepTimerDurationMs: number | null;
  sleepTimerRemainingMs: number | null;
  sleepTimerFadeDuration: number;
}

const initialState: PlayerState = {
  playback: 'Paused',
  currentTime: 0,
  totalDuration: 0,
  volume: 100,
  isMuted: false,
  volumeExpanded: 'Collapsed',
  speed: 1.0,

  // Playback preferences
  skipForwardDurationSeconds: 30,
  skipBackwardDurationSeconds: 15,
  autoSaveIntervalSeconds: 5,

  // Sleep timer
  sleepTimer: null,
  sleepTimerStartMs: null,
  sleepTimerDurationMs: null,
  sleepTimerRemainingMs: null,
  sleepTimerFadeDuration: 10,
};

const playerSlice = createSlice({
  name: 'player',
  initialState,
  reducers: {
    setPlayback: (state, action: PayloadAction<PlaybackStatus>) => {
      state.playback = action.payload;
    },
    setCurrentTime: (state, action: PayloadAction<number>) => {
      state.currentTime = action.payload;
    },
    setTotalDuration: (state, action: PayloadAction<number>) => {
      state.totalDuration = action.payload;
    },
    setVolume: (state, action: PayloadAction<number>) => {
      state.volume = Math.max(0, Math.min(200, action.payload));
    },
    setVolumeExpanded: (state, action: PayloadAction<VolumeExpansion>) => {
      state.volumeExpanded = action.payload;
    },
    toggleVolumeExpanded: (state) => {
      state.volumeExpanded = state.volumeExpanded === 'Expanded' ? 'Collapsed' : 'Expanded';
    },
    setSpeed: (state, action: PayloadAction<number>) => {
      state.speed = Math.max(0.5, Math.min(2.0, action.payload));
    },
    setSkipForwardDurationSeconds: (state, action: PayloadAction<number>) => {
      state.skipForwardDurationSeconds = Math.max(1, Math.min(120, action.payload));
    },
    setSkipBackwardDurationSeconds: (state, action: PayloadAction<number>) => {
      state.skipBackwardDurationSeconds = Math.max(1, Math.min(120, action.payload));
    },
    setAutoSaveIntervalSeconds: (state, action: PayloadAction<number>) => {
      state.autoSaveIntervalSeconds = Math.max(1, Math.min(60, action.payload));
    },
    cycleSpeed: (state) => {
      state.speed = state.speed >= 2.0 ? 0.5 : state.speed + 0.25;
    },
    toggleMute: (state) => {
      state.isMuted = !state.isMuted;
    },
    setMuted: (state, action: PayloadAction<boolean>) => {
      state.isMuted = action.payload;
    },
    resetPlayer: (state) => {
      state.playback = 'Stopped';
      state.currentTime = 0;
      state.totalDuration = 0;
    },
    // Sleep timer actions
    setSleepTimer: (state, action: PayloadAction<{ mode: SleepTimerMode; durationMinutes: number }>) => {
      const now = Date.now();
      state.sleepTimer = action.payload.mode;
      state.sleepTimerStartMs = now;
      state.sleepTimerDurationMs = action.payload.durationMinutes * 60 * 1000;
      state.sleepTimerRemainingMs = action.payload.durationMinutes * 60 * 1000;
    },
    clearSleepTimer: (state) => {
      state.sleepTimer = null;
      state.sleepTimerStartMs = null;
      state.sleepTimerDurationMs = null;
      state.sleepTimerRemainingMs = null;
    },
    updateSleepTimerRemaining: (state, action: PayloadAction<number>) => {
      state.sleepTimerRemainingMs = action.payload;
    },
    setFadeDuration: (state, action: PayloadAction<number>) => {
      state.sleepTimerFadeDuration = action.payload;
    },
  },
});

export const {
  setPlayback,
  setCurrentTime,
  setTotalDuration,
  setVolume,
  setVolumeExpanded,
  toggleVolumeExpanded,
  setSpeed,
  setSkipForwardDurationSeconds,
  setSkipBackwardDurationSeconds,
  setAutoSaveIntervalSeconds,
  cycleSpeed,
  toggleMute,
  setMuted,
  resetPlayer,
  setSleepTimer,
  clearSleepTimer,
  updateSleepTimerRemaining,
  setFadeDuration,
} = playerSlice.actions;

export default playerSlice.reducer;
