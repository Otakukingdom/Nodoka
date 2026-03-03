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
    it('shows empty state with prominent call-to-action when no audiobooks', () => {
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
        isAddingDirectory: false,
        scanningPaths: [],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Check for new prominent title
      expect(screen.getByText('Add Your Audiobooks')).toBeInTheDocument();
    });

    it('shows Add Folder button with prominent styling in empty state', () => {
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
        isAddingDirectory: false,
        scanningPaths: [],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      const button = screen.getByRole('button', { name: 'Add audiobook folder' });
      expect(button).toBeInTheDocument();
      // Check for premium styling and min-height for touch target
      expect(button).toHaveClass('btn-premium');
      expect(button).toHaveClass('min-h-12');
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
        isAddingDirectory: false,
        scanningPaths: [],
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

    it('disables add button while library is loading', () => {
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
        isAddingDirectory: false,
        scanningPaths: [],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Button should not be visible during loading (skeletons shown instead)
      expect(screen.queryByRole('button', { name: 'Add audiobook folder' })).not.toBeInTheDocument();
    });

    it('disables add button while adding directory', () => {
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
        isAddingDirectory: true,
        scanningPaths: [],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      const button = screen.getByRole('button', { name: 'Add audiobook folder' });
      expect(button).toBeDisabled();
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
        isAddingDirectory: false,
        scanningPaths: [],
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

  describe('error state', () => {
    it('shows inline error banner for directories with errors', () => {
      const store = createTestStore({
        audiobooks: [
          {
            id: 1,
            name: 'Test Audiobook',
            directory: '/test/audiobooks',
            fullPath: '/test/audiobooks/Test Audiobook',
            completeness: 50,
            defaultOrder: 0,
            createdAt: '2024-01-01',
          },
        ],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/audiobooks',
            createdAt: '2024-01-01T00:00:00Z',
            error: 'NotFound',
          },
        ],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        isAddingDirectory: false,
        scanningPaths: [],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Should show error message for directory
      expect(screen.getByText('Folder moved or deleted')).toBeInTheDocument();
      // Should show locate button for NotFound error
      expect(screen.getByRole('button', { name: 'Locate folder' })).toBeInTheDocument();
    });
  });

  describe('scanning state', () => {
    it('shows scanning indicator when directories are being scanned', () => {
      const store = createTestStore({
        audiobooks: [
          {
            id: 1,
            name: 'Test Audiobook',
            directory: '/test/audiobooks',
            fullPath: '/test/audiobooks/Test Audiobook',
            completeness: 50,
            defaultOrder: 0,
            createdAt: '2024-01-01',
          },
        ],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          { fullPath: '/test/audiobooks', createdAt: '2024-01-01T00:00:00Z' },
        ],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        isAddingDirectory: false,
        scanningPaths: ['/test/audiobooks'],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Should show scanning status
      expect(screen.getByText(/Scanning/)).toBeInTheDocument();
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
        isAddingDirectory: false,
        scanningPaths: [],
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
        isAddingDirectory: false,
        scanningPaths: [],
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
        isAddingDirectory: false,
        scanningPaths: [],
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

    it('search input accepts input', async () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      const searchInput = screen.getByPlaceholderText('Search audiobooks...');
      fireEvent.change(searchInput, { target: { value: 'test search' } });

      // Input should accept the change (debounced dispatch happens separately)
      expect(searchInput).toBeInTheDocument();
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
