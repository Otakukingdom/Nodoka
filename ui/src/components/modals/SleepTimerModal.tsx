/**
 * Sleep timer modal component for setting playback sleep timers
 */

import { X, Moon, Timer, BookOpen } from 'lucide-react';
import { useAppSelector, useAppDispatch } from '../../hooks/useRedux';
import { closeModal } from '../../store/slices/appSlice';
import { setSleepTimer, clearSleepTimer, setFadeDuration } from '../../store/slices/playerSlice';

const DURATION_OPTIONS = [
  { label: '15 minutes', value: 15 },
  { label: '30 minutes', value: 30 },
  { label: '60 minutes', value: 60 },
  { label: '90 minutes', value: 90 },
];

const FADE_OPTIONS = [
  { label: 'None', value: 0 },
  { label: '5 seconds', value: 5 },
  { label: '10 seconds', value: 10 },
  { label: '15 seconds', value: 15 },
];

export default function SleepTimerModal() {
  const dispatch = useAppDispatch();
  const { sleepTimer, sleepTimerFadeDuration, sleepTimerRemainingMs } = useAppSelector(state => state.player);

  const handleClose = () => {
    dispatch(closeModal());
  };

  const handleDurationSelect = (minutes: number) => {
    dispatch(setSleepTimer({ mode: 'Duration', durationMinutes: minutes }));
    handleClose();
  };

  const handleEndOfChapter = () => {
    dispatch(setSleepTimer({ mode: 'EndOfChapter', durationMinutes: 0 }));
    handleClose();
  };

  const handleCancel = () => {
    dispatch(clearSleepTimer());
    handleClose();
  };

  const handleFadeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    dispatch(setFadeDuration(parseInt(e.target.value)));
  };

  const formatRemainingTime = (ms: number): string => {
    const totalSeconds = Math.floor(ms / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    if (hours > 0) {
      return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
  };

  const isTimerActive = sleepTimer !== null;

  return (
    <div className="modal-box shadow-2xl border border-base-300/50 max-w-md backdrop-blur-sm">
      {/* Close button */}
      <button
        className="btn btn-sm btn-circle btn-ghost absolute right-3 top-3 hover:bg-base-200 hover:rotate-90 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 touch-target-min"
        style={{ minWidth: '44px', minHeight: '44px' }}
        onClick={handleClose}
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </button>

      {/* Header with icon */}
      <div className="flex items-center gap-3 mb-6 pr-8">
        <div
          className="w-10 h-10 rounded-full gradient-premium flex items-center justify-center text-white shadow-lg"
          style={{
            boxShadow: '0 4px 12px rgba(225, 29, 72, 0.3), 0 2px 4px rgba(0, 0, 0, 0.1)',
          }}
        >
          <Moon className="h-5 w-5" />
        </div>
        <div>
          <h3 className="font-bold text-xl">Sleep Timer</h3>
          <p className="text-xs text-description">Stop playback automatically</p>
        </div>
      </div>

      {/* Active timer display */}
      {isTimerActive && (
        <div className="mb-6 p-4 bg-primary/10 rounded-lg border border-primary/20">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Timer className="h-5 w-5 text-primary" />
              <span className="font-medium">
                {sleepTimer === 'Duration' ? 'Duration Timer' : 'End of Chapter'}
              </span>
            </div>
            {sleepTimer === 'Duration' && sleepTimerRemainingMs !== null && (
              <span className="font-mono text-lg text-primary font-semibold">
                {formatRemainingTime(sleepTimerRemainingMs)}
              </span>
            )}
          </div>
          <button
            className="btn btn-sm btn-error btn-outline mt-3 w-full"
            onClick={handleCancel}
            aria-label="Cancel sleep timer"
          >
            Cancel Timer
          </button>
        </div>
      )}

      {/* Duration options - grouped in a card-like container */}
      <div className="mb-5">
        <div className="flex items-center gap-2 mb-3 pb-2 border-b border-base-300/50">
          <Timer className="h-4 w-4 text-primary/70" />
          <span className="text-sm font-semibold text-base-content/80">Set Duration</span>
        </div>
        <div className="grid grid-cols-2 gap-2">
          {DURATION_OPTIONS.map((option) => (
            <button
              key={option.value}
              className="btn btn-outline btn-sm justify-start gap-2 hover:bg-primary/10 hover:border-primary hover:text-primary transition-all duration-200 group"
              onClick={() => handleDurationSelect(option.value)}
              aria-label={`Set sleep timer for ${option.label}`}
            >
              <Timer className="h-4 w-4 group-hover:scale-110 transition-transform" />
              <span>{option.label}</span>
            </button>
          ))}
        </div>
      </div>

      {/* End of chapter option - visually distinct */}
      <div className="mb-5">
        <div className="flex items-center gap-2 mb-3 pb-2 border-b border-base-300/50">
          <BookOpen className="h-4 w-4 text-primary/70" />
          <span className="text-sm font-semibold text-base-content/80">Smart Stop</span>
        </div>
        <button
          className="btn btn-outline btn-sm w-full justify-start gap-2 hover:bg-primary/10 hover:border-primary hover:text-primary transition-all duration-200 group"
          onClick={handleEndOfChapter}
          aria-label="Set sleep timer for end of chapter"
        >
          <BookOpen className="h-4 w-4 group-hover:scale-110 transition-transform" />
          <span>End of Chapter</span>
          <span className="text-xs text-base-content/70 ml-auto">Stops after current file</span>
        </button>
      </div>

      {/* Fade duration setting */}
      <div className="form-control mb-6">
        <label className="label py-2">
          <span className="label-text font-medium">Fade Out Duration</span>
          <span className="label-text-alt">Gradually lower volume</span>
        </label>
        <select
          className="select select-bordered w-full bg-base-100 focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all duration-200 cursor-pointer"
          value={sleepTimerFadeDuration}
          onChange={handleFadeChange}
          aria-label="Select fade out duration"
        >
          {FADE_OPTIONS.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </div>

      {/* Footer */}
      <div className="modal-action">
        <button
          className="btn btn-ghost"
          onClick={handleClose}
          aria-label="Close sleep timer modal"
        >
          Cancel
        </button>
      </div>
    </div>
  );
}
