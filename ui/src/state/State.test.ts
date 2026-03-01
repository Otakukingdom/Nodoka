/**
 * Unit tests for AppState
 *
 * Tests state management, notification handling, and value clamping
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { AppState } from './State';

describe('AppState', () => {
  let state: AppState;

  beforeEach(() => {
    state = new AppState();
  });

  describe('initial state', () => {
    it('should initialize with default values', () => {
      expect(state.playback).toBe('Paused');
      expect(state.volume).toBe(100);
      expect(state.speed).toBe(1.0);
      expect(state.audiobooks).toEqual([]);
      expect(state.currentFiles).toEqual([]);
      expect(state.bookmarks).toEqual([]);
      expect(state.directories).toEqual([]);
      expect(state.isLoading).toBe(true);
      expect(state.loadState).toBe('Loading');
      expect(state.theme).toBe('Light');
      expect(state.activeModal).toBe('None');
      expect(state.focusMode).toBe('Disabled');
      expect(state.prefersReducedMotion).toBe(false);
    });
  });

  describe('volume clamping', () => {
    it('should clamp volume to maximum 200', () => {
      state.setVolume(250);
      expect(state.volume).toBe(200);
    });

    it('should clamp volume to minimum 0', () => {
      state.setVolume(-10);
      expect(state.volume).toBe(0);
    });

    it('should accept valid volume values', () => {
      state.setVolume(75);
      expect(state.volume).toBe(75);
    });
  });

  describe('speed clamping', () => {
    it('should clamp speed to maximum 2.0', () => {
      state.setSpeed(3.0);
      expect(state.speed).toBe(2.0);
    });

    it('should clamp speed to minimum 0.5', () => {
      state.setSpeed(0.3);
      expect(state.speed).toBe(0.5);
    });

    it('should accept valid speed values', () => {
      state.setSpeed(1.5);
      expect(state.speed).toBe(1.5);
    });
  });

  describe('panel ratio clamping', () => {
    it('should clamp panel ratio to maximum 0.8', () => {
      state.setPanelRatio(0.9);
      expect(state.panelRatio).toBe(0.8);
    });

    it('should clamp panel ratio to minimum 0.2', () => {
      state.setPanelRatio(0.1);
      expect(state.panelRatio).toBe(0.2);
    });

    it('should accept valid panel ratio values', () => {
      state.setPanelRatio(0.5);
      expect(state.panelRatio).toBe(0.5);
    });
  });

  describe('notification management', () => {
    it('should add notifications', () => {
      const notification = {
        id: 1,
        kind: 'Info' as const,
        message: 'Test notification',
        durationMs: 3000,
        createdAt: Date.now(),
      };

      state.addNotification(notification);
      expect(state.notifications).toHaveLength(1);
      expect(state.notifications[0].id).toBe(1);
    });

    it('should remove notifications by id', () => {
      const notification1 = {
        id: 1,
        kind: 'Info' as const,
        message: 'Test 1',
        durationMs: 3000,
        createdAt: Date.now(),
      };

      const notification2 = {
        id: 2,
        kind: 'Success' as const,
        message: 'Test 2',
        durationMs: 3000,
        createdAt: Date.now(),
      };

      state.addNotification(notification1);
      state.addNotification(notification2);
      expect(state.notifications).toHaveLength(2);

      state.removeNotification(1);
      expect(state.notifications).toHaveLength(1);
      expect(state.notifications[0].id).toBe(2);
    });

    it('should limit notifications to maximum 3', () => {
      for (let i = 0; i < 5; i++) {
        state.addNotification({
          id: i,
          kind: 'Info' as const,
          message: `Test ${i}`,
          durationMs: 3000,
          createdAt: Date.now(),
        });
      }

      expect(state.notifications).toHaveLength(3);
      // Should keep the most recent ones (2, 3, 4)
      expect(state.notifications.map(n => n.id)).toEqual([2, 3, 4]);
    });
  });

  describe('modal management', () => {
    it('should open modal', () => {
      state.openModal('Settings');
      expect(state.activeModal).toBe('Settings');
    });

    it('should close modal', () => {
      state.openModal('Settings');
      state.closeModal();
      expect(state.activeModal).toBe('None');
    });

    it('should clear bookmark editor when closing modal', () => {
      state.setBookmarkEditor({
        audiobookId: 1,
        filePath: '/test/path.mp3',
        positionMs: 5000,
        label: 'Test',
        note: 'Note',
      });
      state.openModal('BookmarkEditor');
      state.closeModal();
      expect(state.bookmarkEditor).toBeUndefined();
    });
  });

  describe('loading state', () => {
    it('should set loading state', () => {
      state.setLoading(true);
      expect(state.isLoading).toBe(true);
      expect(state.loadState).toBe('Loading');
    });

    it('should set ready state', () => {
      state.setLoading(false);
      expect(state.isLoading).toBe(false);
      expect(state.loadState).toBe('Ready');
    });
  });

  describe('subscriber pattern', () => {
    it('should notify subscribers on state change', () => {
      let callCount = 0;
      const unsubscribe = state.subscribe(() => {
        callCount++;
      });

      state.setVolume(50);
      expect(callCount).toBe(1);

      state.setSpeed(1.5);
      expect(callCount).toBe(2);

      unsubscribe();
      state.setVolume(75);
      expect(callCount).toBe(2); // No new calls after unsubscribe
    });

    it('should support multiple subscribers', () => {
      let callCount1 = 0;
      let callCount2 = 0;

      state.subscribe(() => callCount1++);
      state.subscribe(() => callCount2++);

      state.setVolume(50);
      expect(callCount1).toBe(1);
      expect(callCount2).toBe(1);
    });
  });

  describe('state setters', () => {
    it('should set audiobooks', () => {
      const audiobooks = [
        { id: 1, name: 'Book 1', directory: '/test', fullPath: '/test/book1', completeness: 0, defaultOrder: 0, createdAt: '2024-01-01' },
      ];
      state.setAudiobooks(audiobooks);
      expect(state.audiobooks).toEqual(audiobooks);
    });

    it('should set current files', () => {
      const files = [
        { audiobookId: 1, name: 'File 1', fullPath: '/test/file1.mp3', position: 0, completeness: 0, fileExists: true, createdAt: '2024-01-01' },
      ];
      state.setCurrentFiles(files);
      expect(state.currentFiles).toEqual(files);
    });

    it('should set bookmarks', () => {
      const bookmarks = [
        { audiobookId: 1, filePath: '/test/file.mp3', positionMs: 1000, label: 'Bookmark 1', createdAt: '2024-01-01' },
      ];
      state.setBookmarks(bookmarks);
      expect(state.bookmarks).toEqual(bookmarks);
    });

    it('should set directories', () => {
      const directories = [
        { fullPath: '/audiobooks', createdAt: '2024-01-01' },
      ];
      state.setDirectories(directories);
      expect(state.directories).toEqual(directories);
    });

    it('should set playback status', () => {
      state.setPlayback('Playing');
      expect(state.playback).toBe('Playing');
    });

    it('should set theme', () => {
      state.setTheme('Dark');
      expect(state.theme).toBe('Dark');
    });

    it('should set focus mode', () => {
      state.setFocusMode('Enabled');
      expect(state.focusMode).toBe('Enabled');
    });

    it('should set search query', () => {
      state.setSearchQuery('test query');
      expect(state.searchQuery).toBe('test query');
    });

    it('should set sort order', () => {
      state.setSortOrder('Title');
      expect(state.sortOrder).toBe('Title');
    });

    it('should set filter status', () => {
      state.setFilterStatus('InProgress');
      expect(state.filterStatus).toBe('InProgress');
    });
  });
});
