/**
 * Component barrel exports
 *
 * Centralized exports for all UI components to improve import ergonomics
 * and document the component hierarchy.
 */

// Main layout components
export { default as TopBar } from './TopBar';
export { default as LibraryPanel } from './LibraryPanel';
export { default as DetailsPanel } from './DetailsPanel';
export { default as PlayerControls } from './PlayerControls';
export { default as VolumeSlider } from './VolumeSlider';

// Container components
export { default as ModalContainer } from './ModalContainer';
export { default as NotificationContainer } from './NotificationContainer';

// Modal components
export { default as SettingsModal } from './modals/SettingsModal';
export { default as KeyboardHelpModal } from './modals/KeyboardHelpModal';
export { default as ConfirmationDialog } from './modals/ConfirmationDialog';
export type { ConfirmationDialogProps } from './modals/ConfirmationDialog';
