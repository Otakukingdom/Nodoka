/**
 * Notification Page Object Model
 * Encapsulates notification system interactions for testing
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from '../../pom/BasePage.js';

export class NotificationPage extends BasePage {
  readonly notifications: Locator;
  readonly successNotification: Locator;
  readonly errorNotification: Locator;
  readonly infoNotification: Locator;
  readonly dismissButton: Locator;
  readonly notificationIcon: Locator;
  readonly notificationContent: Locator;
  readonly progressBar: Locator;

  constructor(page: Page) {
    super(page);
    this.notifications = page.locator('.toast, .notification, [role="alert"]');
    this.successNotification = page.locator('.toast.success, .notification.success, .alert-success');
    this.errorNotification = page.locator('.toast.error, .notification.error, .alert-error');
    this.infoNotification = page.locator('.toast.info, .notification.info, .alert-info');
    this.dismissButton = page.locator('.toast button, .notification button, [role="alert"] button');
    this.notificationIcon = page.locator('.toast svg, .notification svg, .toast .icon, .notification .icon');
    this.notificationContent = page.locator('.toast .content, .notification .content, .toast .message, .notification .message');
    this.progressBar = page.locator('.toast .progress, .notification .progress, [role="progressbar"]');
  }

  /**
   * Get notification count
   */
  async getNotificationCount(): Promise<number> {
    return await this.notifications.count();
  }

  /**
   * Check if success notification is visible
   */
  async isSuccessVisible(): Promise<boolean> {
    return await this.successNotification.first().isVisible();
  }

  /**
   * Check if error notification is visible
   */
  async isErrorVisible(): Promise<boolean> {
    return await this.errorNotification.first().isVisible();
  }

  /**
   * Check if info notification is visible
   */
  async isInfoVisible(): Promise<boolean> {
    return await this.infoNotification.first().isVisible();
  }

  /**
   * Check if any notification is visible
   */
  async isNotificationVisible(): Promise<boolean> {
    return await this.notifications.first().isVisible();
  }

  /**
   * Wait for notification to appear
   */
  async waitForNotification(timeout: number = 5000): Promise<void> {
    await this.notifications.first().waitFor({ state: 'visible', timeout });
  }

  /**
   * Wait for notification to disappear
   */
  async waitForNotificationToDisappear(timeout: number = 10000): Promise<void> {
    await this.notifications.first().waitFor({ state: 'hidden', timeout });
  }

  /**
   * Dismiss first notification
   */
  async dismissFirst(): Promise<void> {
    const dismissBtn = this.dismissButton.first();
    if (await dismissBtn.isVisible()) {
      await dismissBtn.click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Dismiss all notifications
   */
  async dismissAll(): Promise<void> {
    const count = await this.notifications.count();
    for (let i = 0; i < count; i++) {
      await this.dismissFirst();
      await this.page.waitForTimeout(100);
    }
  }

  /**
   * Click on notification
   */
  async clickNotification(index: number = 0): Promise<void> {
    await this.notifications.nth(index).click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Get notification text
   */
  async getNotificationText(index: number = 0): Promise<string> {
    return (await this.notifications.nth(index).textContent()) ?? '';
  }

  /**
   * Get notification type (success, error, info)
   */
  async getNotificationType(index: number = 0): Promise<string> {
    const notification = this.notifications.nth(index);
    const classes = await notification.getAttribute('class') ?? '';

    if (classes.includes('success')) return 'success';
    if (classes.includes('error')) return 'error';
    if (classes.includes('info')) return 'info';
    if (classes.includes('warning')) return 'warning';

    return 'unknown';
  }

  /**
   * Check if notification has icon
   */
  async hasIcon(index: number = 0): Promise<boolean> {
    const icon = this.notifications.nth(index).locator('svg, .icon, [class*="icon"]');
    return await icon.count() > 0;
  }

  /**
   * Check if notification has progress bar
   */
  async hasProgressBar(index: number = 0): Promise<boolean> {
    const progress = this.notifications.nth(index).locator('.progress, [role="progressbar"]');
    return await progress.count() > 0;
  }

  /**
   * Get stacked notifications count (for multiple notifications test)
   */
  async getStackedCount(): Promise<number> {
    return await this.notifications.count();
  }

  /**
   * Verify notification positions (newer above older)
   */
  async verifyStackingOrder(): Promise<boolean> {
    const count = await this.notifications.count();
    if (count < 2) return true;

    const positions: number[] = [];
    for (let i = 0; i < count; i++) {
      const box = await this.notifications.nth(i).boundingBox();
      if (box) {
        positions.push(box.y);
      }
    }

    // First notification should be higher (lower Y value) or at same level
    for (let i = 0; i < positions.length - 1; i++) {
      if (positions[i] > positions[i + 1] + 10) {
        return false;
      }
    }
    return true;
  }

  /**
   * Verify success notification is visible
   */
  async expectSuccessVisible(): Promise<void> {
    await expect(this.successNotification.first()).toBeVisible();
  }

  /**
   * Verify error notification is visible
   */
  async expectErrorVisible(): Promise<void> {
    await expect(this.errorNotification.first()).toBeVisible();
  }

  /**
   * Verify info notification is visible
   */
  async expectInfoVisible(): Promise<void> {
    await expect(this.infoNotification.first()).toBeVisible();
  }

  /**
   * Verify no notifications are visible
   */
  async expectNoNotifications(): Promise<void> {
    await expect(this.notifications.first()).not.toBeVisible();
  }

  /**
   * Verify notification count
   */
  async expectNotificationCount(count: number): Promise<void> {
    const actualCount = await this.notifications.count();
    expect(actualCount).toBe(count);
  }
}
