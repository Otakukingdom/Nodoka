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
    <div className="modal-box">
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
        onClick={handleClose}
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </button>
      <h3 className="font-bold text-lg mb-4">Settings</h3>
      <div className="py-4 space-y-4">
        {/* Theme selection */}
        <div className="form-control">
          <label className="label">
            <span className="label-text">Theme</span>
          </label>
          <select
            className="select select-bordered"
            value={theme}
            onChange={handleThemeChange}
          >
            <option value="Light">Light</option>
            <option value="Dark">Dark</option>
          </select>
        </div>
      </div>
      <div className="modal-action">
        <button className="btn" onClick={handleClose}>
          Close
        </button>
      </div>
    </div>
  );
}
