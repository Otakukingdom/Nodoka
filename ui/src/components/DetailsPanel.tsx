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
    <div className="flex flex-col flex-1 overflow-hidden bg-base-100">
      {/* Files section */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4">
        <div className="flex items-center gap-2 mb-4">
          <div className="w-6 h-6 rounded-md bg-primary/10 flex items-center justify-center">
            <FileText className="w-3.5 h-3.5 text-primary" />
          </div>
          <h2 className="text-base font-semibold text-base-content/80">Files</h2>
          {currentFiles.length > 0 && (
            <span className="text-xs text-base-content/50 ml-auto">{currentFiles.length} files</span>
          )}
        </div>
        <div className="flex flex-col gap-2" id="files-list">
          {currentFiles.length === 0 ? (
            <div className="text-center py-12 px-4">
              <div className="w-12 h-12 mx-auto mb-3 rounded-full bg-gradient-to-br from-base-200 to-base-300 flex items-center justify-center shadow-inner">
                <FileText className="w-6 h-6 text-base-content/40" />
              </div>
              <div className="text-base-content/60 font-medium">No files available</div>
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

      {/* Section divider */}
      <div className="h-px bg-gradient-to-r from-transparent via-base-300 to-transparent" />

      {/* Bookmarks section */}
      <div className="h-1/3 min-h-[150px] overflow-y-auto custom-scrollbar p-4 bg-base-200/20">
        <div className="flex items-center gap-2 mb-4">
          <div className="w-6 h-6 rounded-md bg-primary/10 flex items-center justify-center">
            <Bookmark className="w-3.5 h-3.5 text-primary" />
          </div>
          <h2 className="text-base font-semibold text-base-content/80">Bookmarks</h2>
          {bookmarks.length > 0 && (
            <span className="text-xs text-base-content/50 ml-auto">{bookmarks.length}</span>
          )}
        </div>
        <div className="flex flex-col gap-2" id="bookmarks-list">
          {bookmarks.length === 0 ? (
            <div className="text-center py-8 px-4">
              <div className="w-10 h-10 mx-auto mb-2 rounded-full bg-gradient-to-br from-base-200 to-base-300 flex items-center justify-center shadow-inner">
                <Bookmark className="w-5 h-5 text-base-content/40" />
              </div>
              <div className="text-sm text-base-content/60 font-medium">No bookmarks yet</div>
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
      className={`card card-compact bg-base-100 cursor-pointer transition-all duration-200 card-elevated ${
        isSelected
          ? 'ring-2 ring-primary shadow-md bg-primary/5 border-primary/30'
          : 'border border-base-300/50 hover:border-primary/30'
      }`}
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => e.key === 'Enter' && onClick()}
    >
      <div className="card-body p-3">
        <div className="flex items-center gap-2">
          <span className="text-xs font-mono text-base-content/40 w-6">#{file.position + 1}</span>
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
            <span className="text-xs text-base-content/50 tabular-nums w-8 text-right">{file.completeness}%</span>
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
    <div className="card card-compact bg-base-100 border border-base-300/50 p-3 transition-all duration-200 hover:shadow-md hover:border-primary/20 card-elevated cursor-pointer group">
      <div className="flex items-center justify-between gap-2">
        <span className="text-sm font-medium text-truncate group-hover:text-primary transition-colors">{bookmark.label}</span>
        <span className="badge badge-primary/20 text-primary text-xs font-mono shrink-0 border-primary/30">
          {formatTime(bookmark.positionMs)}
        </span>
      </div>
      {bookmark.note !== undefined && (
        <div className="text-xs text-base-content/60 mt-2 pl-0.5 border-l-2 border-primary/30 pl-2 italic">
          {bookmark.note}
        </div>
      )}
    </div>
  );
}
