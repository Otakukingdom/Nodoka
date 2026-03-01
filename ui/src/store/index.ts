/**
 * Redux store configuration
 */

import { configureStore } from '@reduxjs/toolkit';
import appReducer from './slices/appSlice';
import playerReducer from './slices/playerSlice';
import libraryReducer from './slices/librarySlice';

export const store = configureStore({
  reducer: {
    app: appReducer,
    player: playerReducer,
    library: libraryReducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      // Allow non-serializable values in thunks for Tauri commands
      serializableCheck: {
        ignoredActions: ['player/setPlayback'],
      },
    }),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
