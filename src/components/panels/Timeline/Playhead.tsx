import React from 'react';

interface PlayheadProps {
  zoomLevel: number;
  scrollPosition: number;
  currentTime: number;
  isPlaying: boolean;
}

export const Playhead: React.FC<PlayheadProps> = ({ zoomLevel, scrollPosition, currentTime, isPlaying }) => {
  const left = (currentTime - scrollPosition) * zoomLevel;
  return (
    <div
      className="playhead"
      style={{
        left: `${left}px`,
        opacity: isPlaying ? 1 : 0.7,
      }}
    >
      <div className="playhead-line"></div>
      <div className="playhead-time">
        {formatTime(currentTime)}
      </div>
    </div>
  );
};

function formatTime(seconds: number): string {
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  const ms = Math.floor((seconds % 1) * 1000);
  return [
    hrs.toString().padStart(2, '0'),
    mins.toString().padStart(2, '0'),
    secs.toString().padStart(2, '0'),
    ms.toString().padStart(3, '0')
  ].join(':');
}
