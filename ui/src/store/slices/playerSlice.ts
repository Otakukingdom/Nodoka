/**
 * Redux slice for audio player state
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type { PlaybackStatus, VolumeExpansion } from '../types';

interface PlayerState {
  playback: PlaybackStatus;
  currentTime: number;
  totalDuration: number;
  volume: number;
  volumeExpanded: VolumeExpansion;
  speed: number;
}

const initialState: PlayerState = {
  playback: 'Paused',
  currentTime: 0,
  totalDuration: 0,
  volume: 100,
  volumeExpanded: 'Collapsed',
  speed: 1.0,
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
    cycleSpeed: (state) => {
      state.speed = state.speed >= 2.0 ? 0.5 : state.speed + 0.25;
    },
    resetPlayer: (state) => {
      state.playback = 'Stopped';
      state.currentTime = 0;
      state.totalDuration = 0;
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
  cycleSpeed,
  resetPlayer,
} = playerSlice.actions;

export default playerSlice.reducer;
