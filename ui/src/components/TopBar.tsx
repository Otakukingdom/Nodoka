/**
 * Top bar component with logo and settings
 */

import { Settings } from 'lucide-react';
import { useAppDispatch } from '../hooks/useRedux';
import { openModal } from '../store/slices/appSlice';

export default function TopBar() {
  const dispatch = useAppDispatch();

  const handleSettingsClick = () => {
    dispatch(openModal('Settings'));
  };

  return (
    <div className="navbar bg-base-200 border-b border-base-300 px-4 shadow-sm">
      <div className="flex items-center gap-3">
        {/* Logo with enhanced gradient and subtle glow */}
        <div
          className="w-9 h-9 rounded-full gradient-premium flex items-center justify-center text-white text-base shadow-lg"
          style={{
            boxShadow: '0 4px 12px rgba(225, 29, 72, 0.3), 0 2px 4px rgba(0, 0, 0, 0.1)',
          }}
        >
          <span className="drop-shadow-sm">♪</span>
        </div>
        <div className="flex flex-col">
          <span className="text-lg font-bold tracking-tight">Nodoka</span>
          <span className="text-xs text-base-content/60 leading-tight">Audiobook Reader</span>
        </div>
      </div>
      <div className="flex-1" />
      <button
        className="btn btn-ghost btn-circle group hover:bg-primary/10 hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
        onClick={handleSettingsClick}
        aria-label="Settings"
        title="Settings (Ctrl+,)"
      >
        <Settings className="h-5 w-5 transition-transform duration-200 group-hover:rotate-45" />
      </button>
    </div>
  );
}
