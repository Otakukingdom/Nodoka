/**
 * Cucumber World class for Playwright-based UI tests
 * Holds test state and page instance for step definitions
 */

import { World } from '@cucumber/cucumber';
import { Page } from '@playwright/test';

export class NodokaWorld extends World {
  public page!: Page;
  public lastAudiobookId: string | null = null;
  public lastBookmarkId: string | null = null;
  public lastCount: number = 0;
  public lastValue: string | number | boolean | null = null;
  public lastError: Error | null = null;
  public testStartTime: number = 0;

  // Additional state for comprehensive testing
  public selectedAudiobookIds: string[] = [];
  public lastNotificationType: 'success' | 'error' | 'info' | null = null;
  public lastKeyPress: string | null = null;
  public lastDialogResult: boolean | null = null;
  public cachedCoverArtPaths: string[] = [];
  public loadedFiles: string[] = [];
  public lastPosition: number = 0;
  public lastDuration: number = 0;
}
