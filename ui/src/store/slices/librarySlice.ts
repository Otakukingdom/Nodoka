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
} = librarySlice.actions;

export default librarySlice.reducer;
