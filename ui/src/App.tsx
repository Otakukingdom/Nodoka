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
import { loadInitialData } from './store/thunks/libraryThunks';
import { updatePlayerTime, playPause } from './store/thunks/playerThunks';
import { openModal, closeModal } from './store/slices/appSlice';

function App() {
  const dispatch = useAppDispatch();
  const { theme, isLoading } = useAppSelector(state => state.app);

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
  }, [dispatch]);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyboard);
    return () => document.removeEventListener('keydown', handleKeyboard);
  }, [handleKeyboard]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <span className="loading loading-spinner loading-lg"></span>
      </div>
    );
  }

  return (
    <div className={`flex flex-col h-screen ${theme === 'Dark' ? 'dark' : ''}`}>
      <TopBar />
      <main className="flex flex-1 overflow-hidden">
        <LibraryPanel />
        <div className="w-1 bg-base-300 cursor-col-resize hover:bg-primary/50 transition-colors" />
        <DetailsPanel />
      </main>
      <PlayerControls />
      <ModalContainer />
      <NotificationContainer />
    </div>
  );
}

export default App;
