/**
 * Scan progress indicator component
 *
 * Displays real-time progress during directory scanning operations.
 * Shows current path, audiobook count, elapsed time, and estimated remaining time.
 */

import { useMemo } from 'react';
import { X, FolderOpen, BookOpen } from 'lucide-react';

export interface ScanProgressProps {
  /** Path currently being scanned */
  currentPath: string;
  /** Number of audiobooks found so far */
  scannedCount: number;
  /** Total expected count (optional, for progress bar) */
  totalCount?: number;
  /** Elapsed time in milliseconds */
  elapsedTime?: number;
  /** Handler for cancel button click */
  onCancel?: () => void;
  /** Whether cancel is disabled */
  isCancelDisabled?: boolean;
  /** Current subfolder being scanned (optional) */
  currentSubPath?: string;
}

/**
 * Formats elapsed time to human-readable string
 */
function formatElapsedTime(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);

  if (totalSeconds < 60) {
    return `${totalSeconds}s`;
  }

  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;

  if (minutes < 60) {
    return `${minutes}m ${seconds}s`;
  }

  const hours = Math.floor(minutes / 60);
  const remainingMinutes = minutes % 60;
  return `${hours}h ${remainingMinutes}m`;
}

/**
 * Estimates remaining time based on current progress
 */
function estimateRemaining(elapsedMs: number, foundCount: number, totalCount?: number): number | undefined {
  if (totalCount === undefined || foundCount === 0 || totalCount <= foundCount) {
    return undefined;
  }

  const rate = foundCount / elapsedMs; // audiobooks per millisecond
  const remaining = totalCount - foundCount;
  return Math.round(remaining / rate);
}

/**
 * Truncates path to show the most relevant part (folder name)
 */
function truncatePath(path: string, maxLength = 40): string {
  if (path.length <= maxLength) {
    return path;
  }

  // Try to keep the folder name visible
  const parts = path.split(/[/\\]/);
  const folderName = parts[parts.length - 1] ?? '';

  if (folderName.length >= maxLength) {
    return `...${folderName.slice(-(maxLength - 3))}`;
  }

  // Show as much of the beginning as possible while keeping folder name
  const availableLength = maxLength - folderName.length - 4; // 4 for ".../"
  if (availableLength > 0) {
    return `${path.slice(0, availableLength)}.../${folderName}`;
  }

  return `...${folderName}`;
}

export default function ScanProgressIndicator({
  currentPath,
  scannedCount,
  totalCount,
  elapsedTime = 0,
  onCancel,
  isCancelDisabled = false,
  currentSubPath,
}: ScanProgressProps) {
  // Calculate progress percentage if total is known
  const progressPercent = useMemo(() => {
    if (totalCount === undefined || totalCount === 0) {
      return undefined;
    }
    return Math.min(100, Math.round((scannedCount / totalCount) * 100));
  }, [scannedCount, totalCount]);

  // Estimate remaining time for long scans
  const estimatedRemaining = useMemo(() => {
    // Only show estimate after 5 seconds of scanning
    if (elapsedTime < 5000) {
      return undefined;
    }
    return estimateRemaining(elapsedTime, scannedCount, totalCount);
  }, [elapsedTime, scannedCount, totalCount]);

  // Determine if we should show elapsed time (only after 2 seconds)
  const showElapsedTime = elapsedTime >= 2000;

  // Determine if we should show cancel button (for scans over 3 seconds)
  const showCancelButton = elapsedTime >= 3000 && onCancel;

  // Get display path
  const displayPath = truncatePath(currentPath);

  // Format audiobook count text
  const audiobookText = scannedCount === 1 ? 'audiobook' : 'audiobooks';

  // Get subfolder display if available
  const subfolderDisplay = currentSubPath ? truncatePath(currentSubPath, 30) : null;

  return (
    <div
      className="bg-primary/5 border-t border-b border-primary/20 py-3 px-4 animate-fade-in"
      role="status"
      aria-live="polite"
      aria-label="Scanning progress"
    >
      <div className="flex items-center gap-3">
        {/* Animated scanning icon */}
        <div className="relative shrink-0">
          <FolderOpen className="w-5 h-5 text-primary animate-pulse" />
          <div
            className="absolute inset-0 rounded-full bg-primary/20"
            style={{
              animation: 'ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite',
            }}
          />
        </div>

        {/* Progress info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <span className="text-sm font-medium text-primary">
              Scanning...
            </span>
            {showElapsedTime && (
              <span className="text-xs text-base-content/70 tabular-nums">
                {formatElapsedTime(elapsedTime)}
                {estimatedRemaining !== undefined && (
                  <span className="text-base-content/50 ml-1">
                    (~{formatElapsedTime(estimatedRemaining)} remaining)
                  </span>
                )}
              </span>
            )}
          </div>

          {/* Current path */}
          <div
            className="text-xs text-base-content/70 truncate"
            title={currentPath}
          >
            {displayPath}
          </div>

          {/* Current subfolder (if different from main path) */}
          {subfolderDisplay && subfolderDisplay !== displayPath && (
            <div
              className="text-xs text-base-content/50 truncate mt-0.5"
              title={currentSubPath}
            >
              Currently: {subfolderDisplay}
            </div>
          )}

          {/* Progress bar (if total known) */}
          {progressPercent !== undefined && (
            <div className="mt-2 h-1.5 bg-base-200 rounded-full overflow-hidden">
              <div
                className="h-full bg-primary rounded-full transition-all duration-300 ease-out"
                style={{ width: `${progressPercent}%` }}
                role="progressbar"
                aria-valuenow={progressPercent}
                aria-valuemin={0}
                aria-valuemax={100}
              />
            </div>
          )}

          {/* Audiobook count */}
          <div className="flex items-center gap-1.5 mt-1.5">
            <BookOpen className="w-3 h-3 text-success" />
            <span className="text-xs text-success font-medium">
              {scannedCount} {audiobookText} found
            </span>
            {totalCount !== undefined && totalCount > 0 && (
              <span className="text-xs text-base-content/60">
                of {totalCount}
              </span>
            )}
          </div>
        </div>

        {/* Cancel button - shown for longer scans */}
        {showCancelButton && (
          <button
            className="btn btn-ghost btn-sm btn-circle shrink-0 hover:bg-error/10 hover:text-error focus-visible:ring-2 focus-visible:ring-error focus-visible:ring-offset-2"
            onClick={onCancel}
            disabled={isCancelDisabled}
            aria-label="Cancel scanning"
            title="Cancel scanning"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
}
