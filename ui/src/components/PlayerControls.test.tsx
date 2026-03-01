/**
 * Unit tests for PlayerControls component
 */

import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Provider } from 'react-redux';
import { configureStore } from '@reduxjs/toolkit';
import PlayerControls from './PlayerControls';
import appReducer from '../store/slices/appSlice';
import playerReducer from '../store/slices/playerSlice';
import libraryReducer from '../store/slices/librarySlice';

const createTestStore = (playerState?: {
  playback?: 'Playing' | 'Paused' | 'Stopped' | 'Ended';
  speed?: number;
}) =>
  configureStore({
    reducer: {
      app: appReducer,
      player: playerReducer,
      library: libraryReducer,
    },
    preloadedState: playerState ? {
      player: {
        playback: playerState.playback ?? 'Paused',
        currentTime: 0,
        totalDuration: 0,
        volume: 100,
        volumeExpanded: 'Collapsed' as const,
        speed: playerState.speed ?? 1.0,
      },
    } : undefined,
  });

describe('PlayerControls', () => {
  it('renders play button when paused', () => {
    const store = createTestStore({ playback: 'Paused' });
    render(
      <Provider store={store}>
        <PlayerControls />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Play' })).toBeInTheDocument();
  });

  it('renders pause button when playing', () => {
    const store = createTestStore({ playback: 'Playing' });
    render(
      <Provider store={store}>
        <PlayerControls />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Pause' })).toBeInTheDocument();
  });

  it('renders stop button', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <PlayerControls />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Stop' })).toBeInTheDocument();
  });

  it('renders volume button', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <PlayerControls />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Volume' })).toBeInTheDocument();
  });

  it('renders seek slider', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <PlayerControls />
      </Provider>
    );
    expect(screen.getByRole('slider', { name: 'Seek' })).toBeInTheDocument();
  });

  it('displays current speed', () => {
    const store = createTestStore({ speed: 1.5 });
    render(
      <Provider store={store}>
        <PlayerControls />
      </Provider>
    );
    expect(screen.getByText('1.5x')).toBeInTheDocument();
  });
});
