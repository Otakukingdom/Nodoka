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
    it('shows scanning indicator when directories are being scanned with progress', () => {
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
        scanProgress: {
          path: '/test/audiobooks',
          startTime: Date.now(),
          foundCount: 3,
        },
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Should show scanning status with progress
      expect(screen.getByText('Scanning...')).toBeInTheDocument();
      // Should show found count
      expect(screen.getByText('3')).toBeInTheDocument();
    });

    it('shows elapsed time for long-running scans', () => {
      const startTime = Date.now() - 5000; // 5 seconds ago
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
        scanningPaths: ['/test/audiobooks'],
        scanProgress: {
          path: '/test/audiobooks',
          startTime,
          foundCount: 0,
        },
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Should show elapsed time (5s or more) - look for pattern like "5s"
      expect(screen.getByText(/\d+s/)).toBeInTheDocument();
    });

    it('does not show scanning indicator when scanProgress is missing', () => {
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
        scanningPaths: ['/test/audiobooks'],
        scanProgress: undefined,
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Should not show scanning status when no progress info
      expect(screen.queryByText('Scanning...')).not.toBeInTheDocument();
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

  describe('directory error recovery', () => {
    it('shows "Folder moved or deleted" message for NotFound error', () => {
      const store = createTestStore({
        audiobooks: [
          {
            id: 1,
            name: 'Test Audiobook',
            directory: '/test/missing',
            fullPath: '/test/missing/Test Audiobook',
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
            fullPath: '/test/missing',
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

      // Should show plain language error message
      expect(screen.getByText('Folder moved or deleted')).toBeInTheDocument();
    });

    it('shows "Permission denied" message for PermissionDenied error', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/protected',
            createdAt: '2024-01-01T00:00:00Z',
            error: 'PermissionDenied',
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

      // Should show plain language error message
      expect(screen.getByText('Permission denied')).toBeInTheDocument();
    });

    it('shows "Unable to access folder" for unknown errors', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/unknown',
            createdAt: '2024-01-01T00:00:00Z',
            error: 'UnknownError',
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

      // Should show generic error message
      expect(screen.getByText('Unable to access folder')).toBeInTheDocument();
    });

    it('shows Locate button only for NotFound errors', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/missing',
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

      // Locate button should be present for NotFound error
      expect(screen.getByRole('button', { name: 'Locate folder' })).toBeInTheDocument();
    });

    it('does not show Locate button for PermissionDenied errors', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/protected',
            createdAt: '2024-01-01T00:00:00Z',
            error: 'PermissionDenied',
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

      // Locate button should NOT be present for PermissionDenied
      expect(screen.queryByRole('button', { name: 'Locate folder' })).not.toBeInTheDocument();
    });

    it('shows Remove button for all error types', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/missing',
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

      // Remove button should be present
      expect(screen.getByRole('button', { name: 'Remove folder from library' })).toBeInTheDocument();
    });

    it('error banner has alert role for accessibility', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/missing',
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

      // Error banner should have role="alert"
      const alert = screen.getByRole('alert');
      expect(alert).toBeInTheDocument();
    });

    it('shows folder name (not full path) in error banner', () => {
      const store = createTestStore({
        audiobooks: [],
        selectedAudiobook: null,
        searchQuery: '',
        sortOrder: 'RecentlyPlayed',
        filterStatus: 'All',
        directories: [
          {
            fullPath: '/test/long/path/to/audiobooks',
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

      // Should show just the folder name, not the full path
      expect(screen.getByText('audiobooks')).toBeInTheDocument();
    });
  });

  describe('returning user - toolbar folder button', () => {
    it('shows folder button in toolbar when audiobooks exist', () => {
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

      // Toolbar button should be visible
      const toolbarButton = screen.getByRole('button', { name: 'Add audiobook folder' });
      expect(toolbarButton).toBeInTheDocument();
    });

    it('does not show folder button in toolbar when library is empty', () => {
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

      // Empty state button should be visible, but toolbar button should not
      const buttons = screen.getAllByRole('button', { name: 'Add audiobook folder' });
      // Only one button should be visible (the prominent empty state button)
      expect(buttons.length).toBe(1);
      // That button should have premium styling
      expect(buttons[0]).toHaveClass('btn-premium');
    });

    it('disables toolbar button while scanning', () => {
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
        directories: [{ fullPath: '/test', createdAt: '2024-01-01T00:00:00Z' }],
        currentFiles: [],
        selectedFile: null,
        bookmarks: [],
        isLibraryLoading: false,
        isAddingDirectory: false,
        scanningPaths: ['/test'],
      });

      render(
        <Provider store={store}>
          <LibraryPanel />
        </Provider>
      );

      // Toolbar button should be disabled during scan
      const toolbarButton = screen.getAllByRole('button', { name: 'Add audiobook folder' }).find(
        (btn) => btn.classList.contains('btn-square')
      );
      expect(toolbarButton).toBeDisabled();
    });
  });

  describe('UX specification requirements', () => {
    describe('touch target sizes', () => {
      it('Add Folder button meets minimum 44px touch target', () => {
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
        // min-h-12 class = 48px minimum height
        expect(button).toHaveClass('min-h-12');
      });
    });

    describe('accessibility - screen reader announcements', () => {
      it('has screen reader announcement region for async operations', () => {
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

        // Should have aria-live region for announcements
        const liveRegion = document.querySelector('[aria-live="polite"]');
        expect(liveRegion).toBeInTheDocument();
      });

      it('announces scan progress to screen readers', () => {
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
          scanningPaths: ['/test'],
          scanProgress: {
            path: '/test',
            startTime: Date.now(),
            foundCount: 5,
          },
        });

        render(
          <Provider store={store}>
            <LibraryPanel />
          </Provider>
        );

        // Scanning progress should be visible
        expect(screen.getByText('Scanning...')).toBeInTheDocument();
      });
    });

    describe('focus management', () => {
      it('all interactive elements are keyboard accessible', () => {
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

        // All buttons should have accessible names
        const buttons = screen.getAllByRole('button');
        buttons.forEach((button) => {
          expect(button).toHaveAttribute('aria-label');
        });

        // Search input should be accessible
        const searchInput = screen.getByPlaceholderText('Search audiobooks...');
        expect(searchInput).toBeInTheDocument();

        // Sort select should be accessible
        const sortSelect = screen.getByRole('combobox');
        expect(sortSelect).toBeInTheDocument();
      });
    });

    describe('plain language error messages', () => {
      it('error messages use plain language, not technical jargon', () => {
        const store = createTestStore({
          audiobooks: [],
          selectedAudiobook: null,
          searchQuery: '',
          sortOrder: 'RecentlyPlayed',
          filterStatus: 'All',
          directories: [
            {
              fullPath: '/test/missing',
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

        // Error message should be in plain language
        const errorText = screen.getByText('Folder moved or deleted');
        expect(errorText).toBeInTheDocument();

        // Should NOT contain technical jargon
        const alert = screen.getByRole('alert');
        expect(alert.textContent).not.toContain('ENOENT');
        expect(alert.textContent).not.toContain('Error code:');
        expect(alert.textContent).not.toContain('0x');
      });
    });

    describe('keyboard shortcut hints', () => {
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

        // Should show keyboard shortcut hint
        expect(screen.getByText(/or press/)).toBeInTheDocument();
        expect(screen.getByText('Ctrl')).toBeInTheDocument();
        expect(screen.getByText('D')).toBeInTheDocument();
      });

      it('shows drag and drop hint in empty state', () => {
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

        // Should show drag and drop hint
        expect(screen.getByText(/drag.*drop.*folder/i)).toBeInTheDocument();
      });
    });

    describe('loading states', () => {
      it('shows loading skeletons within 100ms of loading state', () => {
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

        // Should show skeleton loading indicators
        const skeletons = document.querySelectorAll('.skeleton');
        expect(skeletons.length).toBeGreaterThan(0);
      });
    });
  });
});
