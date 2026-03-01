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
    <div className="flex flex-col flex-1 overflow-hidden">
      {/* Files section */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4">
        <h2 className="text-lg font-bold mb-4">Files</h2>
        <div className="flex flex-col gap-2" id="files-list">
          {currentFiles.length === 0 ? (
            <div className="text-center text-base-content/60 py-8">No files available</div>
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
      <div className="h-1/3 overflow-y-auto custom-scrollbar p-4 border-t border-base-300">
        <h2 className="text-lg font-bold mb-4">Bookmarks</h2>
        <div className="flex flex-col gap-2" id="bookmarks-list">
          {bookmarks.length === 0 ? (
            <div className="text-center text-base-content/60 py-4">No bookmarks</div>
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
      className={`card card-compact bg-base-100 shadow-sm cursor-pointer transition-smooth ${
        isSelected ? 'ring-1 ring-primary' : 'hover:shadow-md'
      }`}
      onClick={onClick}
    >
      <div className="card-body p-2">
        <div className="text-sm">#{file.position + 1} {file.name}</div>
        {file.completeness > 0 && (
          <progress
            className="progress progress-xs w-full mt-1"
            value={file.completeness}
            max={100}
          />
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
    <div className="card card-compact bg-base-100 shadow-sm p-2">
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium">{bookmark.label}</span>
        <span className="badge badge-ghost text-xs">{formatTime(bookmark.positionMs)}</span>
      </div>
      {bookmark.note !== undefined && (
        <div className="text-xs text-base-content/60 mt-1">{bookmark.note}</div>
      )}
    </div>
  );
}
