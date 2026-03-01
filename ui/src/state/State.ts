/**
 * Frontend state manager - mirrors Rust State struct
 * Uses a simple pub/sub pattern for reactivity
 */

import type {
  Audiobook,
  AudiobookFile,
  Bookmark,
  Directory,
  BookmarkEditor,
  Notification,
  PlaybackStatus,
  ThemeMode,
  ActiveModal,
  AudiobookSortOrder,
  AudiobookFilterStatus,
  FocusMode,
  VolumeExpansion,
  LoadState,
} from './types';

type Listener = () => void;

export class AppState {
  private listeners: Set<Listener> = new Set();

  // Audiobook library
  public audiobooks: Audiobook[] = [];
  public selectedAudiobook?: number;
  public currentFiles: AudiobookFile[] = [];
  public selectedFile?: string;
  public directories: Directory[] = [];

  // Search and filter
  public searchQuery = '';
  public sortOrder: AudiobookSortOrder = 'RecentlyPlayed';
  public filterStatus: AudiobookFilterStatus = 'All';

  // Playback
  public playback: PlaybackStatus = 'Paused';
  public currentTime = 0;
  public totalDuration = 0;
  public volume = 100;
  public volumeExpanded: VolumeExpansion = 'Collapsed';
  public speed = 1.0;

  // UI state
  public theme: ThemeMode = 'Light';
  public activeModal: ActiveModal = 'None';
  public panelRatio = 0.4;
  public panelDividerDragging = false;
  public focusMode: FocusMode = 'Disabled';
  public windowWidth = 1280;
  public windowHeight = 800;

  // Bookmarks
  public bookmarks: Bookmark[] = [];
  public bookmarkEditor?: BookmarkEditor;

  // Loading states
  public isLoading = true;
  public loadState: LoadState = 'Loading';
  public operationInProgress = false;

  // Notifications
  public notifications: Notification[] = [];

  // Preferences
  public prefersReducedMotion = false;

  // Subscribe to state changes
  public subscribe(listener: Listener): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  // Notify all listeners of state change
  private notify(): void {
    this.listeners.forEach(listener => listener());
  }

  // Update methods that trigger re-renders
  public setAudiobooks(audiobooks: Audiobook[]): void {
    this.audiobooks = audiobooks;
    this.notify();
  }

  public setSelectedAudiobook(id: number | undefined): void {
    this.selectedAudiobook = id;
    this.notify();
  }

  public setCurrentFiles(files: AudiobookFile[]): void {
    this.currentFiles = files;
    this.notify();
  }

  public setSelectedFile(path: string | undefined): void {
    this.selectedFile = path;
    this.notify();
  }

  public setPlayback(status: PlaybackStatus): void {
    this.playback = status;
    this.notify();
  }

  public setCurrentTime(time: number): void {
    this.currentTime = time;
    this.notify();
  }

  public setTotalDuration(duration: number): void {
    this.totalDuration = duration;
    this.notify();
  }

  public setVolume(volume: number): void {
    this.volume = Math.max(0, Math.min(200, volume));
    this.notify();
  }

  public setVolumeExpanded(expanded: VolumeExpansion): void {
    this.volumeExpanded = expanded;
    this.notify();
  }

  public setSpeed(speed: number): void {
    this.speed = Math.max(0.5, Math.min(2.0, speed));
    this.notify();
  }

  public setDirectories(directories: Directory[]): void {
    this.directories = directories;
    this.notify();
  }

  public setSearchQuery(query: string): void {
    this.searchQuery = query;
    this.notify();
  }

  public setSortOrder(order: AudiobookSortOrder): void {
    this.sortOrder = order;
    this.notify();
  }

  public setFilterStatus(status: AudiobookFilterStatus): void {
    this.filterStatus = status;
    this.notify();
  }

  public setTheme(theme: ThemeMode): void {
    this.theme = theme;
    this.notify();
  }

  public setFocusMode(mode: FocusMode): void {
    this.focusMode = mode;
    this.notify();
  }

  public setPanelRatio(ratio: number): void {
    this.panelRatio = Math.max(0.2, Math.min(0.8, ratio));
    this.notify();
  }

  public setBookmarks(bookmarks: Bookmark[]): void {
    this.bookmarks = bookmarks;
    this.notify();
  }

  public setBookmarkEditor(editor: BookmarkEditor | undefined): void {
    this.bookmarkEditor = editor;
    this.notify();
  }

  public addNotification(notification: Notification): void {
    this.notifications.push(notification);
    if (this.notifications.length > 3) {
      this.notifications.shift();
    }
    this.notify();
  }

  public removeNotification(id: number): void {
    this.notifications = this.notifications.filter(n => n.id !== id);
    this.notify();
  }

  public openModal(modal: ActiveModal): void {
    this.activeModal = modal;
    this.notify();
  }

  public closeModal(): void {
    this.activeModal = 'None';
    this.bookmarkEditor = undefined;
    this.notify();
  }

  public setLoading(loading: boolean): void {
    this.isLoading = loading;
    this.loadState = loading ? 'Loading' : 'Ready';
    this.notify();
  }

  public setOperationInProgress(inProgress: boolean): void {
    this.operationInProgress = inProgress;
    this.notify();
  }
}

// Global state instance
export const appState = new AppState();
