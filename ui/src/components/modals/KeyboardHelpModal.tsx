/**
 * Keyboard help modal component
 */

import { X } from 'lucide-react';
import { useAppDispatch } from '../../hooks/useRedux';
import { closeModal } from '../../store/slices/appSlice';

interface KeyboardShortcut {
  key: string;
  action: string;
}

const shortcuts: KeyboardShortcut[] = [
  { key: 'Space', action: 'Play/Pause' },
  { key: '←', action: 'Seek backward 30s' },
  { key: '→', action: 'Seek forward 30s' },
  { key: '↑', action: 'Previous file' },
  { key: '↓', action: 'Next file' },
  { key: 'Ctrl/Cmd + B', action: 'Create bookmark' },
  { key: 'Ctrl/Cmd + ,', action: 'Open settings' },
  { key: 'F1', action: 'Show keyboard help' },
  { key: 'Escape', action: 'Close modal' },
];

export default function KeyboardHelpModal() {
  const dispatch = useAppDispatch();

  const handleClose = () => {
    dispatch(closeModal());
  };

  return (
    <div className="modal-box">
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
        onClick={handleClose}
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </button>
      <h3 className="font-bold text-lg mb-4">Keyboard Shortcuts</h3>
      <div className="py-4">
        <table className="table table-zebra">
          <thead>
            <tr>
              <th>Key</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {shortcuts.map((shortcut, index) => (
              <tr key={index}>
                <td className="font-mono">{shortcut.key}</td>
                <td>{shortcut.action}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="modal-action">
        <button className="btn" onClick={handleClose}>
          Close
        </button>
      </div>
    </div>
  );
}
