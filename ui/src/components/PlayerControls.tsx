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
  const {
    playback,
    currentTime,
    totalDuration,
    speed,
    volume,
    isMuted,
    sleepTimer,
    sleepTimerRemainingMs,
    skipForwardDurationSeconds,
    skipBackwardDurationSeconds,
  } = useAppSelector(state => state.player);

  const skipForwardDuration = skipForwardDurationSeconds || DEFAULT_SKIP_FORWARD;
  const skipBackwardDuration = skipBackwardDurationSeconds || DEFAULT_SKIP_BACKWARD;

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
      className="btm-nav bg-base-200 border-t border-base-300 px-2 sm:px-4 py-2 sm:py-3 gap-0.5 sm:gap-2 md:gap-3 shadow-[0_-4px_12px_rgba(0,0,0,0.08)] overflow-x-hidden player-controls-bar"
      role="region"
      aria-label="Audio player controls"
    >
      {/* Screen reader time announcements */}
      <div className="sr-only" aria-live="polite" aria-atomic="true">
        {isPlaying && `Playing: ${formatTime(currentTime)} of ${formatTime(totalDuration)}`}
      </div>

      {/* Left controls - playback navigation group with clear grouping */}
      <div className="flex items-center gap-0.5 sm:gap-1 flex-shrink-0 controls-group navigation-controls" role="group" aria-label="Playback navigation">
        {/* Skip backward button - clear affordance */}
        <button
          className="btn btn-circle btn-ghost btn-sm sm:btn-md hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 touch-target-min"
          onClick={handleSkipBackward}
          aria-label={`Skip backward ${skipBackwardDuration} seconds`}
          title={`Skip backward ${skipBackwardDuration}s (←)`}
        >
          <SkipBack className="h-4 w-4 sm:h-5 sm:w-5" />
        </button>
        {/* Stop button - desktop only for space efficiency */}
        <button
          className="btn btn-circle btn-ghost btn-sm hover:bg-base-300 hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 hidden md:flex touch-target-min"
          onClick={handleStop}
          aria-label="Stop"
          title="Stop playback"
        >
          <Square className="h-4 w-4" />
        </button>
      </div>

      {/* Play/Pause button - PRIMARY ACTION with maximum visual prominence */}
      <button
        className={`btn btn-circle btn-primary shadow-lg hover:shadow-xl hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 btn-press flex-shrink-0 play-button-primary ${
          isPlaying ? 'pulse-playing' : ''
        }`}
        style={{
          background: 'linear-gradient(135deg, #E11D48 0%, #FB7185 100%)',
          border: 'none',
          minWidth: '44px',
          minHeight: '44px',
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

      {/* Right navigation - skip forward */}
      <div className="flex items-center gap-0.5 sm:gap-1 flex-shrink-0 controls-group" role="group" aria-label="Skip forward">
        {/* Skip forward button */}
        <button
          className="btn btn-circle btn-ghost btn-sm sm:btn-md hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 touch-target-min"
          onClick={handleSkipForward}
          aria-label={`Skip forward ${skipForwardDuration} seconds`}
          title={`Skip forward ${skipForwardDuration}s (→)`}
        >
          <SkipForward className="h-4 w-4 sm:h-5 sm:w-5" />
        </button>
      </div>

      {/* Center - Seek bar (flexible with min-width:0 to allow shrinking) */}
      <div className="flex-1 min-w-0 flex items-center gap-1 sm:gap-2 seek-bar-container">
        {/* Current time - readable monospace */}
        <span className="font-mono text-[11px] sm:text-xs md:text-sm w-9 sm:w-11 md:w-12 text-right tabular-nums text-base-content/70 select-none flex-shrink-0 time-display" aria-label={`Current time: ${formatTime(currentTime)}`}>
          {formatTime(currentTime)}
        </span>
        {/* Seek bar with enhanced interaction clarity */}
        <div className="flex-1 min-w-0 relative group h-8 sm:h-6 flex items-center seek-bar-wrapper" role="group" aria-label="Seek bar">
          {/* Background track with enhanced visibility */}
          <div className="absolute inset-x-0 h-2 top-1/2 -translate-y-1/2 rounded-full seek-bar-track transition-all duration-200 group-hover:h-3" aria-hidden="true" />
          {/* Progress track with premium gradient */}
          <div
            className="absolute left-0 h-2 top-1/2 -translate-y-1/2 rounded-full seek-bar-fill transition-all duration-100 group-hover:h-3"
            style={{ width: `${progress}%` }}
            role="progressbar"
            aria-valuenow={Math.round(progress)}
            aria-valuemin={0}
            aria-valuemax={100}
            aria-label={`${Math.round(progress)}% complete`}
          />
          <input
            type="range"
            min="0"
            max={totalDuration || 100}
            value={currentTime}
            onChange={handleSeek}
            className="range range-primary range-sm w-full seek-bar-styled relative z-10 cursor-pointer"
            aria-label="Seek"
          />
          {/* Visible thumb on hover/focus with enhanced shadow */}
          <div
            className="seek-bar-thumb opacity-0 group-hover:opacity-100 focus-within:opacity-100 pointer-events-none"
            style={{ left: `${progress}%` }}
          />
        </div>
        {/* Total duration */}
        <span className="font-mono text-[10px] sm:text-xs md:text-sm w-8 sm:w-10 md:w-12 tabular-nums text-base-content/70 select-none flex-shrink-0 time-display" aria-label={`Total duration: ${formatTime(totalDuration)}`}>
          {formatTime(totalDuration)}
        </span>
      </div>

      {/* Right controls - settings and volume group with clear grouping */}
      <div className="flex items-center gap-0.5 sm:gap-1 flex-shrink-0 controls-group settings-controls" role="group" aria-label="Player settings">
        {/* Sleep Timer button with enhanced active state */}
        <button
          className={`btn btn-ghost btn-sm btn-circle hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 relative touch-target-min ${
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
        {/* Speed button with enhanced visual feedback and clear affordance */}
        <button
          className="btn btn-ghost btn-sm px-2 sm:px-2.5 font-mono text-[11px] sm:text-xs md:text-sm hover:bg-primary/10 hover:text-primary hover:scale-105 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 group relative border border-transparent hover:border-primary/20 rounded-full min-w-[2.75rem] sm:min-w-[3rem] md:min-w-[3.5rem] speed-control touch-target-min"
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
