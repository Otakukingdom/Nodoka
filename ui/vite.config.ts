/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

// Check if running in test mode or standalone web mode
const isTestMode = process.env.VITEST === 'true' || process.env.PLAYWRIGHT_TEST === 'true';
const isTauriDev = process.env.TAURI_DEV === 'true';

export default defineConfig(async () => {
  return {
    plugins: [
      react(),
      // Only start Tauri in dev mode when explicitly requested
      ...(isTestMode || !isTauriDev ? [] : [{
        name: 'tauri-dev',
        configureServer(server: { httpServer?: { once: (event: string, callback: () => void) => void } }) {
          server.httpServer?.once('listening', () => {
            execAsync('cargo tauri dev').catch((err: Error) => {
              console.error('Failed to start Tauri dev:', err);
            });
          });
        }
      }])
    ],
    clearScreen: false,
    server: {
      port: 5173,
      strictPort: true,
      watch: {
        ignored: ['**/src-tauri/**']
      }
    },
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
      target: ['es2021', 'chrome100', 'safari13'],
      minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
      sourcemap: !!process.env.TAURI_DEBUG
    },
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: ['./src/test/setup.ts'],
      include: ['src/**/*.test.{ts,tsx}'],
      coverage: {
        provider: 'v8',
        reporter: ['text', 'json', 'html'],
        include: ['src/**/*.{ts,tsx}'],
        exclude: ['src/**/*.test.{ts,tsx}', 'src/main.tsx']
      }
    }
  };
});
