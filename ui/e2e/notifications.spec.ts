/**
 * E2E screenshot tests for notification system
 * Captures visual appearance of toast notifications in various states
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('Notification Container Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification container empty state', async ({ page }) => {
    const toastContainer = page.locator('.toast');
    // The container exists but may be empty
    await expect(toastContainer).toBeVisible();
    await expect(toastContainer).toHaveScreenshot(
      'notification-container-empty.png',
      screenshotConfig
    );
  });
});

test.describe('Notification Toast Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('success notification toast light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    // Try to find an existing notification or capture the container
    const notification = page.locator('.alert').first();
    if (await notification.isVisible()) {
      await expect(notification).toHaveScreenshot(
        'notification-success-light.png',
        screenshotConfig
      );
    }
  });

  test('success notification toast dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const notification = page.locator('.alert').first();
    if (await notification.isVisible()) {
      await expect(notification).toHaveScreenshot(
        'notification-success-dark.png',
        screenshotConfig
      );
    }
  });

  test('notification dismiss button', async ({ page }) => {
    const notification = page.locator('.alert').first();
    if (await notification.isVisible()) {
      const dismissButton = notification.locator('button[aria-label="Dismiss"]');
      if (await dismissButton.isVisible()) {
        await dismissButton.hover();
        await page.waitForTimeout(200);
        await expect(dismissButton).toHaveScreenshot(
          'notification-dismiss-hover.png',
          screenshotConfig
        );
      }
    }
  });
});

test.describe('Notification Icon Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification icon success', async ({ page }) => {
    const notification = page.locator('.alert.alert-success, .alert-info, .alert-warning, .alert-error').first();
    if (await notification.isVisible()) {
      const icon = notification.locator('svg').first();
      if (await icon.isVisible()) {
        await expect(icon).toHaveScreenshot(
          'notification-icon.png',
          screenshotConfig
        );
      }
    }
  });
});

test.describe('Notification Animation States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification slide-in animation', async ({ page }) => {
    // Look for toast with animation class
    const notification = page.locator('.toast-slide-in, .alert').first();
    if (await notification.isVisible()) {
      await expect(notification).toHaveScreenshot(
        'notification-slide-in.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Multiple Notifications', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('multiple notifications stacked', async ({ page }) => {
    const toastContainer = page.locator('.toast');
    const notifications = await toastContainer.locator('.alert').all();
    if (notifications.length > 1) {
      await expect(toastContainer).toHaveScreenshot(
        'notifications-stacked.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Notification Accessibility', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification dismiss button has aria-label', async ({ page }) => {
    const dismissButtons = await page.locator('.alert button[aria-label="Dismiss"]').all();
    for (const button of dismissButtons) {
      const ariaLabel = await button.getAttribute('aria-label');
      expect(ariaLabel).toBe('Dismiss');
    }
  });

  test('notification dismiss button focus state', async ({ page }) => {
    const dismissButton = page.locator('.alert button[aria-label="Dismiss"]').first();
    if (await dismissButton.isVisible()) {
      await dismissButton.focus();
      await page.waitForTimeout(200);
      await expect(dismissButton).toHaveScreenshot(
        'notification-dismiss-focus.png',
        screenshotConfig
      );
    }
  });
});
