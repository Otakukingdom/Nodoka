/**
 * Library panel component showing audiobook list
 */

import { useMemo, useCallback, useRef, useEffect, useState } from 'react';
import { Check, BookOpen, Search, FolderPlus, ExternalLink, AlertTriangle, FolderSearch, RefreshCw, Volume2 } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { setSearchQuery, setSortOrder } from '../store/slices/librarySlice';
import { selectAudiobook, addDirectory, removeDirectory, updateDirectoryPath } from '../store/thunks/libraryThunks';
import { getFilteredAudiobooks } from '../store/thunks/libraryThunks';
import type { AudiobookSortOrder, Audiobook, Directory } from '../store/types';

export default function LibraryPanel() {
  const dispatch = useAppDispatch();
  const { audiobooks, selectedAudiobook, searchQuery, sortOrder, filterStatus, isLibraryLoading, isAddingDirectory, directories, scanningPaths, scanProgress } =
    useAppSelector(state => state.library);
  const { playback } = useAppSelector(state => state.player);

  // Debounce timer ref for search input
  const searchDebounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Check if any directory is being scanned (must be declared before useEffect that uses it)
  const isScanning = scanningPaths.length > 0;

  // Track previous scanning state for screen reader announcements
  const [prevScanningState, setPrevScanningState] = useState(isScanning);
  const [announcement, setAnnouncement] = useState<string | null>(null);

  // Cleanup debounce timer on unmount
  useEffect(() => {
    return () => {
      if (searchDebounceRef.current) {
        clearTimeout(searchDebounceRef.current);
      }
    };
  }, []);

  // Screen reader announcements for scan state changes
  useEffect(() => {
    if (prevScanningState && !isScanning) {
      // Scan completed
      const foundCount = scanProgress?.foundCount ?? audiobooks.length;
      setAnnouncement(`Scan complete. Found ${foundCount} audiobook${foundCount === 1 ? '' : 's'}.`);
      // Clear announcement after 3 seconds
      setTimeout(() => setAnnouncement(null), 3000);
    } else if (!prevScanningState && isScanning) {
      // Scan started
      setAnnouncement('Scanning folder for audiobooks...');
    }
    setPrevScanningState(isScanning);
  }, [isScanning, prevScanningState, scanProgress?.foundCount, audiobooks.length]);

  // Get directories with errors for inline error display
  const directoriesWithErrors = useMemo(() =>
    directories.filter(dir => dir.error !== undefined),
    [directories]
  );

  const filteredBooks = useMemo(() => {
    return getFilteredAudiobooks(audiobooks, searchQuery, filterStatus, sortOrder);
  }, [audiobooks, searchQuery, filterStatus, sortOrder]);

  // Debounced search handler for performance with large libraries
  const handleSearchChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;

    // Clear existing timer
    if (searchDebounceRef.current) {
      clearTimeout(searchDebounceRef.current);
    }

    // Set new timer - 150ms debounce
    searchDebounceRef.current = setTimeout(() => {
      dispatch(setSearchQuery(value));
    }, 150);
  }, [dispatch]);

  const handleSortChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    dispatch(setSortOrder(e.target.value as AudiobookSortOrder));
  };

  const handleBookClick = (book: Audiobook) => {
    if (book.id !== undefined) {
      void dispatch(selectAudiobook(book.id));
    }
  };

  const handleAddDirectory = async () => {
    if (isAddingDirectory || isLibraryLoading) return;

    try {
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

  const panelWidth = useAppSelector(state => state.app.panelRatio) * 100;

  return (
    <div
      className="flex flex-col border-r border-base-300 overflow-hidden library-panel bg-base-100"
      style={{ width: `${panelWidth}%` }}
      role="navigation"
      aria-label="Audiobook library"
    >
      {/* Screen reader announcements for async operations */}
      <div className="sr-only" aria-live="polite" aria-atomic="true">
        {announcement}
        {isScanning && scanProgress && `Scanning: ${scanProgress.foundCount} audiobooks found`}
      </div>

      {/* Toolbar */}
      <div className="p-3 border-b border-base-300 flex gap-2 bg-base-200/50">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-base-content/50 pointer-events-none" />
          <input
            type="text"
            placeholder="Search audiobooks..."
            className="input input-bordered input-sm w-full pl-9 bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200"
            value={searchQuery}
            onChange={handleSearchChange}
          />
        </div>
        <select
          className="select select-bordered select-sm bg-base-100 transition-all duration-200 select-interactive"
          value={sortOrder}
          onChange={handleSortChange}
          aria-label="Sort audiobooks"
        >
          <option value="RecentlyPlayed">Recent</option>
          <option value="Title">Title</option>
          <option value="Progress">Progress</option>
        </select>
        {/* Add folder button - visible when audiobooks exist (returning user) */}
        {audiobooks.length > 0 && (
          <button
            className="btn btn-ghost btn-sm btn-square hover:bg-primary/10 hover:text-primary transition-all focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
            onClick={() => void handleAddDirectory()}
            disabled={isAddingDirectory || isLibraryLoading || isScanning}
            aria-label="Add audiobook folder"
            title="Add Folder (Ctrl+D)"
          >
            <FolderPlus className="h-4 w-4" />
          </button>
        )}
      </div>

      {/* Audiobook list */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-3" id="audiobook-list">
        {/* Directory error banners - show at top when directories have issues */}
        {!isLibraryLoading && directoriesWithErrors.length > 0 && (
          <div className="mb-3 space-y-2">
            {directoriesWithErrors.map((dir) => (
              <DirectoryErrorBannerInline
                key={dir.fullPath}
                directory={dir}
                onLocate={(oldPath, newPath) => {
                  void dispatch(updateDirectoryPath({ oldPath, newPath }));
                }}
                onRemove={(path) => {
                  void dispatch(removeDirectory(path));
                }}
              />
            ))}
          </div>
        )}

        {isLibraryLoading ? (
          <div className="flex flex-col gap-2">
            {Array.from({ length: 4 }).map((_, i) => (
              <AudiobookCardSkeleton key={i} />
            ))}
          </div>
        ) : filteredBooks.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 px-6 min-h-[300px] animate-fade-in">
            {searchQuery ? (
              <>
                <div className="relative w-16 h-16 mb-5">
                  <div className="absolute inset-0 rounded-full bg-gradient-to-br from-base-300/50 to-base-200/30" />
                  <div className="absolute inset-2 rounded-full bg-gradient-to-br from-base-100 to-base-200 flex items-center justify-center shadow-depth-1 border border-base-300/30">
                    <Search className="w-7 h-7 text-base-content/40" />
                  </div>
                </div>
                <div className="text-title text-base-content/80 mb-2 text-center">
                  No matches found
                </div>
                <div className="text-body text-base-content/60 max-w-xs mx-auto text-center leading-relaxed">
                  Try adjusting your search terms or clear the search
                </div>
              </>
            ) : (
              <>
                {/* Prominent empty state with animated icon */}
                <div className="relative w-24 h-24 mb-5">
                  {/* Outer pulse ring - draws attention */}
                  <div
                    className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/20 to-primary/5"
                    style={{
                      animation: 'pulse-ring 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                    }}
                  />
                  {/* Inner glow */}
                  <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/10 to-transparent" />
                  {/* Center icon container */}
                  <div className="absolute inset-4 rounded-full bg-gradient-to-br from-base-100 to-base-200 flex items-center justify-center shadow-depth-2 border border-base-300/30">
                    <BookOpen
                      className="w-9 h-9 text-primary/70"
                      style={{ animation: 'pulse 2.5s ease-in-out infinite' }}
                    />
                  </div>
                </div>

                {/* Clear visual hierarchy: title > description > action */}
                <div className="text-title-lg text-base-content/90 mb-1.5 text-center font-semibold">
                  Add Your Audiobooks
                </div>
                <div className="text-body text-base-content/70 mb-5 max-w-xs mx-auto text-center leading-relaxed">
                  Select a folder containing your audiobook files to get started
                </div>

                {/* Primary action - large touch target (min-h-12 = 48px) */}
                <button
                  className="btn btn-premium min-h-12 h-12 px-8 shadow-lg hover:shadow-xl transition-all duration-200 gap-2.5 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 text-base"
                  onClick={() => void handleAddDirectory()}
                  disabled={isAddingDirectory || isLibraryLoading}
                  aria-label="Add audiobook folder"
                >
                  {isAddingDirectory ? (
                    <span className="loading loading-spinner loading-sm" />
                  ) : (
                    <FolderPlus className="h-5 w-5" />
                  )}
                  <span className="font-semibold">Add Folder</span>
                </button>

                {/* Keyboard shortcut hint - secondary visual element */}
                <div className="text-caption mt-4 flex items-center gap-1.5 text-base-content/70">
                  <span>or press</span>
                  <kbd className="kbd kbd-sm px-2 bg-base-200 border-base-300">Ctrl</kbd>
                  <span className="text-base-content/50">+</span>
                  <kbd className="kbd kbd-sm px-2 bg-base-200 border-base-300">D</kbd>
                </div>

                {/* Drag and drop hint - tertiary visual element */}
                <div className="mt-3 flex items-center gap-2 text-xs text-base-content/60">
                  <ExternalLink className="w-3 h-3" />
                  <span>Drag and drop a folder also works</span>
                </div>
              </>
            )}
          </div>
        ) : (
          <div className="flex flex-col gap-2">
            {filteredBooks.map(book => (
              <AudiobookCard
                key={book.id ?? book.fullPath}
                book={book}
                isSelected={selectedAudiobook === book.id}
                isPlaying={selectedAudiobook === book.id && playback === 'Playing'}
                onClick={() => handleBookClick(book)}
              />
            ))}
          </div>
        )}
      </div>

      {/* Scanning status footer - detailed progress indicator */}
      {isScanning && scanProgress && (
        <ScanProgressIndicator
          currentPath={scanProgress.path}
          scannedCount={scanProgress.foundCount}
          totalCount={scanProgress.totalCount}
          elapsedTime={Date.now() - scanProgress.startTime}
        />
      )}
    </div>
  );
}

interface AudiobookCardProps {
  book: Audiobook;
  isSelected: boolean;
  isPlaying: boolean;
  onClick: () => void;
}

function AudiobookCard({ book, isSelected, isPlaying, onClick }: AudiobookCardProps) {
  return (
    <div
      className={`card card-compact bg-base-100 cursor-pointer transition-all duration-200 group card-performance card-interactive ${
        isSelected
          ? 'ring-[3px] ring-primary shadow-lg bg-primary/5 scale-[1.01] shadow-primary/10'
          : 'shadow-sm border border-base-300/50 hover:border-primary/50 hover:shadow-lg hover:bg-primary/5'
      }`}
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => e.key === 'Enter' && onClick()}
    >
      <div className="card-body p-3">
        <div className="flex items-start justify-between gap-2">
          <div className="flex items-center gap-2 min-w-0">
            {isPlaying && (
              <Volume2 className="h-3.5 w-3.5 text-primary shrink-0 animate-pulse" />
            )}
            <h3 className="card-title text-sm font-semibold text-truncate leading-tight group-hover:text-primary transition-colors">
              {book.name}
            </h3>
          </div>
          {book.completeness === 100 && (
            <Check className="h-3.5 w-3.5 text-success shrink-0" />
          )}
        </div>
        {book.author && (
          <span className="text-xs text-base-content/60 -mt-1 text-truncate group-hover:text-base-content/70 transition-colors">{book.author}</span>
        )}
        <div className="flex items-center gap-2 mt-1.5">
          <div className="flex-1 h-2 bg-base-300/70 rounded-full overflow-hidden group-hover:bg-base-300 transition-colors progress-bar-enhanced">
            <div
              className="h-full gradient-premium rounded-full transition-all duration-500 ease-out"
              style={{ width: `${book.completeness}%` }}
            />
          </div>
          <span className="text-xs font-medium text-base-content/70 w-9 text-right tabular-nums group-hover:text-base-content/80 transition-colors">
            {book.completeness}%
          </span>
        </div>
      </div>
    </div>
  );
}

function AudiobookCardSkeleton() {
  return (
    <div className="card card-compact bg-base-100 shadow-sm border border-base-300/50">
      <div className="card-body p-3">
        <div className="flex items-start justify-between gap-2">
          <div className="h-5 w-3/4 skeleton rounded" />
          <div className="h-3.5 w-3.5 skeleton rounded shrink-0" />
        </div>
        <div className="h-3 w-1/2 skeleton rounded -mt-1" />
        <div className="flex items-center gap-2 mt-1.5">
          <div className="flex-1 h-2 skeleton rounded-full" />
          <div className="h-3 w-9 skeleton rounded" />
        </div>
      </div>
    </div>
  );
}

/**
 * Inline error banner for directory issues displayed in library panel
 * Provides immediate visibility of directory problems with recovery actions
 */
interface DirectoryErrorBannerInlineProps {
  directory: Directory;
  onLocate: (oldPath: string, newPath: string) => void;
  onRemove: (path: string) => void;
}

function DirectoryErrorBannerInline({ directory, onLocate, onRemove }: DirectoryErrorBannerInlineProps) {
  const folderName = directory.fullPath.split(/[/\\]/).pop() ?? directory.fullPath;

  const getErrorMessage = (error: string | undefined): string => {
    switch (error) {
      case 'NotFound':
        return 'Folder moved or deleted';
      case 'PermissionDenied':
        return 'Permission denied';
      default:
        return 'Unable to access folder';
    }
  };

  const handleLocate = async () => {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: `Locate "${folderName}" Folder`,
      });
      if (selected && typeof selected === 'string') {
        onLocate(directory.fullPath, selected);
      }
    } catch (error) {
      console.error('Failed to locate folder:', error);
    }
  };

  return (
    <div
      className="alert alert-warning alert-sm py-2 px-3 shadow-sm"
      role="alert"
      aria-label={`Error for folder ${folderName}`}
    >
      <AlertTriangle className="w-4 h-4 shrink-0" />
      <div className="flex-1 min-w-0">
        <p className="font-medium text-sm truncate" title={directory.fullPath}>
          {folderName}
        </p>
        <p className="text-xs opacity-80">{getErrorMessage(directory.error)}</p>
      </div>
      <div className="flex gap-1.5 shrink-0">
        {directory.error === 'NotFound' && (
          <button
            className="btn btn-xs btn-ghost gap-1"
            onClick={() => void handleLocate()}
            aria-label="Locate folder"
          >
            <FolderSearch className="w-3.5 h-3.5" />
            <span>Locate</span>
          </button>
        )}
        <button
          className="btn btn-xs btn-ghost text-error hover:bg-error/10 gap-1"
          onClick={() => void onRemove(directory.fullPath)}
          aria-label="Remove folder from library"
        >
          <span>Remove</span>
        </button>
      </div>
    </div>
  );
}

/**
 * Scan progress indicator component that displays real-time scan progress
 * Uses the ScanProgressIndicator from components folder
 */
function ScanProgressIndicator({
  currentPath,
  scannedCount,
  totalCount,
  elapsedTime,
}: {
  currentPath: string;
  scannedCount: number;
  totalCount?: number;
  elapsedTime: number;
}) {
  return (
    <div
      className="px-3 py-2.5 border-t border-primary/20 bg-primary/5 animate-fade-in"
      role="status"
      aria-live="polite"
      aria-label="Scanning progress"
    >
      <div className="flex items-center gap-3">
        {/* Animated scanning icon */}
        <div className="relative shrink-0">
          <RefreshCw className="w-4 h-4 text-primary animate-spin" />
        </div>

        {/* Progress info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-xs text-primary font-medium">
              Scanning...
            </span>
            {elapsedTime >= 2000 && (
              <span className="text-xs text-base-content/50 tabular-nums">
                {formatScanTime(elapsedTime)}
              </span>
            )}
          </div>

          {/* Current path - truncated */}
          <div
            className="text-xs text-base-content/60 truncate mt-0.5"
            title={currentPath}
          >
            {truncateScanPath(currentPath)}
          </div>
        </div>

        {/* Audiobook count badge */}
        {scannedCount > 0 && (
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-full bg-success/10">
            <BookOpen className="w-3 h-3 text-success" />
            <span className="text-xs text-success font-medium tabular-nums">
              {scannedCount}
            </span>
            {totalCount !== undefined && totalCount > 0 && (
              <span className="text-xs text-base-content/40">
                of {totalCount}
              </span>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

/**
 * Formats elapsed time for scan progress display
 */
function formatScanTime(ms: number): string {
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
 * Truncates path to show the most relevant part
 */
function truncateScanPath(path: string, maxLength = 50): string {
  if (path.length <= maxLength) {
    return path;
  }

  // Try to keep the folder name visible
  const parts = path.split(/[/\\]/);
  const folderName = parts[parts.length - 1] ?? '';

  if (folderName.length >= maxLength) {
    return `...${folderName.slice(-(maxLength - 3))}`;
  }

  // Show as much of the end of the path as possible
  const availableLength = maxLength - folderName.length - 4;
  if (availableLength > 10) {
    return `${path.slice(0, availableLength)}.../${folderName}`;
  }

  return `...${folderName}`;
}
