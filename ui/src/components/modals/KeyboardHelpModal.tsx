/**
 * Keyboard help modal component
 */

import { X, Keyboard } from 'lucide-react';
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
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-lg">
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:bg-base-200 transition-colors"
        onClick={handleClose}
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </button>

      <div className="flex items-center gap-3 mb-6 pr-8">
        <div className="w-10 h-10 rounded-full gradient-premium flex items-center justify-center text-white shadow-md">
          <Keyboard className="h-5 w-5" />
        </div>
        <h3 className="font-bold text-xl">Keyboard Shortcuts</h3>
      </div>

      <div className="overflow-x-auto">
        <table className="table table-sm">
          <thead>
            <tr className="border-b border-base-300">
              <th className="text-base-content/70 font-medium py-3">Key</th>
              <th className="text-base-content/70 font-medium py-3">Action</th>
            </tr>
          </thead>
          <tbody>
            {shortcuts.map((shortcut, index) => (
              <tr key={index} className="hover:bg-base-200/50 transition-colors">
                <td className="py-3">
                  <kbd className="kbd kbd-sm bg-base-200 border-base-300 font-mono text-xs px-2 py-1">
                    {shortcut.key}
                  </kbd>
                </td>
                <td className="py-3 text-sm">{shortcut.action}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="modal-action mt-6">
        <button
          className="btn btn-primary shadow-md hover:shadow-lg transition-all"
          onClick={handleClose}
        >
          Got it
        </button>
      </div>
    </div>
  );
}
