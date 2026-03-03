/**
 * Unit tests for DirectoryErrorBanner component
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import DirectoryErrorBanner from './DirectoryErrorBanner';
import type { Directory } from '../store/types';

// Mock Tauri dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}));

describe('DirectoryErrorBanner', () => {
  const mockOnLocate = vi.fn();
  const mockOnRemove = vi.fn();

  const createDirectory = (overrides?: Partial<Directory>): Directory => ({
    fullPath: '/Users/test/audiobooks',
    createdAt: '2024-01-01T00:00:00Z',
    ...overrides,
  });

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders folder name correctly', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    expect(screen.getByText('audiobooks')).toBeInTheDocument();
  });

  it('shows NotFound error message', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    expect(screen.getByText('This folder has been moved or deleted')).toBeInTheDocument();
  });

  it('shows PermissionDenied error message', () => {
    const directory = createDirectory({ error: 'PermissionDenied' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    expect(screen.getByText('Permission denied - check folder access')).toBeInTheDocument();
  });

  it('shows Locate button for NotFound errors', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    expect(screen.getByRole('button', { name: /locate/i })).toBeInTheDocument();
  });

  it('does not show Locate button for PermissionDenied errors', () => {
    const directory = createDirectory({ error: 'PermissionDenied' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    expect(screen.queryByRole('button', { name: /locate/i })).not.toBeInTheDocument();
  });

  it('shows Remove button for all error types', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    expect(screen.getByRole('button', { name: /remove/i })).toBeInTheDocument();
  });

  it('calls onRemove when Remove button is clicked', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    const removeButton = screen.getByRole('button', { name: /remove/i });
    fireEvent.click(removeButton);

    expect(mockOnRemove).toHaveBeenCalledWith('/Users/test/audiobooks');
  });

  it('disables buttons when isLoading is true', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
        isLoading={true}
      />
    );

    const removeButton = screen.getByRole('button', { name: /remove/i });
    expect(removeButton).toBeDisabled();
  });

  it('has correct accessibility attributes', () => {
    const directory = createDirectory({ error: 'NotFound' });
    render(
      <DirectoryErrorBanner
        directory={directory}
        onLocate={mockOnLocate}
        onRemove={mockOnRemove}
      />
    );

    const alert = screen.getByRole('alert');
    expect(alert).toHaveAttribute('aria-label', 'Error for folder audiobooks');
  });
});
