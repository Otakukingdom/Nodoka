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
  addBookmark,
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
  async (path: string, { dispatch }) => {
    try {
      await TauriCommands.insertDirectory(path);
      const directories = await TauriCommands.getDirectories();
      dispatch(setDirectories(directories));
    } catch (error) {
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to add directory: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
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

export const rescanDirectory = createAsyncThunk(
  'library/rescanDirectory',
  async (path: string, { dispatch }) => {
    dispatch(setLibraryLoading(true));
    try {
      await TauriCommands.scanDirectory(path);
      const audiobooks = await TauriCommands.getAllAudiobooks();
      dispatch(setAudiobooks(audiobooks));
      dispatch(setLibraryLoading(false));
    } catch (error) {
      dispatch(setLibraryLoading(false));
      dispatch(addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to rescan directory: ${error}`,
        durationMs: 5000,
        createdAt: Date.now(),
      }));
    }
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
