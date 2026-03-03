/**
 * Directory error banner component
 *
 * Displays when a directory has accessibility issues (not found, permission denied)
 * and provides actions to resolve the issue (locate folder, remove from library).
 */

import { useState } from 'react';
import { AlertTriangle, FolderSearch, Trash2 } from 'lucide-react';
import type { Directory } from '../store/types';

interface DirectoryErrorBannerProps {
  /** The directory with an error */
  directory: Directory;
  /** Callback to locate a new folder path */
  onLocate: (oldPath: string, newPath: string) => void;
  /** Callback to remove the directory */
  onRemove: (path: string) => void;
  /** Whether an operation is in progress */
  isLoading?: boolean;
}

/**
 * Get human-readable error message for directory errors
 */
function getErrorMessage(error: string | undefined): string {
  switch (error) {
    case 'NotFound':
      return 'This folder has been moved or deleted';
    case 'PermissionDenied':
      return 'Permission denied - check folder access';
    default:
      return 'Unable to access this folder';
  }
}

export default function DirectoryErrorBanner({
  directory,
  onLocate,
  onRemove,
  isLoading = false,
}: DirectoryErrorBannerProps) {
  const [isLocating, setIsLocating] = useState(false);

  const folderName = directory.fullPath.split(/[/\\]/).pop() ?? directory.fullPath;

  const handleLocate = async () => {
    setIsLocating(true);
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
    } finally {
      setIsLocating(false);
    }
  };

  return (
    <div
      className="alert alert-warning shadow-sm mb-2"
      role="alert"
      aria-label={`Error for folder ${folderName}`}
    >
      <AlertTriangle className="w-5 h-5 shrink-0" />
      <div className="flex-1 min-w-0">
        <p className="font-medium truncate" title={directory.fullPath}>
          {folderName}
        </p>
        <p className="text-sm opacity-80">{getErrorMessage(directory.error)}</p>
      </div>
      <div className="flex gap-2 shrink-0">
        {directory.error === 'NotFound' && (
          <button
            className="btn btn-sm btn-ghost"
            onClick={() => void handleLocate()}
            disabled={isLoading || isLocating}
            aria-label="Locate folder"
          >
            {isLocating ? (
              <span className="loading loading-spinner loading-xs" />
            ) : (
              <FolderSearch className="w-4 h-4" />
            )}
            <span className="hidden sm:inline">Locate</span>
          </button>
        )}
        <button
          className="btn btn-sm btn-ghost text-error hover:bg-error/10"
          onClick={() => onRemove(directory.fullPath)}
          disabled={isLoading}
          aria-label="Remove folder from library"
        >
          <Trash2 className="w-4 h-4" />
          <span className="hidden sm:inline">Remove</span>
        </button>
      </div>
    </div>
  );
}
