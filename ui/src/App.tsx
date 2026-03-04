/**
 * Main application component
 */

import { useEffect, useCallback, useRef } from 'react';
import { useAppSelector, useAppDispatch } from './hooks/useRedux';
import {
  TopBar,
  LibraryPanel,
  DetailsPanel,
  PlayerControls,
  ModalContainer,
  NotificationContainer,
  DragDropOverlay,
} from './components';
import { loadInitialData, selectFile, createBookmark, addDirectory, rescanDirectory } from './store/thunks/libraryThunks';
import { updatePlayerTime, playPause, seekForward, seekBackward, updateVolume } from './store/thunks/playerThunks';
import {
  openModal,
  closeModal,
  setPanelRatio,
  setPanelDividerDragging,
} from './store/slices/appSlice';
import { toggleMute, setVolume } from './store/slices/playerSlice';

function App() {
  const dispatch = useAppDispatch();
  const { theme, isLoading, panelRatio, panelDividerDragging } = useAppSelector(state => state.app);
  const { currentFiles, selectedFile, isLibraryLoading, isAddingDirectory, audiobooks, selectedAudiobook, directories } = useAppSelector(state => state.library);
  const { volume } = useAppSelector(state => state.player);
  const mainRef = useRef<HTMLElement>(null);

  useEffect(() => {
    void dispatch(loadInitialData());
  }, [dispatch]);

  // Drag-and-drop handler for adding directories
  const handleDragDrop = useCallback((path: string) => {
    void dispatch(addDirectory(path));
  }, [dispatch]);

  // Apply theme to HTML element for DaisyUI
  useEffect(() => {
    const applyTheme = (themeValue: string) => {
      document.documentElement.setAttribute('data-theme', themeValue.toLowerCase());
    };

    if (theme === 'System') {
      // Detect system preference
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      applyTheme(mediaQuery.matches ? 'dark' : 'light');

      // Listen for system theme changes
      const handler = (e: MediaQueryListEvent) => {
        applyTheme(e.matches ? 'dark' : 'light');
      };
      mediaQuery.addEventListener('change', handler);
      return () => mediaQuery.removeEventListener('change', handler);
    }

    applyTheme(theme);
    return undefined;
  }, [theme]);

  // Player time updates
  useEffect(() => {
    const interval = window.setInterval(() => {
      void dispatch(updatePlayerTime());
    }, 1000);

    return () => clearInterval(interval);
  }, [dispatch]);

  // Panel resize handlers
  const handleDividerMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    dispatch(setPanelDividerDragging(true));
  }, [dispatch]);

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!panelDividerDragging || !mainRef.current) return;
    const rect = mainRef.current.getBoundingClientRect();
    const newRatio = (e.clientX - rect.left) / rect.width;
    dispatch(setPanelRatio(newRatio));
  }, [panelDividerDragging, dispatch]);

  const handleMouseUp = useCallback(() => {
    if (panelDividerDragging) {
      dispatch(setPanelDividerDragging(false));
    }
  }, [panelDividerDragging, dispatch]);

  const handleDividerDoubleClick = useCallback(() => {
    dispatch(setPanelRatio(0.4)); // Reset to default
  }, [dispatch]);

  useEffect(() => {
    if (panelDividerDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = 'col-resize';
      document.body.style.userSelect = 'none';
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      };
    }
    return undefined;
  }, [panelDividerDragging, handleMouseMove, handleMouseUp]);

  // Keyboard shortcuts
  const handleKeyboard = useCallback((e: KeyboardEvent) => {
    // Ignore if typing in an input
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    switch (e.key) {
      case ' ':
        e.preventDefault();
        void dispatch(playPause());
        break;
      case 'Escape':
        dispatch(closeModal());
        break;
      case 'F1':
        e.preventDefault();
        dispatch(openModal('KeyboardHelp'));
        break;
      case 'ArrowLeft':
        e.preventDefault();
        void dispatch(seekBackward(30));
        break;
      case 'ArrowRight':
        e.preventDefault();
        void dispatch(seekForward(30));
        break;
      case 'ArrowUp': {
        e.preventDefault();
        const currentIndex = currentFiles.findIndex(f => f.fullPath === selectedFile);
        const prevFile = currentFiles[currentIndex - 1];
        if (currentIndex > 0 && prevFile) {
          void dispatch(selectFile(prevFile.fullPath));
        }
        break;
      }
      case 'ArrowDown': {
        e.preventDefault();
        const currentIndex = currentFiles.findIndex(f => f.fullPath === selectedFile);
        const nextFile = currentFiles[currentIndex + 1];
        if (currentIndex < currentFiles.length - 1 && nextFile) {
          void dispatch(selectFile(nextFile.fullPath));
        }
        break;
      }
      case 'm':
      case 'M':
        e.preventDefault();
        dispatch(toggleMute());
        break;
      case '+':
      case '=':
        e.preventDefault();
        dispatch(setVolume(Math.min(100, volume + 5)));
        void dispatch(updateVolume(Math.min(100, volume + 5)));
        break;
      case '-':
      case '_':
        e.preventDefault();
        dispatch(setVolume(Math.max(0, volume - 5)));
        void dispatch(updateVolume(Math.max(0, volume - 5)));
        break;
    }

    // Ctrl/Cmd + B for bookmark
    if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
      e.preventDefault();
      void dispatch(createBookmark());
    }

    // Ctrl/Cmd + , for settings
    if ((e.ctrlKey || e.metaKey) && e.key === ',') {
      e.preventDefault();
      dispatch(openModal('Settings'));
    }

    // Ctrl/Cmd + T for sleep timer
    if ((e.ctrlKey || e.metaKey) && e.key === 't') {
      e.preventDefault();
      dispatch(openModal('SleepTimer'));
    }

    // Ctrl/Cmd + D for add directory
    if ((e.ctrlKey || e.metaKey) && e.key === 'd') {
      e.preventDefault();
      void (async () => {
        if (isAddingDirectory || isLibraryLoading) return;
        try {
          const { open } = await import('@tauri-apps/plugin-dialog');
          const selected = await open({
            directory: true,
            multiple: false,
            title: 'Select Audiobook Directory',
          });
          if (selected) {
            await dispatch(addDirectory(selected));
          }
        } catch (error) {
          console.error('Failed to add directory:', error);
        }
      })();
    }

    // Ctrl/Cmd + R for rescan current audiobook's directory
    if ((e.ctrlKey || e.metaKey) && e.key === 'r' && !e.shiftKey) {
      e.preventDefault();
      if (selectedAudiobook !== undefined) {
        const selectedBook = audiobooks.find(b => b.id === selectedAudiobook);
        if (selectedBook?.directory) {
          void dispatch(rescanDirectory(selectedBook.directory));
        }
      }
    }

    // Ctrl/Cmd + Shift + R for rescan all directories
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'R') {
      e.preventDefault();
      // Rescan all directories that don't have errors
      for (const dir of directories) {
        if (!dir.error) {
          void dispatch(rescanDirectory(dir.fullPath));
        }
      }
    }
  }, [dispatch, currentFiles, selectedFile, volume, isAddingDirectory, isLibraryLoading, selectedAudiobook, audiobooks, directories]);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyboard);
    return () => document.removeEventListener('keydown', handleKeyboard);
  }, [handleKeyboard]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen bg-base-100" data-testid="app-container">
        <div className="flex flex-col items-center gap-4">
          <div
            className="w-12 h-12 rounded-full gradient-premium animate-spin"
            style={{ animationDuration: '1.5s' }}
          />
          <span className="text-sm text-base-content/60">Loading...</span>
        </div>
      </div>
    );
  }

  return (
    <div className={`flex flex-col h-screen ${theme === 'Dark' ? 'dark' : ''}`} data-testid="app-container">
      <TopBar />
      <main className="flex flex-1 overflow-hidden" role="main" ref={mainRef}>
        <div style={{ width: `${panelRatio * 100}%` }} className="flex-shrink-0 min-w-[180px] max-w-[400px] sm:min-w-[220px] sm:max-w-none">
          <LibraryPanel />
        </div>
        <div
          className={`w-1.5 bg-base-300 cursor-col-resize hover:bg-primary/50 active:bg-primary/70 transition-all duration-200 hover:shadow-[2px_0_8px_rgba(225,29,72,0.2)] flex-shrink-0 touch-manipulation ${
            panelDividerDragging ? 'bg-primary/70 shadow-[2px_0_12px_rgba(225,29,72,0.3)]' : ''
          }`}
          onMouseDown={handleDividerMouseDown}
          onDoubleClick={handleDividerDoubleClick}
          role="separator"
          aria-orientation="vertical"
          aria-label="Resize panels"
          aria-valuenow={Math.round(panelRatio * 100)}
          tabIndex={0}
          onKeyDown={(e) => {
            if (e.key === 'ArrowLeft') {
              dispatch(setPanelRatio(Math.max(0.2, panelRatio - 0.05)));
            } else if (e.key === 'ArrowRight') {
              dispatch(setPanelRatio(Math.min(0.8, panelRatio + 0.05)));
            }
          }}
        />
        <div className="flex-1 min-w-[240px] sm:min-w-[300px]">
          <DetailsPanel />
        </div>
      </main>
      <PlayerControls />
      <ModalContainer />
      <NotificationContainer />
      <DragDropOverlay
        onDrop={handleDragDrop}
        isDisabled={isAddingDirectory || isLibraryLoading}
      />
    </div>
  );
}

export default App;
