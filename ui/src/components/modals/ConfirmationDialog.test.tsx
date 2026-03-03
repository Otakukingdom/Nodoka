/**
 * Unit tests for ConfirmationDialog component
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import ConfirmationDialog from './ConfirmationDialog';

describe('ConfirmationDialog', () => {
  const mockOnConfirm = vi.fn();
  const mockOnCancel = vi.fn();

  beforeEach(() => {
    mockOnConfirm.mockClear();
    mockOnCancel.mockClear();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders when isOpen is true', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    expect(screen.getByText('Test Title')).toBeInTheDocument();
    expect(screen.getByText('Test message')).toBeInTheDocument();
  });

  it('does not render when isOpen is false', () => {
    render(
      <ConfirmationDialog
        isOpen={false}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    expect(screen.queryByText('Test Title')).not.toBeInTheDocument();
  });

  it('shows confirm and cancel buttons with default labels', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    // Use getByText for buttons since they're accessible
    expect(screen.getByText('Confirm')).toBeInTheDocument();
    expect(screen.getByText('Cancel')).toBeInTheDocument();
  });

  it('shows custom button labels', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        confirmLabel="Delete"
        cancelLabel="Keep"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    expect(screen.getByText('Delete')).toBeInTheDocument();
    expect(screen.getByText('Keep')).toBeInTheDocument();
  });

  it('calls onConfirm when confirm button is clicked', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    fireEvent.click(screen.getByText('Confirm'));
    expect(mockOnConfirm).toHaveBeenCalledTimes(1);
  });

  it('calls onCancel when cancel button is clicked', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    fireEvent.click(screen.getByText('Cancel'));
    expect(mockOnCancel).toHaveBeenCalledTimes(1);
  });

  it('calls onCancel when Escape key is pressed', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    fireEvent.keyDown(document, { key: 'Escape' });
    expect(mockOnCancel).toHaveBeenCalledTimes(1);
  });

  it('applies danger variant styling', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        variant="danger"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    const confirmButton = screen.getByText('Confirm').closest('button');
    expect(confirmButton).toHaveClass('btn-error');
  });

  it('applies warning variant styling', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        variant="warning"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    const confirmButton = screen.getByText('Confirm').closest('button');
    expect(confirmButton).toHaveClass('btn-warning');
  });

  it('applies info variant styling', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        variant="info"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    const confirmButton = screen.getByText('Confirm').closest('button');
    expect(confirmButton).toHaveClass('btn-primary');
  });

  it('has proper ARIA attributes', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    const dialog = screen.getByRole('dialog', { hidden: true });
    expect(dialog).toHaveAttribute('aria-modal', 'true');
    expect(dialog).toHaveAttribute('aria-labelledby', 'confirmation-title');
    expect(dialog).toHaveAttribute('aria-describedby', 'confirmation-message');
  });

  it('calls onCancel when clicking outside the dialog', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    const dialog = screen.getByRole('dialog', { hidden: true });
    fireEvent.click(dialog);
    expect(mockOnCancel).toHaveBeenCalledTimes(1);
  });

  it('has close button that calls onCancel', () => {
    render(
      <ConfirmationDialog
        isOpen={true}
        title="Test Title"
        message="Test message"
        onConfirm={mockOnConfirm}
        onCancel={mockOnCancel}
      />
    );

    const closeButton = screen.getByLabelText('Close');
    fireEvent.click(closeButton);
    expect(mockOnCancel).toHaveBeenCalledTimes(1);
  });
});
