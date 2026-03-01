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

  return (
    <div className="btm-nav bg-base-200 border-t border-base-300 px-4 py-3">
      <div className="flex items-center gap-2">
        <button
          className="btn btn-circle btn-ghost btn-sm"
          onClick={handleStop}
          aria-label="Stop"
        >
          <Square className="h-5 w-5" />
        </button>
        <button
          className="btn btn-circle btn-primary"
          onClick={handlePlayPause}
          aria-label={playback === 'Playing' ? 'Pause' : 'Play'}
        >
          {playback === 'Playing' ? (
            <Pause className="h-6 w-6" />
          ) : (
            <Play className="h-6 w-6" />
          )}
        </button>
      </div>

      <div className="flex-1 mx-4 flex items-center gap-2">
        <span className="font-mono text-sm w-16 text-right">{formatTime(currentTime)}</span>
        <input
          type="range"
          min="0"
          max={totalDuration || 100}
          value={currentTime}
          onChange={handleSeek}
          className="range range-primary flex-1"
          aria-label="Seek"
        />
        <span className="font-mono text-sm w-16">{formatTime(totalDuration)}</span>
      </div>

      <div className="flex items-center gap-2">
        <button className="btn btn-ghost btn-sm" onClick={handleSpeedClick}>
          {speed}x
        </button>
        <button
          className="btn btn-ghost btn-sm btn-circle"
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
