/**
 * Details panel component showing files and bookmarks
 */

import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { selectFile } from '../store/thunks/libraryThunks';
import type { AudiobookFile, Bookmark } from '../store/types';

export default function DetailsPanel() {
  const { currentFiles, selectedFile, bookmarks } = useAppSelector(state => state.library);
  const dispatch = useAppDispatch();

  const handleFileClick = (file: AudiobookFile) => {
    dispatch(selectFile(file.fullPath));
  };

  return (
    <div className="flex flex-col flex-1 overflow-hidden bg-base-100">
      {/* Files section */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4">
        <h2 className="text-lg font-bold mb-4 text-base-content/80">Files</h2>
        <div className="flex flex-col gap-2" id="files-list">
          {currentFiles.length === 0 ? (
            <div className="text-center py-12 px-4">
              <div className="text-4xl mb-3 opacity-60">📂</div>
              <div className="text-base-content/60">No files available</div>
              <div className="text-sm text-base-content/40 mt-1">Select an audiobook to view files</div>
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

      {/* Bookmarks section */}
      <div className="h-1/3 min-h-[150px] overflow-y-auto custom-scrollbar p-4 border-t border-base-300 bg-base-200/30">
        <h2 className="text-lg font-bold mb-4 text-base-content/80">Bookmarks</h2>
        <div className="flex flex-col gap-2" id="bookmarks-list">
          {bookmarks.length === 0 ? (
            <div className="text-center py-8 px-4">
              <div className="text-3xl mb-2 opacity-60">🔖</div>
              <div className="text-sm text-base-content/60">No bookmarks yet</div>
              <div className="text-xs text-base-content/40 mt-1">Press Ctrl+B to create a bookmark</div>
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
      className={`card card-compact bg-base-100 shadow-sm cursor-pointer transition-all duration-200 hover:shadow-md ${
        isSelected
          ? 'ring-2 ring-primary shadow-md bg-primary/5'
          : 'border border-base-300/50 hover:border-primary/30'
      }`}
      onClick={onClick}
    >
      <div className="card-body p-3">
        <div className="flex items-center gap-2">
          <span className="text-xs font-mono text-base-content/50 w-6">#{file.position + 1}</span>
          <span className="text-sm font-medium text-truncate flex-1">{file.name}</span>
        </div>
        {file.completeness > 0 && (
          <div className="flex items-center gap-2 mt-2">
            <div className="flex-1 h-1.5 bg-base-200 rounded-full overflow-hidden">
              <div
                className="h-full gradient-premium rounded-full transition-all duration-300"
                style={{ width: `${file.completeness}%` }}
              />
            </div>
            <span className="text-xs text-base-content/50 tabular-nums">{file.completeness}%</span>
          </div>
        )}
      </div>
    </div>
  );
}

interface BookmarkCardProps {
  bookmark: Bookmark;
}

function BookmarkCard({ bookmark }: BookmarkCardProps) {
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
    <div className="card card-compact bg-base-100 shadow-sm border border-base-300/50 p-3 transition-all duration-200 hover:shadow-md">
      <div className="flex items-center justify-between gap-2">
        <span className="text-sm font-medium text-truncate">{bookmark.label}</span>
        <span className="badge badge-primary/20 text-primary text-xs font-mono shrink-0">
          {formatTime(bookmark.positionMs)}
        </span>
      </div>
      {bookmark.note !== undefined && (
        <div className="text-xs text-base-content/60 mt-2 pl-0.5 border-l-2 border-primary/30 pl-2">
          {bookmark.note}
        </div>
      )}
    </div>
  );
}
