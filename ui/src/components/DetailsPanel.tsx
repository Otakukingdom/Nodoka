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
    <div className="flex flex-col flex-1 overflow-hidden bg-base-100 details-panel-container" role="complementary" aria-label="File and bookmark details">
      {/* Files section */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4">
        <div className="flex items-center gap-2 mb-3">
          <div className="w-6 h-6 rounded-md bg-primary/15 flex items-center justify-center">
            <FileText className="w-3.5 h-3.5 text-primary" />
          </div>
          <h2 className="text-sm font-bold text-base-content/90 tracking-wide uppercase">Files</h2>
          {currentFiles.length > 0 && (
            <span className="text-xs text-primary/70 ml-auto font-semibold">{currentFiles.length} files</span>
          )}
        </div>
        <div className="flex flex-col gap-2" id="files-list">
          {currentFiles.length === 0 ? (
            <div className="text-center py-8 px-4 animate-fade-in">
              <div className="relative w-14 h-14 mx-auto mb-3">
                <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/15 to-primary/5" />
                <div className="absolute inset-1.5 rounded-full bg-gradient-to-br from-base-200 to-base-300 flex items-center justify-center shadow-depth-1">
                  <FileText className="w-5 h-5 text-primary/60" />
                </div>
              </div>
              <div className="text-sm text-base-content/80 font-semibold">No files available</div>
              <div className="text-xs text-base-content/60 mt-1 leading-relaxed">Select an audiobook to view chapters</div>
              <div className="mt-4 flex items-center justify-center">
                <div className="select-hint-prominent cursor-default text-xs">
                  <span className="text-primary font-semibold">←</span>
                  <span className="font-medium">Click an audiobook</span>
                </div>
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

      {/* Bookmarks section */}
      <div className="h-1/3 min-h-[120px] overflow-y-auto custom-scrollbar p-4 bg-base-200/30">
        <div className="flex items-center gap-2 mb-3">
          <div className="w-6 h-6 rounded-md bg-primary/15 flex items-center justify-center">
            <Bookmark className="w-3.5 h-3.5 text-primary" />
          </div>
          <h2 className="text-sm font-bold text-base-content/90 tracking-wide uppercase">Bookmarks</h2>
          {bookmarks.length > 0 && (
            <span className="text-xs text-primary/70 ml-auto font-semibold">{bookmarks.length}</span>
          )}
        </div>
        <div className="flex flex-col gap-2" id="bookmarks-list">
          {bookmarks.length === 0 ? (
            <div className="text-center py-6 px-4 animate-fade-in">
              <div className="relative w-12 h-12 mx-auto mb-2">
                <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/15 to-primary/5" />
                <div className="absolute inset-1.5 rounded-full bg-gradient-to-br from-base-200 to-base-300 flex items-center justify-center shadow-depth-1">
                  <Bookmark className="w-4 h-4 text-primary/60" />
                </div>
              </div>
              <div className="text-xs text-base-content/70 font-semibold">No bookmarks yet</div>
              <div className="text-xs text-base-content/55 mt-1 leading-relaxed">
                Save your place during playback
              </div>
              <div className="mt-2 inline-flex items-center gap-1 text-xs text-base-content/50 bg-base-200/50 px-2 py-1 rounded-full">
                <span>Press</span>
                <kbd className="kbd kbd-xs px-1 bg-base-300 border-base-400 font-mono font-medium" style={{ color: '#111827' }}>Ctrl</kbd>
                <span>+</span>
                <kbd className="kbd kbd-xs px-1 bg-base-300 border-base-400 font-mono font-medium" style={{ color: '#111827' }}>B</kbd>
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
          ? 'ring-[3px] ring-primary shadow-md bg-primary/5 border-primary/30 file-card-selected'
          : 'border border-base-300/50 hover:border-primary/40'
      }`}
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => e.key === 'Enter' && onClick()}
      aria-label={`${file.name}, chapter ${file.position + 1}${file.completeness > 0 ? `, ${file.completeness}% complete` : ''}${isSelected ? ', selected' : ''}`}
      aria-pressed={isSelected}
    >
      <div className="card-body p-3">
        <div className="flex items-center gap-2">
          <span className="text-xs font-mono text-base-content/50 w-6" aria-hidden="true">#{file.position + 1}</span>
          <span className="text-sm font-medium text-truncate flex-1">{file.name}</span>
        </div>
        {file.completeness > 0 && (
          <div className="flex items-center gap-2 mt-2" role="progressbar" aria-valuenow={file.completeness} aria-valuemin={0} aria-valuemax={100} aria-label={`${file.completeness}% complete`}>
            <div className="flex-1 h-1.5 bg-base-300/70 rounded-full overflow-hidden progress-bar-enhanced">
              <div
                className="h-full gradient-premium rounded-full transition-all duration-300"
                style={{ width: `${file.completeness}%` }}
              />
            </div>
            <span className="text-xs text-base-content/60 tabular-nums w-8 text-right" aria-hidden="true">{file.completeness}%</span>
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
      className="card card-compact bg-base-100 border border-base-300/50 p-3 transition-all duration-200 hover:shadow-md hover:border-primary/30 card-elevated card-interactive cursor-pointer group"
      role="button"
      tabIndex={0}
      aria-label={`${bookmark.label} at ${formatTime(bookmark.positionMs)}${bookmark.note ? `, note: ${bookmark.note}` : ''}`}
    >
      <div className="flex items-center justify-between gap-3">
        <span className="text-sm font-medium truncate flex-1 min-w-0 group-hover:text-primary transition-colors" title={bookmark.label}>{bookmark.label}</span>
        <span className="badge badge-primary/20 text-primary text-xs font-mono shrink-0 border-primary/30 min-w-fit" aria-label={`Position: ${formatTime(bookmark.positionMs)}`}>
          {formatTime(bookmark.positionMs)}
        </span>
      </div>
      {bookmark.note !== undefined && (
        <div className="text-xs text-base-content/60 mt-2 pl-0.5 border-l-2 border-primary/30 pl-2 italic truncate" title={bookmark.note}>
          {bookmark.note}
        </div>
      )}
    </div>
  );
}
