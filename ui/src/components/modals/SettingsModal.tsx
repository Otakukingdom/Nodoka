/**
 * Settings modal component
 */

import { X, Settings } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../../hooks/useRedux';
import { closeModal } from '../../store/slices/appSlice';
import { updateTheme } from '../../store/thunks/libraryThunks';
import type { ThemeMode } from '../../store/types';

export default function SettingsModal() {
  const dispatch = useAppDispatch();
  const { theme } = useAppSelector(state => state.app);

  const handleClose = () => {
    dispatch(closeModal());
  };

  const handleThemeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    dispatch(updateTheme(e.target.value as ThemeMode));
  };

  return (
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-md backdrop-blur-sm">
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
          <Settings className="h-5 w-5" />
        </div>
        <div>
          <h3 className="font-bold text-xl">Settings</h3>
          <p className="text-xs text-base-content/60">Customize your experience</p>
        </div>
      </div>

      {/* Settings content */}
      <div className="space-y-6">
        {/* Theme selection */}
        <div className="form-control">
          <label className="label py-2">
            <span className="label-text font-medium text-base">Theme</span>
            <span className="label-text-alt text-base-content/60">Choose your preferred appearance</span>
          </label>
          <select
            className="select select-bordered w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
            value={theme}
            onChange={handleThemeChange}
          >
            <option value="Light">Light</option>
            <option value="Dark">Dark</option>
          </select>
        </div>
      </div>

      {/* Footer */}
      <div className="modal-action mt-8">
        <button
          className="btn btn-primary shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={handleClose}
        >
          Done
        </button>
      </div>
    </div>
  );
}
