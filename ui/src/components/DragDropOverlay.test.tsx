/**
 * Unit tests for DragDropOverlay component
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import DragDropOverlay from './DragDropOverlay';

describe('DragDropOverlay', () => {
  const mockOnDrop = vi.fn();
  const mockOpen = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('does not render when not dragging', () => {
    render(<DragDropOverlay onDrop={mockOnDrop} isDisabled={false} />);
    expect(screen.queryByText('Drop audiobook folder here')).not.toBeInTheDocument();
  });

  it('renders overlay when dragenter event fires', async () => {
    render(<DragDropOverlay onDrop={mockOnDrop} isDisabled={false} />);

    // Simulate dragenter event
    fireEvent.dragEnter(window, {
      dataTransfer: { types: ['Files'] },
    });

    await waitFor(() => {
      expect(screen.getByText('Drop audiobook folder here')).toBeInTheDocument();
    });
  });

  it('hides overlay when dragleave event fires', async () => {
    render(<DragDropOverlay onDrop={mockOnDrop} isDisabled={false} />);

    // Show the overlay first
    fireEvent.dragEnter(window, {
      dataTransfer: { types: ['Files'] },
    });

    await waitFor(() => {
      expect(screen.getByText('Drop audiobook folder here')).toBeInTheDocument();
    });

    // Then hide it with dragleave
    fireEvent.dragLeave(window);

    await waitFor(() => {
      expect(screen.queryByText('Drop audiobook folder here')).not.toBeInTheDocument();
    });
  });

  it('calls onDrop with path when file is dropped', async () => {
    render(<DragDropOverlay onDrop={mockOnDrop} isDisabled={false} />);

    // Show the overlay first
    fireEvent.dragEnter(window, {
      dataTransfer: { types: ['Files'] },
    });

    await waitFor(() => {
      expect(screen.getByText('Drop audiobook folder here')).toBeInTheDocument();
    });

    // Simulate drop event
    const mockFile = new File([''], 'test-folder', { type: '' });
    Object.defineProperty(mockFile, 'path', {
      value: '/path/to/test-folder',
      writable: false,
    });

    fireEvent.drop(window, {
      dataTransfer: { files: [mockFile] },
    });

    expect(mockOnDrop).toHaveBeenCalledWith('/path/to/test-folder');
  });

  it('does not call onDrop when disabled', async () => {
    render(<DragDropOverlay onDrop={mockOnDrop} isDisabled={true} />);

    // Try to trigger dragenter while disabled
    fireEvent.dragEnter(window, {
      dataTransfer: { types: ['Files'] },
    });

    // Should not show overlay when disabled
    expect(screen.queryByText('Drop audiobook folder here')).not.toBeInTheDocument();
    expect(mockOnDrop).not.toHaveBeenCalled();
  });

  it('has correct accessibility attributes', async () => {
    render(<DragDropOverlay onDrop={mockOnDrop} isDisabled={false} />);

    // Show the overlay
    fireEvent.dragEnter(window, {
      dataTransfer: { types: ['Files'] },
    });

    await waitFor(() => {
      const overlay = screen.getByRole('status');
      expect(overlay).toHaveAttribute('aria-label', 'Drop folder to add to library');
      expect(overlay).toHaveAttribute('aria-live', 'polite');
    });
  });
});
