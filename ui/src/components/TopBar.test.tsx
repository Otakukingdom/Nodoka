/**
 * Unit tests for TopBar component
 */

import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Provider } from 'react-redux';
import { configureStore } from '@reduxjs/toolkit';
import TopBar from './TopBar';
import appReducer from '../store/slices/appSlice';
import playerReducer from '../store/slices/playerSlice';
import libraryReducer from '../store/slices/librarySlice';

const createTestStore = () =>
  configureStore({
    reducer: {
      app: appReducer,
      player: playerReducer,
      library: libraryReducer,
    },
  });

describe('TopBar', () => {
  it('renders the app title', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    expect(screen.getByText('Nodoka')).toBeInTheDocument();
    expect(screen.getByText('Audiobook Reader')).toBeInTheDocument();
  });

  it('renders settings button', () => {
    const store = createTestStore();
    render(
      <Provider store={store}>
        <TopBar />
      </Provider>
    );
    expect(screen.getByRole('button', { name: 'Settings' })).toBeInTheDocument();
  });
});
