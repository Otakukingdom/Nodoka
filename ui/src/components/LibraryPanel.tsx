/**
 * Library panel component showing audiobook list
 */

import { useMemo, useState } from 'react';
import { Check, BookOpen, Search, FolderPlus, Folder, ExternalLink } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { setSearchQuery, setSortOrder } from '../store/slices/librarySlice';
import { selectAudiobook, addDirectory } from '../store/thunks/libraryThunks';
import { getFilteredAudiobooks } from '../store/thunks/libraryThunks';
import type { AudiobookSortOrder, Audiobook } from '../store/types';

/**
 * Get common audiobook folder suggestions based on platform
 */
function getCommonFolderSuggestions(): { label: string; path: string }[] {
  // Detect platform from userAgent or use Tauri API if available
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
  const isWindows = navigator.platform.toUpperCase().indexOf('WIN') >= 0;
  const homeDir = '~';

  if (isMac) {
    return [
      { label: 'Music/Audiobooks', path: `${homeDir}/Music/Audiobooks` },
      { label: 'Documents/Audiobooks', path: `${homeDir}/Documents/Audiobooks` },
      { label: 'Downloads/Audiobooks', path: `${homeDir}/Downloads/Audiobooks` },
    ];
  }

  if (isWindows) {
    return [
      { label: 'Music\\Audiobooks', path: `%USERPROFILE%\\Music\\Audiobooks` },
      { label: 'Documents\\Audiobooks', path: `%USERPROFILE%\\Documents\\Audiobooks` },
      { label: 'Downloads\\Audiobooks', path: `%USERPROFILE%\\Downloads\\Audiobooks` },
    ];
  }

  // Linux and others
  return [
    { label: 'Music/Audiobooks', path: `${homeDir}/Music/Audiobooks` },
    { label: 'Documents/Audiobooks', path: `${homeDir}/Documents/Audiobooks` },
    { label: 'Downloads/Audiobooks', path: `${homeDir}/Downloads/Audiobooks` },
  ];
}

export default function LibraryPanel() {
  const dispatch = useAppDispatch();
  const { audiobooks, selectedAudiobook, searchQuery, sortOrder, filterStatus, isLibraryLoading } =
    useAppSelector(state => state.library);
  const [isAddingDirectory, setIsAddingDirectory] = useState(false);

  const filteredBooks = useMemo(() => {
    return getFilteredAudiobooks(audiobooks, searchQuery, filterStatus, sortOrder);
  }, [audiobooks, searchQuery, filterStatus, sortOrder]);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(setSearchQuery(e.target.value));
  };

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

    setIsAddingDirectory(true);
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
    } finally {
      setIsAddingDirectory(false);
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
          className="select select-bordered select-sm bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
          value={sortOrder}
          onChange={handleSortChange}
        >
          <option value="RecentlyPlayed">Recent</option>
          <option value="Title">Title</option>
          <option value="Progress">Progress</option>
        </select>
      </div>

      {/* Audiobook list */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-3" id="audiobook-list">
        {isLibraryLoading ? (
          <div className="flex flex-col gap-2">
            {Array.from({ length: 4 }).map((_, i) => (
              <AudiobookCardSkeleton key={i} />
            ))}
          </div>
        ) : filteredBooks.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-16 px-6 min-h-[300px]">
            <div className="relative w-20 h-20 mb-6">
              <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/25 to-primary/5 animate-pulse" style={{ animationDuration: '3s' }} />
              <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/10 to-transparent" />
              <div className="absolute inset-3 rounded-full bg-gradient-to-br from-base-100 to-base-200 flex items-center justify-center shadow-depth-2 border border-base-300/30">
                <BookOpen className="w-8 h-8 text-primary/60" />
              </div>
            </div>
            {searchQuery ? (
              <>
                <div className="text-title text-base-content/80 mb-2 text-center">
                  No matches found
                </div>
                <div className="text-body text-base-content/60 max-w-xs mx-auto text-center leading-relaxed">
                  Try adjusting your search terms or clear the search
                </div>
              </>
            ) : (
              <>
                <div className="text-title-lg text-base-content/90 mb-2 text-center font-semibold">
                  No audiobooks yet
                </div>
                <div className="text-body text-base-content/60 mb-6 max-w-xs mx-auto text-center leading-relaxed">
                  Add a folder containing your audiobook files to get started
                </div>
                <button
                  className="btn btn-premium px-6 py-3 shadow-md hover:shadow-lg transition-all duration-200 gap-2 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
                  onClick={() => void handleAddDirectory()}
                  disabled={isAddingDirectory || isLibraryLoading}
                  aria-label="Add audiobook folder"
                >
                  {isAddingDirectory ? (
                    <span className="loading loading-spinner loading-sm" />
                  ) : (
                    <FolderPlus className="h-5 w-5" />
                  )}
                  <span className="font-medium">Add Audiobook Folder</span>
                </button>

                {/* Common folder suggestions */}
                <div className="mt-6 w-full max-w-xs">
                  <p className="text-xs text-base-content/50 mb-3 text-center">
                    Try these common locations:
                  </p>
                  <div className="flex flex-col gap-1.5">
                    {getCommonFolderSuggestions().map((suggestion) => (
                      <button
                        key={suggestion.path}
                        className="btn btn-ghost btn-sm justify-start gap-2 text-xs text-base-content/70 hover:text-primary hover:bg-primary/10"
                        onClick={() => void handleAddDirectory()}
                        disabled={isAddingDirectory || isLibraryLoading}
                      >
                        <Folder className="w-3.5 h-3.5 shrink-0" />
                        <span className="truncate">{suggestion.label}</span>
                      </button>
                    ))}
                  </div>
                </div>

                <div className="text-caption mt-5 flex items-center gap-1.5">
                  <span>or press</span>
                  <kbd className="kbd kbd-sm px-2">Ctrl</kbd>
                  <span>+</span>
                  <kbd className="kbd kbd-sm px-2">D</kbd>
                </div>

                {/* Drag and drop hint */}
                <div className="mt-4 flex items-center gap-2 text-xs text-base-content/40">
                  <ExternalLink className="w-3 h-3" />
                  <span>You can also drag and drop a folder</span>
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
                onClick={() => handleBookClick(book)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

interface AudiobookCardProps {
  book: Audiobook;
  isSelected: boolean;
  onClick: () => void;
}

function AudiobookCard({ book, isSelected, onClick }: AudiobookCardProps) {
  return (
    <div
      className={`card card-compact bg-base-100 cursor-pointer transition-all duration-200 group ${
        isSelected
          ? 'ring-2 ring-primary shadow-lg bg-primary/5 scale-[1.01]'
          : 'shadow-sm border border-base-300/50 hover:border-primary/30 hover:shadow-md hover:-translate-y-0.5'
      }`}
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => e.key === 'Enter' && onClick()}
    >
      <div className="card-body p-3">
        <div className="flex items-start justify-between gap-2">
          <h3 className="card-title text-sm font-semibold text-truncate leading-tight group-hover:text-primary transition-colors">
            {book.name}
          </h3>
          {book.completeness === 100 && (
            <Check className="h-3.5 w-3.5 text-success shrink-0" />
          )}
        </div>
        {book.author && (
          <span className="text-xs text-base-content/50 -mt-1 text-truncate">{book.author}</span>
        )}
        <div className="flex items-center gap-2 mt-1.5">
          <div className="flex-1 h-2 bg-base-200 rounded-full overflow-hidden">
            <div
              className="h-full gradient-premium rounded-full transition-all duration-500 ease-out"
              style={{ width: `${book.completeness}%` }}
            />
          </div>
          <span className="text-xs font-medium text-base-content/70 w-9 text-right tabular-nums">
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
