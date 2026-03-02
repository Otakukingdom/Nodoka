/**
 * Shared test utilities for e2e screenshot testing
 */

import { Page } from '@playwright/test';

/**
 * Wait for the application to fully load
 */
export async function waitForAppReady(page: Page): Promise<void> {
  await page.waitForSelector('.navbar', { timeout: 10000 });
  await page.waitForFunction(
    () => {
      const loading = document.querySelector('.loading-spinner');
      return loading === null;
    },
    { timeout: 15000 }
  );
}

/**
 * Navigate to the app and wait for it to be ready
 */
export async function gotoAndWait(page: Page): Promise<void> {
  await page.goto('/');
  await waitForAppReady(page);
}

/**
 * Open the settings modal
 */
export async function openSettingsModal(page: Page): Promise<void> {
  await page.click('button[aria-label="Settings"]');
  await page.waitForSelector('.modal-box');
  await page.waitForTimeout(300);
}

/**
 * Close the current modal
 */
export async function closeModal(page: Page): Promise<void> {
  const closeButton = page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  } else {
    await page.click('.modal-box button:has-text("Done")');
  }
  await page.waitForTimeout(300);
}

/**
 * Set the theme via settings modal
 */
export async function setTheme(page: Page, theme: 'Light' | 'Dark' | 'System'): Promise<void> {
  await openSettingsModal(page);
  await page.selectOption('.modal-box select', theme);
  await page.waitForTimeout(500);
  await closeModal(page);
}

/**
 * Open keyboard help modal via F1
 */
export async function openKeyboardHelp(page: Page): Promise<void> {
  await page.keyboard.press('F1');
  await page.waitForSelector('.modal-box');
  await page.waitForTimeout(300);
}

/**
 * Screenshot configuration for consistent visual regression testing
 */
export const screenshotConfig = {
  animations: 'disabled' as const,
  fullPage: false,
  maxDiffPixels: 100,
};

/**
 * Full page screenshot configuration
 */
export const fullPageScreenshotConfig = {
  ...screenshotConfig,
  fullPage: true as const,
};

/**
 * Notification types matching Redux store
 */
export type NotificationKind = 'Success' | 'Info' | 'Warning' | 'Error';

interface NotificationOptions {
  kind: NotificationKind;
  title?: string;
  message: string;
  durationMs?: number;
}

/**
 * Trigger a notification via Redux dispatch
 * This function injects into the page to dispatch Redux actions directly
 */
export async function triggerNotification(
  page: Page,
  options: NotificationOptions
): Promise<void> {
  await page.evaluate(
    ({ kind, title, message, durationMs }) => {
      // Access the Redux store from window (set up by the app)
      const store = (window as any).__REDUX_STORE__;
      if (store) {
        store.dispatch({
          type: 'app/addNotification',
          payload: {
            id: Date.now(),
            kind,
            title,
            message,
            durationMs: durationMs ?? 5000,
            createdAt: Date.now(),
          },
        });
      } else {
        // Fallback: create notification DOM directly
        const toast = document.querySelector('.toast') || document.createElement('div');
        if (!toast.classList.contains('toast')) {
          toast.className = 'toast toast-top toast-end';
          document.body.appendChild(toast);
        }

        const alertClass = {
          Success: 'alert-success',
          Info: 'alert-info',
          Warning: 'alert-warning',
          Error: 'alert-error',
        }[kind];

        const alert = document.createElement('div');
        alert.className = `alert ${alertClass}`;
        alert.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="${
              kind === 'Success'
                ? 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
                : kind === 'Error'
                  ? 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z'
                  : kind === 'Warning'
                    ? 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z'
                    : 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
            }" />
          </svg>
          <div>
            ${title ? `<h3 class="font-bold">${title}</h3>` : ''}
            <div class="text-xs">${message}</div>
          </div>
          <button aria-label="Dismiss" class="btn btn-ghost btn-sm">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        `;
        toast.appendChild(alert);
      }
    },
    {
      kind: options.kind,
      title: options.title,
      message: options.message,
      durationMs: options.durationMs ?? 5000,
    }
  );
  // Wait for notification to render
  await page.waitForTimeout(200);
}

/**
 * Force loading state in the app by setting Redux state
 */
export async function forceLoadingState(page: Page, isLoading: boolean): Promise<void> {
  await page.evaluate((loading) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setLoading',
        payload: loading,
      });
    }
  }, isLoading);
}

/**
 * Wait for skeleton elements to appear (useful for capturing loading states)
 */
export async function waitForSkeletons(
  page: Page,
  timeout = 3000
): Promise<boolean> {
  try {
    await page.waitForSelector('.skeleton', { timeout });
    return true;
  } catch {
    return false;
  }
}

/**
 * Delay page load to capture loading states
 * Use this with page.route() to intercept and delay API responses
 */
export async function delayPageLoad(page: Page, delayMs = 2000): Promise<void> {
  await page.route('**/*', async (route) => {
    await new Promise((resolve) => setTimeout(resolve, delayMs));
    await route.continue();
  });
}
