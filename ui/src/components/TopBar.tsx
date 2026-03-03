/**
 * Top bar component with logo and settings
 */

import { useState } from 'react';
import { Settings, FolderPlus } from 'lucide-react';
import { useAppDispatch, useAppSelector } from '../hooks/useRedux';
import { openModal } from '../store/slices/appSlice';
import { addDirectory } from '../store/thunks/libraryThunks';

export default function TopBar() {
  const dispatch = useAppDispatch();
  const { isLibraryLoading, directories } = useAppSelector(state => state.library);
  const [isAddingDirectory, setIsAddingDirectory] = useState(false);

  const handleSettingsClick = () => {
    dispatch(openModal('Settings'));
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

  return (
    <div className="navbar bg-base-200 border-b border-base-300 px-4 shadow-sm" role="banner">
      <div className="flex items-center gap-3">
        {/* Logo with enhanced gradient and subtle glow */}
        <div
          className="w-9 h-9 rounded-full gradient-premium flex items-center justify-center text-white text-base shadow-lg logo-hover-pulse cursor-pointer transition-transform duration-200 hover:scale-105"
          style={{
            boxShadow: '0 4px 12px rgba(225, 29, 72, 0.3), 0 2px 4px rgba(0, 0, 0, 0.1)',
          }}
        >
          <span className="drop-shadow-sm">♪</span>
        </div>
        <div className="flex flex-col">
          <span className="text-lg font-bold tracking-tight bg-gradient-to-r from-primary to-primary/70 bg-clip-text text-transparent">Nodoka</span>
          <span className="text-xs text-base-content/60 leading-tight">Audiobook Reader</span>
        </div>
      </div>
      <div className="flex-1" />
      <div className="flex items-center gap-1">
        <button
          className="btn btn-ghost btn-circle group hover:bg-primary/10 hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={() => void handleAddDirectory()}
          disabled={isAddingDirectory || isLibraryLoading}
          aria-label="Add audiobook folder"
          title="Add Folder (Ctrl+D)"
        >
          <FolderPlus className={`h-5 w-5 transition-transform duration-200 group-hover:scale-110 ${isAddingDirectory ? 'animate-pulse' : ''}`} />
        </button>
        <button
          className="btn btn-ghost btn-circle group hover:bg-primary/10 hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 relative"
          onClick={handleSettingsClick}
          aria-label="Settings"
          title="Settings (Ctrl+,)"
        >
          <Settings className="h-5 w-5 transition-transform duration-200 group-hover:rotate-45" />
          {directories.length > 0 && (
            <span className="badge badge-xs badge-primary absolute -top-1 -right-1 px-1.5 font-medium">
              {directories.length}
            </span>
          )}
        </button>
      </div>
    </div>
  );
}
