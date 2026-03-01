/**
 * Unit tests for EventBus
 *
 * Tests event subscription, emission, and handler management
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { EventBus, eventBus, EventType } from './EventBus';

describe('EventBus', () => {
  let bus: EventBus;

  beforeEach(() => {
    bus = new EventBus();
  });

  describe('subscription', () => {
    it('should subscribe to an event', () => {
      const handler = vi.fn();
      bus.on('PlayPause', handler);

      bus.emit('PlayPause');
      expect(handler).toHaveBeenCalledTimes(1);
    });

    it('should subscribe multiple handlers to the same event', async () => {
      const handler1 = vi.fn();
      const handler2 = vi.fn();

      bus.on('PlayPause', handler1);
      bus.on('PlayPause', handler2);

      await bus.emit('PlayPause');
      expect(handler1).toHaveBeenCalledTimes(1);
      expect(handler2).toHaveBeenCalledTimes(1);
    });

    it('should return unsubscribe function', () => {
      const handler = vi.fn();
      const unsubscribe = bus.on('PlayPause', handler);

      bus.emit('PlayPause');
      expect(handler).toHaveBeenCalledTimes(1);

      unsubscribe();

      bus.emit('PlayPause');
      expect(handler).toHaveBeenCalledTimes(1); // Not called again
    });
  });

  describe('emission', () => {
    it('should emit event without payload', async () => {
      const handler = vi.fn();
      bus.on('PlayPause', handler);

      await bus.emit('PlayPause');
      expect(handler).toHaveBeenCalledWith(undefined);
    });

    it('should emit event with payload', async () => {
      const handler = vi.fn();
      bus.on('SeekTo', handler);

      await bus.emit('SeekTo', { positionMs: 5000 });
      expect(handler).toHaveBeenCalledWith({ positionMs: 5000 });
    });

    it('should not throw when emitting event with no handlers', async () => {
      await expect(bus.emit('PlayPause')).resolves.not.toThrow();
    });
  });

  describe('event types', () => {
    it('should handle player events', async () => {
      const events: EventType[] = ['PlayPause', 'Stop', 'SeekTo', 'VolumeChanged', 'SpeedChanged', 'PlayerTimeUpdated'];

      for (const event of events) {
        const handler = vi.fn();
        bus.on(event, handler);
        // Emit with appropriate payload type
        if (event === 'SeekTo') {
          await bus.emit(event, { positionMs: 1000 });
        } else if (event === 'VolumeChanged') {
          await bus.emit(event, { volume: 50 });
        } else if (event === 'SpeedChanged') {
          await bus.emit(event, { speed: 1.5 });
        } else if (event === 'PlayerTimeUpdated') {
          await bus.emit(event, { timeMs: 5000 });
        } else {
          await bus.emit(event);
        }
        expect(handler).toHaveBeenCalled();
      }
    });

    it('should handle navigation events', async () => {
      const events: EventType[] = ['AudiobookSelected', 'FileSelected', 'NextFile', 'PreviousFile', 'SeekForward', 'SeekBackward'];

      for (const event of events) {
        const handler = vi.fn();
        bus.on(event, handler);
        // Emit with appropriate payload type
        if (event === 'AudiobookSelected') {
          await bus.emit(event, { id: 1 });
        } else if (event === 'FileSelected') {
          await bus.emit(event, { path: '/test/file.mp3' });
        } else if (event === 'SeekForward' || event === 'SeekBackward') {
          await bus.emit(event, { seconds: 5 });
        } else {
          await bus.emit(event);
        }
        expect(handler).toHaveBeenCalled();
      }
    });

    it('should handle bookmark events', async () => {
      const events: EventType[] = ['CreateBookmark', 'BookmarkEdit', 'BookmarkDelete', 'BookmarkJump', 'BookmarkSave', 'BookmarkCancel'];

      for (const event of events) {
        const handler = vi.fn();
        bus.on(event, handler);
        // Emit with appropriate payload type
        if (event === 'BookmarkEdit' || event === 'BookmarkDelete' || event === 'BookmarkJump') {
          await bus.emit(event, { id: 1 });
        } else {
          await bus.emit(event);
        }
        expect(handler).toHaveBeenCalled();
      }
    });

    it('should handle library events', async () => {
      const events: EventType[] = ['DirectoryAdd', 'DirectoryRemove', 'DirectoryRescan', 'SearchQueryChanged', 'SortOrderChanged', 'FilterStatusChanged'];

      for (const event of events) {
        const handler = vi.fn();
        bus.on(event, handler);
        // Emit with appropriate payload type
        if (event === 'DirectoryRemove' || event === 'DirectoryRescan') {
          await bus.emit(event, { path: '/test/path' });
        } else if (event === 'SearchQueryChanged') {
          await bus.emit(event, { query: 'test' });
        } else if (event === 'SortOrderChanged') {
          await bus.emit(event, { order: 'Title' });
        } else if (event === 'FilterStatusChanged') {
          await bus.emit(event, { status: 'InProgress' });
        } else {
          await bus.emit(event);
        }
        expect(handler).toHaveBeenCalled();
      }
    });

    it('should handle UI events', async () => {
      const events: EventType[] = ['OpenSettings', 'CloseSettings', 'ThemeChanged', 'PanelRatioChanged', 'ToggleFocusMode', 'ShowKeyboardHelp', 'HideKeyboardHelp', 'CloseModal'];

      for (const event of events) {
        const handler = vi.fn();
        bus.on(event, handler);
        // Emit with appropriate payload type
        if (event === 'ThemeChanged') {
          await bus.emit(event, { theme: 'Dark' });
        } else if (event === 'PanelRatioChanged') {
          await bus.emit(event, { ratio: 0.5 });
        } else {
          await bus.emit(event);
        }
        expect(handler).toHaveBeenCalled();
      }
    });
  });

  describe('async handlers', () => {
    it('should handle async event handlers', async () => {
      const results: number[] = [];

      bus.on('PlayPause', async () => {
        await new Promise(resolve => setTimeout(resolve, 10));
        results.push(1);
      });

      bus.on('PlayPause', async () => {
        results.push(2);
      });

      await bus.emit('PlayPause');

      expect(results).toEqual([1, 2]);
    });
  });

  describe('global instance', () => {
    it('should export a global eventBus instance', () => {
      expect(eventBus).toBeInstanceOf(EventBus);
    });
  });
});
