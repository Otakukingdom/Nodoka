/**
 * Settings modal component
 */

import { useEffect, useState, useMemo } from 'react';
import {
  X,
  Settings,
  FolderPlus,
  Trash2,
  RefreshCw,
  RotateCcw,
  Gauge,
  Accessibility,
  Folder,
  AlertTriangle,
  Check,
} from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../../hooks/useRedux';
import { closeModal, setFocusMode, setPrefersReducedMotion } from '../../store/slices/appSlice';
import { updateTheme } from '../../store/thunks/libraryThunks';
import { updateSpeed, updateVolume } from '../../store/thunks/playerThunks';
import {
  addDirectory,
  removeDirectory,
  rescanDirectory,
} from '../../store/thunks/libraryThunks';
import type { ThemeMode, Directory } from '../../store/types';
import ConfirmationDialog from './ConfirmationDialog';
import * as TauriCommands from '../../commands/tauri';
import {
  setSkipForwardDurationSeconds,
  setSkipBackwardDurationSeconds,
  setAutoSaveIntervalSeconds,
  setVolume,
  setSpeed,
} from '../../store/slices/playerSlice';

const SKIP_DURATIONS = [15, 30, 60];
const SKIP_BACKWARD_DURATIONS = [15, 30];
const AUTO_SAVE_INTERVALS = [1, 5, 10, 15, 30, 60];

/**
 * Format a date string to relative time (e.g., "2 hours ago")
 */
function formatRelativeTime(dateString: string | undefined): string | null {
  if (!dateString) return null;

  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffSeconds < 60) return 'Just now';
  if (diffMinutes < 60) return `${diffMinutes}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;

  return date.toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
}

/**
 * Truncate path to show folder name and parent directory
 */
function truncatePath(fullPath: string): { display: string; tooltip: string } {
  const parts = fullPath.split(/[/\\]/).filter(Boolean);
  const folderName = parts[parts.length - 1] ?? fullPath;

  if (parts.length <= 2) {
    return { display: fullPath, tooltip: fullPath };
  }

  // Show "...parent/folderName" for deep paths
  const parentDir = parts[parts.length - 2];
  const display = `...${parentDir}/${folderName}`;

  return { display, tooltip: fullPath };
}

export default function SettingsModal() {
  const dispatch = useAppDispatch();
  const { theme, focusMode, prefersReducedMotion } = useAppSelector(state => state.app);
  const { directories, isLibraryLoading, audiobooks } = useAppSelector(state => state.library);
  const {
    skipForwardDurationSeconds,
    skipBackwardDurationSeconds,
    autoSaveIntervalSeconds,
  } = useAppSelector(state => state.player);

  const [skipForwardDuration, setSkipForwardDuration] = useState(skipForwardDurationSeconds);
  const [skipBackwardDuration, setSkipBackwardDuration] = useState(skipBackwardDurationSeconds);
  const [autoSaveInterval, setAutoSaveInterval] = useState(autoSaveIntervalSeconds);
  const [scanningPaths, setScanningPaths] = useState<Set<string>>(new Set());
  const [expandedSection, setExpandedSection] = useState<string | null>(null);
  const [pendingRemoval, setPendingRemoval] = useState<Directory | null>(null);
  const [isRescanningAll, setIsRescanningAll] = useState(false);
  const [rescanProgress, setRescanProgress] = useState<{ current: number; total: number } | null>(null);
  const [successPaths, setSuccessPaths] = useState<Set<string>>(new Set());

  // Calculate audiobook counts per directory
  const audiobookCountsByDirectory = useMemo(() => {
    const counts = new Map<string, number>();
    for (const book of audiobooks) {
      const current = counts.get(book.directory) ?? 0;
      counts.set(book.directory, current + 1);
    }
    return counts;
  }, [audiobooks]);

  // Sort directories: errors first, then alphabetically by folder name
  const sortedDirectories = useMemo(() => {
    return [...directories].sort((a, b) => {
      // Directories with errors come first
      const aHasError = a.error !== undefined;
      const bHasError = b.error !== undefined;
      if (aHasError && !bHasError) return -1;
      if (!aHasError && bHasError) return 1;

      // Then sort alphabetically by folder name
      const aName = a.fullPath.split(/[/\\]/).pop() ?? a.fullPath;
      const bName = b.fullPath.split(/[/\\]/).pop() ?? b.fullPath;
      return aName.localeCompare(bName);
    });
  }, [directories]);

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

  const handleRemoveClick = (dir: Directory) => {
    setPendingRemoval(dir);
  };

  const confirmRemoval = async () => {
    if (pendingRemoval) {
      try {
        await dispatch(removeDirectory(pendingRemoval.fullPath));
      } catch (error) {
        console.error('Failed to remove directory:', error);
      } finally {
        setPendingRemoval(null);
      }
    }
  };

  const cancelRemoval = () => {
    setPendingRemoval(null);
  };

  const handleRescanDirectory = async (path: string) => {
    setScanningPaths(prev => new Set(prev).add(path));
    setSuccessPaths(prev => {
      const next = new Set(prev);
      next.delete(path);
      return next;
    });
    try {
      await dispatch(rescanDirectory(path));
      // Show success indicator for 2 seconds
      setSuccessPaths(prev => new Set(prev).add(path));
      setTimeout(() => {
        setSuccessPaths(prev => {
          const next = new Set(prev);
          next.delete(path);
          return next;
        });
      }, 2000);
    } catch (error) {
      console.error('Failed to rescan directory:', error);
    } finally {
      setScanningPaths(prev => {
        const next = new Set(prev);
        next.delete(path);
        return next;
      });
    }
  };

  const handleRescanAll = async () => {
    if (directories.length === 0 || isRescanningAll) return;

    setIsRescanningAll(true);
    setRescanProgress({ current: 0, total: directories.length });
    try {
      // Rescan all directories sequentially
      for (let i = 0; i < directories.length; i++) {
        const dir = directories[i];
        if (dir && !dir.error) {
          await dispatch(rescanDirectory(dir.fullPath));
        }
        setRescanProgress({ current: i + 1, total: directories.length });
      }
    } catch (error) {
      console.error('Failed to rescan all directories:', error);
    } finally {
      setRescanProgress(null);
      setIsRescanningAll(false);
    }
  };

  const handleResetDefaults = () => {
    setSkipForwardDuration(30);
    setSkipBackwardDuration(15);
    setAutoSaveInterval(5);

    dispatch(setSkipForwardDurationSeconds(30));
    dispatch(setSkipBackwardDurationSeconds(15));
    dispatch(setAutoSaveIntervalSeconds(5));

    void TauriCommands.setSkipForwardDuration(30);
    void TauriCommands.setSkipBackwardDuration(15);
    void TauriCommands.setAutoSaveInterval(5);

    dispatch(setVolume(100));
    void dispatch(updateVolume(100));

    dispatch(setSpeed(1.0));
    void dispatch(updateSpeed());

    void dispatch(updateTheme('Light'));
    dispatch(setFocusMode('Disabled'));
    dispatch(setPrefersReducedMotion(false));

    void TauriCommands.setVolumePreference(100);
    void TauriCommands.setSpeedPreference(1.0);
    void TauriCommands.setFocusMode(false);
    void TauriCommands.setPrefersReducedMotion(false);
  };

  const toggleSection = (section: string) => {
    setExpandedSection(expandedSection === section ? null : section);
  };

  // Load persisted playback preferences when opening the modal
  useEffect(() => {
    void (async () => {
      try {
        const [skipFwd, skipBack, interval, savedVolume, savedSpeed] = await Promise.all([
          TauriCommands.getSkipForwardDuration().catch(() => 30),
          TauriCommands.getSkipBackwardDuration().catch(() => 15),
          TauriCommands.getAutoSaveInterval().catch(() => 5),
          TauriCommands.getVolume().catch(() => 100),
          TauriCommands.getSpeed().catch(() => 1.0),
        ]);

        dispatch(setSkipForwardDurationSeconds(skipFwd));
        dispatch(setSkipBackwardDurationSeconds(skipBack));
        dispatch(setAutoSaveIntervalSeconds(interval));
        dispatch(setVolume(savedVolume));
        dispatch(setSpeed(savedSpeed));

        setSkipForwardDuration(skipFwd);
        setSkipBackwardDuration(skipBack);
        setAutoSaveInterval(interval);
      } catch {
        // Keep current Redux defaults if settings cannot be loaded
      }
    })();
  }, [dispatch]);

  return (
    <>
      <div className="modal-box shadow-2xl border border-base-300/50 max-w-lg backdrop-blur-sm max-h-[90vh] overflow-y-auto overscroll-contain">
      {/* Close button with enhanced hover state */}
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:bg-base-200 hover:rotate-90 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
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
          <p className="text-xs text-description">Customize your experience</p>
        </div>
      </div>

      {/* Settings content */}
      <div className="space-y-4">
        {/* Theme selection */}
        <div className="form-control">
          <label className="label py-2">
            <span className="label-text font-medium text-base">Theme</span>
            <span className="label-text-alt">Choose your preferred appearance</span>
          </label>
          <select
            className="select select-bordered w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
            value={theme}
            onChange={handleThemeChange}
            aria-label="Select theme"
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
            <span className="label-text-alt">Manage your library folders</span>
          </label>
          <div className="space-y-2">
            {/* Directory list */}
            <div className="bg-base-200 rounded-lg max-h-52 overflow-y-auto custom-scrollbar">
              {directories.length === 0 ? (
                <div className="text-sm text-base-content/50 py-6 text-center">
                  <Folder className="w-8 h-8 mx-auto mb-2 text-base-content/30" />
                  No directories added yet
                </div>
              ) : (
                <div className="divide-y divide-base-300/50">
                  {sortedDirectories.map((dir: Directory) => {
                    const isScanning = scanningPaths.has(dir.fullPath);
                    const hasError = dir.error !== undefined;
                    const isSuccess = successPaths.has(dir.fullPath);
                    const folderName = dir.fullPath.split(/[/\\]/).pop() ?? dir.fullPath;
                    const audiobookCount = audiobookCountsByDirectory.get(dir.fullPath) ?? dir.audiobookCount ?? 0;
                    const relativeTime = formatRelativeTime(dir.lastScanned);
                    const truncatedPath = truncatePath(dir.fullPath);

                    const getErrorMessage = (error: string) => {
                      switch (error) {
                        case 'NotFound':
                          return 'Folder not found';
                        case 'PermissionDenied':
                          return 'Permission denied';
                        default:
                          return 'Unable to access folder';
                      }
                    };

                    return (
                      <div
                        key={dir.fullPath}
                        className={`flex items-center justify-between px-3 py-2.5 group hover:bg-base-300/30 transition-all duration-200 ${
                          isScanning ? 'bg-primary/5' : ''
                        } ${hasError ? 'bg-warning/5' : ''} ${isSuccess ? 'bg-success/5' : ''}`}
                      >
                        <div className="flex-1 flex items-center gap-2.5 min-w-0">
                          <div className="relative shrink-0">
                            {hasError ? (
                              <AlertTriangle className="w-4 h-4 text-warning" />
                            ) : isSuccess ? (
                              <Check className="w-4 h-4 text-success animate-pulse" />
                            ) : (
                              <Folder className="w-4 h-4 text-primary/70" />
                            )}
                          </div>
                          <div className="flex flex-col min-w-0 flex-1">
                            <div className="flex items-center gap-2">
                              <span
                                className={`text-sm font-medium truncate ${hasError ? 'text-warning' : ''}`}
                                title={dir.fullPath}
                              >
                                {folderName}
                              </span>
                              {audiobookCount > 0 && (
                                <span className="badge badge-xs badge-ghost text-[10px] font-medium px-1.5 py-0.5 shrink-0">
                                  {audiobookCount} {audiobookCount === 1 ? 'book' : 'books'}
                                </span>
                              )}
                            </div>
                            {hasError && dir.error ? (
                              <span className="text-xs text-warning font-medium">
                                {getErrorMessage(dir.error)}
                              </span>
                            ) : (
                              <div className="flex items-center gap-2 text-xs text-base-content/70">
                                <span
                                  className="truncate directory-item-path"
                                  title={truncatedPath.tooltip}
                                >
                                  {truncatedPath.display}
                                </span>
                                {relativeTime && (
                                  <>
                                    <span className="text-base-content/40">•</span>
                                    <span className="shrink-0">{relativeTime}</span>
                                  </>
                                )}
                              </div>
                            )}
                            {isScanning && (
                              <span className="text-xs text-primary italic mt-0.5">
                                Scanning...
                              </span>
                            )}
                          </div>
                        </div>
                        <div className="flex items-center gap-0.5 shrink-0">
                          <button
                            className="btn btn-ghost btn-xs hover:bg-primary/10 hover:text-primary transition-color"
                            onClick={() => void handleRescanDirectory(dir.fullPath)}
                            disabled={isScanning || isRescanningAll}
                            aria-label={hasError ? 'Retry' : 'Rescan directory'}
                            title={hasError ? 'Retry' : 'Rescan directory'}
                          >
                            <RefreshCw className={`w-3.5 h-3.5 ${isScanning ? 'animate-spin' : ''}`} />
                          </button>
                          <button
                            className="btn btn-ghost btn-xs text-base-content/50 hover:text-error hover:bg-error/10 transition-color"
                            onClick={() => handleRemoveClick(dir)}
                            disabled={isScanning || isRescanningAll}
                            aria-label="Remove directory"
                            title="Remove directory"
                          >
                            <Trash2 className="w-3.5 h-3.5" />
                          </button>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
            {/* Action buttons */}
            <div className="flex gap-2 mt-2">
              <button
                className="btn btn-outline btn-sm flex-1 gap-2 hover:bg-primary hover:text-primary-content hover:border-primary transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
                onClick={() => void handleAddDirectory()}
                disabled={isLibraryLoading || isRescanningAll}
                aria-label="Add directory"
              >
                <FolderPlus className="w-4 h-4" />
                <span>Add Folder</span>
              </button>
              {directories.length > 1 && (
                <button
                  className="btn btn-ghost btn-sm gap-2 hover:bg-primary/10 hover:text-primary transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
                  onClick={() => void handleRescanAll()}
                  disabled={isLibraryLoading || isRescanningAll}
                  aria-label="Rescan all directories"
                  title="Rescan all directories"
                >
                  <RefreshCw className={`w-4 h-4 ${isRescanningAll ? 'animate-spin' : ''}`} />
                  <span>Rescan All</span>
                  {isRescanningAll && rescanProgress && (
                    <span className="text-xs text-base-content/60 tabular-nums">
                      {rescanProgress.current}/{rescanProgress.total}
                    </span>
                  )}
                </button>
              )}
            </div>
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
            aria-label="Toggle playback settings section"
          />
          <div className="collapse-title flex items-center gap-2 font-medium">
            <Gauge className="w-4 h-4" />
            <span>Playback Settings</span>
          </div>
          <div className="collapse-content space-y-4 peer-checked:pt-4" aria-hidden={expandedSection !== 'playback'}>
            {expandedSection === 'playback' ? (
              <>
                {/* Skip Forward Duration */}
                <div className="form-control">
              <label className="label py-1">
                <span className="label-text font-medium text-sm">Skip Forward Duration</span>
                <span className="label-text-alt">Seconds to skip ahead</span>
              </label>
              <select
                className="select select-bordered select-sm w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
                value={skipForwardDuration}
                onChange={(e) => {
                  const seconds = parseInt(e.target.value);
                  setSkipForwardDuration(seconds);
                  dispatch(setSkipForwardDurationSeconds(seconds));
                  void TauriCommands.setSkipForwardDuration(seconds);
                }}
                aria-label="Select skip forward duration"
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
                <span className="label-text-alt">Seconds to skip back</span>
              </label>
              <select
                className="select select-bordered select-sm w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
                value={skipBackwardDuration}
                onChange={(e) => {
                  const seconds = parseInt(e.target.value);
                  setSkipBackwardDuration(seconds);
                  dispatch(setSkipBackwardDurationSeconds(seconds));
                  void TauriCommands.setSkipBackwardDuration(seconds);
                }}
                aria-label="Select skip backward duration"
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
                <span className="label-text-alt">How often to save progress</span>
              </label>
              <select
                className="select select-bordered select-sm w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
                value={autoSaveInterval}
                onChange={(e) => {
                  const interval = parseInt(e.target.value);
                  setAutoSaveInterval(interval);
                  dispatch(setAutoSaveIntervalSeconds(interval));
                  void TauriCommands.setAutoSaveInterval(interval);
                }}
                aria-label="Select auto-save interval"
              >
                {AUTO_SAVE_INTERVALS.map((interval) => (
                  <option key={interval} value={interval}>
                    {interval}s
                  </option>
                ))}
              </select>
            </div>
              </>
            ) : null}
          </div>
        </div>

        {/* Accessibility Settings */}
        <div className="collapse collapse-arrow bg-base-200 rounded-lg mt-2">
          <input
            type="checkbox"
            className="peer"
            checked={expandedSection === 'accessibility'}
            onChange={() => toggleSection('accessibility')}
            aria-label="Toggle accessibility settings section"
          />
          <div className="collapse-title flex items-center gap-2 font-medium">
            <Accessibility className="w-4 h-4" />
            <span>Accessibility</span>
          </div>
          <div className="collapse-content space-y-4 peer-checked:pt-4" aria-hidden={expandedSection !== 'accessibility'}>
            {expandedSection === 'accessibility' ? (
              <>
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
                      <span className="text-xs text-description">Minimize distractions while listening</span>
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
                      <span className="text-xs text-description">Minimize animations</span>
                    </div>
                  </label>
                </div>
              </>
            ) : null}
          </div>
        </div>

        {/* Reset to Defaults */}
        <div className="mt-6">
          <button
            className="btn btn-ghost btn-sm w-full text-base-content/70 hover:text-error hover:bg-error/10 border-2 hover:border-error/50 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-error focus-visible:ring-offset-2"
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
          className="btn btn-primary btn-premium shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={handleClose}
        >
          Done
        </button>
      </div>
      </div>

      {/* Confirmation Dialog for directory removal */}
      <ConfirmationDialog
        isOpen={pendingRemoval !== null}
        title={pendingRemoval ? `Remove ${pendingRemoval.fullPath.split(/[/\\]/).pop()}?` : ''}
        message="This will remove the folder from your library. Your files won't be deleted from disk."
        confirmLabel="Remove"
        cancelLabel="Cancel"
        variant="warning"
        onConfirm={() => void confirmRemoval()}
        onCancel={cancelRemoval}
      />
    </>
  );
}
