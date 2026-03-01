/**
 * Library panel component showing audiobook list
 */

import { useMemo } from 'react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { setSearchQuery, setSortOrder } from '../store/slices/librarySlice';
import { selectAudiobook } from '../store/thunks/libraryThunks';
import { getFilteredAudiobooks } from '../store/thunks/libraryThunks';
import type { AudiobookSortOrder, Audiobook } from '../store/types';

export default function LibraryPanel() {
  const dispatch = useAppDispatch();
  const { audiobooks, selectedAudiobook, searchQuery, sortOrder, filterStatus } =
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
      dispatch(selectAudiobook(book.id));
    }
  };

  const panelWidth = useAppSelector(state => state.app.panelRatio) * 100;

  return (
    <div
      className="flex flex-col border-r border-base-300 overflow-hidden library-panel bg-base-100"
      style={{ width: `${panelWidth}%` }}
    >
      {/* Toolbar */}
      <div className="p-3 border-b border-base-300 flex gap-2 bg-base-200/50">
        <input
          type="text"
          placeholder="Search audiobooks..."
          className="input input-bordered input-sm flex-1 bg-base-100 focus:border-primary transition-colors"
          value={searchQuery}
          onChange={handleSearchChange}
        />
        <select
          className="select select-bordered select-sm bg-base-100"
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
        {filteredBooks.length === 0 ? (
          <div className="text-center py-12 px-4">
            <div className="text-5xl mb-3 opacity-80">📚</div>
            <div className="text-lg font-semibold mb-1">No audiobooks found</div>
            <div className="text-sm text-base-content/60">Add a directory to get started</div>
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
      className={`card card-compact bg-base-100 shadow-sm cursor-pointer transition-all duration-200 hover:shadow-md hover:-translate-y-0.5 ${
        isSelected ? 'ring-2 ring-primary shadow-md' : 'border border-base-300/50 hover:border-primary/30'
      }`}
      onClick={onClick}
    >
      <div className="card-body p-3">
        <h3 className="card-title text-sm font-semibold text-truncate leading-tight">{book.name}</h3>
        <div className="flex items-center gap-2 mt-1">
          <div className="flex-1 h-2 bg-base-200 rounded-full overflow-hidden">
            <div
              className="h-full gradient-premium rounded-full transition-all duration-300"
              style={{ width: `${book.completeness}%` }}
            />
          </div>
          <span className="text-xs font-medium text-base-content/70 w-8 text-right tabular-nums">{book.completeness}%</span>
        </div>
      </div>
    </div>
  );
}
