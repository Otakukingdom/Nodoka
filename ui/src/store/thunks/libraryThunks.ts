/**
 * Async thunks for library-related Tauri commands
 */

import { createAsyncThunk } from '@reduxjs/toolkit';
import {
  setAudiobooks,
  setSelectedAudiobook,
  setCurrentFiles,
  setBookmarks,
  setSelectedFile,
  setDirectories,
  setLibraryLoading,
  setIsAddingDirectory,
  addBookmark,
  addScanningPath,
  removeScanningPath,
  setDirectoryError,
} from '../slices/librarySlice';
import { setTheme, setPanelRatio, setLoading, addNotification } from '../slices/appSlice';
import { setVolume, setSpeed, setPlayback } from '../slices/playerSlice';
import * as TauriCommands from '../../commands/tauri';
import type { AudiobookSortOrder, ThemeMode, Audiobook, Bookmark } from '../types';
import type { RootState } from '../index';

export const loadInitialData = createAsyncThunk(
  'library/loadInitialData',
  async (_, { dispatch }) => {
    try {
      const [audiobooks, directories, theme, panelRatio, volume, speed] = await Promise.all([
        TauriCommands.getAllAudiobooks(),
        TauriCommands.getDirectories(),
        TauriCommands.getTheme().catch(() => 'Light'),
        TauriCommands.getPanelRatio().catch(() => 0.4),
        TauriCommands.getVolume().catch(() => 100),
        TauriCommands.getSpeed().catch(() => 1.0),
      ]);

      dispatch(setAudiobooks(audiobooks));
      dispatch(setDirectories(directories));
      dispatch(setTheme(theme as ThemeMode));
      dispatch(setPanelRatio(panelRatio));
      dispatch(setVolume(volume));
      dispatch(setSpeed(speed));
      dispatch(setLibraryLoading(false));
      dispatch(setLoading(false));

      // Validate directories after loading
      if (directories.length > 0) {
        // Run validation in background without blocking UI
        void dispatch(validateDirectoriesOnStartup());
      }
    } catch (error) {
      console.error('Failed to load initial data:', error);
      dispatch(setLibraryLoading(false));
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to load library: ${error}`,
        durationMs: Infinity,
        createdAt: Date.now(),
      }));
      dispatch(setLoading(false));
    }
  }
);

export const selectAudiobook = createAsyncThunk(
  'library/selectAudiobook',
  async (id: number, { dispatch }) => {
    dispatch(setSelectedAudiobook(id));
    try {
      const [files, bookmarks] = await Promise.all([
        TauriCommands.getAudiobookFiles(id),
        TauriCommands.getBookmarks(id),
      ]);
      dispatch(setCurrentFiles(files));
      dispatch(setBookmarks(bookmarks));
    } catch (error) {
      console.error('Failed to load audiobook details:', error);
    }
  }
);

export const selectFile = createAsyncThunk(
  'library/selectFile',
  async (path: string, { dispatch }) => {
    dispatch(setSelectedFile(path));
    try {
      await TauriCommands.loadFile(path);
      dispatch(setPlayback('Playing'));
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to load file: ${error}`,
        durationMs: Infinity,
        createdAt: Date.now(),
      }));
    }
  }
);

export const addDirectory = createAsyncThunk(
  'library/addDirectory',
  async (path: string, { dispatch, getState }) => {
    const state = getState() as RootState;

    // Check for duplicate directory
    const exists = state.library.directories.some(
      d => d.fullPath === path
    );
    if (exists) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Warning',
        message: 'That folder is already in your library',
        durationMs: 3000,
        createdAt: Date.now(),
      }));
      return;
    }

    // Set immediate loading state for UI feedback
    dispatch(setIsAddingDirectory(true));

    try {
      // Get audiobook count before adding
      const audiobooksBefore = await TauriCommands.getAllAudiobooks();
      const countBefore = audiobooksBefore.length;

      await TauriCommands.insertDirectory(path);
      const directories = await TauriCommands.getDirectories();
      dispatch(setDirectories(directories));

      // Get audiobook count after adding to check if any were found
      const audiobooksAfter = await TauriCommands.getAllAudiobooks();
      dispatch(setAudiobooks(audiobooksAfter));

      // Show feedback if no audiobooks were found in the folder
      if (audiobooksAfter.length === countBefore) {
        dispatch(addNotification({
          id: Date.now(),
          kind: 'Info',
          message: 'No audiobooks found in that folder. Try a folder with audio files.',
          durationMs: 5000,
          createdAt: Date.now(),
        }));
      }
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to add directory: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
    } finally {
      dispatch(setIsAddingDirectory(false));
    }
  }
);

export const removeDirectory = createAsyncThunk(
  'library/removeDirectory',
  async (path: string, { dispatch }) => {
    try {
      await TauriCommands.removeDirectory(path);
      const directories = await TauriCommands.getDirectories();
      dispatch(setDirectories(directories));
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to remove directory: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
    }
  }
);

/**
 * Update directory path for relocated folders
 * Called when a user locates a folder that was moved
 */
export const updateDirectoryPath = createAsyncThunk(
  'library/updateDirectoryPath',
  async ({ oldPath, newPath }: { oldPath: string; newPath: string }, { dispatch }) => {
    try {
      await TauriCommands.updateDirectoryPath(oldPath, newPath);
      // Refresh both directories and audiobooks since paths have changed
      const [directories, audiobooks] = await Promise.all([
        TauriCommands.getDirectories(),
        TauriCommands.getAllAudiobooks(),
      ]);
      dispatch(setDirectories(directories));
      dispatch(setAudiobooks(audiobooks));
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Success',
        message: 'Folder path updated successfully',
        durationMs: 3000,
        createdAt: Date.now(),
      }));
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to update folder path: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
    }
  }
);

export const rescanDirectory = createAsyncThunk(
  'library/rescanDirectory',
  async (path: string, { dispatch }) => {
    dispatch(addScanningPath(path));
    try {
      await TauriCommands.scanDirectory(path);
      const audiobooks = await TauriCommands.getAllAudiobooks();
      dispatch(setAudiobooks(audiobooks));
      // Clear any error for this directory on successful scan
      dispatch(setDirectoryError({ path, error: null }));
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to rescan directory: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
    } finally {
      dispatch(removeScanningPath(path));
    }
  }
);

/**
 * Validate all directories on startup
 * Checks each directory for accessibility and sets error state if issues found
 */
export const validateDirectoriesOnStartup = createAsyncThunk(
  'library/validateDirectoriesOnStartup',
  async (_, { dispatch, getState }) => {
    const state = getState() as RootState;
    const { directories } = state.library;

    // Validate each directory in parallel
    const validationPromises = directories.map(async (dir) => {
      try {
        const validation = await TauriCommands.validateDirectory(dir.fullPath);
        if (!validation.exists) {
          dispatch(setDirectoryError({ path: dir.fullPath, error: 'NotFound' }));
        } else if (!validation.readable) {
          dispatch(setDirectoryError({ path: dir.fullPath, error: 'PermissionDenied' }));
        } else {
          dispatch(setDirectoryError({ path: dir.fullPath, error: null }));
        }
      } catch (error) {
        console.error(`Failed to validate directory ${dir.fullPath}:`, error);
        dispatch(setDirectoryError({ path: dir.fullPath, error: 'Unknown' }));
      }
    });

    await Promise.all(validationPromises);
  }
);

export const updateTheme = createAsyncThunk(
  'library/updateTheme',
  async (theme: ThemeMode, { dispatch }) => {
    dispatch(setTheme(theme));
    try {
      await TauriCommands.setTheme(theme);
    } catch (error) {
      console.error('Failed to save theme:', error);
    }
  }
);

export const getFilteredAudiobooks = (
  audiobooks: Audiobook[],
  searchQuery: string,
  filterStatus: string,
  sortOrder: AudiobookSortOrder
): Audiobook[] => {
  let filtered = [...audiobooks];

  // Apply search filter
  if (searchQuery.length > 0) {
    const query = searchQuery.toLowerCase();
    filtered = filtered.filter(book =>
      book.name.toLowerCase().includes(query)
    );
  }

  // Apply status filter
  switch (filterStatus) {
    case 'InProgress':
      filtered = filtered.filter(book => book.completeness > 0 && book.completeness < 100);
      break;
    case 'Complete':
      filtered = filtered.filter(book => book.completeness === 100);
      break;
    case 'NotStarted':
      filtered = filtered.filter(book => book.completeness === 0);
      break;
  }

  // Apply sort order
  switch (sortOrder) {
    case 'Title':
      filtered.sort((a, b) => a.name.localeCompare(b.name));
      break;
    case 'Progress':
      filtered.sort((a, b) => b.completeness - a.completeness);
      break;
    case 'RecentlyPlayed':
      filtered.sort((a, b) => {
        const aTime = a.lastListened !== undefined ? new Date(a.lastListened).getTime() : 0;
        const bTime = b.lastListened !== undefined ? new Date(b.lastListened).getTime() : 0;
        return bTime - aTime;
      });
      break;
  }

  return filtered;
};

/**
 * Format milliseconds to MM:SS or HH:MM:SS string
 */
function formatBookmarkTime(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}

/**
 * Create a bookmark at the current playback position
 * Triggered by Ctrl+B keyboard shortcut
 */
export const createBookmark = createAsyncThunk(
  'library/createBookmark',
  async (_, { getState, dispatch }) => {
    const state = getState() as RootState;
    const { selectedAudiobook, selectedFile } = state.library;
    const { currentTime } = state.player;

    // Validate required state
    if (selectedAudiobook === undefined || !selectedFile) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Warning',
        message: 'Cannot create bookmark: no file is playing',
        durationMs: 3000,
        createdAt: Date.now(),
      }));
      return null;
    }

    const label = `Bookmark at ${formatBookmarkTime(currentTime)}`;

    try {
      const bookmarkId = await TauriCommands.createBookmark(
        selectedAudiobook,
        selectedFile,
        currentTime,
        label,
        undefined
      );

      const bookmark: Bookmark = {
        id: bookmarkId,
        audiobookId: selectedAudiobook,
        filePath: selectedFile,
        positionMs: currentTime,
        label,
        note: undefined,
        createdAt: new Date().toISOString(),
      };

      dispatch(addBookmark(bookmark));

      dispatch(addNotification({
        id: Date.now(),
        kind: 'Success',
        message: `Bookmark created at ${formatBookmarkTime(currentTime)}`,
        durationMs: 2000,
        createdAt: Date.now(),
      }));

      return bookmark;
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to create bookmark: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
      return null;
    }
  }
);
