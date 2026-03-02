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

  // ==================== Enhanced Methods for Comprehensive UI Testing ====================

  /**
   * Get notification icon type (success, error, info, warning)
   */
  async getNotificationIconType(index: number = 0): Promise<string> {
    const notification = this.notifications.nth(index);
    const icon = notification.locator('svg');

    // Check class names to determine icon type
    const classes = await icon.getAttribute('class') ?? '';
    if (classes.includes('check') || classes.includes('success')) return 'success';
    if (classes.includes('x') || classes.includes('error') || classes.includes('alert')) return 'error';
    if (classes.includes('info')) return 'info';
    if (classes.includes('warning') || classes.includes('alert-triangle')) return 'warning';

    return 'unknown';
  }

  /**
   * Wait for specific notification type
   */
  async waitForNotificationType(type: 'success' | 'error' | 'info', timeout: number = 5000): Promise<void> {
    const selector = `.toast.${type}, .notification.${type}, .alert-${type}`;
    await this.page.locator(selector).waitFor({ state: 'visible', timeout });
  }

  /**
   * Get notification title
   */
  async getNotificationTitle(index: number = 0): Promise<string> {
    const notification = this.notifications.nth(index);
    const title = notification.locator('.title, .notification-title, h4, strong');
    if (await title.isVisible()) {
      return (await title.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Get notification message body
   */
  async getNotificationMessage(index: number = 0): Promise<string> {
    const notification = this.notifications.nth(index);
    const message = notification.locator('.message, .notification-message, .content, p');
    if (await message.isVisible()) {
      return (await message.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Check if notification has action button
   */
  async hasActionButton(index: number = 0): Promise<boolean> {
    const notification = this.notifications.nth(index);
    const actionButton = notification.locator('button:not([aria-label*="dismiss"]):not([aria-label*="close"])');
    return await actionButton.count() > 0;
  }

  /**
   * Click notification action button
   */
  async clickActionButton(index: number = 0): Promise<void> {
    const notification = this.notifications.nth(index);
    const actionButton = notification.locator('button:not([aria-label*="dismiss"]):not([aria-label*="close"])').first();
    if (await actionButton.isVisible()) {
      await actionButton.click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Dismiss notification by swipe gesture
   */
  async swipeToDismiss(index: number = 0): Promise<void> {
    const notification = this.notifications.nth(index);
    const box = await notification.boundingBox();
    if (box) {
      await notification.hover();
      await this.page.mouse.down();
      await this.page.mouse.move(box.x + box.width + 100, box.y);
      await this.page.mouse.up();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Get notification auto-dismiss duration
   */
  async getAutoDismissDuration(index: number = 0): Promise<number> {
    const notification = this.notifications.nth(index);
    const progress = notification.locator('.progress, [role="progressbar"]');
    if (await progress.isVisible()) {
      const duration = await progress.getAttribute('data-duration');
      return duration ? parseInt(duration) : 5000; // Default 5 seconds
    }
    return 5000;
  }

  /**
   * Check if notifications are stacked vertically
   */
  async areNotificationsStacked(): Promise<boolean> {
    const count = await this.notifications.count();
    if (count < 2) return true;

    const positions: number[] = [];
    for (let i = 0; i < count; i++) {
      const box = await this.notifications.nth(i).boundingBox();
      if (box) {
        positions.push(box.y);
      }
    }

    // Check that notifications are at different Y positions (stacked)
    for (let i = 0; i < positions.length - 1; i++) {
      if (Math.abs(positions[i] - positions[i + 1]) < 10) {
        return false; // Too close together, not stacked
      }
    }
    return true;
  }

  /**
   * Get notification z-index order (higher = on top)
   */
  async getNotificationZIndices(): Promise<number[]> {
    const count = await this.notifications.count();
    const zIndices: number[] = [];

    for (let i = 0; i < count; i++) {
      const zIndex = await this.notifications.nth(i).evaluate(el => {
        return parseInt(window.getComputedStyle(el).zIndex) || 0;
      });
      zIndices.push(zIndex);
    }
    return zIndices;
  }

  /**
   * Verify notification has correct icon for type
   */
  async verifyNotificationIconMatchesType(index: number = 0): Promise<boolean> {
    const type = await this.getNotificationType(index);
    const iconType = await this.getNotificationIconType(index);
    return type === iconType || iconType === 'unknown';
  }
}
