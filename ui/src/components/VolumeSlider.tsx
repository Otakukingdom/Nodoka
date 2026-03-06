/**
 * Volume slider component with mute toggle
 */

import { Volume2, Volume1, VolumeX } from 'lucide-react';

interface VolumeSliderProps {
  volume: number;
  isMuted: boolean;
  onVolumeChange: (value: number) => void;
  onMuteToggle: () => void;
}

export default function VolumeSlider({
  volume,
  isMuted,
  onVolumeChange,
  onMuteToggle,
}: VolumeSliderProps) {
  const displayVolume = isMuted ? 0 : volume;
  const VolumeIcon = displayVolume === 0 ? VolumeX : displayVolume < 50 ? Volume1 : Volume2;

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onVolumeChange(Number(e.target.value));
  };

  return (
    <div className="flex items-center gap-1 group/volume flex-shrink-0 volume-control-wrapper" data-testid="volume-control" role="group" aria-label="Volume control">
      <button
        className="btn btn-ghost btn-sm btn-circle hover:bg-primary/10 hover:text-primary hover:scale-110 active:scale-95 transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 touch-target-min"
        onClick={onMuteToggle}
        aria-label={isMuted ? 'Unmute' : 'Mute'}
        title={isMuted ? 'Unmute (M)' : 'Mute (M)'}
      >
        <VolumeIcon className="h-4 w-4" />
      </button>
      <div className="relative w-16 sm:w-20 lg:w-24 h-10 sm:h-8 flex items-center flex-shrink-0 volume-slider-track">
        {/* Background track - always visible with enhanced contrast */}
        <div className="absolute inset-x-0 h-2 top-1/2 -translate-y-1/2 rounded-full bg-base-300/80 transition-all duration-200 group-hover/volume:h-2.5" />
        {/* Progress track with premium gradient */}
        <div
          className="absolute left-0 h-2 top-1/2 -translate-y-1/2 rounded-full gradient-premium transition-all duration-100 group-hover/volume:h-2.5"
          style={{ width: `${displayVolume}%` }}
        />
        <input
          type="range"
          min="0"
          max="100"
          value={displayVolume}
          onChange={handleSliderChange}
          className="range range-primary range-xs w-full absolute z-10 opacity-0 group-hover/volume:opacity-100 focus:opacity-100 transition-opacity cursor-pointer"
          aria-label="Volume level"
        />
        {/* Visible thumb on hover/focus with enhanced visibility */}
        <div
          className="absolute top-1/2 -translate-y-1/2 w-4 h-4 rounded-full bg-primary shadow-md opacity-0 group-hover/volume:opacity-100 focus-within:opacity-100 transition-all duration-200 pointer-events-none ring-2 ring-white/40"
          style={{ left: `calc(${displayVolume}% - 8px)` }}
        />
      </div>
    </div>
  );
}
