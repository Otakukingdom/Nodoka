/**
 * Comprehensive mock data for screenshot generation scenarios
 * Provides realistic data for all UI states including edge cases
 */

/**
 * Standard audiobooks with varying completion states
 */
export const screenshotAudiobooks = [
  {
    id: 1,
    directory: '/audiobooks',
    name: 'The Great Adventure',
    author: 'Jane Author',
    fullPath: '/audiobooks/The Great Adventure',
    completeness: 45,
    defaultOrder: 0,
    totalDurationMs: 3600000,
    remainingDurationMs: 1980000,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
    lastListened: new Date(Date.now() - 3600000).toISOString(),
  },
  {
    id: 2,
    directory: '/audiobooks',
    name: 'Science Fiction Stories',
    author: 'John Writer',
    fullPath: '/audiobooks/Science Fiction Stories',
    completeness: 100,
    defaultOrder: 1,
    totalDurationMs: 7200000,
    remainingDurationMs: 0,
    createdAt: new Date(Date.now() - 86400000 * 30).toISOString(),
    lastListened: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: 3,
    directory: '/audiobooks',
    name: 'Mystery Novel',
    author: 'Unknown Author',
    fullPath: '/audiobooks/Mystery Novel',
    completeness: 0,
    defaultOrder: 2,
    totalDurationMs: 5400000,
    remainingDurationMs: 5400000,
    createdAt: new Date(Date.now() - 86400000 * 2).toISOString(),
  },
  {
    id: 4,
    directory: '/audiobooks',
    name: 'Partially Listened Book',
    author: 'Test Person',
    fullPath: '/audiobooks/Partially Listened Book',
    completeness: 25,
    defaultOrder: 3,
    totalDurationMs: 4800000,
    remainingDurationMs: 3600000,
    createdAt: new Date(Date.now() - 86400000 * 5).toISOString(),
    lastListened: new Date(Date.now() - 86400000 * 2).toISOString(),
  },
  {
    id: 5,
    directory: '/audiobooks',
    name: 'Almost Complete Audiobook',
    author: 'Final Author',
    fullPath: '/audiobooks/Almost Complete Audiobook',
    completeness: 95,
    defaultOrder: 4,
    totalDurationMs: 6000000,
    remainingDurationMs: 300000,
    createdAt: new Date(Date.now() - 86400000 * 14).toISOString(),
    lastListened: new Date(Date.now() - 7200000).toISOString(),
  },
];

/**
 * Audiobook without author metadata
 */
export const audiobookWithoutAuthor = {
  id: 10,
  directory: '/audiobooks',
  name: 'Unknown Author Book',
  fullPath: '/audiobooks/Unknown Author Book',
  completeness: 50,
  defaultOrder: 5,
  totalDurationMs: 3000000,
  remainingDurationMs: 1500000,
  createdAt: new Date().toISOString(),
};

/**
 * Audiobook with very long title for truncation testing
 */
export const audiobookWithLongTitle = {
  id: 20,
  directory: '/audiobooks',
  name: 'This is an Extremely Long Audiobook Title That Should Definitely Be Truncated in the User Interface Display Area Because It Exceeds the Maximum Allowed Character Count for Proper Display',
  author: 'Author with a Very Long Name That Should Also Be Truncated in the User Interface Display Area',
  fullPath: '/audiobooks/Long Title Book',
  completeness: 30,
  defaultOrder: 6,
  totalDurationMs: 9000000,
  remainingDurationMs: 6300000,
  createdAt: new Date(Date.now() - 86400000).toISOString(),
};

/**
 * Audiobook with many chapters for scroll testing
 */
export const audiobookWithManyChapters = {
  id: 30,
  directory: '/audiobooks',
  name: 'Book With Many Chapters',
  author: 'Chapter Author',
  fullPath: '/audiobooks/Book With Many Chapters',
  completeness: 15,
  defaultOrder: 7,
  totalDurationMs: 14400000,
  remainingDurationMs: 12240000,
  createdAt: new Date(Date.now() - 86400000 * 3).toISOString(),
};

/**
 * Files for the standard audiobook (id: 1)
 */
export const standardFiles = [
  {
    audiobookId: 1,
    name: 'Chapter 1 - The Beginning',
    fullPath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
    lengthOfFile: 1800000,
    seekPosition: 900000,
    position: 0,
    completeness: 100,
    fileExists: true,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
  {
    audiobookId: 1,
    name: 'Chapter 2 - The Journey',
    fullPath: '/audiobooks/The Great Adventure/Chapter 2.mp3',
    lengthOfFile: 1800000,
    seekPosition: 0,
    position: 1,
    completeness: 0,
    fileExists: true,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
  {
    audiobookId: 1,
    name: 'Chapter 3 - The Discovery',
    fullPath: '/audiobooks/The Great Adventure/Chapter 3.mp3',
    lengthOfFile: 1800000,
    seekPosition: 0,
    position: 2,
    completeness: 0,
    fileExists: true,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
];

/**
 * Files with missing file indicator
 */
export const filesWithMissing = [
  ...standardFiles,
  {
    audiobookId: 1,
    name: 'Chapter 4 - Missing File',
    fullPath: '/audiobooks/The Great Adventure/Chapter 4.mp3',
    lengthOfFile: 1800000,
    seekPosition: 0,
    position: 3,
    completeness: 0,
    fileExists: false,
    createdAt: new Date().toISOString(),
  },
];

/**
 * Files for audiobook with many chapters (25 chapters)
 */
export const manyChapterFiles = Array.from({ length: 25 }, (_, i) => ({
  audiobookId: 30,
  name: `Chapter ${String(i + 1).padStart(2, '0')} - Part ${i + 1} of the Epic Story`,
  fullPath: `/audiobooks/Book With Many Chapters/Chapter ${String(i + 1).padStart(2, '0')}.mp3`,
  lengthOfFile: 576000,
  seekPosition: i < 3 ? 576000 : 0,
  position: i,
  completeness: i < 4 ? 100 : 0,
  fileExists: true,
  createdAt: new Date(Date.now() - 86400000 * 3).toISOString(),
}));

/**
 * Standard bookmarks
 */
export const standardBookmarks = [
  {
    id: 1,
    audiobookId: 1,
    filePath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
    positionMs: 300000,
    label: 'Important scene',
    note: 'This is where the protagonist meets the mentor character for the first time',
    createdAt: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: 2,
    audiobookId: 1,
    filePath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
    positionMs: 600000,
    label: 'Plot twist',
    note: null,
    createdAt: new Date(Date.now() - 3600000).toISOString(),
  },
  {
    id: 3,
    audiobookId: 1,
    filePath: '/audiobooks/The Great Adventure/Chapter 2.mp3',
    positionMs: 120000,
    label: 'Funny moment',
    note: 'Great comedic timing here',
    createdAt: new Date(Date.now() - 7200000).toISOString(),
  },
];

/**
 * Many bookmarks for scroll testing
 */
export const manyBookmarks = [
  ...standardBookmarks,
  ...Array.from({ length: 12 }, (_, i) => ({
    id: i + 10,
    audiobookId: 1,
    filePath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
    positionMs: (i + 3) * 120000,
    label: i % 3 === 0
      ? `Bookmark ${i + 1} with a longer label to test truncation behavior in the bookmark list display area`
      : `Bookmark ${i + 1}`,
    note: i % 2 === 0 ? `Note for bookmark ${i + 1}` : null,
    createdAt: new Date(Date.now() - (i + 1) * 3600000).toISOString(),
  })),
];

/**
 * Bookmark with very long label
 */
export const bookmarkWithLongLabel = {
  id: 100,
  audiobookId: 1,
  filePath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
  positionMs: 450000,
  label: 'This is an extremely long bookmark label that should definitely be truncated in the user interface because it exceeds the maximum allowed width for bookmark labels in the list view',
  note: 'This is a very long note that provides additional context about why this bookmark was created and what the user should remember when they return to this position in the audiobook',
  createdAt: new Date().toISOString(),
};

/**
 * Directory configurations
 */
export const screenshotDirectories = [
  {
    fullPath: '/audiobooks',
    createdAt: new Date(Date.now() - 86400000 * 30).toISOString(),
    lastScanned: new Date(Date.now() - 3600000).toISOString(),
  },
  {
    fullPath: '/audiobooks/series',
    createdAt: new Date(Date.now() - 86400000 * 15).toISOString(),
    lastScanned: new Date(Date.now() - 86400000).toISOString(),
  },
];

/**
 * Directory with error
 */
export const directoryWithError = {
  fullPath: '/audiobooks/inaccessible',
  createdAt: new Date(Date.now() - 86400000).toISOString(),
  lastScanned: null,
  error: 'PermissionDenied' as const,
};

/**
 * Settings configuration
 */
export const screenshotSettings = {
  theme: 'Light' as const,
  panelRatio: 0.4,
  focusMode: false,
  volume: 80,
  speed: 1.0,
  prefersReducedMotion: false,
};

/**
 * Sleep timer configurations for different states
 */
export const sleepTimerStates = {
  inactive: {
    active: false,
    mode: null,
    remainingSeconds: null,
    fadeDuration: 0,
  },
  durationMode: {
    active: true,
    mode: 'Duration' as const,
    remainingSeconds: 1800,
    fadeDuration: 10,
  },
  endOfChapterMode: {
    active: true,
    mode: 'EndOfChapter' as const,
    remainingSeconds: null,
    fadeDuration: 0,
  },
  activeCountdown: {
    active: true,
    mode: 'Duration' as const,
    remainingSeconds: 300,
    fadeDuration: 30,
  },
};

/**
 * Player states for different scenarios
 */
export const playerStates = {
  idle: {
    playbackState: 'Stopped' as const,
    currentAudiobookId: null,
    currentFilePath: null,
    currentPosition: 0,
    duration: 0,
  },
  paused: {
    playbackState: 'Paused' as const,
    currentAudiobookId: 1,
    currentFilePath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
    currentPosition: 900000,
    duration: 1800000,
  },
  playing: {
    playbackState: 'Playing' as const,
    currentAudiobookId: 1,
    currentFilePath: '/audiobooks/The Great Adventure/Chapter 1.mp3',
    currentPosition: 900000,
    duration: 1800000,
  },
};

/**
 * Volume and speed variations
 */
export const volumeStates = {
  muted: { volume: 0, muted: true },
  low: { volume: 25, muted: false },
  medium: { volume: 50, muted: false },
  high: { volume: 80, muted: false },
  max: { volume: 100, muted: false },
};

export const speedVariations = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

/**
 * Progress levels for player controls
 */
export const progressLevels = [0, 25, 50, 75, 100];

/**
 * Viewport configurations for responsive testing
 */
export const responsiveViewports = {
  desktop: { width: 1280, height: 720 },
  tablet: { width: 1024, height: 768 },
  small: { width: 800, height: 600 },
  compact: { width: 640, height: 480 },
} as const;

/**
 * Combined scenario data for injection
 */
export function getScenarioData(scenario: 'default' | 'empty' | 'edge-cases' | 'many-chapters') {
  switch (scenario) {
    case 'empty':
      return {
        audiobooks: [],
        files: [],
        bookmarks: [],
        directories: screenshotDirectories,
        settings: screenshotSettings,
      };
    case 'edge-cases':
      return {
        audiobooks: [audiobookWithLongTitle, audiobookWithoutAuthor],
        files: filesWithMissing,
        bookmarks: [bookmarkWithLongLabel],
        directories: [...screenshotDirectories, directoryWithError],
        settings: screenshotSettings,
      };
    case 'many-chapters':
      return {
        audiobooks: [audiobookWithManyChapters],
        files: manyChapterFiles,
        bookmarks: manyBookmarks,
        directories: screenshotDirectories,
        settings: screenshotSettings,
      };
    default:
      return {
        audiobooks: screenshotAudiobooks,
        files: standardFiles,
        bookmarks: standardBookmarks,
        directories: screenshotDirectories,
        settings: screenshotSettings,
      };
  }
}
