use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Represents a single media clip on the timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    /// Unique identifier for the clip.
    pub id: Uuid,
    /// Source media file path.
    pub source_path: String,
    /// Start time in the source media (in seconds).
    pub source_start: f64,
    /// End time in the source media (in seconds).
    pub source_end: f64,
    /// Start time on the timeline track (in seconds).
    pub timeline_start: f64,
    /// End time on the timeline track (in seconds).
    pub timeline_end: f64,
    /// Whether the clip is enabled (not muted/soloed etc).
    pub enabled: bool,
    /// Volume multiplier (0.0 to 2.0).
    pub volume: f64,
    /// Opacity (0.0 to 1.0).
    pub opacity: f64,
}

/// Represents a track that can contain multiple clips.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Unique identifier for the track.
    pub id: Uuid,
    /// Name of the track (e.g., "Video 1", "Audio 2").
    pub name: String,
    /// Type of track: Video, Audio, Subtitle, etc.
    pub track_type: TrackType,
    /// Whether the track is enabled.
    pub enabled: bool,
    /// Whether the track is soloed (only this track plays).
    pub solo: bool,
    /// Whether the track is muted.
    pub mute: bool,
    /// Clips on this track, sorted by timeline_start.
    pub clips: Vec<Clip>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrackType {
    Video,
    Audio,
    Subtitle,
}

/// The main timeline containing multiple tracks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    /// Unique identifier for the timeline.
    pub id: Uuid,
    /// Name of the timeline (sequence).
    pub name: String,
    /// Tracks in the timeline.
    pub tracks: Vec<Track>,
    /// Duration of the timeline (in seconds).
    pub duration: f64,
    /// Frame rate (frames per second).
    pub frame_rate: f64,
}

impl Timeline {
    /// Create a new empty timeline with default settings.
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            tracks: Vec::new(),
            duration: 0.0,
            frame_rate: 24.0,
        }
    }

    /// Add a new track to the timeline.
    pub fn add_track(&mut self, name: String, track_type: TrackType) -> Uuid {
        let track_id = Uuid::new_v4();
        let track = Track {
            id: track_id,
            name,
            track_type,
            enabled: true,
            solo: false,
            mute: false,
            clips: Vec::new(),
        };
        self.tracks.push(track);
        track_id
    }

    /// Remove a track by its ID.
    pub fn remove_track(&mut self, track_id: &Uuid) -> bool {
        let len_before = self.tracks.len();
        self.tracks.retain(|t| t.id != *track_id);
        self.tracks.len() < len_before
    }

    /// Get a mutable reference to a track by ID.
    pub fn get_track_mut(&mut self, track_id: &Uuid) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == *track_id)
    }

    /// Get an immutable reference to a track by ID.
    pub fn get_track(&self, track_id: &Uuid) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == *track_id)
    }

    /// Add a clip to a specific track.
    /// Returns the clip ID if successful, None if track not found.
    pub fn add_clip_to_track(
        &mut self,
        track_id: &Uuid,
        source_path: String,
        source_start: f64,
        source_end: f64,
        timeline_start: f64,
    ) -> Option<Uuid> {
        if let Some(track) = self.get_track_mut(track_id) {
            let clip_id = Uuid::new_v4();
            let duration = source_end - source_start;
            let clip = Clip {
                id: clip_id,
                source_path,
                source_start,
                source_end,
                timeline_start,
                timeline_end: timeline_start + duration,
                enabled: true,
                volume: 1.0,
                opacity: 1.0,
            };
            // Insert clip sorted by timeline_start
            let pos = track.clips.binary_search_by(|c| c.timeline_start.partial_cmp(&timeline_start).unwrap());
            match pos {
                Ok(index) => track.clips.insert(index, clip),
                Err(index) => track.clips.insert(index, clip),
            };
            // Recalculate duration if needed
            self.recalculate_duration();
            Some(clip_id)
        } else {
            None
        }
    }

    /// Recalculate the total duration of the timeline based on all tracks and clips.
    pub fn recalculate_duration(&mut self) {
        let mut max_end = 0.0;
        for track in &self.tracks {
            for clip in &track.clips {
                if clip.timeline_end > max_end {
                    max_end = clip.timeline_end;
                }
            }
        }
        self.duration = max_end;
    }

    /// Get all clips that intersect a given time range.
    pub fn get_clips_in_range(&self, start: f64, end: f64) -> Vec<(Uuid, Uuid, &Clip)> {
        let mut result = Vec::new();
        for track in &self.tracks {
            for clip in &track.clips {
                if clip.timeline_start < end && clip.timeline_end > start {
                    result.push((track.id, clip.id, clip));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let tl = Timeline::new("Test Timeline");
        assert_eq!(tl.name, "Test Timeline");
        assert_eq!(tl.tracks.len(), 0);
        assert_eq!(tl.duration, 0.0);
        assert_eq!(tl.frame_rate, 24.0);
    }

    #[test]
    fn test_add_track() {
        let mut tl = Timeline::new("Test");
        let vid_id = tl.add_track("Video 1".to_string(), TrackType::Video);
        let aud_id = tl.add_track("Audio 1".to_string(), TrackType::Audio);
        assert_eq!(tl.tracks.len(), 2);
        assert!(tl.get_track(&vid_id).is_some());
        assert!(tl.get_track(&aud_id).is_some());
        assert_eq!(tl.get_track(&vid_id).unwrap().track_type, TrackType::Video);
        assert_eq!(tl.get_track(&aud_id).unwrap().track_type, TrackType::Audio);
    }

    #[test]
    fn test_add_clip() {
        let mut tl = Timeline::new("Test");
        let track_id = tl.add_track("Video 1".to_string(), TrackType::Video);
        let clip_id = tl.add_clip_to_track(
            &track_id,
            "/path/to/video.mp4".to_string(),
            0.0,
            10.0,
            5.0,
        );
        assert!(clip_id.is_some());
        let clip_id = clip_id.unwrap();
        let track = tl.get_track(&track_id).unwrap();
        assert_eq!(track.clips.len(), 1);
        let clip = &track.clips[0];
        assert_eq!(clip.id, clip_id);
        assert_eq!(clip.source_path, "/path/to/video.mp4");
        assert_eq!(clip.source_start, 0.0);
        assert_eq!(clip.source_end, 10.0);
        assert_eq!(clip.timeline_start, 5.0);
        assert_eq!(clip.timeline_end, 15.0);
        // Duration should be updated
        assert_eq!(tl.duration, 15.0);
    }

    #[test]
    fn test_multiple_clips_sorted() {
        let mut tl = Timeline::new("Test");
        let track_id = tl.add_track("Video 1".to_string(), TrackType::Video);
        // Add clip at 10s
        tl.add_clip_to_track(&track_id, "a.mp4".to_string(), 0.0, 5.0, 10.0);
        // Add clip at 0s
        tl.add_clip_to_track(&track_id, "b.mp4".to_string(), 0.0, 3.0, 0.0);
        // Add clip at 5s
        tl.add_clip_to_track(&track_id, "c.mp4".to_string(), 0.0, 2.0, 5.0);

        let track = tl.get_track(&track_id).unwrap();
        assert_eq!(track.clips.len(), 3);
        // Should be sorted by timeline_start: 0, 5, 10
        assert_eq!(track.clips[0].timeline_start, 0.0);
        assert_eq!(track.clips[1].timeline_start, 5.0);
        assert_eq!(track.clips[2].timeline_start, 10.0);
        assert_eq!(track.clips[0].source_path, "b.mp4");
        assert_eq!(track.clips[1].source_path, "c.mp4");
        assert_eq!(track.clips[2].source_path, "a.mp4");
    }
}
/// Playback state of the timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    /// Whether the timeline is currently playing.
    pub is_playing: bool,
    /// Current playback position in seconds.
    pub current_time: f64,
    /// Playback rate (1.0 = normal speed).
    pub playback_rate: f64,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_time: 0.0,
            playback_rate: 1.0,
        }
    }
}

impl PlaybackState {
    /// Update current time based on elapsed time and playback rate.
    /// This would typically be called by a timer tick.
    pub fn update(&mut self, delta_seconds: f64) {
        if self.is_playing {
            self.current_time += delta_seconds * self.playback_rate;
            // Clamp to timeline duration (if we had access to it here)
            // For now, we just let it go beyond; the frontend can handle clamping.
        }
    }
}
