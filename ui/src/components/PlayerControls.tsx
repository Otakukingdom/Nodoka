/**
 * Player controls component at the bottom of the app
 */

import { Play, Pause, Square, SkipForward, SkipBack, Moon } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../hooks/useRedux';
import { cycleSpeed, toggleMute } from '../store/slices/playerSlice';
import { openModal } from '../store/slices/appSlice';
import { playPause, stop, seek, updateSpeed, updateVolume, seekForward, seekBackward } from '../store/thunks/playerThunks';
import VolumeSlider from './VolumeSlider';

// Default skip durations in seconds
const DEFAULT_SKIP_FORWARD = 30;
const DEFAULT_SKIP_BACKWARD = 15;

export default function PlayerControls() {
  const dispatch = useAppDispatch();
  const { playback, currentTime, totalDuration, speed, volume, isMuted, sleepTimer, sleepTimerRemainingMs } = useAppSelector(state => state.player);

  // Get skip durations from settings (could be stored in app state in future)
  const skipForwardDuration = DEFAULT_SKIP_FORWARD;
  const skipBackwardDuration = DEFAULT_SKIP_BACKWARD;

  const handlePlayPause = () => {
    void dispatch(playPause());
  };

  const handleStop = () => {
    void dispatch(stop());
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    void dispatch(seek(Number(e.target.value)));
  };

  const handleSpeedClick = () => {
    dispatch(cycleSpeed());
    void dispatch(updateSpeed());
  };

  const handleVolumeChange = (newVolume: number) => {
    void dispatch(updateVolume(newVolume));
  };

  const handleMuteToggle = () => {
    dispatch(toggleMute());
  };

  const handleSkipForward = () => {
    void dispatch(seekForward(skipForwardDuration));
  };

  const handleSkipBackward = () => {
    void dispatch(seekBackward(skipBackwardDuration));
  };

  const handleSleepTimer = () => {
    dispatch(openModal('SleepTimer'));
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
    <div
      className="btm-nav bg-base-200 border-t border-base-300 px-2 sm:px-4 py-2 sm:py-3 gap-2 sm:gap-4 shadow-[0_-4px_12px_rgba(0,0,0,0.08)]"
      role="region"
      aria-label="Audio player controls"
    >
      {/* Left controls */}
      <div className="flex items-center gap-1 sm:gap-2">
        {/* Skip backward button */}
        <button
          className="btn btn-circle btn-ghost btn-sm hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={handleSkipBackward}
          aria-label={`Skip backward ${skipBackwardDuration} seconds`}
          title={`Skip backward ${skipBackwardDuration}s (←)`}
        >
          <SkipBack className="h-4 w-4" />
        </button>
        {/* Stop button */}
        <button
          className="btn btn-circle btn-ghost btn-sm hover:bg-base-300 hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 hidden sm:flex"
          onClick={handleStop}
          aria-label="Stop"
          title="Stop playback"
        >
          <Square className="h-4 w-4" />
        </button>
        {/* Play/Pause button - focal point with enhanced styling */}
        <button
          className={`btn btn-circle btn-primary shadow-lg hover:shadow-xl hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 btn-press ${
            isPlaying ? 'pulse-playing' : ''
          }`}
          style={{
            background: 'linear-gradient(135deg, #E11D48 0%, #FB7185 100%)',
            border: 'none',
          }}
          onClick={handlePlayPause}
          aria-label={isPlaying ? 'Pause' : 'Play'}
          title={isPlaying ? 'Pause (Space)' : 'Play (Space)'}
        >
          {isPlaying ? (
            <Pause className="h-5 w-5" />
          ) : (
            <Play className="h-5 w-5 ml-0.5" />
          )}
        </button>
        {/* Skip forward button */}
        <button
          className="btn btn-circle btn-ghost btn-sm hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
          onClick={handleSkipForward}
          aria-label={`Skip forward ${skipForwardDuration} seconds`}
          title={`Skip forward ${skipForwardDuration}s (→)`}
        >
          <SkipForward className="h-4 w-4" />
        </button>
      </div>

      {/* Center - Seek bar */}
      <div className="flex-1 flex items-center gap-1 sm:gap-3">
        <span className="font-mono text-xs sm:text-sm w-10 sm:w-14 text-right tabular-nums text-base-content/70 select-none">
          {formatTime(currentTime)}
        </span>
        <div className="flex-1 relative group h-8 sm:h-6 flex items-center">
          {/* Background track with enhanced visibility */}
          <div className="absolute inset-x-0 h-2 top-1/2 -translate-y-1/2 rounded-full seek-bar-track transition-all duration-200 group-hover:h-3" />
          {/* Progress track with premium gradient */}
          <div
            className="absolute left-0 h-2 top-1/2 -translate-y-1/2 rounded-full seek-bar-fill transition-all duration-100 group-hover:h-3"
            style={{ width: `${progress}%` }}
          />
          <input
            type="range"
            min="0"
            max={totalDuration || 100}
            value={currentTime}
            onChange={handleSeek}
            className="range range-primary range-sm w-full seek-bar-styled relative z-10 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity cursor-pointer"
            aria-label="Seek"
          />
          {/* Visible thumb on hover/focus with enhanced shadow */}
          <div
            className="seek-bar-thumb opacity-0 group-hover:opacity-100 focus-within:opacity-100 pointer-events-none"
            style={{ left: `${progress}%` }}
          />
        </div>
        <span className="font-mono text-xs sm:text-sm w-10 sm:w-14 tabular-nums text-base-content/70 select-none">
          {formatTime(totalDuration)}
        </span>
      </div>

      {/* Right controls */}
      <div className="flex items-center gap-0.5 sm:gap-1">
        {/* Sleep Timer button with enhanced active state */}
        <button
          className={`btn btn-ghost btn-sm btn-circle hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 relative ${
            sleepTimer ? 'text-primary bg-primary/10 ring-1 ring-primary/30' : ''
          }`}
          onClick={handleSleepTimer}
          aria-label={sleepTimer ? `Sleep timer active: ${sleepTimerRemainingMs ? formatTime(sleepTimerRemainingMs) : 'waiting'}` : 'Set sleep timer'}
          title={sleepTimer ? `Sleep timer: ${sleepTimerRemainingMs ? formatTime(sleepTimerRemainingMs) : 'End of chapter'}` : 'Sleep timer (Ctrl+T)'}
        >
          <Moon className="h-4 w-4" />
          {sleepTimer && sleepTimerRemainingMs && (
            <span className="absolute -top-1 -right-1 text-[8px] font-mono bg-primary text-primary-content rounded-full px-1 shadow-sm">
              {Math.floor(sleepTimerRemainingMs / 60000)}
            </span>
          )}
        </button>
        {/* Speed button with enhanced visual feedback */}
        <button
          className="btn btn-ghost btn-sm px-2 sm:px-3 font-mono text-xs sm:text-sm hover:bg-primary/10 hover:text-primary hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 group relative border border-transparent hover:border-primary/20 rounded-full"
          onClick={handleSpeedClick}
          aria-label={`Playback speed: ${speed}x. Click to cycle`}
          title="Playback speed (click to cycle)"
        >
          <span className={`${speed !== 1.0 ? 'text-primary font-semibold' : ''} group-hover:scale-110 inline-block transition-transform`}>
            {speed}x
          </span>
          <span className="absolute -bottom-0.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full bg-primary/50 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
        <VolumeSlider
          volume={volume}
          isMuted={isMuted}
          onVolumeChange={handleVolumeChange}
          onMuteToggle={handleMuteToggle}
        />
      </div>
    </div>
  );
}
