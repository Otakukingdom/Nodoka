/**
 * Unit tests for VolumeSlider component
 */

import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import VolumeSlider from './VolumeSlider';

describe('VolumeSlider', () => {
  const defaultProps = {
    volume: 75,
    isMuted: false,
    onVolumeChange: vi.fn(),
    onMuteToggle: vi.fn(),
  };

  it('renders mute button when not muted', () => {
    render(<VolumeSlider {...defaultProps} isMuted={false} />);
    expect(screen.getByRole('button', { name: 'Mute' })).toBeInTheDocument();
  });

  it('renders unmute button when muted', () => {
    render(<VolumeSlider {...defaultProps} isMuted={true} />);
    expect(screen.getByRole('button', { name: 'Unmute' })).toBeInTheDocument();
  });

  it('renders volume slider', () => {
    render(<VolumeSlider {...defaultProps} />);
    expect(screen.getByRole('slider', { name: 'Volume level' })).toBeInTheDocument();
  });

  it('displays correct volume value when not muted', () => {
    render(<VolumeSlider {...defaultProps} volume={60} />);
    const slider = screen.getByRole('slider', { name: 'Volume level' });
    expect(slider).toHaveValue('60');
  });

  it('displays zero volume when muted', () => {
    render(<VolumeSlider {...defaultProps} volume={75} isMuted={true} />);
    const slider = screen.getByRole('slider', { name: 'Volume level' });
    expect(slider).toHaveValue('0');
  });

  it('calls onMuteToggle when mute button is clicked', () => {
    const onMuteToggle = vi.fn();
    render(<VolumeSlider {...defaultProps} onMuteToggle={onMuteToggle} />);

    fireEvent.click(screen.getByRole('button', { name: 'Mute' }));
    expect(onMuteToggle).toHaveBeenCalledTimes(1);
  });

  it('calls onVolumeChange when slider value changes', () => {
    const onVolumeChange = vi.fn();
    render(<VolumeSlider {...defaultProps} onVolumeChange={onVolumeChange} />);

    const slider = screen.getByRole('slider', { name: 'Volume level' });
    fireEvent.change(slider, { target: { value: '50' } });
    expect(onVolumeChange).toHaveBeenCalledWith(50);
  });

  it('has correct aria-label on mute button', () => {
    render(<VolumeSlider {...defaultProps} isMuted={false} />);
    expect(screen.getByRole('button', { name: 'Mute' })).toHaveAttribute('aria-label', 'Mute');
  });

  it('has correct aria-label on unmute button', () => {
    render(<VolumeSlider {...defaultProps} isMuted={true} />);
    expect(screen.getByRole('button', { name: 'Unmute' })).toHaveAttribute('aria-label', 'Unmute');
  });
});
