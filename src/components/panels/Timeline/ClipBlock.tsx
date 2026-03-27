import React from 'react';

interface ClipBlockProps {
  clip: {
    id: string;
    source_path: string;
    source_start: number;
    source_end: number;
    timeline_start: number;
    timeline_end: number;
    enabled: boolean;
    volume: number;
    opacity: number;
    custom_name?: string | null;
  };
  zoomLevel: number; // pixels per second
  scrollPosition: number; // seconds
}

export const ClipBlock: React.FC<ClipBlockProps> = ({ clip, zoomLevel, scrollPosition }) => {
  const startX = (clip.timeline_start - scrollPosition) * zoomLevel;
  const width = (clip.timeline_end - clip.timeline_start) * zoomLevel;

  // Determine display name: custom name or filename from source_path
  const displayName = clip.custom_name && clip.custom_name.trim() !== '' 
    ? clip.custom_name 
    : clip.source_path.split(/[\\/]/).pop() || 'unknown';

  return (
    <div
      className={`clip-block ${clip.enabled ? '' : 'disabled'}`}
      style={{
        left: `${startX}px`,
        width: `${width}px`,
        opacity: clip.opacity,
      }}
    >
      <div className="clip-content">
        <div className="clip-name">{displayName}</div>
        <div className="clip-timestamp">
          {formatTime(clip.timeline_start)} – {formatTime(clip.timeline_end)}
        </div>
      </div>
    </div>
  );
};

// Helper function to format seconds as HH:MM:SS.mmm
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
