import React, { useEffect, useRef, useState } from 'react';
import { useTimelineStore } from '../../../stores/useTimelineStore';
import { invoke } from '@tauri-apps/api/tauri';
import { TrackHeader } from './TrackHeader';
import { ClipBlock } from './ClipBlock';
import { Playhead } from './Playhead';
import { TimeRuler } from './TimeRuler';
import { ZoomControls } from './ZoomControls';

interface TimelineProps {
  // We could pass in specific props, but for now we'll use the store
}

export const Timeline: React.FC<TimelineProps> = () => {
  const { tracks, isPlaying, currentTime, playbackRate, fetchTimeline, play, pause, setPlaybackRate, seek, addTrack, addClip } = useTimelineStore();
  const timelineRef = useRef<HTMLDivElement>(null);
  const [zoomLevel, setZoomLevel] = useState(100); // pixels per second
  const [scrollPosition, setScrollPosition] = useState(0); // seconds
  const [duration, setDuration] = useState(0);
  const [isUpdatingTime, setIsUpdatingTime] = useState(false);

  // Fetch timeline on mount and periodically if playing
  useEffect(() => {
    const loadTimeline = async () => {
      try {
        const timeline = await invoke('get_timeline');
        set({ tracks: timeline.tracks || [] });
        setDuration(timeline.duration || 0);
      } catch (error) {
        console.error('Failed to fetch timeline:', error);
      }
    };

    loadTimeline();
    
    // Set up interval to update current time when playing
    let interval: NodeJS.Timeout;
    if (isPlaying) {
      interval = setInterval(async () => {
        if (!isUpdatingTime) {
          setIsUpdatingTime(true);
          try {
            const newTime = currentTime + (0.1 * playbackRate); // 100ms ticks
            const timeline = await invoke('get_timeline');
            const maxDuration = timeline.duration || 0;
            
            if (newTime < maxDuration) {
              await invoke('seek_to_time', { time: newTime });
              set({ currentTime: newTime });
            } else {
              // End of timeline
              await invoke('pause_timeline');
              set({ isPlaying: false, currentTime: 0 });
            }
          } catch (error) {
            console.error('Failed to update time:', error);
          } finally {
            setIsUpdatingTime(false);
          }
        }
      }, 100);
    }
    
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [fetchTimeline, isPlaying, currentTime, playbackRate, invoke, set]);

  // Handle scroll
  const handleScroll = (e: React.UIEvent<HTMLDivElement>) => {
    const target = e.target as HTMLDivElement;
    setScrollPosition(target.scrollLeft);
  };

  // Handle click on timeline to set playhead position
  const handleTimelineClick = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
    if (!timelineRef.current) return;
    const rect = timelineRef.current.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const secondsFromStart = scrollPosition + (clickX / zoomLevel);
    seek(secondsFromStart);
  };

  // Handle drop from media browser
  const handleDrop = async (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    const data = e.dataTransfer.getData('text/plain');
    try {
      const fileInfo = JSON.parse(data);
      // Find first video track
      const videoTrack = tracks.find(t => t.track_type === 'Video');
      if (videoTrack) {
        await addClip(
          videoTrack.id,
          fileInfo.path,
          0, // sourceStart - we'd get this from probing or assume 0 for now
          fileInfo.duration || 10, // sourceEnd - assume 10 seconds if not known
          currentTime // timelineStart
        );
        // Refetch timeline to get updated clips
        await fetchTimeline();
      }
    } catch (error) {
      console.error('Error processing drop:', error);
    }
  };

  // Handle dragover to allow drop
  const handleDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };

  // Handle keyboard shortcuts
  const handleKeyDown = (e: React.KeyboardEvent) => {
    // Prevent shortcuts when typing in inputs
    if ((e.target as HTMLInputElement).tagName === 'INPUT' || 
        (e.target as HTMLTextAreaElement).tagName === 'TEXTAREA') {
      return;
    }

    switch (e.key) {
      case ' ': // Space - play/pause
        e.preventDefault();
        if (isPlaying) {
          pause();
        } else {
          play();
        }
        break;
      case 'k': // K - pause
        e.preventDefault();
        pause();
        break;
      case 'j': // J - reverse play
        e.preventDefault();
        // For simplicity, we'll just pause and seek back a bit
        pause();
        seek(Math.max(0, currentTime - 2));
        break;
      case 'l': // L - forward play
        e.preventDefault();
        play();
        break;
      case 'ArrowLeft': // Left arrow - step back
        e.preventDefault();
        seek(Math.max(0, currentTime - 0.1));
        break;
      case 'ArrowRight': // Right arrow - step forward
        e.preventDefault();
        seek(Math.min(duration, currentTime + 0.1));
        break;
      case '+': // Zoom in
        e.preventDefault();
        setZoomLevel(Math.min(500, zoomLevel + 20));
        break;
      case '-': // Zoom out
        e.preventDefault();
        setZoomLevel(Math.max(20, zoomLevel - 20));
        break;
      case 'Home': // Go to start
        e.preventDefault();
        seek(0);
        break;
      case 'End': // Go to end
        e.preventDefault();
        seek(duration);
        break;
      default:
        break;
    }
  };

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown, pause, play, seek, setZoomLevel, zoomLevel, duration]);

  return (
    <div className="timeline-panel" onDrop={handleDrop} onDragOver={handleDragOver} tabIndex="-1" onFocus={() => {}}>
      <div className="timeline-header">
        <TimeRuler zoomLevel={zoomLevel} duration={duration} scrollPosition={scrollPosition} />
        <ZoomControls zoomLevel={zoomLevel} onZoomChange={setZoomLevel} />
      </div>
      <div 
        className="timeline-container" 
        ref={timelineRef}
        onScroll={handleScroll}
        onClick={handleTimelineClick}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onFocus={() => {}} // Ensure it can receive focus for keyboard events
      >
        <div className="timeline-content">
          {tracks.map((track) => (
            <div key={track.id} className="timeline-track">
              <TrackHeader track={track} />
              <div className="timeline-track-clips" style={{ width: `${duration * zoomLevel}px` }}>
                {track.clips.map((clip) => (
                  <ClipBlock 
                    key={clip.id} 
                    clip={clip} 
                    zoomLevel={zoomLevel} 
                    scrollPosition={scrollPosition} 
                  />
                ))}
                <Playhead 
                  zoomLevel={zoomLevel} 
                  scrollPosition={scrollPosition} 
                  currentTime={currentTime} 
                  isPlaying={isPlaying} 
                />
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
