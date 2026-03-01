/**
 * Main application component
 */

import { useEffect, useCallback } from 'react';
import { useAppSelector, useAppDispatch } from './hooks/useRedux';
import {
  TopBar,
  LibraryPanel,
  DetailsPanel,
  PlayerControls,
  ModalContainer,
  NotificationContainer,
} from './components';
import { loadInitialData, selectFile } from './store/thunks/libraryThunks';
import { updatePlayerTime, playPause, seekForward, seekBackward } from './store/thunks/playerThunks';
import { openModal, closeModal } from './store/slices/appSlice';

function App() {
  const dispatch = useAppDispatch();
  const { theme, isLoading } = useAppSelector(state => state.app);
  const { currentFiles, selectedFile } = useAppSelector(state => state.library);

  useEffect(() => {
    void dispatch(loadInitialData());
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
    }

    // Ctrl/Cmd + B for bookmark
    if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
      e.preventDefault();
      // TODO: Implement create bookmark
    }

    // Ctrl/Cmd + , for settings
    if ((e.ctrlKey || e.metaKey) && e.key === ',') {
      e.preventDefault();
      dispatch(openModal('Settings'));
    }
  }, [dispatch, currentFiles, selectedFile]);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyboard);
    return () => document.removeEventListener('keydown', handleKeyboard);
  }, [handleKeyboard]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen bg-base-100">
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
    <div className={`flex flex-col h-screen ${theme === 'Dark' ? 'dark' : ''}`}>
      <TopBar />
      <main className="flex flex-1 overflow-hidden">
        <LibraryPanel />
        <div className="w-1 bg-base-300 cursor-col-resize hover:bg-primary/40 active:bg-primary/60 transition-all duration-200 hover:shadow-[2px_0_8px_rgba(225,29,72,0.2)]" />
        <DetailsPanel />
      </main>
      <PlayerControls />
      <ModalContainer />
      <NotificationContainer />
    </div>
  );
}

export default App;
