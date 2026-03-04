/**
 * Keyboard help modal component
 */

import { X, Keyboard, PlayCircle, FolderOpen, Settings2 } from 'lucide-react';
import { useAppDispatch } from '../../hooks/useRedux';
import { closeModal } from '../../store/slices/appSlice';

interface KeyboardShortcut {
  key: string;
  action: string;
}

interface ShortcutGroup {
  title: string;
  icon: typeof PlayCircle;
  shortcuts: KeyboardShortcut[];
}

const shortcutGroups: ShortcutGroup[] = [
  {
    title: 'Playback',
    icon: PlayCircle,
    shortcuts: [
      { key: 'Space', action: 'Play/Pause' },
      { key: '←', action: 'Seek back 30s' },
      { key: '→', action: 'Seek forward 30s' },
      { key: 'M', action: 'Mute/Unmute' },
      { key: '+ / =', action: 'Volume up' },
      { key: '-', action: 'Volume down' },
    ],
  },
  {
    title: 'Navigation',
    icon: FolderOpen,
    shortcuts: [
      { key: '↑', action: 'Previous file' },
      { key: '↓', action: 'Next file' },
    ],
  },
  {
    title: 'Library & Settings',
    icon: Settings2,
    shortcuts: [
      { key: 'Ctrl/Cmd + D', action: 'Add folder' },
      { key: 'Ctrl/Cmd + R', action: 'Rescan folder' },
      { key: 'Ctrl/Cmd + Shift + R', action: 'Rescan all' },
      { key: 'Ctrl/Cmd + B', action: 'Create bookmark' },
      { key: 'Ctrl/Cmd + ,', action: 'Open settings' },
      { key: 'F1', action: 'Keyboard help' },
      { key: 'Escape', action: 'Close modal' },
    ],
  },
];

export default function KeyboardHelpModal() {
  const dispatch = useAppDispatch();

  const handleClose = () => {
    dispatch(closeModal());
  };

  return (
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-lg backdrop-blur-sm max-h-[85vh] overflow-y-auto">
      {/* Close button with enhanced hover state */}
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:bg-base-200 hover:rotate-90 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary z-10"
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
          <p className="text-xs text-description">Quick navigation tips</p>
        </div>
      </div>

      {/* Grouped shortcuts */}
      <div className="space-y-5">
        {shortcutGroups.map((group) => (
          <div key={group.title}>
            {/* Group header */}
            <div className="flex items-center gap-2 mb-3 pb-2 border-b border-base-300/50">
              <group.icon className="h-4 w-4 text-primary/70" />
              <span className="text-sm font-semibold text-base-content/80">{group.title}</span>
            </div>
            {/* Shortcuts grid */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
              {group.shortcuts.map((shortcut, index) => (
                <div
                  key={index}
                  className="flex items-center gap-3 py-1.5 px-2 rounded-md hover:bg-base-200/50 transition-colors duration-150"
                >
                  <kbd className="kbd kbd-sm bg-base-200 border-base-300 font-mono text-xs px-2 py-1 shadow-sm min-w-[60px] text-center" style={{ color: '#404040' }}>
                    {shortcut.key}
                  </kbd>
                  <span className="text-sm text-base-content/80">{shortcut.action}</span>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      {/* Tip section for drag-and-drop */}
      <div className="mt-6 p-4 bg-base-200/50 rounded-lg border border-base-300/50">
        <p className="text-sm text-base-content/80">
          <span className="font-medium">Tip:</span> You can also{' '}
          <span className="font-semibold text-primary">drag and drop folders</span> directly onto the window to add
          audiobook directories.
        </p>
      </div>

      {/* Footer */}
      <div className="modal-action mt-6">
        <button
          className="btn btn-premium shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={handleClose}
        >
          Got it
        </button>
      </div>
    </div>
  );
}
