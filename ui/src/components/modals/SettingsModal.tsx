/**
 * Settings modal component
 */

import { useState } from 'react';
import {
  X,
  Settings,
  FolderPlus,
  Trash2,
  RefreshCw,
  RotateCcw,
  Gauge,
  Accessibility,
} from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../../hooks/useRedux';
import { closeModal, setFocusMode, setPrefersReducedMotion } from '../../store/slices/appSlice';
import { updateTheme } from '../../store/thunks/libraryThunks';
import {
  addDirectory,
  removeDirectory,
  rescanDirectory,
} from '../../store/thunks/libraryThunks';
import type { ThemeMode, Directory } from '../../store/types';

const SKIP_DURATIONS = [15, 30, 60];
const SKIP_BACKWARD_DURATIONS = [15, 30];
const AUTO_SAVE_INTERVALS = [1, 5, 10, 15, 30, 60];

export default function SettingsModal() {
  const dispatch = useAppDispatch();
  const { theme, focusMode, prefersReducedMotion } = useAppSelector(state => state.app);
  const { directories, isLibraryLoading } = useAppSelector(state => state.library);

  const [skipForwardDuration, setSkipForwardDuration] = useState(30);
  const [skipBackwardDuration, setSkipBackwardDuration] = useState(30);
  const [autoSaveInterval, setAutoSaveInterval] = useState(10);
  const [isRescanning, setIsRescanning] = useState(false);
  const [expandedSection, setExpandedSection] = useState<string | null>(null);

  const handleClose = () => {
    dispatch(closeModal());
  };

  const handleThemeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    void dispatch(updateTheme(e.target.value as ThemeMode));
  };

  const handleFocusModeChange = (mode: 'Enabled' | 'Disabled') => {
    dispatch(setFocusMode(mode));
  };

  const handleReducedMotionChange = (prefers: boolean) => {
    dispatch(setPrefersReducedMotion(prefers));
  };

  const handleAddDirectory = async () => {
    try {
      // Use Tauri dialog to select directory
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Audiobook Directory',
      });

      if (selected) {
        await dispatch(addDirectory(selected));
      }
    } catch (error) {
      console.error('Failed to add directory:', error);
    }
  };

  const handleRemoveDirectory = async (path: string) => {
    try {
      await dispatch(removeDirectory(path));
    } catch (error) {
      console.error('Failed to remove directory:', error);
    }
  };

  const handleRescanDirectory = async (path: string) => {
    setIsRescanning(true);
    try {
      await dispatch(rescanDirectory(path));
    } catch (error) {
      console.error('Failed to rescan directory:', error);
    } finally {
      setIsRescanning(false);
    }
  };

  const handleResetDefaults = () => {
    setSkipForwardDuration(30);
    setSkipBackwardDuration(30);
    setAutoSaveInterval(10);
    void dispatch(updateTheme('Light'));
    dispatch(setFocusMode('Disabled'));
    dispatch(setPrefersReducedMotion(false));
  };

  const toggleSection = (section: string) => {
    setExpandedSection(expandedSection === section ? null : section);
  };

  return (
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-lg backdrop-blur-sm max-h-[90vh] overflow-y-auto">
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
      <div className="space-y-4">
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
            <option value="System">System</option>
          </select>
        </div>

        {/* Directory Management Section */}
        <div className="form-control">
          <label className="label py-2">
            <span className="label-text font-medium text-base">Audiobook Directories</span>
            <span className="label-text-alt text-base-content/60">Manage your library folders</span>
          </label>
          <div className="space-y-2">
            {/* Directory list */}
            <div className="bg-base-200 rounded-lg max-h-40 overflow-y-auto">
              {directories.length === 0 ? (
                <div className="text-sm text-base-content/50 py-4 text-center">
                  No directories added yet. Click &quot;Add Directory&quot; to get started.
                </div>
              ) : (
                <div className="space-y-1 p-2">
                  {directories.map((dir: Directory) => (
                    <div
                      key={dir.fullPath}
                      className="flex items-center justify-between bg-base-100 rounded border border-base-300/50 group hover:border-primary/20 transition-all duration-200"
                    >
                      <div className="flex-1 flex items-center gap-2 min-w-0 px-2">
                        <FolderPlus className="w-4 h-4 text-base-content/40" />
                        <span className="text-sm truncate">{dir.fullPath}</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <button
                          className="btn btn-ghost btn-xs hover:bg-primary/10 hover:text-primary transition-color"
                          onClick={() => handleRescanDirectory(dir.fullPath)}
                          disabled={isRescanning}
                          aria-label="Rescan directory"
                          title="Rescan directory"
                        >
                          {isRescanning ? (
                            <RefreshCw className="w-3 h-3 animate-spin" />
                          ) : (
                            <RefreshCw className="w-3 h-3" />
                          )}
                        </button>
                        <button
                          className="btn btn-ghost btn-xs text-error hover:bg-error/10"
                          onClick={() => handleRemoveDirectory(dir.fullPath)}
                          aria-label="Remove directory"
                          title="Remove directory"
                        >
                          <Trash2 className="w-3 h-3" />
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
            {/* Add directory button */}
            <button
              className="btn btn-outline btn-sm w-full mt-2 gap-2 hover:bg-primary hover:text-primary hover:border-primary transition-all duration-200"
              onClick={handleAddDirectory}
              disabled={isRescanning || isLibraryLoading}
              aria-label="Add directory"
            >
              <FolderPlus className="w-4 h-4" />
              <span>Add Directory</span>
            </button>
          </div>
        </div>

        {/* Expandable Settings Sections */}
        <div className="divider my-4" />

        {/* Playback Settings */}
        <div className="collapse collapse-arrow bg-base-200 rounded-lg">
          <input
            type="checkbox"
            className="peer"
            checked={expandedSection === 'playback'}
            onChange={() => toggleSection('playback')}
          />
          <div className="collapse-title flex items-center gap-2 font-medium">
            <Gauge className="w-4 h-4" />
            <span>Playback Settings</span>
          </div>
          <div className="collapse-content space-y-4 peer-checked:pt-4">
            {/* Skip Forward Duration */}
            <div className="form-control">
              <label className="label py-1">
                <span className="label-text font-medium text-sm">Skip Forward Duration</span>
                <span className="label-text-alt text-base-content/60">Seconds to skip ahead</span>
              </label>
              <select
                className="select select-bordered select-sm w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
                value={skipForwardDuration}
                onChange={(e) => setSkipForwardDuration(parseInt(e.target.value))}
              >
                {SKIP_DURATIONS.map((duration) => (
                  <option key={duration} value={duration}>
                    {duration}s
                  </option>
                ))}
              </select>
            </div>

            {/* Skip Backward Duration */}
            <div className="form-control">
              <label className="label py-1">
                <span className="label-text font-medium text-sm">Skip Backward Duration</span>
                <span className="label-text-alt text-base-content/60">Seconds to skip back</span>
              </label>
              <select
                className="select select-bordered select-sm w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
                value={skipBackwardDuration}
                onChange={(e) => setSkipBackwardDuration(parseInt(e.target.value))}
              >
                {SKIP_BACKWARD_DURATIONS.map((duration) => (
                  <option key={duration} value={duration}>
                    {duration}s
                  </option>
                ))}
              </select>
            </div>

            {/* Auto-save Interval */}
            <div className="form-control">
              <label className="label py-1">
                <span className="label-text font-medium text-sm">Auto-save Interval</span>
                <span className="label-text-alt text-base-content/60">How often to save progress</span>
              </label>
              <select
                className="select select-bordered select-sm w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
                value={autoSaveInterval}
                onChange={(e) => setAutoSaveInterval(parseInt(e.target.value))}
              >
                {AUTO_SAVE_INTERVALS.map((interval) => (
                  <option key={interval} value={interval}>
                    {interval}s
                  </option>
                ))}
              </select>
            </div>
          </div>
        </div>

        {/* Accessibility Settings */}
        <div className="collapse collapse-arrow bg-base-200 rounded-lg mt-2">
          <input
            type="checkbox"
            className="peer"
            checked={expandedSection === 'accessibility'}
            onChange={() => toggleSection('accessibility')}
          />
          <div className="collapse-title flex items-center gap-2 font-medium">
            <Accessibility className="w-4 h-4" />
            <span>Accessibility</span>
          </div>
          <div className="collapse-content space-y-4 peer-checked:pt-4">
            {/* Focus Mode */}
            <div className="form-control">
              <label className="label cursor-pointer justify-start gap-4 py-2">
                <input
                  type="checkbox"
                  className="checkbox checkbox-primary checkbox-sm"
                  checked={focusMode === 'Enabled'}
                  onChange={(e) => handleFocusModeChange(e.target.checked ? 'Enabled' : 'Disabled')}
                />
                <div className="flex flex-col">
                  <span className="label-text font-medium">Focus Mode</span>
                  <span className="text-xs text-base-content/60">Minimize distractions while listening</span>
                </div>
              </label>
            </div>

            {/* Reduced Motion */}
            <div className="form-control">
              <label className="label cursor-pointer justify-start gap-4 py-2">
                <input
                  type="checkbox"
                  className="checkbox checkbox-primary checkbox-sm"
                  checked={prefersReducedMotion}
                  onChange={(e) => handleReducedMotionChange(e.target.checked)}
                />
                <div className="flex flex-col">
                  <span className="label-text font-medium">Reduced Motion</span>
                  <span className="text-xs text-base-content/60">Minimize animations</span>
                </div>
              </label>
            </div>
          </div>
        </div>

        {/* Reset to Defaults */}
        <div className="mt-6">
          <button
            className="btn btn-outline btn-error btn-sm w-full hover:bg-error hover:text-error-content transition-all duration-200"
            onClick={handleResetDefaults}
          >
            <RotateCcw className="w-4 h-4" />
            <span>Reset to Defaults</span>
          </button>
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
