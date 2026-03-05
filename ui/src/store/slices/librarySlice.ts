/**
 * Redux slice for audiobook library state
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type {
  Audiobook,
  AudiobookFile,
  Bookmark,
  Directory,
  AudiobookSortOrder,
  AudiobookFilterStatus,
} from '../types';

/** Detailed progress information for an ongoing scan */
export interface ScanProgressInfo {
  /** Path being scanned */
  path: string;
  /** Timestamp when scan started (ms since epoch) */
  startTime: number;
  /** Number of audiobooks found so far */
  foundCount: number;
  /** Total expected count (if known) */
  totalCount?: number;
  /** Current subfolder being scanned */
  currentSubPath?: string;
}

interface LibraryState {
  // Audiobook library
  audiobooks: Audiobook[];
  selectedAudiobook?: number;
  currentFiles: AudiobookFile[];
  selectedFile?: string;
  directories: Directory[];

  // Bookmarks
  bookmarks: Bookmark[];

  // Search and filter
  searchQuery: string;
  sortOrder: AudiobookSortOrder;
  filterStatus: AudiobookFilterStatus;

  // Loading state
  isLibraryLoading: boolean;
  isAddingDirectory: boolean;

  // Scanning progress
  scanningPaths: string[];
  /** Detailed progress for the most recent/current scan */
  scanProgress?: ScanProgressInfo;
}

const initialState: LibraryState = {
  audiobooks: [],
  selectedAudiobook: undefined,
  currentFiles: [],
  selectedFile: undefined,
  directories: [],
  bookmarks: [],
  searchQuery: '',
  sortOrder: 'RecentlyPlayed',
  filterStatus: 'All',
  isLibraryLoading: true,
  isAddingDirectory: false,
  scanningPaths: [],
  scanProgress: undefined,
};

const librarySlice = createSlice({
  name: 'library',
  initialState,
  reducers: {
    setAudiobooks: (state, action: PayloadAction<Audiobook[]>) => {
      state.audiobooks = action.payload;
    },
    setSelectedAudiobook: (state, action: PayloadAction<number | undefined>) => {
      state.selectedAudiobook = action.payload;
    },
    setCurrentFiles: (state, action: PayloadAction<AudiobookFile[]>) => {
      state.currentFiles = action.payload;
    },
    setSelectedFile: (state, action: PayloadAction<string | undefined>) => {
      state.selectedFile = action.payload;
    },
    setDirectories: (state, action: PayloadAction<Directory[]>) => {
      state.directories = action.payload;
    },
    setBookmarks: (state, action: PayloadAction<Bookmark[]>) => {
      state.bookmarks = action.payload;
    },
    addBookmark: (state, action: PayloadAction<Bookmark>) => {
      state.bookmarks.push(action.payload);
    },
    removeBookmark: (state, action: PayloadAction<number>) => {
      state.bookmarks = state.bookmarks.filter(b => b.id !== action.payload);
    },
    setSearchQuery: (state, action: PayloadAction<string>) => {
      state.searchQuery = action.payload;
    },
    setSortOrder: (state, action: PayloadAction<AudiobookSortOrder>) => {
      state.sortOrder = action.payload;
    },
    setFilterStatus: (state, action: PayloadAction<AudiobookFilterStatus>) => {
      state.filterStatus = action.payload;
    },
    clearLibrarySelection: (state) => {
      state.selectedAudiobook = undefined;
      state.currentFiles = [];
      state.selectedFile = undefined;
      state.bookmarks = [];
    },
    setLibraryLoading: (state, action: PayloadAction<boolean>) => {
      state.isLibraryLoading = action.payload;
    },
    addScanningPath: (state, action: PayloadAction<string>) => {
      if (!state.scanningPaths.includes(action.payload)) {
        state.scanningPaths.push(action.payload);
      }
    },
    removeScanningPath: (state, action: PayloadAction<string>) => {
      state.scanningPaths = state.scanningPaths.filter(p => p !== action.payload);
    },
    setDirectoryError: (state, action: PayloadAction<{ path: string; error: Directory['error'] | null }>) => {
      const dir = state.directories.find(d => d.fullPath === action.payload.path);
      if (dir) {
        // Normalize `null` (no error) to `undefined` so UI can reliably treat
        // `error !== undefined` as “has an error”.
        dir.error = action.payload.error ?? undefined;
      }
    },
    setIsAddingDirectory: (state, action: PayloadAction<boolean>) => {
      state.isAddingDirectory = action.payload;
    },
    /** Start tracking scan progress for a path */
    startScanProgress: (state, action: PayloadAction<{ path: string; totalCount?: number }>) => {
      state.scanProgress = {
        path: action.payload.path,
        startTime: Date.now(),
        foundCount: 0,
        totalCount: action.payload.totalCount,
      };
    },
    /** Update scan progress with found count */
    updateScanProgress: (state, action: PayloadAction<{ foundCount: number; currentSubPath?: string }>) => {
      if (state.scanProgress) {
        state.scanProgress.foundCount = action.payload.foundCount;
        if (action.payload.currentSubPath) {
          state.scanProgress.currentSubPath = action.payload.currentSubPath;
        }
      }
    },
    /** Clear scan progress when scan completes */
    clearScanProgress: (state) => {
      state.scanProgress = undefined;
    },
  },
});

export const {
  setAudiobooks,
  setSelectedAudiobook,
  setCurrentFiles,
  setSelectedFile,
  setDirectories,
  setBookmarks,
  addBookmark,
  removeBookmark,
  setSearchQuery,
  setSortOrder,
  setFilterStatus,
  clearLibrarySelection,
  setLibraryLoading,
  setIsAddingDirectory,
  addScanningPath,
  removeScanningPath,
  setDirectoryError,
  startScanProgress,
  updateScanProgress,
  clearScanProgress,
} = librarySlice.actions;

// ScanProgressInfo is already exported via the interface definition above

export default librarySlice.reducer;

