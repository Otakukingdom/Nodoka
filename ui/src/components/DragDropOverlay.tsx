/**
 * Drag-and-drop overlay for adding audiobook folders
 *
 * Provides a full-screen drop zone that appears when files are dragged
 * over the window, allowing users to add directories by dropping them.
 */

import { useState, useEffect, useCallback, useRef } from 'react';
import { FolderPlus } from 'lucide-react';

interface DragDropOverlayProps {
  /** Callback when a folder path is dropped */
  onDrop: (path: string) => void;
  /** Whether the overlay should be disabled (e.g., during loading) */
  isDisabled: boolean;
}

export default function DragDropOverlay({ onDrop, isDisabled }: DragDropOverlayProps) {
  const [isDragging, setIsDragging] = useState(false);
  const dragCounter = useRef(0);

  const handleDragEnter = useCallback(
    (e: DragEvent) => {
      if (isDisabled) return;
      e.preventDefault();
      dragCounter.current++;
      if (e.dataTransfer?.types.includes('Files') && dragCounter.current === 1) {
        setIsDragging(true);
      }
    },
    [isDisabled]
  );

  const handleDragLeave = useCallback((e: DragEvent) => {
    e.preventDefault();
    dragCounter.current--;
    if (dragCounter.current === 0) {
      setIsDragging(false);
    }
  }, []);

  const handleDragOver = useCallback((e: DragEvent) => {
    e.preventDefault();
  }, []);

  const handleDrop = useCallback(
    (e: DragEvent) => {
      e.preventDefault();
      setIsDragging(false);
      dragCounter.current = 0;

      if (isDisabled) return;

      const files = e.dataTransfer?.files;
      if (files && files.length > 0) {
        // In Tauri, the path property is available on File objects
        const item = files[0];
        // Access the path property that Tauri adds to File objects
        const path = (item as File & { path?: string }).path;
        if (path) {
          onDrop(path);
        }
      }
    },
    [onDrop, isDisabled]
  );

  useEffect(() => {
    window.addEventListener('dragenter', handleDragEnter);
    window.addEventListener('dragleave', handleDragLeave);
    window.addEventListener('dragover', handleDragOver);
    window.addEventListener('drop', handleDrop);
    return () => {
      window.removeEventListener('dragenter', handleDragEnter);
      window.removeEventListener('dragleave', handleDragLeave);
      window.removeEventListener('dragover', handleDragOver);
      window.removeEventListener('drop', handleDrop);
    };
  }, [handleDragEnter, handleDragLeave, handleDragOver, handleDrop]);

  if (!isDragging) return null;

  return (
    <div
      className="fixed inset-0 z-50 bg-primary/10 backdrop-blur-sm flex items-center justify-center border-4 border-dashed border-primary rounded-xl m-4 transition-all duration-200"
      role="status"
      aria-label="Drop folder to add to library"
      aria-live="polite"
    >
      <div className="text-center">
        <div className="relative w-24 h-24 mx-auto mb-4">
          <div className="absolute inset-0 rounded-full bg-primary/20 animate-ping" style={{ animationDuration: '1.5s' }} />
          <div className="absolute inset-0 rounded-full bg-gradient-to-br from-primary/30 to-primary/10 flex items-center justify-center shadow-lg">
            <FolderPlus className="w-12 h-12 text-primary animate-bounce" style={{ animationDuration: '1s' }} />
          </div>
        </div>
        <p className="text-xl font-semibold text-primary mb-2">Drop audiobook folder here</p>
        <p className="text-sm text-base-content/60">Your library will be updated automatically</p>
      </div>
    </div>
  );
}
