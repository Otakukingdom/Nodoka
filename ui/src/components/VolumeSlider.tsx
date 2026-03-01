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
    <div className="flex items-center gap-1 sm:gap-2 group/volume">
      <button
        className="btn btn-ghost btn-sm btn-circle hover:bg-primary/10 hover:text-primary transition-all duration-200 focus-visible:ring-2 focus-visible:ring-primary"
        onClick={onMuteToggle}
        aria-label={isMuted ? 'Unmute' : 'Mute'}
        title={isMuted ? 'Unmute (M)' : 'Mute (M)'}
      >
        <VolumeIcon className="h-4 w-4 sm:h-5 sm:w-5" />
      </button>
      <div className="relative w-16 sm:w-20 lg:w-24 h-8 flex items-center">
        {/* Background track */}
        <div className="absolute inset-x-0 h-1.5 top-1/2 -translate-y-1/2 rounded-full bg-base-300 transition-all duration-200 group-hover/volume:h-2" />
        {/* Progress track */}
        <div
          className="absolute left-0 h-1.5 top-1/2 -translate-y-1/2 rounded-full gradient-premium transition-all duration-100 group-hover/volume:h-2"
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
        {/* Visible thumb on hover/focus */}
        <div
          className="absolute top-1/2 -translate-y-1/2 w-3.5 h-3.5 rounded-full bg-primary shadow-md opacity-0 group-hover/volume:opacity-100 focus-within:opacity-100 transition-all duration-200 pointer-events-none ring-2 ring-white/30"
          style={{ left: `calc(${displayVolume}% - 7px)` }}
        />
      </div>
    </div>
  );
}
