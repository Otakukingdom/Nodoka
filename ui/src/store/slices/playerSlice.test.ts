/**
 * Unit tests for playerSlice
 */

import { describe, it, expect } from 'vitest';
import playerReducer, {
  setPlayback,
  setCurrentTime,
  setTotalDuration,
  setVolume,
  setSpeed,
  cycleSpeed,
  resetPlayer,
  toggleVolumeExpanded,
} from './playerSlice';

describe('playerSlice', () => {
  const initialState = {
    playback: 'Paused' as const,
    currentTime: 0,
    totalDuration: 0,
    volume: 100,
    volumeExpanded: 'Collapsed' as const,
    speed: 1.0,
  };

  it('should return the initial state', () => {
    expect(playerReducer(undefined, { type: 'unknown' })).toEqual(initialState);
  });

  describe('setPlayback', () => {
    it('should set playback to Playing', () => {
      const state = playerReducer(initialState, setPlayback('Playing'));
      expect(state.playback).toBe('Playing');
    });

    it('should set playback to Stopped', () => {
      const state = playerReducer(initialState, setPlayback('Stopped'));
      expect(state.playback).toBe('Stopped');
    });
  });

  describe('setCurrentTime', () => {
    it('should set current time', () => {
      const state = playerReducer(initialState, setCurrentTime(5000));
      expect(state.currentTime).toBe(5000);
    });
  });

  describe('setTotalDuration', () => {
    it('should set total duration', () => {
      const state = playerReducer(initialState, setTotalDuration(300000));
      expect(state.totalDuration).toBe(300000);
    });
  });

  describe('setVolume', () => {
    it('should set volume', () => {
      const state = playerReducer(initialState, setVolume(75));
      expect(state.volume).toBe(75);
    });

    it('should clamp volume to max 200', () => {
      const state = playerReducer(initialState, setVolume(250));
      expect(state.volume).toBe(200);
    });

    it('should clamp volume to min 0', () => {
      const state = playerReducer(initialState, setVolume(-50));
      expect(state.volume).toBe(0);
    });
  });

  describe('setSpeed', () => {
    it('should set speed', () => {
      const state = playerReducer(initialState, setSpeed(1.5));
      expect(state.speed).toBe(1.5);
    });

    it('should clamp speed to max 2.0', () => {
      const state = playerReducer(initialState, setSpeed(3.0));
      expect(state.speed).toBe(2.0);
    });

    it('should clamp speed to min 0.5', () => {
      const state = playerReducer(initialState, setSpeed(0.25));
      expect(state.speed).toBe(0.5);
    });
  });

  describe('cycleSpeed', () => {
    it('should cycle speed from 1.0 to 1.25', () => {
      const state = playerReducer(initialState, cycleSpeed());
      expect(state.speed).toBe(1.25);
    });

    it('should reset speed to 0.5 when at 2.0', () => {
      const state = playerReducer({ ...initialState, speed: 2.0 }, cycleSpeed());
      expect(state.speed).toBe(0.5);
    });
  });

  describe('resetPlayer', () => {
    it('should reset player state', () => {
      const playingState = {
        playback: 'Playing' as const,
        currentTime: 10000,
        totalDuration: 300000,
        volume: 100,
        volumeExpanded: 'Collapsed' as const,
        speed: 1.0,
      };
      const state = playerReducer(playingState, resetPlayer());
      expect(state.playback).toBe('Stopped');
      expect(state.currentTime).toBe(0);
      expect(state.totalDuration).toBe(0);
    });
  });

  describe('toggleVolumeExpanded', () => {
    it('should toggle volume expanded state', () => {
      const state = playerReducer(initialState, toggleVolumeExpanded());
      expect(state.volumeExpanded).toBe('Expanded');
    });

    it('should toggle volume expanded back to collapsed', () => {
      const expandedState = { ...initialState, volumeExpanded: 'Expanded' as const };
      const state = playerReducer(expandedState, toggleVolumeExpanded());
      expect(state.volumeExpanded).toBe('Collapsed');
    });
  });
});
