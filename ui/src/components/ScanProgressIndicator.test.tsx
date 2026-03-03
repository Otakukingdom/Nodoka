/**
 * Unit tests for ScanProgressIndicator component
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import ScanProgressIndicator from './ScanProgressIndicator';

describe('ScanProgressIndicator', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('rendering', () => {
    it('renders with required props', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
        />
      );

      expect(screen.getByText('Scanning...')).toBeInTheDocument();
      // Path should be visible
      expect(screen.getByTitle('/test/audiobooks')).toBeInTheDocument();
    });

    it('shows scanned count when greater than zero', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={5}
        />
      );

      // Component shows "5 audiobooks found" text
      expect(screen.getByText(/5 audiobooks found/)).toBeInTheDocument();
    });

    it('shows singular text for one audiobook', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={1}
        />
      );

      expect(screen.getByText('1 audiobook found')).toBeInTheDocument();
    });

    it('shows zero count with correct text', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
        />
      );

      // Component shows "0 audiobooks found"
      expect(screen.getByText('0 audiobooks found')).toBeInTheDocument();
    });

    it('shows total count when provided', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={3}
          totalCount={10}
        />
      );

      expect(screen.getByText(/3 audiobooks found/)).toBeInTheDocument();
      expect(screen.getByText('of 10')).toBeInTheDocument();
    });

    it('does not show total count when undefined', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={3}
        />
      );

      expect(screen.queryByText('of')).not.toBeInTheDocument();
    });
  });

  describe('elapsed time display', () => {
    it('hides elapsed time when less than 2 seconds', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          elapsedTime={1000} // 1 second
        />
      );

      // Elapsed time should not be shown for short durations
      expect(screen.queryByText('1s')).not.toBeInTheDocument();
    });

    it('shows elapsed time when 2 seconds or more', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          elapsedTime={2000} // 2 seconds
        />
      );

      expect(screen.getByText('2s')).toBeInTheDocument();
    });

    it('formats minutes and seconds correctly', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          elapsedTime={90000} // 1m 30s
        />
      );

      expect(screen.getByText('1m 30s')).toBeInTheDocument();
    });

    it('formats hours and minutes correctly', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          elapsedTime={3720000} // 1h 2m
        />
      );

      expect(screen.getByText('1h 2m')).toBeInTheDocument();
    });
  });

  describe('path truncation', () => {
    it('shows full path when short enough', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/books"
          scannedCount={0}
        />
      );

      expect(screen.getByTitle('/test/books')).toBeInTheDocument();
    });

    it('truncates long paths while preserving folder name', () => {
      const longPath = '/Users/testuser/Documents/Audiobooks/Very Long Author Name/Book Series/Book Title';
      render(
        <ScanProgressIndicator
          currentPath={longPath}
          scannedCount={0}
        />
      );

      // Should show truncated path with folder name in title attribute
      const pathElement = screen.getByTitle(longPath);
      expect(pathElement).toBeInTheDocument();
      // The displayed text should contain Book Title
      expect(pathElement.textContent).toContain('Book Title');
    });
  });

  describe('accessibility', () => {
    it('has correct ARIA attributes', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
        />
      );

      const indicator = screen.getByRole('status');
      expect(indicator).toHaveAttribute('aria-live', 'polite');
      expect(indicator).toHaveAttribute('aria-label', 'Scanning progress');
    });
  });

  describe('cancel functionality', () => {
    it('shows cancel button when onCancel provided', () => {
      const onCancel = vi.fn();
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          onCancel={onCancel}
        />
      );

      expect(screen.getByRole('button', { name: 'Cancel scanning' })).toBeInTheDocument();
    });

    it('hides cancel button when onCancel not provided', () => {
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
        />
      );

      expect(screen.queryByRole('button', { name: 'Cancel scanning' })).not.toBeInTheDocument();
    });

    it('calls onCancel when cancel button clicked', async () => {
      const onCancel = vi.fn();
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          onCancel={onCancel}
        />
      );

      const cancelButton = screen.getByRole('button', { name: 'Cancel scanning' });
      cancelButton.click();

      expect(onCancel).toHaveBeenCalledTimes(1);
    });

    it('disables cancel button when isCancelDisabled is true', () => {
      const onCancel = vi.fn();
      render(
        <ScanProgressIndicator
          currentPath="/test/audiobooks"
          scannedCount={0}
          onCancel={onCancel}
          isCancelDisabled={true}
        />
      );

      const cancelButton = screen.getByRole('button', { name: 'Cancel scanning' });
      expect(cancelButton).toBeDisabled();
    });
  });
});
