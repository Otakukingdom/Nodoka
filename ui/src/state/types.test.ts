/**
 * Unit tests for TypeScript type definitions
 *
 * Tests type guards and runtime type validation for frontend types
 */

import { describe, it, expect } from 'vitest';
import type {
  Audiobook,
  AudiobookFile,
  Bookmark,
  Directory,
  Notification,
  BookmarkEditor,
  PlaybackStatus,
  ThemeMode,
  ActiveModal,
  AudiobookSortOrder,
  AudiobookFilterStatus,
  FocusMode,
  VolumeExpansion,
  LoadState,
  NotificationKind,
} from './types';

describe('Types', () => {
  describe('Audiobook type', () => {
    it('should create valid audiobook object', () => {
      const audiobook: Audiobook = {
        id: 1,
        directory: '/audiobooks',
        name: 'Test Book',
        author: 'Test Author',
        fullPath: '/audiobooks/test-book',
        completeness: 50,
        defaultOrder: 0,
        selectedFile: '/audiobooks/test-book/chapter1.mp3',
        totalDurationMs: 3600000,
        remainingDurationMs: 1800000,
        createdAt: '2024-01-01T00:00:00Z',
        lastListened: '2024-01-15T12:00:00Z',
      };

      expect(audiobook.id).toBe(1);
      expect(audiobook.name).toBe('Test Book');
      expect(audiobook.completeness).toBe(50);
    });

    it('should allow minimal audiobook object', () => {
      const audiobook: Audiobook = {
        directory: '/audiobooks',
        name: 'Minimal Book',
        fullPath: '/audiobooks/minimal-book',
        completeness: 0,
        defaultOrder: 0,
        createdAt: '2024-01-01T00:00:00Z',
      };

      expect(audiobook.id).toBeUndefined();
      expect(audiobook.author).toBeUndefined();
    });
  });

  describe('AudiobookFile type', () => {
    it('should create valid audiobook file object', () => {
      const file: AudiobookFile = {
        audiobookId: 1,
        name: 'Chapter 1',
        fullPath: '/audiobooks/test-book/chapter1.mp3',
        lengthOfFile: 1800000,
        seekPosition: 0,
        checksum: 'abc123',
        position: 0,
        completeness: 0,
        fileExists: true,
        createdAt: '2024-01-01T00:00:00Z',
      };

      expect(file.audiobookId).toBe(1);
      expect(file.position).toBe(0);
      expect(file.fileExists).toBe(true);
    });
  });

  describe('Bookmark type', () => {
    it('should create valid bookmark object', () => {
      const bookmark: Bookmark = {
        id: 1,
        audiobookId: 1,
        filePath: '/audiobooks/test-book/chapter1.mp3',
        positionMs: 5000,
        label: 'Important part',
        note: 'This is a note',
        createdAt: '2024-01-01T00:00:00Z',
      };

      expect(bookmark.id).toBe(1);
      expect(bookmark.positionMs).toBe(5000);
      expect(bookmark.note).toBe('This is a note');
    });

    it('should allow bookmark without note', () => {
      const bookmark: Bookmark = {
        audiobookId: 1,
        filePath: '/audiobooks/test-book/chapter1.mp3',
        positionMs: 10000,
        label: 'Chapter mark',
        createdAt: '2024-01-01T00:00:00Z',
      };

      expect(bookmark.note).toBeUndefined();
    });
  });

  describe('Directory type', () => {
    it('should create valid directory object', () => {
      const directory: Directory = {
        fullPath: '/audiobooks',
        createdAt: '2024-01-01T00:00:00Z',
        lastScanned: '2024-01-15T12:00:00Z',
      };

      expect(directory.fullPath).toBe('/audiobooks');
      expect(directory.lastScanned).toBeDefined();
    });
  });

  describe('Notification type', () => {
    it('should create valid notification object', () => {
      const notification: Notification = {
        id: 1,
        kind: 'Success',
        title: 'Action completed',
        message: 'The operation was successful',
        actionLabel: 'Dismiss',
        durationMs: 3000,
        createdAt: Date.now(),
      };

      expect(notification.kind).toBe('Success');
      expect(notification.durationMs).toBe(3000);
    });
  });

  describe('BookmarkEditor type', () => {
    it('should create valid bookmark editor state', () => {
      const editor: BookmarkEditor = {
        id: 1,
        audiobookId: 1,
        filePath: '/audiobooks/test-book/chapter1.mp3',
        positionMs: 5000,
        label: 'My bookmark',
        note: 'My note',
      };

      expect(editor.id).toBe(1);
      expect(editor.label).toBe('My bookmark');
    });

    it('should allow new bookmark editor state', () => {
      const editor: BookmarkEditor = {
        audiobookId: 1,
        filePath: '/audiobooks/test-book/chapter1.mp3',
        positionMs: 0,
        label: '',
        note: '',
      };

      expect(editor.id).toBeUndefined();
    });
  });

  describe('Union types', () => {
    it('should have valid PlaybackStatus values', () => {
      const statuses: PlaybackStatus[] = ['Playing', 'Paused', 'Stopped', 'Ended'];
      expect(statuses).toHaveLength(4);
    });

    it('should have valid ThemeMode values', () => {
      const themes: ThemeMode[] = ['Light', 'Dark'];
      expect(themes).toHaveLength(2);
    });

    it('should have valid ActiveModal values', () => {
      const modals: ActiveModal[] = ['None', 'Settings', 'BookmarkEditor', 'KeyboardHelp'];
      expect(modals).toHaveLength(4);
    });

    it('should have valid AudiobookSortOrder values', () => {
      const orders: AudiobookSortOrder[] = ['RecentlyPlayed', 'Title', 'Progress', 'DateAdded'];
      expect(orders).toHaveLength(4);
    });

    it('should have valid AudiobookFilterStatus values', () => {
      const statuses: AudiobookFilterStatus[] = ['All', 'InProgress', 'Complete', 'NotStarted'];
      expect(statuses).toHaveLength(4);
    });

    it('should have valid FocusMode values', () => {
      const modes: FocusMode[] = ['Enabled', 'Disabled'];
      expect(modes).toHaveLength(2);
    });

    it('should have valid VolumeExpansion values', () => {
      const states: VolumeExpansion[] = ['Expanded', 'Collapsed'];
      expect(states).toHaveLength(2);
    });

    it('should have valid LoadState values', () => {
      const states: LoadState[] = ['Loading', 'Ready'];
      expect(states).toHaveLength(2);
    });

    it('should have valid NotificationKind values', () => {
      const kinds: NotificationKind[] = ['Success', 'Info', 'Warning', 'Error'];
      expect(kinds).toHaveLength(4);
    });
  });
});
