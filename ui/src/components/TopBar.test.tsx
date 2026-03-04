/**
 * Unit tests for TopBar component
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Provider } from 'react-redux';
import { configureStore } from '@reduxjs/toolkit';
import TopBar from './TopBar';
import appReducer from '../store/slices/appSlice';
import playerReducer from '../store/slices/playerSlice';
import libraryReducer from '../store/slices/librarySlice';

// Mock Tauri dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn().mockResolvedValue(null),
}));

const createTestStore = (libraryLoading = false) =>
  configureStore({
    reducer: {
      app: appReducer,
      player: playerReducer,
      library: libraryReducer,
    },
    preloadedState: {
      app: {
        theme: 'Light',
        isLoading: false,
        activeModal: 'None',
        focusMode: 'Disabled',
        prefersReducedMotion: false,
        panelRatio: 0.4,
        panelDividerDragging: false,
        notifications: [],
      },
      player: {
        status: 'Stopped',
        currentTimeMs: 0,
        durationMs: 0,
        volume: 100,
        isMuted: false,
        playbackRate: 1,
        volumeExpansion: 'Collapsed',
      },
      library: {
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: libraryLoading,
        loadState: 'Ready',
      },
    },
  });

describe('TopBar', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the app title', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    expect(screen.getByText('Nodoka')).toBeInTheDocument();
    expect(screen.getByText('Audiobook Reader')).toBeInTheDocument();
  });

  it('renders settings button', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Settings' })).toBeInTheDocument();
  });

  it('renders add folder button with correct aria-label', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Add audiobook folder' })).toBeInTheDocument();
  });

  it('add folder button has tooltip showing keyboard shortcut', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    const addFolderButton = screen.getByRole('button', { name: 'Add audiobook folder' });
    // DaisyUI tooltip uses data-tip on wrapper element
    const tooltipWrapper = addFolderButton.closest('.tooltip');
    expect(tooltipWrapper).toHaveAttribute('data-tip', 'Add Folder (Ctrl+D)');
  });

  it('add folder button is disabled while library is loading', () => {
    const store = createTestStore(true);
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    const addFolderButton = screen.getByRole('button', { name: 'Add audiobook folder' });
    expect(addFolderButton).toBeDisabled();
  });

  it('settings button has tooltip showing keyboard shortcut', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    const settingsButton = screen.getByRole('button', { name: 'Settings' });
    // DaisyUI tooltip uses data-tip on wrapper element
    const tooltipWrapper = settingsButton.closest('.tooltip');
    expect(tooltipWrapper).toHaveAttribute('data-tip', 'Settings (Ctrl+,)');
  });
});
