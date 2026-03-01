/**
 * Custom Playwright fixtures for e2e testing
 */

import { test as base, expect } from '@playwright/test';

// Re-export expect for convenience
export { expect };

/**
 * Extended test fixture with app-specific setup
 */
export const test = base.extend({});

/**
 * Viewport configurations for responsive testing
 */
export const viewports = {
  desktop: { width: 1280, height: 720 },
  tablet: { width: 1024, height: 768 },
  small: { width: 800, height: 600 },
  mobile: { width: 375, height: 667 },
} as const;
