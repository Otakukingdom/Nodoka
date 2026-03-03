/**
 * Confirmation dialog component for destructive actions
 *
 * A reusable modal dialog that requires user confirmation before proceeding
 * with potentially destructive operations like removing directories.
 */

import { useEffect, useRef } from 'react';
import { AlertTriangle, AlertCircle, Info, X } from 'lucide-react';

export interface ConfirmationDialogProps {
  isOpen: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: 'danger' | 'warning' | 'info';
  onConfirm: () => void;
  onCancel: () => void;
}

const variantConfig = {
  danger: {
    icon: AlertTriangle,
    iconColor: 'text-error',
    iconBg: 'bg-error/10',
    confirmBtn: 'btn-error',
  },
  warning: {
    icon: AlertCircle,
    iconColor: 'text-warning',
    iconBg: 'bg-warning/10',
    confirmBtn: 'btn-warning',
  },
  info: {
    icon: Info,
    iconColor: 'text-info',
    iconBg: 'bg-info/10',
    confirmBtn: 'btn-primary',
  },
};

export default function ConfirmationDialog({
  isOpen,
  title,
  message,
  confirmLabel = 'Confirm',
  cancelLabel = 'Cancel',
  variant = 'danger',
  onConfirm,
  onCancel,
}: ConfirmationDialogProps) {
  const cancelRef = useRef<HTMLButtonElement>(null);
  const config = variantConfig[variant];
  const IconComponent = config.icon;

  // Focus cancel button when dialog opens
  useEffect(() => {
    if (isOpen && cancelRef.current) {
      cancelRef.current.focus();
    }
  }, [isOpen]);

  // Handle escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (isOpen && e.key === 'Escape') {
        onCancel();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      return () => document.removeEventListener('keydown', handleEscape);
    }
    return undefined;
  }, [isOpen, onCancel]);

  // Handle click outside
  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onCancel();
    }
  };

  if (!isOpen) return null;

  return (
    <dialog
      className="modal modal-open"
      onClick={handleBackdropClick}
      aria-modal="true"
      aria-labelledby="confirmation-title"
      aria-describedby="confirmation-message"
    >
      <div className="modal-box shadow-2xl border border-base-300/50 max-w-md backdrop-blur-sm p-0 overflow-hidden">
        {/* Header with icon */}
        <div className={`p-6 pb-4 ${config.iconBg}`}>
          <div className="flex items-start gap-4">
            <div className={`p-2 rounded-full ${config.iconBg}`}>
              <IconComponent className={`h-6 w-6 ${config.iconColor}`} />
            </div>
            <div className="flex-1 pt-1">
              <h3 id="confirmation-title" className="font-bold text-lg text-base-content">
                {title}
              </h3>
            </div>
            <button
              className="btn btn-sm btn-circle btn-ghost hover:bg-base-200 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
              onClick={onCancel}
              aria-label="Close"
            >
              <X className="h-4 w-4" />
            </button>
          </div>
        </div>

        {/* Message */}
        <div className="px-6 pb-4">
          <p id="confirmation-message" className="text-sm text-base-content/70 leading-relaxed">
            {message}
          </p>
        </div>

        {/* Actions */}
        <div className="modal-action p-4 pt-2 bg-base-200/50 flex gap-3">
          <button
            ref={cancelRef}
            className="btn btn-ghost flex-1 hover:bg-base-300 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
            onClick={onCancel}
          >
            {cancelLabel}
          </button>
          <button
            className={`btn ${config.confirmBtn} flex-1 shadow-md hover:shadow-lg transition-all duration-200 focus-visible:ring-2 focus-visible:ring-offset-2`}
            onClick={onConfirm}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </dialog>
  );
}
