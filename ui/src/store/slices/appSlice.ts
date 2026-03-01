/**
 * Redux slice for app-wide UI state
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import type {
  ThemeMode,
  ActiveModal,
  LoadState,
  Notification,
  BookmarkEditor,
} from '../types';

interface AppState {
  // UI state
  theme: ThemeMode;
  activeModal: ActiveModal;
  panelRatio: number;
  panelDividerDragging: boolean;
  focusMode: 'Enabled' | 'Disabled';

  // Loading states
  isLoading: boolean;
  loadState: LoadState;
  operationInProgress: boolean;

  // Notifications
  notifications: Notification[];

  // Bookmark editor
  bookmarkEditor?: BookmarkEditor;

  // Preferences
  prefersReducedMotion: boolean;
}

const initialState: AppState = {
  theme: 'Light',
  activeModal: 'None',
  panelRatio: 0.4,
  panelDividerDragging: false,
  focusMode: 'Disabled',
  isLoading: true,
  loadState: 'Loading',
  operationInProgress: false,
  notifications: [],
  bookmarkEditor: undefined,
  prefersReducedMotion: false,
};

const appSlice = createSlice({
  name: 'app',
  initialState,
  reducers: {
    setTheme: (state, action: PayloadAction<ThemeMode>) => {
      state.theme = action.payload;
    },
    openModal: (state, action: PayloadAction<ActiveModal>) => {
      state.activeModal = action.payload;
    },
    closeModal: (state) => {
      state.activeModal = 'None';
      state.bookmarkEditor = undefined;
    },
    setPanelRatio: (state, action: PayloadAction<number>) => {
      state.panelRatio = Math.max(0.2, Math.min(0.8, action.payload));
    },
    setPanelDividerDragging: (state, action: PayloadAction<boolean>) => {
      state.panelDividerDragging = action.payload;
    },
    setFocusMode: (state, action: PayloadAction<'Enabled' | 'Disabled'>) => {
      state.focusMode = action.payload;
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.isLoading = action.payload;
      state.loadState = action.payload ? 'Loading' : 'Ready';
    },
    setOperationInProgress: (state, action: PayloadAction<boolean>) => {
      state.operationInProgress = action.payload;
    },
    addNotification: (state, action: PayloadAction<Notification>) => {
      state.notifications.push(action.payload);
      if (state.notifications.length > 3) {
        state.notifications.shift();
      }
    },
    removeNotification: (state, action: PayloadAction<number>) => {
      state.notifications = state.notifications.filter(n => n.id !== action.payload);
    },
    setBookmarkEditor: (state, action: PayloadAction<BookmarkEditor | undefined>) => {
      state.bookmarkEditor = action.payload;
    },
    setPrefersReducedMotion: (state, action: PayloadAction<boolean>) => {
      state.prefersReducedMotion = action.payload;
    },
  },
});

export const {
  setTheme,
  openModal,
  closeModal,
  setPanelRatio,
  setPanelDividerDragging,
  setFocusMode,
  setLoading,
  setOperationInProgress,
  addNotification,
  removeNotification,
  setBookmarkEditor,
  setPrefersReducedMotion,
} = appSlice.actions;

export default appSlice.reducer;
