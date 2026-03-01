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
} from '../slices/librarySlice';
import { setTheme, setPanelRatio, setLoading, addNotification } from '../slices/appSlice';
import { setVolume, setSpeed, setPlayback } from '../slices/playerSlice';
import * as TauriCommands from '../../commands/tauri';
import type { AudiobookSortOrder, ThemeMode, Audiobook } from '../types';

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
      dispatch(setLoading(false));
    } catch (error) {
      console.error('Failed to load initial data:', error);
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
    try {
      await TauriCommands.scanDirectory(path);
      const audiobooks = await TauriCommands.getAllAudiobooks();
      dispatch(setAudiobooks(audiobooks));
    } catch (error) {
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
