/**
 * Application entry point
 * Initializes the app and mounts it to the DOM
 */

import { App } from './App.js';
import './styles/main.css';

// Initialize the app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  const appContainer = document.getElementById('app');
  if (!appContainer) {
    console.error('App container not found');
    return;
  }

  const app = new App();
  app.mount(appContainer);

  // Setup global error handler
  window.addEventListener('error', (event) => {
    console.error('Global error:', event.error);
  });

  // Setup unhandled rejection handler
  window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
  });

  console.log('Nodoka Audiobook Reader initialized');
});
