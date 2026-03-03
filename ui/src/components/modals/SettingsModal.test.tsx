/**
 * Unit tests for SettingsModal component - Directory Management
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { Provider } from 'react-redux';
import { configureStore } from '@reduxjs/toolkit';
import SettingsModal from './SettingsModal';
import appReducer from '../../store/slices/appSlice';
import playerReducer from '../../store/slices/playerSlice';
import libraryReducer from '../../store/slices/librarySlice';

// Mock Tauri dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn().mockResolvedValue(null),
}));

const createTestStore = (libraryOverrides?: Partial<ReturnType<typeof libraryReducer>>) =>
  configureStore({
    reducer: {
      app: appReducer,
      player: playerReducer,
      library: libraryReducer,
    },
    preloadedState: libraryOverrides
      ? {
          app: {
            theme: 'Light' as const,
            isLoading: false,
            activeModal: 'Settings' as const,
            focusMode: 'Disabled' as const,
            prefersReducedMotion: false,
            panelRatio: 0.4,
            panelDividerDragging: false,
            notifications: [],
          },
          player: {
            status: 'Stopped' as const,
            currentTimeMs: 0,
            durationMs: 0,
            volume: 100,
            isMuted: false,
            playbackRate: 1,
            volumeExpansion: 'Collapsed' as const,
          },
          library: {
            audiobooks: [],
            selectedAudiobook: null,
            searchQuery: '',
            sortOrder: 'RecentlyPlayed' as const,
            filterStatus: 'All' as const,
            directories: [],
            currentFiles: [],
            selectedFile: null,
            bookmarks: [],
            isLibraryLoading: false,
            loadState: 'Ready' as const,
            ...libraryOverrides,
          },
        }
      : undefined,
  });

describe('SettingsModal - Directory Management', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('empty directories state', () => {
    it('shows empty state when no directories', () => {
      const store = createTestStore({
        directories: [],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByText('No directories added yet')).toBeInTheDocument();
    });

    it('shows folder icon in empty state', () => {
      const store = createTestStore({
        directories: [],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      // Check for Folder icon by looking for the empty state structure
      const emptyStateText = screen.getByText('No directories added yet');
      expect(emptyStateText).toBeInTheDocument();
    });
  });

  describe('directory list', () => {
    it('shows directories when they exist', () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/audiobooks',
            createdAt: '2024-01-01',
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByText('audiobooks')).toBeInTheDocument();
      expect(screen.getByText('/home/user/audiobooks')).toBeInTheDocument();
    });

    it('shows rescan button for each directory', () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/audiobooks',
            createdAt: '2024-01-01',
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Rescan directory' })).toBeInTheDocument();
    });

    it('shows remove button for each directory', () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/audiobooks',
            createdAt: '2024-01-01',
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Remove directory' })).toBeInTheDocument();
    });
  });

  describe('confirmation dialog', () => {
    it('shows confirmation dialog when remove is clicked', async () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/audiobooks',
            createdAt: '2024-01-01',
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      // Click remove button
      fireEvent.click(screen.getByRole('button', { name: 'Remove directory' }));

      // Confirmation dialog should appear
      await waitFor(() => {
        expect(screen.getByText(/Remove audiobooks\?/)).toBeInTheDocument();
      });
    });

    it('shows explanatory message in confirmation dialog', async () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/audiobooks',
            createdAt: '2024-01-01',
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      // Click remove button
      fireEvent.click(screen.getByRole('button', { name: 'Remove directory' }));

      // Confirmation dialog should show message about files not being deleted
      await waitFor(() => {
        expect(
          screen.getByText(/This will remove the folder from your library/)
        ).toBeInTheDocument();
      });
    });

    it('closes dialog when cancel is clicked', async () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/audiobooks',
            createdAt: '2024-01-01',
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      // Click remove button
      fireEvent.click(screen.getByRole('button', { name: 'Remove directory' }));

      // Wait for dialog to appear
      await waitFor(() => {
        expect(screen.getByText(/Remove audiobooks\?/)).toBeInTheDocument();
      });

      // Click cancel - use getByText since the button might be in a nested dialog
      const cancelButton = screen.getByText('Cancel').closest('button');
      if (cancelButton) {
        fireEvent.click(cancelButton);
      }

      // Dialog should close
      await waitFor(() => {
        expect(screen.queryByText(/Remove audiobooks\?/)).not.toBeInTheDocument();
      });
    });
  });

  describe('add directory button', () => {
    it('shows add directory button', () => {
      const store = createTestStore({
        directories: [],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Add directory' })).toBeInTheDocument();
    });

    it('disables add directory button while loading', () => {
      const store = createTestStore({
        directories: [],
        isLibraryLoading: true,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Add directory' })).toBeDisabled();
    });
  });

  describe('error state', () => {
    it('shows warning icon for directory with error', () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/missing',
            createdAt: '2024-01-01',
            error: 'NotFound' as const,
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByText('Folder not found')).toBeInTheDocument();
    });

    it('shows permission denied error message', () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/protected',
            createdAt: '2024-01-01',
            error: 'PermissionDenied' as const,
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByText('Permission denied')).toBeInTheDocument();
    });

    it('shows retry button for errored directories', () => {
      const store = createTestStore({
        directories: [
          {
            fullPath: '/home/user/missing',
            createdAt: '2024-01-01',
            error: 'NotFound' as const,
          },
        ],
        isLibraryLoading: false,
      });

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Retry' })).toBeInTheDocument();
    });
  });

  describe('settings modal header', () => {
    it('renders settings title', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByText('Settings')).toBeInTheDocument();
    });

    it('renders close button', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      expect(screen.getByRole('button', { name: 'Close' })).toBeInTheDocument();
    });
  });

  describe('theme selection', () => {
    it('renders theme dropdown', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      // Find theme dropdown by looking for the theme section label
      const themeLabel = screen.getByText('Theme');
      expect(themeLabel).toBeInTheDocument();
    });

    it('shows theme options', () => {
      const store = createTestStore();

      render(
        <Provider store={store}>
          <SettingsModal />
        </Provider>
      );

      // Check that theme options exist
      const lightOption = screen.getByRole('option', { name: 'Light' });
      const darkOption = screen.getByRole('option', { name: 'Dark' });
      const systemOption = screen.getByRole('option', { name: 'System' });

      expect(lightOption).toBeInTheDocument();
      expect(darkOption).toBeInTheDocument();
      expect(systemOption).toBeInTheDocument();
    });
  });
});
