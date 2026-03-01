/**
 * Main application component
 * Handles layout, modals, and coordinates all child components
 */

import { Component } from './components/Component.js';
import { appState } from './state/State.js';
import { eventBus } from './events/EventBus.js';
import * as TauriCommands from './commands/tauri.js';
import type { Audiobook, AudiobookFile, Bookmark } from './state/types.js';

export class App extends Component {
  private playerInterval: number | null = null;
  private unsubscribeFunctions: Array<() => void> = [];

  public render(): HTMLElement {
    const app = document.createElement('div');
    app.className = `flex flex-col h-screen ${appState.theme === 'Dark' ? 'dark' : ''}`;

    // Top bar
    const topBar = this.renderTopBar();
    app.appendChild(topBar);

    // Main content area with two panels
    const mainContent = document.createElement('div');
    mainContent.className = 'flex flex-1 overflow-hidden';

    // Left panel (library)
    const leftPanel = this.renderLeftPanel();
    mainContent.appendChild(leftPanel);

    // Panel divider
    const divider = this.renderDivider();
    mainContent.appendChild(divider);

    // Right panel (details)
    const rightPanel = this.renderRightPanel();
    mainContent.appendChild(rightPanel);

    app.appendChild(mainContent);

    // Player controls at bottom
    const playerControls = this.renderPlayerControls();
    app.appendChild(playerControls);

    // Modal container
    const modalContainer = document.createElement('div');
    modalContainer.id = 'modal-container';
    modalContainer.className = 'hidden';
    app.appendChild(modalContainer);

    // Notification container
    const notificationContainer = document.createElement('div');
    notificationContainer.id = 'notification-container';
    notificationContainer.className = 'toast toast-top toast-end z-50';
    app.appendChild(notificationContainer);

    return app;
  }

  private renderTopBar(): HTMLElement {
    const topBar = document.createElement('div');
    topBar.className = 'navbar bg-base-200 border-b border-base-300 px-4';

    const logo = document.createElement('div');
    logo.className = 'flex items-center gap-2';
    logo.innerHTML = `
      <div class="w-8 h-8 rounded-full bg-gradient-to-br from-rose-500 to-pink-600 flex items-center justify-center text-white text-sm">
        ♪
      </div>
      <div>
        <div class="text-lg font-bold">Nodoka</div>
        <div class="text-xs text-base-content/60">Audiobook Reader</div>
      </div>
    `;

    const spacer = document.createElement('div');
    spacer.className = 'flex-1';

    const settingsBtn = document.createElement('button');
    settingsBtn.className = 'btn btn-ghost btn-circle';
    settingsBtn.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>';
    settingsBtn.onclick = () => { void eventBus.emit('OpenSettings'); };

    topBar.append(logo, spacer, settingsBtn);
    return topBar;
  }

  private renderLeftPanel(): HTMLElement {
    const leftPanel = document.createElement('div');
    leftPanel.className = 'flex flex-col border-r border-base-300 overflow-hidden';
    leftPanel.style.width = `${appState.panelRatio * 100}%`;

    // Toolbar
    const toolbar = document.createElement('div');
    toolbar.className = 'p-2 border-b border-base-300 flex gap-2';

    const searchInput = document.createElement('input');
    searchInput.type = 'text';
    searchInput.placeholder = 'Search audiobooks...';
    searchInput.className = 'input input-bordered input-sm flex-1';
    searchInput.value = appState.searchQuery;
    searchInput.oninput = (e) => {
      void eventBus.emit('SearchQueryChanged', {
        query: (e.target as HTMLInputElement).value
      });
    };

    const sortSelect = document.createElement('select');
    sortSelect.className = 'select select-bordered select-sm';
    sortSelect.innerHTML = `
      <option value="RecentlyPlayed" ${appState.sortOrder === 'RecentlyPlayed' ? 'selected' : ''}>Recent</option>
      <option value="Title" ${appState.sortOrder === 'Title' ? 'selected' : ''}>Title</option>
      <option value="Progress" ${appState.sortOrder === 'Progress' ? 'selected' : ''}>Progress</option>
    `;
    sortSelect.onchange = (e) => {
      void eventBus.emit('SortOrderChanged', {
        order: (e.target as HTMLSelectElement).value as import('./state/types.js').AudiobookSortOrder
      });
    };

    toolbar.append(searchInput, sortSelect);

    // Audiobook list
    const listContainer = document.createElement('div');
    listContainer.className = 'flex-1 overflow-y-auto custom-scrollbar p-2';
    listContainer.id = 'audiobook-list';

    // Loading state
    if (appState.isLoading) {
      listContainer.innerHTML = '<div class="flex items-center justify-center h-full"><span class="loading loading-spinner"></span></div>';
    } else if (appState.audiobooks.length === 0) {
      listContainer.innerHTML = `
        <div class="text-center py-8">
          <div class="text-4xl mb-2">📚</div>
          <div class="text-lg font-medium">No audiobooks found</div>
          <div class="text-sm text-base-content/60">Add a directory to get started</div>
        </div>
      `;
    } else {
      const filteredBooks = this.getFilteredAudiobooks();
      filteredBooks.forEach(book => {
        const card = this.renderAudiobookCard(book);
        listContainer.appendChild(card);
      });
    }

    leftPanel.append(toolbar, listContainer);
    return leftPanel;
  }

  private renderAudiobookCard(book: Audiobook): HTMLElement {
    const card = document.createElement('div');
    const isSelected = appState.selectedAudiobook === book.id;

    card.className = `card card-compact bg-base-100 shadow-sm cursor-pointer transition-smooth mb-2 ${isSelected ? 'ring-2 ring-primary' : 'hover:shadow-md'}`;
    card.onclick = () => {
      if (book.id !== undefined) {
        void eventBus.emit('AudiobookSelected', { id: book.id });
      }
    };

    const cardBody = document.createElement('div');
    cardBody.className = 'card-body p-3';

    const title = document.createElement('h3');
    title.className = 'card-title text-sm font-medium text-truncate';
    title.textContent = book.name;

    const progressContainer = document.createElement('div');
    progressContainer.className = 'flex items-center gap-2';

    const progressBar = document.createElement('progress');
    progressBar.className = 'progress progress-primary w-full h-2';
    progressBar.value = book.completeness;
    progressBar.max = 100;

    const progressText = document.createElement('span');
    progressText.className = 'text-xs text-base-content/70';
    progressText.textContent = `${book.completeness}%`;

    progressContainer.append(progressBar, progressText);
    cardBody.append(title, progressContainer);
    card.appendChild(cardBody);

    return card;
  }

  private renderDivider(): HTMLElement {
    const divider = document.createElement('div');
    divider.className = 'w-1 bg-base-300 cursor-col-resize hover:bg-primary/50 transition-colors';
    divider.id = 'panel-divider';
    return divider;
  }

  private renderRightPanel(): HTMLElement {
    const rightPanel = document.createElement('div');
    rightPanel.className = 'flex flex-col flex-1 overflow-hidden';

    // Files section
    const filesSection = document.createElement('div');
    filesSection.className = 'flex-1 overflow-y-auto custom-scrollbar p-4';

    const filesHeader = document.createElement('h2');
    filesHeader.className = 'text-lg font-bold mb-4';
    filesHeader.textContent = 'Files';

    const filesList = document.createElement('div');
    filesList.className = 'flex flex-col gap-2';
    filesList.id = 'files-list';

    if (appState.currentFiles.length === 0) {
      filesList.innerHTML = '<div class="text-center text-base-content/60 py-8">No files available</div>';
    } else {
      appState.currentFiles.forEach(file => {
        const fileCard = this.renderFileCard(file);
        filesList.appendChild(fileCard);
      });
    }

    filesSection.append(filesHeader, filesList);

    // Bookmarks section
    const bookmarksSection = document.createElement('div');
    bookmarksSection.className = 'h-1/3 overflow-y-auto custom-scrollbar p-4 border-t border-base-300';

    const bookmarksHeader = document.createElement('h2');
    bookmarksHeader.className = 'text-lg font-bold mb-4';
    bookmarksHeader.textContent = 'Bookmarks';

    const bookmarksList = document.createElement('div');
    bookmarksList.className = 'flex flex-col gap-2';
    bookmarksList.id = 'bookmarks-list';

    if (appState.bookmarks.length === 0) {
      bookmarksList.innerHTML = '<div class="text-center text-base-content/60 py-4">No bookmarks</div>';
    } else {
      appState.bookmarks.forEach(bookmark => {
        const bookmarkCard = this.renderBookmarkCard(bookmark);
        bookmarksList.appendChild(bookmarkCard);
      });
    }

    bookmarksSection.append(bookmarksHeader, bookmarksList);
    rightPanel.append(filesSection, bookmarksSection);

    return rightPanel;
  }

  private renderFileCard(file: AudiobookFile): HTMLElement {
    const card = document.createElement('div');
    const isSelected = appState.selectedFile === file.fullPath;

    card.className = `card card-compact bg-base-100 shadow-sm cursor-pointer transition-smooth ${isSelected ? 'ring-1 ring-primary' : 'hover:shadow-md'}`;
    card.onclick = () => {
      void eventBus.emit('FileSelected', { path: file.fullPath });
    };

    const cardBody = document.createElement('div');
    cardBody.className = 'card-body p-2';

    const name = document.createElement('div');
    name.className = 'text-sm';
    name.textContent = `#${file.position + 1} ${file.name}`;

    if (file.completeness > 0) {
      const progress = document.createElement('progress');
      progress.className = 'progress progress-xs w-full mt-1';
      progress.value = file.completeness;
      progress.max = 100;
      cardBody.appendChild(progress);
    }

    cardBody.appendChild(name);
    card.appendChild(cardBody);

    return card;
  }

  private renderBookmarkCard(bookmark: Bookmark): HTMLElement {
    const card = document.createElement('div');
    card.className = 'card card-compact bg-base-100 shadow-sm p-2';

    const content = document.createElement('div');
    content.className = 'flex items-center justify-between';

    const label = document.createElement('span');
    label.className = 'text-sm font-medium';
    label.textContent = bookmark.label;

    const time = document.createElement('span');
    time.className = 'badge badge-ghost text-xs';
    time.textContent = this.formatTime(bookmark.positionMs);

    content.append(label, time);
    card.appendChild(content);

    if (bookmark.note !== undefined) {
      const note = document.createElement('div');
      note.className = 'text-xs text-base-content/60 mt-1';
      note.textContent = bookmark.note;
      card.appendChild(note);
    }

    return card;
  }

  private renderPlayerControls(): HTMLElement {
    const controls = document.createElement('div');
    controls.className = 'btm-nav bg-base-200 border-t border-base-300 px-4 py-3';

    const leftControls = document.createElement('div');
    leftControls.className = 'flex items-center gap-2';

    // Stop button
    const stopBtn = document.createElement('button');
    stopBtn.className = 'btn btn-circle btn-ghost btn-sm';
    stopBtn.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><rect x="6" y="6" width="12" height="12" stroke-width="2"/></svg>';
    stopBtn.onclick = () => { void eventBus.emit('Stop'); };

    // Play/Pause button
    const playPauseBtn = document.createElement('button');
    playPauseBtn.className = 'btn btn-circle btn-primary';
    playPauseBtn.innerHTML = appState.playback === 'Playing'
      ? '<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6" /></svg>'
      : '<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" /></svg>';
    playPauseBtn.onclick = () => { void eventBus.emit('PlayPause'); };

    // Time display and seek
    const timeContainer = document.createElement('div');
    timeContainer.className = 'flex-1 mx-4 flex items-center gap-2';

    const currentTimeEl = document.createElement('span');
    currentTimeEl.className = 'font-mono text-sm w-16 text-right';
    currentTimeEl.textContent = this.formatTime(appState.currentTime);

    const progressSlider = document.createElement('input');
    progressSlider.type = 'range';
    progressSlider.min = '0';
    progressSlider.max = String(appState.totalDuration);
    progressSlider.value = String(appState.currentTime);
    progressSlider.className = 'range range-primary flex-1';
    progressSlider.oninput = (e) => {
      void eventBus.emit('SeekTo', {
        positionMs: Number((e.target as HTMLInputElement).value)
      });
    };

    const totalTimeEl = document.createElement('span');
    totalTimeEl.className = 'font-mono text-sm w-16';
    totalTimeEl.textContent = this.formatTime(appState.totalDuration);

    timeContainer.append(currentTimeEl, progressSlider, totalTimeEl);

    // Speed control
    const speedBtn = document.createElement('button');
    speedBtn.className = 'btn btn-ghost btn-sm';
    speedBtn.textContent = `${appState.speed}x`;
    speedBtn.onclick = () => {
      const newSpeed = appState.speed >= 2.0 ? 0.5 : appState.speed + 0.25;
      void eventBus.emit('SpeedChanged', { speed: newSpeed });
    };

    // Volume control
    const volumeBtn = document.createElement('button');
    volumeBtn.className = 'btn btn-ghost btn-sm btn-circle';
    volumeBtn.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" /></svg>`;
    volumeBtn.onclick = () => { void eventBus.emit('VolumeToggleExpanded'); };

    leftControls.append(stopBtn, playPauseBtn);

    const rightControls = document.createElement('div');
    rightControls.className = 'flex items-center gap-2';
    rightControls.append(speedBtn, volumeBtn);

    controls.append(leftControls, timeContainer, rightControls);
    return controls;
  }

  private formatTime(ms: number): string {
    const totalSeconds = Math.floor(ms / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    if (hours > 0) {
      return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
  }

  private getFilteredAudiobooks(): Audiobook[] {
    let filtered = [...appState.audiobooks];

    // Apply search filter
    if (appState.searchQuery.length > 0) {
      const query = appState.searchQuery.toLowerCase();
      filtered = filtered.filter(book =>
        book.name.toLowerCase().includes(query)
      );
    }

    // Apply status filter
    switch (appState.filterStatus) {
      case 'InProgress':
        filtered = filtered.filter(book => book.completeness > 0 && book.completeness < 100);
        break;
      case 'Complete':
        filtered = filtered.filter(book => book.completeness === 100);
        break;
      case 'NotStarted':
        filtered = filtered.filter(book => book.completeness === 0);
        break;
    }

    // Apply sort order
    switch (appState.sortOrder) {
      case 'Title':
        filtered.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case 'Progress':
        filtered.sort((a, b) => b.completeness - a.completeness);
        break;
      case 'RecentlyPlayed':
        filtered.sort((a, b) => {
          const aTime = a.lastListened !== undefined ? new Date(a.lastListened).getTime() : 0;
          const bTime = b.lastListened !== undefined ? new Date(b.lastListened).getTime() : 0;
          return bTime - aTime;
        });
        break;
    }

    return filtered;
  }

  protected override afterMount(): void {
    // Subscribe to state changes for reactivity
    this.unsubscribeFunctions.push(
      appState.subscribe(() => this.update())
    );

    // Register event handlers
    this.registerEventHandlers();

    // Load initial data
    void this.loadInitialData();

    // Setup keyboard shortcuts
    document.addEventListener('keydown', this.handleKeyboard);

    // Start player time updates
    this.playerInterval = window.setInterval(() => {
      void this.updatePlayerTime();
    }, 1000);

    // Initialize modal listener
    this.unsubscribeFunctions.push(
      appState.subscribe(() => this.handleModalChange())
    );
  }

  protected override beforeUnmount(): void {
    // Clean up subscriptions
    this.unsubscribeFunctions.forEach(fn => fn());
    this.unsubscribeFunctions = [];

    // Remove event listeners
    document.removeEventListener('keydown', this.handleKeyboard);

    // Clear interval
    if (this.playerInterval) {
      clearInterval(this.playerInterval);
      this.playerInterval = null;
    }
  }

  private registerEventHandlers(): void {
    // Player events
    eventBus.on('PlayPause', async () => {
      try {
        const isPlaying = await TauriCommands.playPause();
        appState.setPlayback(isPlaying ? 'Playing' : 'Paused');
      } catch (error) {
        console.error('Play/pause failed:', error);
        appState.addNotification({
          id: Date.now(),
          kind: 'Error',
          message: `Failed to toggle playback: ${error}`,
          durationMs: Infinity,
          createdAt: Date.now(),
        });
      }
    });

    eventBus.on('Stop', async () => {
      try {
        await TauriCommands.stop();
        appState.setPlayback('Stopped');
        appState.setCurrentTime(0);
      } catch (error) {
        console.error('Stop failed:', error);
      }
    });

    eventBus.on('SeekTo', async ({ positionMs }) => {
      try {
        await TauriCommands.seek(positionMs);
        appState.setCurrentTime(positionMs);
      } catch (error) {
        console.error('Seek failed:', error);
      }
    });

    eventBus.on('SpeedChanged', async ({ speed }) => {
      try {
        await TauriCommands.setSpeed(speed);
        appState.setSpeed(speed);
      } catch (error) {
        console.error('Speed change failed:', error);
      }
    });

    // Navigation events
    eventBus.on('AudiobookSelected', async ({ id }) => {
      appState.setSelectedAudiobook(id);
      try {
        const files = await TauriCommands.getAudiobookFiles(id);
        appState.setCurrentFiles(files);
        const bookmarks = await TauriCommands.getBookmarks(id);
        appState.setBookmarks(bookmarks);
      } catch (error) {
        console.error('Failed to load audiobook details:', error);
      }
    });

    eventBus.on('FileSelected', async ({ path }) => {
      appState.setSelectedFile(path);
      try {
        await TauriCommands.loadFile(path);
        appState.setPlayback('Playing');
      } catch (error) {
        console.error('Failed to load file:', error);
        appState.addNotification({
          id: Date.now(),
          kind: 'Error',
          message: `Failed to load file: ${error}`,
          durationMs: Infinity,
          createdAt: Date.now(),
        });
      }
    });

    // Search and filter events
    eventBus.on('SearchQueryChanged', ({ query }) => {
      appState.setSearchQuery(query);
    });

    eventBus.on('SortOrderChanged', ({ order }) => {
      appState.setSortOrder(order);
    });

    // UI events
    eventBus.on('OpenSettings', () => {
      appState.openModal('Settings');
    });

    eventBus.on('CloseSettings', () => {
      appState.closeModal();
    });

    eventBus.on('CloseModal', () => {
      appState.closeModal();
    });

    eventBus.on('ThemeChanged', async ({ theme }) => {
      appState.setTheme(theme);
      try {
        await TauriCommands.setTheme(theme);
      } catch (error) {
        console.error('Failed to save theme:', error);
      }
    });
  }

  private async loadInitialData(): Promise<void> {
    try {
      const [audiobooks, directories, theme, panelRatio, volume, speed] = await Promise.all([
        TauriCommands.getAllAudiobooks(),
        TauriCommands.getDirectories(),
        TauriCommands.getTheme().catch(() => 'Light'),
        TauriCommands.getPanelRatio().catch(() => 0.4),
        TauriCommands.getVolume().catch(() => 100),
        TauriCommands.getSpeed().catch(() => 1.0),
      ]);

      appState.setAudiobooks(audiobooks);
      appState.setDirectories(directories);
      appState.setTheme(theme as import('./state/types.js').ThemeMode);
      appState.setPanelRatio(panelRatio);
      appState.setVolume(volume);
      appState.setSpeed(speed);
      appState.setLoading(false);
    } catch (error) {
      console.error('Failed to load initial data:', error);
      appState.addNotification({
        id: Date.now(),
        kind: 'Error',
        message: `Failed to load library: ${error}`,
        durationMs: Infinity,
        createdAt: Date.now(),
      });
      appState.setLoading(false);
    }
  }

  private async updatePlayerTime(): Promise<void> {
    try {
      const time = await TauriCommands.getCurrentTime();
      const duration = await TauriCommands.getDuration();
      const state = await TauriCommands.getPlaybackState();

      appState.setCurrentTime(time);
      appState.setTotalDuration(duration);

      if (state === 'Playing' && appState.playback !== 'Playing') {
        appState.setPlayback('Playing');
      } else if (state !== 'Playing' && appState.playback === 'Playing') {
        appState.setPlayback('Paused');
      }
    } catch {
      // Player might not be initialized
    }
  }

  private handleModalChange(): void {
    const modalContainer = document.getElementById('modal-container');
    if (!modalContainer) return;

    // Clear existing modal
    modalContainer.innerHTML = '';

    switch (appState.activeModal) {
      case 'Settings':
        modalContainer.className = 'modal modal-open';
        modalContainer.appendChild(this.renderSettingsModal());
        break;
      case 'KeyboardHelp':
        modalContainer.className = 'modal modal-open';
        modalContainer.appendChild(this.renderKeyboardHelpModal());
        break;
      case 'None':
        modalContainer.className = 'hidden';
        break;
    }
  }

  private renderSettingsModal(): HTMLElement {
    const modal = document.createElement('div');
    modal.className = 'modal-box';

    const header = document.createElement('h3');
    header.className = 'font-bold text-lg';
    header.textContent = 'Settings';

    const content = document.createElement('div');
    content.className = 'py-4 space-y-4';

    // Theme selection
    const themeSection = document.createElement('div');
    themeSection.className = 'form-control';
    const themeLabel = document.createElement('label');
    themeLabel.className = 'label';
    const themeSpan = document.createElement('span');
    themeSpan.className = 'label-text';
    themeSpan.textContent = 'Theme';
    themeLabel.appendChild(themeSpan);
    themeSection.appendChild(themeLabel);

    const themeSelect = document.createElement('select');
    themeSelect.className = 'select select-bordered';
    themeSelect.innerHTML = `
      <option value="Light" ${appState.theme === 'Light' ? 'selected' : ''}>Light</option>
      <option value="Dark" ${appState.theme === 'Dark' ? 'selected' : ''}>Dark</option>
    `;
    themeSelect.onchange = (e) => {
      void eventBus.emit('ThemeChanged', {
        theme: (e.target as HTMLSelectElement).value as import('./state/types.js').ThemeMode
      });
    };
    themeSection.appendChild(themeSelect);
    content.appendChild(themeSection);

    // Close button
    const actions = document.createElement('div');
    actions.className = 'modal-action';
    const closeButton = document.createElement('button');
    closeButton.className = 'btn';
    closeButton.textContent = 'Close';
    closeButton.onclick = () => { void eventBus.emit('CloseModal'); };
    actions.appendChild(closeButton);

    modal.append(header, content, actions);
    return modal;
  }

  private renderKeyboardHelpModal(): HTMLElement {
    const modal = document.createElement('div');
    modal.className = 'modal-box';

    const header = document.createElement('h3');
    header.className = 'font-bold text-lg';
    header.textContent = 'Keyboard Shortcuts';

    const content = document.createElement('div');
    content.className = 'py-4';
    content.innerHTML = `
      <table class="table table-zebra">
        <thead><tr><th>Key</th><th>Action</th></tr></thead>
        <tbody>
          <tr><td>Space</td><td>Play/Pause</td></tr>
          <tr><td>←</td><td>Seek backward 30s</td></tr>
          <tr><td>→</td><td>Seek forward 30s</td></tr>
          <tr><td>↑</td><td>Previous file</td></tr>
          <tr><td>↓</td><td>Next file</td></tr>
          <tr><td>Ctrl/Cmd + B</td><td>Create bookmark</td></tr>
          <tr><td>Ctrl/Cmd + ,</td><td>Open settings</td></tr>
          <tr><td>F1</td><td>Show keyboard help</td></tr>
          <tr><td>Escape</td><td>Close modal</td></tr>
        </tbody>
      </table>
    `;

    const actions = document.createElement('div');
    actions.className = 'modal-action';
    const closeButton = document.createElement('button');
    closeButton.className = 'btn';
    closeButton.textContent = 'Close';
    closeButton.onclick = () => { void eventBus.emit('CloseModal'); };
    actions.appendChild(closeButton);

    modal.append(header, content, actions);
    return modal;
  }

  private handleKeyboard = (e: KeyboardEvent): void => {
    // Ignore if typing in an input
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    switch (e.key) {
      case ' ':
        e.preventDefault();
        void eventBus.emit('PlayPause');
        break;
      case 'Escape':
        void eventBus.emit('CloseModal');
        break;
      case 'ArrowLeft':
        e.preventDefault();
        void eventBus.emit('SeekBackward', { seconds: 30 });
        break;
      case 'ArrowRight':
        e.preventDefault();
        void eventBus.emit('SeekForward', { seconds: 30 });
        break;
      case 'ArrowUp':
        e.preventDefault();
        void eventBus.emit('PreviousFile');
        break;
      case 'ArrowDown':
        e.preventDefault();
        void eventBus.emit('NextFile');
        break;
      case 'F1':
        e.preventDefault();
        appState.openModal('KeyboardHelp');
        break;
    }

    // Ctrl/Cmd + B for bookmark
    if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
      e.preventDefault();
      void eventBus.emit('CreateBookmark');
    }

    // Ctrl/Cmd + , for settings
    if ((e.ctrlKey || e.metaKey) && e.key === ',') {
      e.preventDefault();
      void eventBus.emit('OpenSettings');
    }
  };
}
