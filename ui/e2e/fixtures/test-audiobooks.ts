/**
 * Test Audiobooks Factory for Cucumber UI Acceptance Tests
 *
 * Provides factory functions to create test audiobooks with various
 * configurations for comprehensive UI testing scenarios.
 */

import type { Page } from '@playwright/test';

/**
 * Test audiobook configuration
 */
export interface TestAudiobookConfig {
  id: string;
  title: string;
  author?: string | null;
  fileCount: number;
  totalDuration: number;
  completeness: number;
  hasCover: boolean;
  archiveType?: 'zip' | 'rar' | '7z' | null;
  lastListened?: string | null;
}

/**
 * Test file configuration
 */
export interface TestFileConfig {
  position: number;
  name: string;
  duration: number;
  size: number;
  format: string;
  corrupted?: boolean;
  exists?: boolean;
}

/**
 * Default test audiobook configurations
 */
export const defaultTestAudiobooks: TestAudiobookConfig[] = [
  {
    id: 'default-1',
    title: 'Default Test Audiobook',
    author: 'Test Author',
    fileCount: 3,
    totalDuration: 3600000,
    completeness: 0,
    hasCover: true,
    archiveType: null,
  },
  {
    id: 'default-2',
    title: 'Completed Audiobook',
    author: 'Another Author',
    fileCount: 5,
    totalDuration: 7200000,
    completeness: 100,
    hasCover: true,
    archiveType: null,
    lastListened: new Date(Date.now() - 86400000).toISOString(),
  },
];

/**
 * Create a test audiobook with specified configuration
 */
export function createTestAudiobook(config: Partial<TestAudiobookConfig> = {}): TestAudiobookConfig {
  return {
    id: config.id ?? `test-${Date.now()}`,
    title: config.title ?? 'Test Audiobook',
    author: config.author ?? 'Test Author',
    fileCount: config.fileCount ?? 3,
    totalDuration: config.totalDuration ?? 3600000,
    completeness: config.completeness ?? 0,
    hasCover: config.hasCover ?? true,
    archiveType: config.archiveType ?? null,
    lastListened: config.lastListened ?? null,
  };
}

/**
 * Create test files for an audiobook
 */
export function createTestFiles(count: number, format: string = 'mp3'): TestFileConfig[] {
  const files: TestFileConfig[] = [];
  const durationPerFile = 1200000; // 20 minutes per file

  for (let i = 0; i < count; i++) {
    files.push({
      position: i + 1,
      name: `Chapter ${String(i + 1).padStart(2, '0')}.${format}`,
      duration: durationPerFile,
      size: 15000000 + Math.floor(Math.random() * 5000000),
      format,
      exists: true,
      corrupted: false,
    });
  }

  return files;
}

/**
 * Create test audiobooks for various scenarios
 */
export const testAudiobookScenarios = {
  /**
   * Standard audiobook with MP3 files
   */
  standardMp3: (): TestAudiobookConfig => ({
    id: 'standard-mp3',
    title: 'Standard MP3 Audiobook',
    author: 'MP3 Author',
    fileCount: 5,
    totalDuration: 6000000,
    completeness: 25,
    hasCover: true,
    archiveType: null,
  }),

  /**
   * Audiobook with M4B files (common for audiobooks)
   */
  m4bFormat: (): TestAudiobookConfig => ({
    id: 'm4b-format',
    title: 'M4B Audiobook',
    author: 'M4B Author',
    fileCount: 1,
    totalDuration: 7200000,
    completeness: 0,
    hasCover: true,
    archiveType: null,
  }),

  /**
   * Audiobook with FLAC files (high quality)
   */
  flacFormat: (): TestAudiobookConfig => ({
    id: 'flac-format',
    title: 'FLAC High Quality Audiobook',
    author: 'FLAC Author',
    fileCount: 3,
    totalDuration: 5400000,
    completeness: 0,
    hasCover: true,
    archiveType: null,
  }),

  /**
   * Audiobook in a ZIP archive
   */
  zipArchive: (): TestAudiobookConfig => ({
    id: 'zip-archive',
    title: 'ZIP Archive Audiobook',
    author: 'Archive Author',
    fileCount: 10,
    totalDuration: 9000000,
    completeness: 0,
    hasCover: true,
    archiveType: 'zip',
  }),

  /**
   * Audiobook in a RAR archive
   */
  rarArchive: (): TestAudiobookConfig => ({
    id: 'rar-archive',
    title: 'RAR Archive Audiobook',
    author: 'Archive Author',
    fileCount: 8,
    totalDuration: 8000000,
    completeness: 0,
    hasCover: true,
    archiveType: 'rar',
  }),

  /**
   * Audiobook in a 7z archive
   */
  sevenZipArchive: (): TestAudiobookConfig => ({
    id: '7z-archive',
    title: '7z Archive Audiobook',
    author: 'Archive Author',
    fileCount: 6,
    totalDuration: 7200000,
    completeness: 0,
    hasCover: true,
    archiveType: '7z',
  }),

  /**
   * Audiobook with corrupted audio file
   */
  corruptedAudio: (): TestAudiobookConfig => ({
    id: 'corrupted-audio',
    title: 'Corrupted Audio Audiobook',
    author: 'Test Author',
    fileCount: 1,
    totalDuration: 1800000,
    completeness: 0,
    hasCover: false,
    archiveType: null,
  }),

  /**
   * Audiobook with unsupported file format
   */
  unsupportedFormat: (): TestAudiobookConfig => ({
    id: 'unsupported-format',
    title: 'Unsupported Format Audiobook',
    author: 'Test Author',
    fileCount: 2,
    totalDuration: 3600000,
    completeness: 0,
    hasCover: false,
    archiveType: null,
  }),

  /**
   * Audiobook with very long duration (over 24 hours)
   */
  longDuration: (): TestAudiobookConfig => ({
    id: 'long-duration',
    title: 'Very Long Audiobook',
    author: 'Long Author',
    fileCount: 50,
    totalDuration: 90000000, // 25 hours
    completeness: 0,
    hasCover: true,
    archiveType: null,
  }),

  /**
   * Completed audiobook (100% progress)
   */
  completed: (): TestAudiobookConfig => ({
    id: 'completed',
    title: 'Completed Audiobook',
    author: 'Finished Author',
    fileCount: 10,
    totalDuration: 12000000,
    completeness: 100,
    hasCover: true,
    archiveType: null,
    lastListened: new Date().toISOString(),
  }),

  /**
   * In-progress audiobook (50% progress)
   */
  inProgress: (): TestAudiobookConfig => ({
    id: 'in-progress',
    title: 'In Progress Audiobook',
    author: 'Reading Author',
    fileCount: 4,
    totalDuration: 4800000,
    completeness: 50,
    hasCover: true,
    archiveType: null,
    lastListened: new Date(Date.now() - 3600000).toISOString(),
  }),

  /**
   * Audiobook without author
   */
  noAuthor: (): TestAudiobookConfig => ({
    id: 'no-author',
    title: 'Unknown Author Audiobook',
    author: null,
    fileCount: 3,
    totalDuration: 3600000,
    completeness: 0,
    hasCover: true,
    archiveType: null,
  }),

  /**
   * Audiobook without cover art
   */
  noCover: (): TestAudiobookConfig => ({
    id: 'no-cover',
    title: 'No Cover Audiobook',
    author: 'Cover Author',
    fileCount: 5,
    totalDuration: 6000000,
    completeness: 0,
    hasCover: false,
    archiveType: null,
  }),
};

/**
 * Inject test audiobooks into the page
 */
export async function injectTestAudiobooks(
  page: Page,
  audiobooks: TestAudiobookConfig[]
): Promise<void> {
  await page.addInitScript(
    (books) => {
      (window as any).__testAudiobooks = books;
    },
    audiobooks
  );
}

/**
 * Create a library with multiple audiobooks
 */
export function createTestLibrary(
  count: number,
  options: Partial<TestAudiobookConfig> = {}
): TestAudiobookConfig[] {
  const audiobooks: TestAudiobookConfig[] = [];

  for (let i = 0; i < count; i++) {
    audiobooks.push(
      createTestAudiobook({
        id: `library-${i}`,
        title: `Audiobook ${i + 1}`,
        author: `Author ${Math.floor(i / 3) + 1}`,
        fileCount: Math.floor(Math.random() * 10) + 1,
        totalDuration: (Math.floor(Math.random() * 10) + 1) * 3600000,
        completeness: Math.floor(Math.random() * 101),
        hasCover: Math.random() > 0.2,
        ...options,
      })
    );
  }

  return audiobooks;
}

/**
 * Generate test URL with audiobook configuration
 */
export function generateTestUrl(
  config: TestAudiobookConfig,
  baseUrl: string = 'http://localhost:5173'
): string {
  const params = new URLSearchParams({
    test: 'true',
    audiobookId: config.id,
    title: config.title,
    fileCount: String(config.fileCount),
    completeness: String(config.completeness),
    hasCover: String(config.hasCover),
  });

  if (config.author) {
    params.set('author', config.author);
  }

  if (config.archiveType) {
    params.set('archiveType', config.archiveType);
  }

  return `${baseUrl}/?${params.toString()}`;
}

/**
 * Wait for audiobook list to load
 */
export async function waitForAudiobooksToLoad(page: Page): Promise<void> {
  await page.waitForSelector('#audiobook-list .card', { timeout: 10000 });
  await page.waitForTimeout(500);
}

/**
 * Verify audiobook count in library
 */
export async function verifyAudiobookCount(
  page: Page,
  expectedCount: number
): Promise<boolean> {
  const cards = page.locator('#audiobook-list .card');
  const count = await cards.count();
  return count === expectedCount;
}
