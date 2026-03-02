/**
 * React application entry point
 */

import React from 'react';
import ReactDOM from 'react-dom/client';
import { Provider } from 'react-redux';
import { store } from './store';
import App from './App';
import './styles/main.css';

// Expose Redux store for Playwright e2e testing
if (import.meta.env.VITE_PLAYWRIGHT_TEST === 'true') {
  (window as any).__REDUX_STORE__ = store;
}

const rootElement = document.getElementById('root');
if (!rootElement) {
  throw new Error('Root element not found');
}

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <Provider store={store}>
      <App />
    </Provider>
  </React.StrictMode>
);
