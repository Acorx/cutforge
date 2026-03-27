import React from 'react';

interface TrackHeaderProps {
  track: {
    id: string;
    name: string;
    track_type: 'Video' | 'Audio' | 'Subtitle';
    enabled: boolean;
    solo: boolean;
    mute: boolean;
    color: string; // We'll store as hex string for simplicity
  };
}

export const TrackHeader: React.FC<TrackHeaderProps> = ({ track }) => {
  const handleToggleEnabled = () => {
    // We would invoke an IPC command to update the track enabled state
    // For now, we'll just log (to be implemented)
    console.log(`Toggle enabled for track ${track.id}`);
  };

  const handleToggleMute = () => {
    console.log(`Toggle mute for track ${track.id}`);
  };

  const handleToggleSolo = () => {
    console.log(`Toggle solo for track ${track.id}`);
  };

  return (
    <div className="track-header">
      <div className="track-header-left">
        <div className="track-color-box" style={{ backgroundColor: track.color }}></div>
        <input
          type="text"
          value={track.name}
          readOnly
          className="track-name"
        />
      </div>
      <div className="track-header-controls">
        <button
          className={track.enabled ? 'active' : ''}
          onClick={handleToggleEnabled}
          title="Enable/Disable"
        >
          E
        </button>
        <button
          className={track.solo ? 'active' : ''}
          onClick={handleToggleSolo}
          title="Solo"
        >
          S
        </button>
        <button
          className={track.mute ? 'active' : ''}
          onClick={handleToggleMute}
          title="Mute"
        >
          M
        </button>
      </div>
    </div>
  );
};
