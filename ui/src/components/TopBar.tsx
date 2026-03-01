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
    <div className="navbar bg-base-200 border-b border-base-300 px-4">
      <div className="flex items-center gap-2">
        <div className="w-8 h-8 rounded-full bg-gradient-to-br from-rose-500 to-pink-600 flex items-center justify-center text-white text-sm">
          ♪
        </div>
        <div>
          <div className="text-lg font-bold">Nodoka</div>
          <div className="text-xs text-base-content/60">Audiobook Reader</div>
        </div>
      </div>
      <div className="flex-1" />
      <button
        className="btn btn-ghost btn-circle"
        onClick={handleSettingsClick}
        aria-label="Settings"
      >
        <Settings className="h-5 w-5" />
      </button>
    </div>
  );
}
