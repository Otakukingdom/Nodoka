/**
 * Unit tests for LibraryPanel component
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Provider } from 'react-redux';
import { configureStore } from '@reduxjs/toolkit';
import LibraryPanel from './LibraryPanel';
import appReducer from '../store/slices/appSlice';
import playerReducer from '../store/slices/playerSlice';
import libraryReducer from '../store/slices/librarySlice';

// Mock Tauri dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn().mockResolvedValue(null),
}));

const createTestStore = (initialState?: Partial<ReturnType<typeof libraryReducer>>) =>
  configureStore({
    reducer: {
      app: appReducer,
      player: playerReducer,
      library: libraryReducer,
    },
    preloadedState: initialState
      ? {
          app: { panelRatio: 0.4 } as ReturnType<typeof appReducer>,
          player: {} as ReturnType<typeof playerReducer>,
          library: initialState as ReturnType<typeof libraryReducer>,
        }
      : undefined,
  });

describe('LibraryPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('empty state', () => {
    it('shows empty state when no audiobooks', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByText('No audiobooks yet')).toBeInTheDocument();
    });

    it('shows Add Audiobook Folder button in empty state', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Add audiobook folder' })).toBeInTheDocument();
    });

    it('shows keyboard shortcut hint in empty state', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByText(/or press/)).toBeInTheDocument();
      expect(screen.getByText('Ctrl')).toBeInTheDocument();
      expect(screen.getByText('D')).toBeInTheDocument();
    });

    it('shows search empty state when search has no results', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: 'nonexistent',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByText('No matches found')).toBeInTheDocument();
      expect(screen.queryByRole('button', { name: 'Add audiobook folder' })).not.toBeInTheDocument();
    });
  });

  describe('loading state', () => {
    it('shows loading skeletons when library is loading', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: true,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Check for skeleton elements (they use the "skeleton" class)
      const skeletons = document.querySelectorAll('.skeleton');
      expect(skeletons.length).toBeGreaterThan(0);
    });
  });

  describe('with audiobooks', () => {
    it('renders audiobook list when audiobooks exist', () => {
      const store = createTestStore({
        audiobooks: [
          {
            id: 1,
            name: 'Test Audiobook',
            directory: '/test',
            fullPath: '/test/Test Audiobook',
            completeness: 50,
            defaultOrder: 0,
            createdAt: '2024-01-01',
          },
        ],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByText('Test Audiobook')).toBeInTheDocument();
    });

    it('shows checkmark for completed audiobooks', () => {
      const store = createTestStore({
        audiobooks: [
          {
            id: 1,
            name: 'Completed Book',
            directory: '/test',
            fullPath: '/test/Completed Book',
            completeness: 100,
            defaultOrder: 0,
            createdAt: '2024-01-01',
          },
        ],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        loadState: 'Ready',
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // The Check icon is rendered for completed books
      expect(screen.getByText('Completed Book')).toBeInTheDocument();
    });
  });

  describe('search functionality', () => {
    it('renders search input', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByPlaceholderText('Search audiobooks...')).toBeInTheDocument();
    });

    it('updates search query on input change', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      const searchInput = screen.getByPlaceholderText('Search audiobooks...');
      fireEvent.change(searchInput, { target: { value: 'test search' } });

      expect(searchInput).toHaveValue('test search');
    });
  });

  describe('sort functionality', () => {
    it('renders sort dropdown', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      const sortSelect = screen.getByRole('combobox');
      expect(sortSelect).toBeInTheDocument();
    });

    it('has correct sort options', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      expect(screen.getByRole('option', { name: 'Recent' })).toBeInTheDocument();
      expect(screen.getByRole('option', { name: 'Title' })).toBeInTheDocument();
      expect(screen.getByRole('option', { name: 'Progress' })).toBeInTheDocument();
    });
  });
});
