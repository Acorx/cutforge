import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

interface TimelineState {
  tracks: any[]; // We'll define a proper type later, but for now any
  isPlaying: boolean;
  currentTime: number;
  playbackRate: number;
  duration: number;
  fetchTimeline: () => Promise<void>;
  play: () => Promise<void>;
  pause: () => Promise<void>;
  setPlaybackRate: (rate: number) => Promise<void>;
  seek: (time: number) => Promise<void>;
  addTrack: (name: string, type: string) => Promise<void>;
  addClip: (trackId: string, sourcePath: string, sourceStart: number, sourceEnd: number, timelineStart: number) => Promise<void>;
  // We'll add more actions as needed
  setTracks: (tracks: any[]) => void;
  setIsPlaying: (isPlaying: boolean) => void;
  setCurrentTime: (time: number) => void;
  setPlaybackRateLocal: (rate: number) => void;
  setDuration: (duration: number) => void;
}

export const useTimelineStore = create<TimelineState>((set, get) => ({
  tracks: [],
  isPlaying: false,
  currentTime: 0,
  playbackRate: 1,
  duration: 0,

  fetchTimeline: async () => {
    try {
      const timeline = await invoke('get_timeline');
      set({ tracks: timeline.tracks || [], duration: timeline.duration || 0 });
    } catch (error) {
      console.error('Failed to fetch timeline:', error);
    }
  },

  play: async () => {
    await invoke('play_timeline');
    set({ isPlaying: true });
  },

  pause: async () => {
    await invoke('pause_timeline');
    set({ isPlaying: false });
  },

  setPlaybackRate: async (rate: number) => {
    await invoke('set_playback_rate', { rate });
    set({ playbackRate: rate });
  },

  seek: async (time: number) => {
    await invoke('seek_to_time', { time });
    // We don't update currentTime here immediately because we rely on the timeline update
    // In a more sophisticated implementation, we might optimistically update it
  },

  addTrack: async (name: string, type: string) => {
    await invoke('add_track', { name, track_type: type });
    await get().fetchTimeline();
  },

  addClip: async (trackId: string, sourcePath: string, sourceStart: number, sourceEnd: number, timelineStart: number) => {
    await invoke('add_clip', {
      track_id: trackId,
      source_path: sourcePath,
      source_start: sourceStart,
      source_end: sourceEnd,
      timeline_start: timelineStart,
    });
    await get().fetchTimeline();
  },

  // Setters for local state updates (used by Timeline component)
  setTracks: (tracks) => set({ tracks }),
  setIsPlaying: (isPlaying) => set({ isPlaying }),
  setCurrentTime: (time) => set({ currentTime: time }),
  setPlaybackRateLocal: (rate) => set({ playbackRate: rate }),
  setDuration: (duration) => set({ duration }),
}));
