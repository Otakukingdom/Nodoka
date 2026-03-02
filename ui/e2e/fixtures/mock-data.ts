/**
 * Mock Data for Cucumber UI Acceptance Tests
 *
 * Provides consistent test data for UI testing scenarios.
 * This data mirrors the Rust backend types for realistic testing.
 */

/**
 * Mock audiobook data for testing
 */
export const mockAudiobooks = [
  {
    id: 'test-1',
    title: 'Test Audiobook One',
    author: 'Test Author',
    path: '/test/audiobook1',
    totalDuration: 3600000, // 1 hour
    completeness: 0,
    files: [
      { position: 1, name: 'Chapter 1.mp3', duration: 1200000, size: 15000000 },
      { position: 2, name: 'Chapter 2.mp3', duration: 1200000, size: 14500000 },
      { position: 3, name: 'Chapter 3.mp3', duration: 1200000, size: 14800000 },
    ],
    hasCover: true,
    archiveType: null,
    lastListened: null,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
  {
    id: 'test-2',
    title: 'Test Audiobook Two',
    author: 'Another Author',
    path: '/test/audiobook2',
    totalDuration: 7200000, // 2 hours
    completeness: 50,
    files: [
      { position: 1, name: 'Part 1.m4b', duration: 3600000, size: 80000000 },
      { position: 2, name: 'Part 2.m4b', duration: 3600000, size: 82000000 },
    ],
    hasCover: true,
    archiveType: 'zip',
    lastListened: new Date(Date.now() - 3600000).toISOString(),
    createdAt: new Date(Date.now() - 86400000 * 30).toISOString(),
  },
  {
    id: 'test-3',
    title: 'Archive Audiobook',
    author: 'ZIP Author',
    path: '/test/archive.zip',
    totalDuration: 5400000, // 1.5 hours
    completeness: 100,
    files: [
      { position: 1, name: 'Chapter 01.flac', duration: 1800000, size: 120000000 },
      { position: 2, name: 'Chapter 02.flac', duration: 1800000, size: 115000000 },
      { position: 3, name: 'Chapter 03.flac', duration: 1800000, size: 118000000 },
    ],
    hasCover: true,
    archiveType: 'zip',
    lastListened: new Date(Date.now() - 86400000).toISOString(),
    createdAt: new Date(Date.now() - 86400000 * 14).toISOString(),
  },
  {
    id: 'test-4',
    title: 'Corrupted Audio Book',
    author: 'Test Author',
    path: '/test/corrupted',
    totalDuration: 1800000,
    completeness: 0,
    files: [
      { position: 1, name: 'Chapter 1.mp3', duration: 1800000, size: 20000000, corrupted: true },
    ],
    hasCover: false,
    archiveType: null,
    lastListened: null,
    createdAt: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: 'test-5',
    title: 'Long Duration Audiobook',
    author: 'Long Author',
    path: '/test/long',
    totalDuration: 90000000, // 25 hours
    completeness: 0,
    files: [
      { position: 1, name: 'Long File.mp3', duration: 90000000, size: 500000000 },
    ],
    hasCover: true,
    archiveType: null,
    lastListened: null,
    createdAt: new Date().toISOString(),
  },
];

/**
 * Mock directories for testing
 */
export const mockDirectories = [
  {
    id: 'dir-1',
    path: '/test/audiobooks',
    lastScanned: new Date(Date.now() - 3600000).toISOString(),
    audiobookCount: 3,
  },
  {
    id: 'dir-2',
    path: '/test/archives',
    lastScanned: new Date(Date.now() - 86400000).toISOString(),
    audiobookCount: 1,
  },
  {
    id: 'dir-3',
    path: '/test/corrupted',
    lastScanned: null,
    audiobookCount: 1,
  },
];

/**
 * Mock bookmarks for testing
 */
export const mockBookmarks = [
  {
    id: 'bm-1',
    audiobookId: 'test-1',
    filePath: '/test/audiobook1/Chapter 1.mp3',
    positionMs: 300000, // 5 minutes
    label: 'Important scene',
    note: 'This is where the protagonist meets the mentor',
    createdAt: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: 'bm-2',
    audiobookId: 'test-1',
    filePath: '/test/audiobook1/Chapter 1.mp3',
    positionMs: 600000, // 10 minutes
    label: 'Plot twist',
    note: null,
    createdAt: new Date(Date.now() - 3600000).toISOString(),
  },
  {
    id: 'bm-3',
    audiobookId: 'test-2',
    filePath: '/test/audiobook2/Part 1.m4b',
    positionMs: 1800000, // 30 minutes
    label: 'Chapter start',
    note: null,
    createdAt: new Date().toISOString(),
  },
];

/**
 * Mock settings for testing
 */
export const mockSettings = {
  theme: 'Light' as 'Light' | 'Dark' | 'System',
  panelRatio: 0.4,
  focusMode: false,
  volume: 100,
  speed: 1.0,
  prefersReducedMotion: false,
  skipForwardDuration: 30,
  skipBackwardDuration: 15,
  autoSaveInterval: 5,
  windowPosition: { x: 100, y: 100 },
  windowSize: { width: 1280, height: 720 },
};

/**
 * Mock player state for testing
 */
export const mockPlayerState = {
  playbackState: 'NothingSpecial' as 'NothingSpecial' | 'Playing' | 'Paused' | 'Stopped' | 'Ended',
  currentAudiobookId: null as string | null,
  currentFilePath: null as string | null,
  currentPosition: 0,
  duration: 0,
  volume: 100,
  speed: 1.0,
  muted: false,
};

/**
 * Mock sleep timer state for testing
 */
export const mockSleepTimerState = {
  active: false,
  mode: null as 'Duration' | 'EndOfChapter' | null,
  remainingSeconds: null as number | null,
  fadeDuration: 0,
};

/**
 * Mock notification for testing
 */
export const mockNotifications = [
  {
    id: 'notif-1',
    type: 'success' as const,
    message: 'Audiobook added successfully',
    timestamp: new Date().toISOString(),
    dismissed: false,
  },
  {
    id: 'notif-2',
    type: 'error' as const,
    message: 'Failed to load audiobook',
    timestamp: new Date(Date.now() - 60000).toISOString(),
    dismissed: false,
  },
];

/**
 * Mock file formats for audio validation testing
 */
export const mockAudioFormats = {
  supported: ['mp3', 'm4b', 'flac', 'ogg', 'wav', 'aac'],
  unsupported: ['wma', 'ra', 'au', 'snd'],
  corrupted: ['corrupted.mp3', 'invalid.m4b'],
};

/**
 * Mock archive types for testing
 */
export const mockArchiveTypes = {
  supported: ['zip', 'rar', '7z'],
  nested: ['nested.zip'],
  passwordProtected: ['protected.zip'],
  corrupted: ['corrupted.zip'],
  multiBook: ['multi-book.zip'],
};

/**
 * Get mock audiobook by ID
 */
export function getMockAudiobook(id: string) {
  return mockAudiobooks.find((a) => a.id === id) ?? null;
}

/**
 * Get mock files for audiobook
 */
export function getMockFiles(audiobookId: string) {
  const audiobook = getMockAudiobook(audiobookId);
  return audiobook?.files ?? [];
}

/**
 * Get mock bookmarks for audiobook
 */
export function getMockBookmarks(audiobookId: string) {
  return mockBookmarks.filter((b) => b.audiobookId === audiobookId);
}

/**
 * Create test URL with mock query parameters
 */
export function createTestUrl(params: Record<string, string | number | boolean>): string {
  const searchParams = new URLSearchParams();

  // Always include test mode
  searchParams.set('test', 'true');

  // Add all params
  for (const [key, value] of Object.entries(params)) {
    searchParams.set(key, String(value));
  }

  return `http://localhost:5173/?${searchParams.toString()}`;
}

/**
 * Test scenario configurations
 */
export const testScenarios = {
  emptyLibrary: {
    audiobookCount: 0,
    empty: true,
  },
  singleAudiobook: {
    audiobookCount: 1,
  },
  largeLibrary: {
    audiobookCount: 100,
  },
  archiveHandling: {
    archiveType: 'zip',
  },
  audioValidation: {
    audioFormat: 'mp3',
  },
  corruptedFiles: {
    audioCorrupted: true,
  },
  longDuration: {
    longDuration: true,
  },
  loading: {
    loading: true,
  },
  error: {
    error: true,
  },
  networkError: {
    networkError: true,
  },
};
