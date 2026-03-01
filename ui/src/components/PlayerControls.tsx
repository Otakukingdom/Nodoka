/**
 * Player controls component at the bottom of the app
 */

import { Play, Pause, Square, Volume2 } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { cycleSpeed } from '../store/slices/playerSlice';
import { playPause, stop, seek, updateSpeed } from '../store/thunks/playerThunks';
import { toggleVolumeExpanded } from '../store/slices/playerSlice';
import { store } from '../store';

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
          className="btn btn-circle btn-ghost btn-sm hover:bg-base-300 hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
          onClick={handleStop}
          aria-label="Stop"
        >
          <Square className="h-4 w-4" />
        </button>
        <button
          className={`btn btn-circle btn-primary shadow-lg hover:shadow-xl hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 ${
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
        <span className="font-mono text-sm w-14 text-right tabular-nums text-base-content/70">
          {formatTime(currentTime)}
        </span>
        <div className="flex-1 relative group">
          {/* Background track */}
          <div className="absolute inset-0 h-1.5 top-1/2 -translate-y-1/2 rounded-full bg-base-300" />
          {/* Progress track */}
          <div
            className="absolute left-0 h-1.5 top-1/2 -translate-y-1/2 rounded-full gradient-premium transition-all duration-100"
            style={{ width: `${progress}%` }}
          />
          <input
            type="range"
            min="0"
            max={totalDuration || 100}
            value={currentTime}
            onChange={handleSeek}
            className="range range-primary range-sm w-full seek-bar-styled relative z-10 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity"
            aria-label="Seek"
          />
          {/* Visible thumb on hover/focus */}
          <div
            className="absolute top-1/2 -translate-y-1/2 w-3.5 h-3.5 rounded-full bg-primary shadow-md opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"
            style={{ left: `calc(${progress}% - 7px)` }}
          />
        </div>
        <span className="font-mono text-sm w-14 tabular-nums text-base-content/70">
          {formatTime(totalDuration)}
        </span>
      </div>

      {/* Right controls */}
      <div className="flex items-center gap-1">
        <button
          className="btn btn-ghost btn-sm px-3 font-mono text-sm hover:bg-primary/10 hover:text-primary transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
          onClick={handleSpeedClick}
          title="Playback speed (click to cycle)"
        >
          <span className={speed !== 1.0 ? 'text-primary font-semibold' : ''}>{speed}x</span>
        </button>
        <button
          className="btn btn-ghost btn-sm btn-circle hover:bg-primary/10 hover:text-primary hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
          onClick={handleVolumeClick}
          aria-label="Volume"
        >
          <Volume2 className="h-5 w-5" />
        </button>
      </div>
    </div>
  );
}
