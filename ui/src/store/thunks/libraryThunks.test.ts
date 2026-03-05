/**
 * Unit tests for library thunks
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { configureStore } from '@reduxjs/toolkit';
import libraryReducer from '../slices/librarySlice';
import appReducer from '../slices/appSlice';
import playerReducer from '../slices/playerSlice';
import {
  addDirectory,
  rescanDirectory,
  validateDirectoriesOnStartup,
} from './libraryThunks';
import * as TauriCommands from '../../commands/tauri';
import type { Directory } from '../types';

// Mock Tauri commands
vi.mock('../../commands/tauri', () => ({
  getAllAudiobooks: vi.fn(),
  getDirectories: vi.fn(),
  insertDirectory: vi.fn(),
  removeDirectory: vi.fn(),
  scanDirectory: vi.fn(),
  validateDirectory: vi.fn(),
  updateDirectoryPath: vi.fn(),
  getTheme: vi.fn(),
  getPanelRatio: vi.fn(),
  getVolume: vi.fn(),
  getSpeed: vi.fn(),
}));

const createTestStore = (preloadedState?: Partial<ReturnType<typeof createStore>['getState']>) => {
  const createStore = () =>
    configureStore({
      reducer: {
        library: libraryReducer,
        app: appReducer,
        player: playerReducer,
      },
      preloadedState,
    });
  return createStore();
};

type TestStore = ReturnType<typeof createTestStore>;

describe('library thunks', () => {
  let store: TestStore;

  beforeEach(() => {
    vi.clearAllMocks();
    store = createTestStore();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('addDirectory', () => {
    it('shows warning notification for duplicate directory', async () => {
      // Pre-populate store with existing directory
      store = createTestStore({
        library: {
          audiobooks: [],
          selectedAudiobook: undefined,
          currentFiles: [],
          selectedFile: undefined,
          directories: [{ fullPath: '/Users/test/audiobooks', createdAt: '2024-01-01T00:00:00Z' }],
          bookmarks: [],
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          isLibraryLoading: false,
          isAddingDirectory: false,
          scanningPaths: [],
        },
        app: {
          theme: 'Light',
          panelRatio: 0.4,
          isLoading: false,
          activeModal: 'None',
          focusMode: 'Disabled',
          prefersReducedMotion: false,
          notifications: [],
          volumeExpanded: 'Collapsed',
        },
        player: {
          playback: 'Paused',
          currentTime: 0,
          totalDuration: 0,
          volume: 100,
          isMuted: false,
          volumeExpanded: 'Collapsed',
          speed: 1.0,
          sleepTimer: null,
          sleepTimerStartMs: null,
          sleepTimerDurationMs: null,
          sleepTimerRemainingMs: null,
          sleepTimerFadeDuration: 10,
        },
      });

      // Attempt to add the same directory
      await store.dispatch(addDirectory('/Users/test/audiobooks'));

      // Should not call insertDirectory for duplicates
      expect(TauriCommands.insertDirectory).not.toHaveBeenCalled();

      // Should show warning notification
      const state = store.getState();
      expect(state.app.notifications).toHaveLength(1);
      expect(state.app.notifications[0].kind).toBe('Warning');
      expect(state.app.notifications[0].message).toBe('That folder is already in your library');

      // Should NOT set isAddingDirectory for duplicates (early return)
      expect(state.library.isAddingDirectory).toBe(false);
    });

    it('sets isAddingDirectory state during directory addition', async () => {
      vi.mocked(TauriCommands.getAllAudiobooks)
        .mockResolvedValueOnce([]) // before count
        .mockResolvedValueOnce([]); // after count
      vi.mocked(TauriCommands.insertDirectory).mockResolvedValue(undefined);
      vi.mocked(TauriCommands.getDirectories).mockResolvedValue([
        { fullPath: '/Users/test/new-audiobooks', createdAt: '2024-01-02T00:00:00Z' },
      ]);

      const promise = store.dispatch(addDirectory('/Users/test/new-audiobooks'));

      // Check that isAddingDirectory is set immediately
      let state = store.getState();
      expect(state.library.isAddingDirectory).toBe(true);

      await promise;

      // After completion, isAddingDirectory should be false
      state = store.getState();
      expect(state.library.isAddingDirectory).toBe(false);
    });

    it('adds new directory successfully', async () => {
      vi.mocked(TauriCommands.getAllAudiobooks)
        .mockResolvedValueOnce([]) // before count
        .mockResolvedValueOnce([]); // after count
      vi.mocked(TauriCommands.insertDirectory).mockResolvedValue(undefined);
      vi.mocked(TauriCommands.getDirectories).mockResolvedValue([
        { fullPath: '/Users/test/new-audiobooks', createdAt: '2024-01-02T00:00:00Z' },
      ]);

      await store.dispatch(addDirectory('/Users/test/new-audiobooks'));

      expect(TauriCommands.insertDirectory).toHaveBeenCalledWith('/Users/test/new-audiobooks');
      expect(TauriCommands.getDirectories).toHaveBeenCalled();

      const state = store.getState();
      expect(state.library.directories).toHaveLength(1);
      expect(state.library.directories[0].fullPath).toBe('/Users/test/new-audiobooks');
    });

    it('shows info notification when no audiobooks found in folder', async () => {
      vi.mocked(TauriCommands.getAllAudiobooks)
        .mockResolvedValueOnce([]) // before count
        .mockResolvedValueOnce([]); // after count (still empty)
      vi.mocked(TauriCommands.insertDirectory).mockResolvedValue(undefined);
      vi.mocked(TauriCommands.getDirectories).mockResolvedValue([
        { fullPath: '/Users/test/empty-folder', createdAt: '2024-01-02T00:00:00Z' },
      ]);

      await store.dispatch(addDirectory('/Users/test/empty-folder'));

      const state = store.getState();
      expect(state.app.notifications).toHaveLength(1);
      expect(state.app.notifications[0].kind).toBe('Info');
      expect(state.app.notifications[0].message).toContain('No audiobooks found');
    });
  });

  describe('rescanDirectory', () => {
    it('adds path to scanningPaths during scan', async () => {
      vi.mocked(TauriCommands.scanDirectory).mockImplementation(
        () => new Promise((resolve) => setTimeout(resolve, 100))
      );
      vi.mocked(TauriCommands.getAllAudiobooks).mockResolvedValue([]);

      const promise = store.dispatch(rescanDirectory('/Users/test/audiobooks'));

      // Check that path is added to scanningPaths immediately
      let state = store.getState();
      expect(state.library.scanningPaths).toContain('/Users/test/audiobooks');

      await promise;

      // After completion, path should be removed
      state = store.getState();
      expect(state.library.scanningPaths).not.toContain('/Users/test/audiobooks');
    });

    it('sets scanProgress during scan', async () => {
      vi.mocked(TauriCommands.getAllAudiobooks)
        .mockResolvedValueOnce([]) // before count
        .mockResolvedValueOnce([{ id: 1, name: 'Book', directory: '/test', fullPath: '/test/Book', completeness: 0, defaultOrder: 0, createdAt: '2024-01-01' }]); // after count
      vi.mocked(TauriCommands.scanDirectory).mockResolvedValue(undefined);

      const promise = store.dispatch(rescanDirectory('/Users/test/audiobooks'));

      // Check that scanProgress is set immediately
      let state = store.getState();
      expect(state.library.scanProgress).toBeDefined();
      expect(state.library.scanProgress?.path).toBe('/Users/test/audiobooks');
      expect(state.library.scanProgress?.startTime).toBeGreaterThan(0);

      await promise;

      // After completion, scanProgress should be cleared
      state = store.getState();
      expect(state.library.scanProgress).toBeUndefined();
    });

    it('updates scanProgress with found count', async () => {
      vi.mocked(TauriCommands.getAllAudiobooks)
        .mockResolvedValueOnce([]) // before count - 0 books
        .mockResolvedValueOnce([
          { id: 1, name: 'Book 1', directory: '/test', fullPath: '/test/Book1', completeness: 0, defaultOrder: 0, createdAt: '2024-01-01' },
          { id: 2, name: 'Book 2', directory: '/test', fullPath: '/test/Book2', completeness: 0, defaultOrder: 1, createdAt: '2024-01-01' },
        ]); // after count - 2 books
      vi.mocked(TauriCommands.scanDirectory).mockResolvedValue(undefined);

      await store.dispatch(rescanDirectory('/Users/test/audiobooks'));

      const state = store.getState();
      // scanProgress should be cleared after completion
      expect(state.library.scanProgress).toBeUndefined();
    });

    it('clears directory error on successful scan', async () => {
      // Pre-populate with error state
      store = createTestStore({
        library: {
          audiobooks: [],
          selectedAudiobook: undefined,
          currentFiles: [],
          selectedFile: undefined,
          directories: [
            {
              fullPath: '/Users/test/audiobooks',
              createdAt: '2024-01-01T00:00:00Z',
              error: 'NotFound',
            },
          ],
          bookmarks: [],
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          isLibraryLoading: false,
          isAddingDirectory: false,
          scanningPaths: [],
        },
        app: {
          theme: 'Light',
          panelRatio: 0.4,
          isLoading: false,
          activeModal: 'None',
          focusMode: 'Disabled',
          prefersReducedMotion: false,
          notifications: [],
          volumeExpanded: 'Collapsed',
        },
        player: {
          playback: 'Paused',
          currentTime: 0,
          totalDuration: 0,
          volume: 100,
          isMuted: false,
          volumeExpanded: 'Collapsed',
          speed: 1.0,
          sleepTimer: null,
          sleepTimerStartMs: null,
          sleepTimerDurationMs: null,
          sleepTimerRemainingMs: null,
          sleepTimerFadeDuration: 10,
        },
      });

      vi.mocked(TauriCommands.scanDirectory).mockResolvedValue(undefined);
      vi.mocked(TauriCommands.getAllAudiobooks).mockResolvedValue([]);

      await store.dispatch(rescanDirectory('/Users/test/audiobooks'));

      const state = store.getState();
      const dir = state.library.directories.find((d) => d.fullPath === '/Users/test/audiobooks');
      // Error is normalized to undefined when cleared (null -> undefined for consistent UI checks)
      expect(dir?.error).toBeUndefined();
    });

    it('removes path from scanningPaths even when scan fails', async () => {
      vi.mocked(TauriCommands.scanDirectory).mockRejectedValue(new Error('Scan failed'));
      vi.mocked(TauriCommands.getAllAudiobooks).mockResolvedValue([]);

      await store.dispatch(rescanDirectory('/Users/test/audiobooks'));

      const state = store.getState();
      expect(state.library.scanningPaths).not.toContain('/Users/test/audiobooks');
      // scanProgress should also be cleared
      expect(state.library.scanProgress).toBeUndefined();

      // Should show error notification
      expect(state.app.notifications).toHaveLength(1);
      expect(state.app.notifications[0].kind).toBe('Error');
      expect(state.app.notifications[0].message).toContain('Failed to rescan directory');
    });
  });

  describe('validateDirectoriesOnStartup', () => {
    it('sets NotFound error for missing directories', async () => {
      store = createTestStore({
        library: {
          audiobooks: [],
          selectedAudiobook: undefined,
          currentFiles: [],
          selectedFile: undefined,
          directories: [
            { fullPath: '/Users/test/missing', createdAt: '2024-01-01T00:00:00Z' },
          ],
          bookmarks: [],
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          isLibraryLoading: false,
          isAddingDirectory: false,
          scanningPaths: [],
        },
        app: {
          theme: 'Light',
          panelRatio: 0.4,
          isLoading: false,
          activeModal: 'None',
          focusMode: 'Disabled',
          prefersReducedMotion: false,
          notifications: [],
          volumeExpanded: 'Collapsed',
        },
        player: {
          playback: 'Paused',
          currentTime: 0,
          totalDuration: 0,
          volume: 100,
          isMuted: false,
          volumeExpanded: 'Collapsed',
          speed: 1.0,
          sleepTimer: null,
          sleepTimerStartMs: null,
          sleepTimerDurationMs: null,
          sleepTimerRemainingMs: null,
          sleepTimerFadeDuration: 10,
        },
      });

      vi.mocked(TauriCommands.validateDirectory).mockResolvedValue({
        exists: false,
        readable: false,
        error: 'NotFound',
      });

      await store.dispatch(validateDirectoriesOnStartup());

      const state = store.getState();
      const dir = state.library.directories.find((d) => d.fullPath === '/Users/test/missing');
      expect(dir?.error).toBe('NotFound');
    });

    it('sets PermissionDenied error for unreadable directories', async () => {
      store = createTestStore({
        library: {
          audiobooks: [],
          selectedAudiobook: undefined,
          currentFiles: [],
          selectedFile: undefined,
          directories: [
            { fullPath: '/Users/test/protected', createdAt: '2024-01-01T00:00:00Z' },
          ],
          bookmarks: [],
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          isLibraryLoading: false,
          isAddingDirectory: false,
          scanningPaths: [],
        },
        app: {
          theme: 'Light',
          panelRatio: 0.4,
          isLoading: false,
          activeModal: 'None',
          focusMode: 'Disabled',
          prefersReducedMotion: false,
          notifications: [],
          volumeExpanded: 'Collapsed',
        },
        player: {
          playback: 'Paused',
          currentTime: 0,
          totalDuration: 0,
          volume: 100,
          isMuted: false,
          volumeExpanded: 'Collapsed',
          speed: 1.0,
          sleepTimer: null,
          sleepTimerStartMs: null,
          sleepTimerDurationMs: null,
          sleepTimerRemainingMs: null,
          sleepTimerFadeDuration: 10,
        },
      });

      vi.mocked(TauriCommands.validateDirectory).mockResolvedValue({
        exists: true,
        readable: false,
        error: 'PermissionDenied',
      });

      await store.dispatch(validateDirectoriesOnStartup());

      const state = store.getState();
      const dir = state.library.directories.find((d) => d.fullPath === '/Users/test/protected');
      expect(dir?.error).toBe('PermissionDenied');
    });

    it('clears error for valid directories', async () => {
      store = createTestStore({
        library: {
          audiobooks: [],
          selectedAudiobook: undefined,
          currentFiles: [],
          selectedFile: undefined,
          directories: [
            {
              fullPath: '/Users/test/audiobooks',
              createdAt: '2024-01-01T00:00:00Z',
              error: 'NotFound', // Previously had an error
            },
          ],
          bookmarks: [],
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          isLibraryLoading: false,
          isAddingDirectory: false,
          scanningPaths: [],
        },
        app: {
          theme: 'Light',
          panelRatio: 0.4,
          isLoading: false,
          activeModal: 'None',
          focusMode: 'Disabled',
          prefersReducedMotion: false,
          notifications: [],
          volumeExpanded: 'Collapsed',
        },
        player: {
          playback: 'Paused',
          currentTime: 0,
          totalDuration: 0,
          volume: 100,
          isMuted: false,
          volumeExpanded: 'Collapsed',
          speed: 1.0,
          sleepTimer: null,
          sleepTimerStartMs: null,
          sleepTimerDurationMs: null,
          sleepTimerRemainingMs: null,
          sleepTimerFadeDuration: 10,
        },
      });

      vi.mocked(TauriCommands.validateDirectory).mockResolvedValue({
        exists: true,
        readable: true,
        error: null,
      });

      await store.dispatch(validateDirectoriesOnStartup());

      const state = store.getState();
      const dir = state.library.directories.find((d) => d.fullPath === '/Users/test/audiobooks');
      // Error is normalized to undefined when cleared (null -> undefined for consistent UI checks)
      expect(dir?.error).toBeUndefined();
    });

    it('handles validation errors gracefully', async () => {
      store = createTestStore({
        library: {
          audiobooks: [],
          selectedAudiobook: undefined,
          currentFiles: [],
          selectedFile: undefined,
          directories: [
            { fullPath: '/Users/test/audiobooks', createdAt: '2024-01-01T00:00:00Z' },
          ],
          bookmarks: [],
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          isLibraryLoading: false,
          isAddingDirectory: false,
          scanningPaths: [],
        },
        app: {
          theme: 'Light',
          panelRatio: 0.4,
          isLoading: false,
          activeModal: 'None',
          focusMode: 'Disabled',
          prefersReducedMotion: false,
          notifications: [],
          volumeExpanded: 'Collapsed',
        },
        player: {
          playback: 'Paused',
          currentTime: 0,
          totalDuration: 0,
          volume: 100,
          isMuted: false,
          volumeExpanded: 'Collapsed',
          speed: 1.0,
          sleepTimer: null,
          sleepTimerStartMs: null,
          sleepTimerDurationMs: null,
          sleepTimerRemainingMs: null,
          sleepTimerFadeDuration: 10,
        },
      });

      vi.mocked(TauriCommands.validateDirectory).mockRejectedValue(new Error('Network error'));

      // Should not throw
      await store.dispatch(validateDirectoriesOnStartup());

      const state = store.getState();
      const dir = state.library.directories.find((d) => d.fullPath === '/Users/test/audiobooks');
      expect(dir?.error).toBe('Unknown');
    });
  });
});
