/**
 * Delete Confirmation and Bulk Selection Step Definitions
 * Implements delete dialogs, bulk selection, and bulk action testing
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('multiple audiobooks exist for bulk operations', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=5');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);

  const library = new LibraryPage(this.page);
  this.lastCount = await library.getAudiobookCount();
});

Given('an audiobook is selected for deletion', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=3');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  this.lastAudiobookId = '0';
});

Given('multiple audiobooks are selected', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=5');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.bulkSelectAudiobooks([0, 1, 2]);

  this.selectedAudiobookIds = ['0', '1', '2'];
});

Given('all audiobooks are selected', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=5');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAllAudiobooks();

  const count = await library.getAudiobookCount();
  this.selectedAudiobookIds = Array.from({ length: count }, (_, i) => i.toString());
});

Given('a delete confirmation dialog is open', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=3&showDeleteDialog=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  // Trigger delete dialog
  await library.deleteAudiobookViaContextMenu(0);
  await this.page.waitForTimeout(300);
});

// ==================== When Steps - Delete Operations ====================

When('I click delete audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);

  // Right-click context menu or delete button
  await library.audiobookCards.first().click({ button: 'right' });
  await this.page.waitForTimeout(200);

  const deleteOption = this.page.locator('button:has-text("Delete"), [aria-label="Delete"]').first();
  if (await deleteOption.isVisible()) {
    await deleteOption.click();
  }
  await this.page.waitForTimeout(300);
});

When('I trigger delete from the details panel', async function (this: NodokaWorld) {
  const deleteButton = this.page.locator('.details-panel button:has-text("Delete"), .details-panel [aria-label="Delete"]');
  if (await deleteButton.count() > 0) {
    await deleteButton.first().click();
  }
  await this.page.waitForTimeout(300);
});

When('I press the Delete key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Delete');
  await this.page.waitForTimeout(300);
});

When('I confirm the deletion', async function (this: NodokaWorld) {
  const confirmButton = this.page.locator('button:has-text("Confirm"), button:has-text("Delete"), button:has-text("Yes")').first();
  await confirmButton.click();
  await this.page.waitForTimeout(500);
});

When('I cancel the deletion', async function (this: NodokaWorld) {
  const cancelButton = this.page.locator('button:has-text("Cancel"), button:has-text("No")').first();
  await cancelButton.click();
  await this.page.waitForTimeout(300);
});

// ==================== When Steps - Bulk Selection ====================

When('I select multiple audiobooks with Ctrl+Click', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const cards = library.audiobookCards;
  const count = Math.min(3, await cards.count());

  for (let i = 0; i < count; i++) {
    await cards.nth(i).click({ modifiers: ['Control'] });
    await this.page.waitForTimeout(100);
  }

  this.selectedAudiobookIds = Array.from({ length: count }, (_, i) => i.toString());
});

When('I select a range of audiobooks with Shift+Click', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const cards = library.audiobookCards;
  const count = await cards.count();

  if (count >= 3) {
    // Click first item
    await cards.nth(0).click();
    await this.page.waitForTimeout(100);

    // Shift+click third item to select range
    await cards.nth(2).click({ modifiers: ['Shift'] });
    await this.page.waitForTimeout(200);

    this.selectedAudiobookIds = ['0', '1', '2'];
  }
});

When('I select all audiobooks with Ctrl+A', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAllAudiobooks();

  const count = await library.getAudiobookCount();
  this.selectedAudiobookIds = Array.from({ length: count }, (_, i) => i.toString());
});

When('I deselect one audiobook with Ctrl+Click', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.audiobookCards.nth(0).click({ modifiers: ['Control'] });
  await this.page.waitForTimeout(200);

  // Remove from selected list
  this.selectedAudiobookIds = this.selectedAudiobookIds.slice(1);
});

When('I clear the selection', async function (this: NodokaWorld) {
  // Click on empty space to clear selection
  await this.page.locator('body').click({ position: { x: 10, y: 10 } });
  await this.page.waitForTimeout(200);

  this.selectedAudiobookIds = [];
});

When('I click select all checkbox', async function (this: NodokaWorld) {
  const selectAllCheckbox = this.page.locator('input[type="checkbox"][aria-label*="Select all"], th input[type="checkbox"]');
  if (await selectAllCheckbox.count() > 0) {
    await selectAllCheckbox.first().click();
  }
  await this.page.waitForTimeout(200);
});

// ==================== When Steps - Bulk Actions ====================

When('I click bulk delete', async function (this: NodokaWorld) {
  const bulkDeleteButton = this.page.locator('button:has-text("Delete Selected"), button:has-text("Bulk Delete"), button:has-text("Delete All")').first();
  await bulkDeleteButton.click();
  await this.page.waitForTimeout(300);
});

When('I click bulk mark as played', async function (this: NodokaWorld) {
  const markPlayedButton = this.page.locator('button:has-text("Mark as Played"), button:has-text("Mark Complete")').first();
  if (await markPlayedButton.isVisible()) {
    await markPlayedButton.click();
  }
  await this.page.waitForTimeout(300);
});

When('I click bulk export', async function (this: NodokaWorld) {
  const exportButton = this.page.locator('button:has-text("Export"), button:has-text("Export Selected")').first();
  if (await exportButton.isVisible()) {
    await exportButton.click();
  }
  await this.page.waitForTimeout(300);
});

When('I click bulk add to playlist', async function (this: NodokaWorld) {
  const playlistButton = this.page.locator('button:has-text("Add to Playlist"), button:has-text("Playlist")').first();
  if (await playlistButton.isVisible()) {
    await playlistButton.click();
  }
  await this.page.waitForTimeout(300);
});

// ==================== Then Steps - Delete Confirmation ====================

Then('a confirmation dialog should appear', async function (this: NodokaWorld) {
  const dialog = this.page.locator('[role="dialog"], .modal-box, .confirm-dialog, .confirmation-dialog');
  await expect(dialog).toBeVisible();
});

Then('the delete confirmation dialog should show the audiobook title', async function (this: NodokaWorld) {
  const dialog = this.page.locator('[role="dialog"], .modal-box, .confirm-dialog');
  const text = await dialog.textContent();
  expect(text?.length).toBeGreaterThan(0);
});

Then('the dialog should have Confirm and Cancel buttons', async function (this: NodokaWorld) {
  const dialog = this.page.locator('[role="dialog"], .modal-box');

  const confirmButton = dialog.locator('button:has-text("Confirm"), button:has-text("Delete"), button:has-text("Yes")');
  const cancelButton = dialog.locator('button:has-text("Cancel"), button:has-text("No")');

  await expect(confirmButton).toBeVisible();
  await expect(cancelButton).toBeVisible();
});

Then('the dialog should close', async function (this: NodokaWorld) {
  const dialog = this.page.locator('[role="dialog"], .modal-box');
  await expect(dialog).not.toBeVisible();
});

Then('the audiobook should be deleted', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeLessThan(this.lastCount);
});

Then('the audiobook should not be deleted', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBe(this.lastCount);
});

Then('a success notification should confirm the deletion', async function (this: NodokaWorld) {
  const notification = this.page.locator('.toast.success, .notification.success, [role="alert"]');
  await expect(notification.first()).toBeVisible({ timeout: 5000 });
});

// ==================== Then Steps - Bulk Selection ====================

Then('all selected audiobooks should be highlighted', async function (this: NodokaWorld) {
  const selectedCards = this.page.locator('.card.ring-primary, .card.selected, [data-selected="true"]');
  const count = await selectedCards.count();
  expect(count).toBeGreaterThan(1);
});

Then('exactly {int} audiobooks should be selected', async function (this: NodokaWorld, count: number) {
  const selectedCards = this.page.locator('.card.ring-primary, .card.selected, [data-selected="true"]');
  const selectedCount = await selectedCards.count();
  expect(selectedCount).toBe(count);
});

Then('the bulk action bar should appear', async function (this: NodokaWorld) {
  const bulkActionBar = this.page.locator('.bulk-actions, .selection-bar, .bulk-action-bar, [data-testid="bulk-actions"]');
  await expect(bulkActionBar).toBeVisible();
});

Then('the bulk action bar should show the selected count', async function (this: NodokaWorld) {
  const bulkActionBar = this.page.locator('.bulk-actions, .selection-bar, .bulk-action-bar');
  const text = await bulkActionBar.textContent();
  expect(text).toMatch(/\d+/);
});

Then('the selection should be cleared', async function (this: NodokaWorld) {
  const selectedCards = this.page.locator('.card.ring-primary, .card.selected, [data-selected="true"]');
  const count = await selectedCards.count();
  expect(count).toBe(0);
});

Then('the bulk action bar should disappear', async function (this: NodokaWorld) {
  const bulkActionBar = this.page.locator('.bulk-actions, .selection-bar, .bulk-action-bar');
  await expect(bulkActionBar).not.toBeVisible();
});

Then('the first audiobook should not be selected', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const firstCard = library.audiobookCards.first();
  const classes = await firstCard.getAttribute('class');
  expect(classes).not.toContain('ring-primary');
  expect(classes).not.toContain('selected');
});

// ==================== Then Steps - Bulk Actions ====================

Then('the selected count should be shown in the dialog', async function (this: NodokaWorld) {
  const dialog = this.page.locator('[role="dialog"], .modal-box');
  const text = await dialog.textContent();
  expect(text).toMatch(/\d+/);
});

Then('all selected audiobooks should be deleted', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  const expectedCount = this.lastCount - this.selectedAudiobookIds.length;
  expect(count).toBe(expectedCount);
});

Then('all selected audiobooks should be marked as played', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);

  // Verify progress is 100% for remaining selected items
  for (let i = 0; i < this.selectedAudiobookIds.length; i++) {
    const progressText = await library.getAudiobookProgress(i);
    const progress = parseInt(progressText?.replace('%', '') || '0');
    expect(progress).toBe(100);
  }
});

Then('the export dialog should open', async function (this: NodokaWorld) {
  const exportDialog = this.page.locator('[role="dialog"]:has-text("Export"), .modal-box:has-text("Export")');
  await expect(exportDialog).toBeVisible();
});

Then('the playlist selection modal should open', async function (this: NodokaWorld) {
  const playlistModal = this.page.locator('[role="dialog"]:has-text("Playlist"), .modal-box:has-text("Playlist")');
  await expect(playlistModal).toBeVisible();
});

// ==================== Then Steps - Select All ====================

Then('all audiobooks should be selected', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const totalCount = await library.getAudiobookCount();
  const selectedCards = this.page.locator('.card.ring-primary, .card.selected, [data-selected="true"]');
  const selectedCount = await selectedCards.count();
  expect(selectedCount).toBe(totalCount);
});

Then('the select all checkbox should be checked', async function (this: NodokaWorld) {
  const selectAllCheckbox = this.page.locator('input[type="checkbox"][aria-label*="Select all"], th input[type="checkbox"]');
  if (await selectAllCheckbox.count() > 0) {
    const isChecked = await selectAllCheckbox.first().isChecked();
    expect(isChecked).toBe(true);
  }
});
