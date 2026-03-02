/**
 * E2E screenshot tests for notification system
 * Captures visual appearance of toast notifications in various states
 * Uses triggerNotification helper to reliably create notifications for screenshots
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme, triggerNotification } from './test-utils';

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
    await triggerNotification(page, {
      kind: 'Success',
      title: 'Success',
      message: 'Operation completed successfully',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    await expect(notification).toHaveScreenshot(
      'notification-success-light.png',
      screenshotConfig
    );
  });

  test('success notification toast dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    await triggerNotification(page, {
      kind: 'Success',
      title: 'Success',
      message: 'Operation completed successfully',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    await expect(notification).toHaveScreenshot(
      'notification-success-dark.png',
      screenshotConfig
    );
  });

  test('error notification toast', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Error',
      title: 'Error',
      message: 'Something went wrong',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    await expect(notification).toHaveScreenshot(
      'notification-error.png',
      screenshotConfig
    );
  });

  test('warning notification toast', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Warning',
      title: 'Warning',
      message: 'Please review before proceeding',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    await expect(notification).toHaveScreenshot(
      'notification-warning.png',
      screenshotConfig
    );
  });

  test('info notification toast', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Info',
      title: 'Info',
      message: 'New updates are available',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    await expect(notification).toHaveScreenshot(
      'notification-info.png',
      screenshotConfig
    );
  });

  test('notification dismiss button', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Success',
      message: 'Test notification',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    const dismissButton = notification.locator('button[aria-label="Dismiss"]');
    if (await dismissButton.isVisible()) {
      await dismissButton.hover();
      await page.waitForTimeout(200);
      await expect(dismissButton).toHaveScreenshot(
        'notification-dismiss-hover.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Notification Icon Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification icon success', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Success',
      message: 'Success message',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    const icon = notification.locator('svg').first();
    await expect(icon).toBeVisible();
    await expect(icon).toHaveScreenshot(
      'notification-icon-success.png',
      screenshotConfig
    );
  });

  test('notification icon error', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Error',
      message: 'Error message',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    const icon = notification.locator('svg').first();
    await expect(icon).toBeVisible();
    await expect(icon).toHaveScreenshot(
      'notification-icon-error.png',
      screenshotConfig
    );
  });
});

test.describe('Notification Animation States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification slide-in animation', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Info',
      message: 'Animation test',
    });
    const notification = page.locator('.alert').first();
    await expect(notification).toBeVisible();
    await expect(notification).toHaveScreenshot(
      'notification-slide-in.png',
      { ...screenshotConfig, animations: 'allow' }
    );
  });
});

test.describe('Multiple Notifications', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('multiple notifications stacked', async ({ page }) => {
    // Trigger multiple notifications
    await triggerNotification(page, {
      kind: 'Success',
      title: 'First',
      message: 'First notification',
    });
    await page.waitForTimeout(100);
    await triggerNotification(page, {
      kind: 'Info',
      title: 'Second',
      message: 'Second notification',
    });
    await page.waitForTimeout(100);
    await triggerNotification(page, {
      kind: 'Warning',
      title: 'Third',
      message: 'Third notification',
    });

    const toastContainer = page.locator('.toast');
    const notifications = await toastContainer.locator('.alert').all();
    // Should have at least 2 notifications visible
    expect(notifications.length).toBeGreaterThanOrEqual(2);
    await expect(toastContainer).toHaveScreenshot(
      'notifications-stacked.png',
      screenshotConfig
    );
  });
});

test.describe('Notification Accessibility', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification dismiss button has aria-label', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Success',
      message: 'Test notification',
    });
    const dismissButton = page.locator('.alert button[aria-label="Dismiss"]').first();
    await expect(dismissButton).toBeVisible();
    const ariaLabel = await dismissButton.getAttribute('aria-label');
    expect(ariaLabel).toBe('Dismiss');
  });

  test('notification dismiss button focus state', async ({ page }) => {
    await triggerNotification(page, {
      kind: 'Success',
      message: 'Test notification',
    });
    const dismissButton = page.locator('.alert button[aria-label="Dismiss"]').first();
    await expect(dismissButton).toBeVisible();
    await dismissButton.focus();
    await page.waitForTimeout(200);
    await expect(dismissButton).toHaveScreenshot(
      'notification-dismiss-focus.png',
      screenshotConfig
    );
  });
});
