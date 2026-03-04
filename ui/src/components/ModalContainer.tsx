/**
 * Modal container component that renders the active modal
 * Implements focus trap for accessibility (WCAG 2.1)
 */

import { useEffect, useRef, useCallback } from 'react';
import { useAppSelector } from '../hooks/useRedux';
import SettingsModal from './modals/SettingsModal';
import KeyboardHelpModal from './modals/KeyboardHelpModal';
import SleepTimerModal from './modals/SleepTimerModal';

export default function ModalContainer() {
  const { activeModal } = useAppSelector(state => state.app);
  const modalRef = useRef<HTMLDivElement>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);

  // Store the previously focused element and set up focus trap
  useEffect(() => {
    if (activeModal !== 'None') {
      // Store the currently focused element to restore later
      previousFocusRef.current = document.activeElement as HTMLElement;

      // Focus the modal container or first focusable element
      const timer = setTimeout(() => {
        if (modalRef.current) {
          // Find the first focusable element in the modal
          const focusableElements = modalRef.current.querySelectorAll<HTMLElement>(
            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
          );
          const firstFocusable = focusableElements[0];
          if (firstFocusable) {
            firstFocusable.focus();
          } else {
            modalRef.current.focus();
          }
        }
      }, 50);

      return () => clearTimeout(timer);
    }
    return undefined;
  }, [activeModal]);

  // Restore focus when modal closes
  useEffect(() => {
    if (activeModal === 'None' && previousFocusRef.current) {
      previousFocusRef.current.focus();
      previousFocusRef.current = null;
    }
  }, [activeModal]);

  // Handle focus trap - keep Tab cycling within modal
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (activeModal === 'None' || !modalRef.current) return;

    if (e.key === 'Tab') {
      const focusableElements = modalRef.current.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      );
      const focusableArray = Array.from(focusableElements).filter(
        el => !el.hasAttribute('disabled') && el.offsetParent !== null
      );

      if (focusableArray.length === 0) return;

      const firstElement = focusableArray[0];
      const lastElement = focusableArray[focusableArray.length - 1];

      if (e.shiftKey) {
        // Shift+Tab: if on first element, go to last
        if (document.activeElement === firstElement && lastElement) {
          e.preventDefault();
          lastElement.focus();
        }
      } else {
        // Tab: if on last element, go to first
        if (document.activeElement === lastElement && firstElement) {
          e.preventDefault();
          firstElement.focus();
        }
      }
    }
  }, [activeModal]);

  useEffect(() => {
    if (activeModal !== 'None') {
      document.addEventListener('keydown', handleKeyDown);
      return () => document.removeEventListener('keydown', handleKeyDown);
    }
    return undefined;
  }, [activeModal, handleKeyDown]);

  if (activeModal === 'None') {
    return null;
  }

  // Get aria-label based on modal type
  const getModalLabel = () => {
    switch (activeModal) {
      case 'Settings':
        return 'Settings dialog';
      case 'KeyboardHelp':
        return 'Keyboard shortcuts help';
      case 'SleepTimer':
        return 'Sleep timer';
      default:
        return 'Dialog';
    }
  };

  return (
    <div
      ref={modalRef}
      className="modal modal-open backdrop-blur-sm bg-base-content/20"
      role="dialog"
      aria-modal="true"
      aria-label={getModalLabel()}
      tabIndex={-1}
    >
      {activeModal === 'Settings' && <SettingsModal />}
      {activeModal === 'KeyboardHelp' && <KeyboardHelpModal />}
      {activeModal === 'SleepTimer' && <SleepTimerModal />}
    </div>
  );
}
