/**
 * Settings modal component
 */

import { X } from 'lucide-react';
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
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-md">
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:bg-base-200 transition-colors"
        onClick={handleClose}
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </button>

      <div className="flex items-center gap-3 mb-6 pr-8">
        <div className="w-10 h-10 rounded-full gradient-premium flex items-center justify-center text-white shadow-md">
          ⚙
        </div>
        <h3 className="font-bold text-xl">Settings</h3>
      </div>

      <div className="space-y-6">
        {/* Theme selection */}
        <div className="form-control">
          <label className="label py-2">
            <span className="label-text font-medium text-base">Theme</span>
            <span className="label-text-alt text-base-content/60">Choose your preferred appearance</span>
          </label>
          <select
            className="select select-bordered w-full bg-base-100 focus:border-primary transition-colors"
            value={theme}
            onChange={handleThemeChange}
          >
            <option value="Light">☀ Light</option>
            <option value="Dark">Dark</option>
          </select>
        </div>
      </div>

      <div className="modal-action mt-8">
        <button
          className="btn btn-primary shadow-md hover:shadow-lg transition-all"
          onClick={handleClose}
        >
          Done
        </button>
      </div>
    </div>
  );
}
