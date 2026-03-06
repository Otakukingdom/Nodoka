/**
 * Details panel component showing files and bookmarks
 */

import { FileText, Bookmark } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { selectFile } from '../store/thunks/libraryThunks';
import type { AudiobookFile, Bookmark as BookmarkType } from '../store/types';

export default function DetailsPanel() {
  const { currentFiles, selectedFile, bookmarks } = useAppSelector(state => state.library);
  const dispatch = useAppDispatch();

  const handleFileClick = (file: AudiobookFile) => {
    void dispatch(selectFile(file.fullPath));
  };

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-base-100 details-panel-container h-full" role="complementary" aria-label="File and bookmark details">
      {/* Files section - primary content area */}
      <div className="flex-1 overflow-y-auto custom-scrollbar px-3 sm:px-4 py-3 sm:py-4 min-h-0" role="region" aria-label="Files list">
        {/* Section header with clear visual hierarchy - using section-header-enhanced class */}
        <div className="flex items-center gap-2 mb-3 sticky top-0 bg-base-100/95 backdrop-blur-sm py-1.5 z-10 section-header-enhanced border-b-0">
          <div className="w-5 h-5 sm:w-6 sm:h-6 rounded-md sm:rounded-lg bg-primary/10 flex items-center justify-center flex-shrink-0">
            <FileText className="w-3 h-3 sm:w-3.5 sm:h-3.5 text-primary" />
          </div>
          <h2 className="text-xs sm:text-xs font-bold text-base-content/70 tracking-wide uppercase flex-1">Files</h2>
          {currentFiles.length > 0 && (
            <span className="badge badge-primary badge-sm px-1.5 sm:px-2 font-medium border-0 text-[11px]">{currentFiles.length}</span>
          )}
        </div>
        <div className="flex flex-col gap-1.5 sm:gap-2" id="files-list">
          {currentFiles.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-6 animate-fade-in">
              <div className="relative w-12 h-12 mb-3">
                <div className="absolute inset-0 rounded-xl bg-gradient-to-br from-primary/20 to-primary/5 animate-pulse" />
                <div className="absolute inset-1.5 rounded-xl bg-gradient-to-br from-base-100 to-base-200 flex items-center justify-center shadow-depth-2 border border-base-300/30">
                  <FileText className="w-5 h-5 text-primary/50" />
                </div>
              </div>
              <p className="text-sm font-semibold text-base-content/80 text-center">No files available</p>
              <p className="text-xs text-base-content/50 text-center max-w-[180px]">Select an audiobook from the library to view its chapters</p>
              <div className="mt-4 flex items-center justify-center gap-2">
                <span className="text-primary text-lg">←</span>
                <span className="text-xs font-medium text-base-content/70">Select an audiobook</span>
              </div>
            </div>
          ) : (
            currentFiles.map(file => (
              <FileCard
                key={file.fullPath}
                file={file}
                isSelected={selectedFile === file.fullPath}
                onClick={() => handleFileClick(file)}
              />
            ))
          )}
        </div>
      </div>

      {/* Section divider */}
      <div className="h-px bg-gradient-to-r from-transparent via-base-300 to-transparent" />

      {/* Bookmarks section - secondary content area with scroll containment */}
      <div className="h-1/3 min-h-[120px] max-h-[200px] sm:max-h-none overflow-y-auto custom-scrollbar p-3 sm:p-4 bg-base-100 details-section-bookmarks" role="region" aria-label="Bookmarks list" tabIndex={0}>
        {/* Section header - matches Files header style with section-header-enhanced class */}
        <div className="flex items-center gap-2 mb-3 sticky top-0 bg-base-100/95 backdrop-blur-sm py-1.5 z-10 section-header-enhanced border-b-0">
          <div className="w-5 h-5 sm:w-6 sm:h-6 rounded-md sm:rounded-lg bg-primary/10 flex items-center justify-center flex-shrink-0">
            <Bookmark className="w-3 h-3 sm:w-3.5 sm:h-3.5 text-primary" />
          </div>
          <h2 className="text-xs sm:text-xs font-bold text-base-content/70 tracking-wide uppercase flex-1">Bookmarks</h2>
          {bookmarks.length > 0 && (
            <span className="badge badge-primary badge-sm px-1.5 sm:px-2 font-medium border-0 text-[11px]">{bookmarks.length}</span>
          )}
        </div>
        <div className="flex flex-col gap-1.5 sm:gap-2" id="bookmarks-list">
          {bookmarks.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-6 animate-fade-in">
              <div className="relative w-12 h-12 mb-3">
                <div className="absolute inset-0 rounded-xl bg-gradient-to-br from-primary/20 to-primary/5 animate-pulse" />
                <div className="absolute inset-1.5 rounded-xl bg-gradient-to-br from-base-100 to-base-200 flex items-center justify-center shadow-depth-2 border border-base-300/30">
                  <Bookmark className="w-5 h-5 text-primary/50" />
                </div>
              </div>
              <p className="text-sm font-semibold text-base-content/80 text-center">No bookmarks yet</p>
              <p className="text-xs text-base-content/50 text-center max-w-[180px]">Save your place in the audiobook for later</p>
              <div className="mt-3 flex items-center gap-1.5">
                <kbd className="kbd kbd-sm px-1.5 text-[11px] font-semibold bg-base-200 border-base-300 kbd-high-contrast">Ctrl</kbd>
                <span className="text-xs text-base-content/40">+</span>
                <kbd className="kbd kbd-sm px-1.5 text-[11px] font-semibold bg-base-200 border-base-300 kbd-high-contrast">B</kbd>
              </div>
            </div>
          ) : (
            bookmarks.map(bookmark => (
              <BookmarkCard key={bookmark.id ?? bookmark.createdAt} bookmark={bookmark} />
            ))
          )}
        </div>
      </div>
    </div>
  );
}

interface FileCardProps {
  file: AudiobookFile;
  isSelected: boolean;
  onClick: () => void;
}

function FileCard({ file, isSelected, onClick }: FileCardProps) {
  return (
    <div
      className={`card card-compact bg-base-100 cursor-pointer transition-all duration-200 card-elevated card-interactive ${
        isSelected
          ? 'ring-2 ring-primary shadow-md bg-primary/5 border-primary/30 file-card-selected'
          : 'border border-base-300/50 hover:border-primary/40 hover:shadow-sm'
      }`}
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => e.key === 'Enter' && onClick()}
      aria-label={`${file.name}, chapter ${file.position + 1}${file.completeness > 0 ? `, ${file.completeness}% complete` : ''}${isSelected ? ', selected' : ''}`}
      aria-pressed={isSelected}
    >
      <div className="card-body p-2.5 sm:p-3">
        <div className="flex items-center gap-2">
          <span className="text-[11px] sm:text-xs font-mono text-base-content/50 w-5 sm:w-6 shrink-0" aria-hidden="true">#{file.position + 1}</span>
          <span className="text-xs sm:text-sm font-medium line-clamp-2 flex-1 min-w-0" title={file.name}>{file.name}</span>
        </div>
        {file.completeness > 0 && (
          <div className="flex items-center gap-2 mt-1.5 sm:mt-2" role="progressbar" aria-valuenow={file.completeness} aria-valuemin={0} aria-valuemax={100} aria-label={`${file.completeness}% complete`}>
            <div className="flex-1 h-1 sm:h-1.5 bg-base-300/70 rounded-full overflow-hidden progress-bar-enhanced">
              <div
                className="h-full gradient-premium rounded-full transition-all duration-300"
                style={{ width: `${file.completeness}%` }}
              />
            </div>
            <span className="text-[11px] sm:text-xs text-base-content/60 tabular-nums w-7 sm:w-8 text-right shrink-0" aria-hidden="true">{file.completeness}%</span>
          </div>
        )}
      </div>
    </div>
  );
}

function BookmarkCard({ bookmark }: { bookmark: BookmarkType }) {
  const formatTime = (ms: number): string => {
    const totalSeconds = Math.floor(ms / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    if (hours > 0) {
      return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
  };

  return (
    <div
      className="card card-compact bg-base-100 border border-base-300/50 p-2.5 sm:p-3 transition-all duration-200 hover:shadow-md hover:border-primary/30 card-elevated card-interactive cursor-pointer group"
      role="button"
      tabIndex={0}
      aria-label={`${bookmark.label} at ${formatTime(bookmark.positionMs)}${bookmark.note ? `, note: ${bookmark.note}` : ''}`}
    >
      <div className="flex items-center justify-between gap-2 sm:gap-3">
        <span className="text-xs sm:text-sm font-medium line-clamp-2 flex-1 min-w-0 group-hover:text-primary transition-colors" title={bookmark.label}>{bookmark.label}</span>
        <span className="badge badge-primary/20 text-primary text-[11px] sm:text-xs font-mono shrink-0 border-primary/30" aria-label={`Position: ${formatTime(bookmark.positionMs)}`}>
          {formatTime(bookmark.positionMs)}
        </span>
      </div>
      {bookmark.note !== undefined && (
        <div className="text-[11px] sm:text-xs text-base-content/60 mt-1.5 sm:mt-2 border-l-2 border-primary/30 pl-2 italic truncate" title={bookmark.note}>
          {bookmark.note}
        </div>
      )}
    </div>
  );
}
