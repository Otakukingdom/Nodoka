/// <reference types="vitest" />
import { defineConfig } from 'vite';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export default defineConfig(async () => {
  return {
    plugins: [
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
      include: ['src/**/*.test.ts'],
      coverage: {
        provider: 'v8',
        reporter: ['text', 'json', 'html'],
        include: ['src/**/*.ts'],
        exclude: ['src/**/*.test.ts', 'src/main.ts']
      }
    }
  };
});
