/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export default defineConfig(async () => {
  return {
    plugins: [
      react(),
      {
        name: 'tauri-dev',
        configureServer(server) {
          server.httpServer?.once('listening', () => {
            execAsync('cargo tauri dev').catch((err) => {
              console.error('Failed to start Tauri dev:', err);
            });
          });
        }
      }
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
