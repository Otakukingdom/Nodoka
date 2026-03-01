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
        <div className="w-9 h-9 rounded-full gradient-premium flex items-center justify-center text-white text-base shadow-md">
          ♪
        </div>
        <div className="flex flex-col">
          <span className="text-lg font-bold tracking-tight">Nodoka</span>
          <span className="text-xs text-base-content/60 leading-tight">Audiobook Reader</span>
        </div>
      </div>
      <div className="flex-1" />
      <button
        className="btn btn-ghost btn-circle hover:bg-primary/10 transition-colors duration-200"
        onClick={handleSettingsClick}
        aria-label="Settings"
        title="Settings (Ctrl+,)"
      >
        <Settings className="h-5 w-5" />
      </button>
    </div>
  );
}
