/**
 * Modal container component that renders the active modal
 */

import { useAppSelector } from '../hooks/useRedux';
import SettingsModal from './modals/SettingsModal';
import KeyboardHelpModal from './modals/KeyboardHelpModal';

export default function ModalContainer() {
  const { activeModal } = useAppSelector(state => state.app);

  if (activeModal === 'None') {
    return null;
  }

  return (
    <div className="modal modal-open backdrop-blur-sm bg-base-content/20">
      {activeModal === 'Settings' && <SettingsModal />}
      {activeModal === 'KeyboardHelp' && <KeyboardHelpModal />}
    </div>
  );
}
