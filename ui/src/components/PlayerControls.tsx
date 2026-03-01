/**
 * Player controls component at the bottom of the app
 */

import { Play, Pause, Square, Volume2 } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { cycleSpeed } from '../store/slices/playerSlice';
import { playPause, stop, seek, updateSpeed } from '../store/thunks/playerThunks';
import { toggleVolumeExpanded } from '../store/slices/playerSlice';

export default function PlayerControls() {
  const dispatch = useAppDispatch();
  const { playback, currentTime, totalDuration, speed } = useAppSelector(state => state.player);

  const handlePlayPause = () => {
    dispatch(playPause());
  };

  const handleStop = () => {
    dispatch(stop());
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(seek(Number(e.target.value)));
  };

  const handleSpeedClick = () => {
    dispatch(cycleSpeed());
    const state = store.getState();
    dispatch(updateSpeed(state.player.speed));
  };

  const handleVolumeClick = () => {
    dispatch(toggleVolumeExpanded());
  };

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

  const isPlaying = playback === 'Playing';
  const progress = totalDuration > 0 ? (currentTime / totalDuration) * 100 : 0;

  return (
    <div className="btm-nav bg-base-200 border-t border-base-300 px-4 py-3 gap-4">
      {/* Left controls */}
      <div className="flex items-center gap-2">
        <button
          className="btn btn-circle btn-ghost btn-sm hover:bg-base-300 transition-colors duration-200"
          onClick={handleStop}
          aria-label="Stop"
        >
          <Square className="h-4 w-4" />
        </button>
        <button
          className={`btn btn-circle btn-primary shadow-md transition-all duration-200 ${
            isPlaying ? 'pulse-playing' : ''
          }`}
          onClick={handlePlayPause}
          aria-label={isPlaying ? 'Pause' : 'Play'}
        >
          {isPlaying ? (
            <Pause className="h-5 w-5" />
          ) : (
            <Play className="h-5 w-5 ml-0.5" />
          )}
        </button>
      </div>

      {/* Center - Seek bar */}
      <div className="flex-1 flex items-center gap-3">
        <span className="font-mono text-sm w-14 text-right tabular-nums">{formatTime(currentTime)}</span>
        <div className="flex-1 relative">
          <input
            type="range"
            min="0"
            max={totalDuration || 100}
            value={currentTime}
            onChange={handleSeek}
            className="range range-primary range-sm w-full seek-bar-styled"
            aria-label="Seek"
            style={{
              background: `linear-gradient(to right, var(--fallback-p, #E11D48) 0%, var(--fallback-p, #FB7185) ${progress}%, var(--fallback-b3, #d1d5db) ${progress}%, var(--fallback-b3, #d1d5db) 100%)`,
            }}
          />
        </div>
        <span className="font-mono text-sm w-14 tabular-nums">{formatTime(totalDuration)}</span>
      </div>

      {/* Right controls */}
      <div className="flex items-center gap-1">
        <button
          className="btn btn-ghost btn-sm px-3 font-mono text-sm hover:bg-primary/10 transition-colors duration-200"
          onClick={handleSpeedClick}
          title="Playback speed"
        >
          {speed}x
        </button>
        <button
          className="btn btn-ghost btn-sm btn-circle hover:bg-primary/10 transition-colors duration-200"
          onClick={handleVolumeClick}
          aria-label="Volume"
        >
          <Volume2 className="h-5 w-5" />
        </button>
      </div>
    </div>
  );
}

// Import store for getting current speed value
import { store } from '../store';
