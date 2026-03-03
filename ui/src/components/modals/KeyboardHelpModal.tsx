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
  { key: 'M', action: 'Mute/Unmute' },
  { key: '+ / =', action: 'Volume up' },
  { key: '-', action: 'Volume down' },
  { key: 'Ctrl/Cmd + D', action: 'Add audiobook folder' },
  { key: 'Ctrl/Cmd + R', action: 'Rescan current folder' },
  { key: 'Ctrl/Cmd + Shift + R', action: 'Rescan all folders' },
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
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-lg backdrop-blur-sm">
      {/* Close button with enhanced hover state */}
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:bg-base-200 hover:rotate-90 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
        onClick={handleClose}
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </button>

      {/* Header with icon */}
      <div className="flex items-center gap-3 mb-6 pr-8">
        <div
          className="w-10 h-10 rounded-full gradient-premium flex items-center justify-center text-white shadow-lg"
          style={{
            boxShadow: '0 4px 12px rgba(225, 29, 72, 0.3), 0 2px 4px rgba(0, 0, 0, 0.1)',
          }}
        >
          <Keyboard className="h-5 w-5" />
        </div>
        <div>
          <h3 className="font-bold text-xl">Keyboard Shortcuts</h3>
          <p className="text-xs text-base-content/60">Quick navigation tips</p>
        </div>
      </div>

      {/* Shortcuts table */}
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
              <tr
                key={index}
                className="hover:bg-base-200/50 transition-colors duration-150"
              >
                <td className="py-3">
                  <kbd className="kbd kbd-sm bg-base-200 border-base-300 font-mono text-xs px-2.5 py-1.5 shadow-sm">
                    {shortcut.key}
                  </kbd>
                </td>
                <td className="py-3 text-sm">{shortcut.action}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Tip section for drag-and-drop */}
      <div className="mt-6 p-4 bg-base-200/50 rounded-lg border border-base-300/50">
        <p className="text-sm text-base-content/80">
          <span className="font-medium">Tip:</span> You can also{' '}
          <span className="font-semibold">drag and drop folders</span> directly onto the window to add
          audiobook directories.
        </p>
      </div>

      {/* Footer */}
      <div className="modal-action mt-6">
        <button
          className="btn btn-primary shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={handleClose}
        >
          Got it
        </button>
      </div>
    </div>
  );
}
