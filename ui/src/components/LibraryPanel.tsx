/**
 * Library panel component showing audiobook list
 */

import { useMemo } from 'react';
import { Check, BookOpen, Search } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { setSearchQuery, setSortOrder } from '../store/slices/librarySlice';
import { selectAudiobook } from '../store/thunks/libraryThunks';
import { getFilteredAudiobooks } from '../store/thunks/libraryThunks';
import type { AudiobookSortOrder, Audiobook } from '../store/types';

export default function LibraryPanel() {
  const dispatch = useAppDispatch();
  const { audiobooks, selectedAudiobook, searchQuery, sortOrder, filterStatus, isLibraryLoading } =
    useAppSelector(state => state.library);

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
          <div className="text-center py-16 px-6">
            <div className="relative w-20 h-20 mx-auto mb-5">
              <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/20 to-primary/5 animate-pulse" style={{ animationDuration: '3s' }} />
              <div className="absolute inset-2 rounded-full bg-gradient-to-br from-base-200 to-base-300 flex items-center justify-center shadow-depth-2">
                <BookOpen className="w-8 h-8 text-base-content/40" />
              </div>
            </div>
            <div className="text-lg font-semibold mb-2 text-base-content/80">
              {searchQuery ? 'No matches found' : 'No audiobooks yet'}
            </div>
            <div className="text-sm text-base-content/60 max-w-xs mx-auto leading-relaxed">
              {searchQuery
                ? 'Try adjusting your search terms or clear the search'
                : 'Click "Add Directory" in the menu to import your audiobook collection'}
            </div>
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
