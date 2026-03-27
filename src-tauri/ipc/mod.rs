use tauri::{command, State};
use uuid::Uuid;
use crate::core::timeline::{Clip, Timeline, Track, TrackType};

// State struct to hold the current timeline
#[derive(Default)]
struct AppState {
    timeline: Option<Timeline>,
}

#[command]
fn create_timeline(state: State<'_, AppState>, name: String) -> Result<String, String> {
    state.timeline = Some(Timeline::new(&name));
    Ok("Timeline created".to_string())
}

#[command]
fn add_track(state: State<'_, AppState>, name: String, track_type: String) -> Result<String, String> {
    let track_type = match track_type.to_lowercase().as_str() {
        "video" => TrackType::Video,
        "audio" => TrackType::Audio,
        "subtitle" => TrackType::Subtitle,
        _ => return Err("Invalid track type".to_string()),
    };
    
    if let Some(timeline) = &mut state.timeline {
        let track_id = timeline.add_track(name, track_type);
        Ok(track_id.to_string())
    } else {
        Err("No timeline exists".to_string())
    }
}

#[command]
fn add_clip(
    state: State<'_, AppState>,
    track_id: String,
    source_path: String,
    source_start: f64,
    source_end: f64,
    timeline_start: f64,
) -> Result<String, String> {
    let track_id = Uuid::parse_str(&track_id)
        .map_err(|_| "Invalid track ID".to_string())?;
    
    if let Some(timeline) = &mut state.timeline {
        let clip_id = timeline.add_clip_to_track(
            &track_id,
            source_path,
            source_start,
            source_end,
            timeline_start,
        )
        .ok_or("Track not found".to_string())?;
        
        Ok(clip_id.to_string())
    } else {
        Err("No timeline exists".to_string())
    }
}

#[command]
fn get_timeline(state: State<'_, AppState>) -> Result<String, String> {
    let timeline = state.timeline.as_ref()
        .ok_or("No timeline exists".to_string())?;
    
    serde_json::to_string(timeline)
        .map_err(|e| format!("Serialization error: {}", e))
}

#[command]
fn get_timeline_duration(state: State<'_, AppState>) -> Result<f64, String> {
    let timeline = state.timeline.as_ref()
        .ok_or("No timeline exists".to_string())?;
    
    Ok(timeline.duration)
}

#[command]
fn update_clip_volume(
    state: State<'_, AppState>,
    track_id: String,
    clip_id: String,
    volume: f64,
) -> Result<(), String> {
    let track_id = Uuid::parse_str(&track_id)
        .map_err(|_| "Invalid track ID".to_string())?;
    let clip_id = Uuid::parse_str(&clip_id)
        .map_err(|_| "Invalid clip ID".to_string())?;
    
    if let Some(timeline) = &mut state.timeline {
        if let Some(track) = timeline.get_track_mut(&track_id) {
            if let Some(clip) = track.clips.iter_mut().find(|c| c.id == clip_id) {
                clip.volume = volume.clamp(0.0, 2.0);
                Ok(())
            } else {
                Err("Clip not found".to_string())
            }
        } else {
            Err("Track not found".to_string())
        }
    } else {
        Err("No timeline exists".to_string())
    }
}

#[command]
fn update_clip_opacity(
    state: State<'_, AppState>,
    track_id: String,
    clip_id: String,
    opacity: f64,
) -> Result<(), String> {
    let track_id = Uuid::parse_str(&track_id)
        .map_err(|_| "Invalid track ID".to_string())?;
    let clip_id = Uuid::parse_str(&clip_id)
        .map_err(|_| "Invalid clip ID".to_string())?;
    
    if let Some(timeline) = &mut state.timeline {
        if let Some(track) = timeline.get_track_mut(&track_id) {
            if let Some(clip) = track.clips.iter_mut().find(|c| c.id == clip_id) {
                clip.opacity = opacity.clamp(0.0, 1.0);
                Ok(())
            } else {
                Err("Clip not found".to_string())
            }
        } else {
            Err("Track not found".to_string())
        }
    } else {
        Err("No timeline exists".to_string())
    }
}

#[command]
fn remove_clip(
    state: State<'_, AppState>,
    track_id: String,
    clip_id: String,
) -> Result<(), String> {
    let track_id = Uuid::parse_str(&track_id)
        .map_err(|_| "Invalid track ID".to_string())?;
    let clip_id = Uuid::parse_str(&clip_id)
        .map_err(|_| "Invalid clip ID".to_string())?;
    
    if let Some(timeline) = &mut state.timeline {
        if let Some(track) = timeline.get_track_mut(&track_id) {
            let len_before = track.clips.len();
            track.clips.retain(|c| c.id != clip_id);
            if track.clips.len() < len_before {
                Ok(())
            } else {
                Err("Clip not found".to_string())
            }
        } else {
            Err("Track not found".to_string())
        }
    } else {
        Err("No timeline exists".to_string())
    }
}

pub fn init() {
    // This function is called from main.rs to register commands
    // In Tauri v2, we use the #[command] attribute directly
}use ffmpeg_next::{format::context::Input, media::Type};

/// Media metadata extracted from a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// Duration in seconds.
    pub duration: f64,
    /// Width in pixels (for video).
    pub width: Option<u32>,
    /// Height in pixels (for video).
    pub height: Option<u32>,
    /// Frame rate (as a fraction, e.g., 24/1).
    pub frame_rate_num: u32,
    pub frame_rate_den: u32,
    /// Whether the media has video.
    pub has_video: bool,
    /// Whether the media has audio.
    pub has_audio: bool,
}

/// Probe a media file to get its metadata.
#[command]
fn probe_media(path: String) -> Result<MediaMetadata, String> {
    let input = Input::open(&path)
        .map_err(|e| format!("Failed to open media file: {}", e))?;
    
    let mut duration = 0.0;
    let mut width = None;
    let mut height = None;
    let mut frame_rate_num = 0;
    let mut frame_rate_den = 1;
    let mut has_video = false;
    let mut has_audio = false;

    for stream in input.streams() {
        match stream.codec().parameters().media_type() {
            Type::Video => {
                has_video = true;
                if let Some(codecpar) = stream.codec_parameters() {
                    width = Some(codecpar.width());
                    height = Some(codecpar.height());
                    // Try to get frame rate
                    if let Ok(avg_frame_rate) = stream.avg_frame_rate() {
                        frame_rate_num = avg_frame_rate.numer();
                        frame_rate_den = avg_frame_rate.denom();
                    }
                }
            }
            Type::Audio => {
                has_audio = true;
            }
            _ => {}
        }
    }

    // Get duration from the format context
    if let Ok(fmt_ctx) = input.format() {
        duration = fmt_ctx.duration() as f64 / AV_TIME_BASE as f64;
    }

    Ok(MediaMetadata {
        duration,
        width,
        height,
        frame_rate_num,
        frame_rate_den,
        has_video,
        has_audio,
    })
}


use std::fs;
use std::path::PathBuf;

/// Directory entry information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    /// File name.
    pub name: String,
    /// Full path.
    pub path: String,
    /// Whether this entry is a directory.
    pub is_dir: bool,
    /// File size in bytes (if file).
    pub size: Option<u64>,
    /// Media duration in seconds (if media file).
    pub duration: Option<f64>,
}

/// Read the contents of a directory.
#[command]
fn read_directory(path: String) -> Result<Vec<DirectoryEntry>, String> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let metadata = entry.metadata().map_err(|e| format!("Failed to get metadata: {}", e))?;
        let is_dir = metadata.is_dir();
        let len = metadata.len();
        let mut duration = None;

        // If it's a file and looks like a media file, probe for duration
        if !is_dir {
            let ext = entry.path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if matches!(ext.as_str(), "mp4" | "mov" | "mkv" | "avi" | "mxf" | "webm" | "mp3" | "wav" | "flac" | "aac") {
                // Probe for duration (we'll reuse the probe_media command logic, but for simplicity we'll do a quick probe here)
                // In a real implementation, we might want to cache this or use a background thread.
                if let Ok(metadata) = probe_media_internal(&entry.path()) {
                    duration = Some(metadata.duration);
                }
            }
        }

        entries.push(DirectoryEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path().to_string_lossy().into_owned(),
            is_dir,
            size: if !is_dir { Some(len) } else { None },
            duration,
        });
    }

    // Sort: directories first, then files, both alphabetically
    entries.sort_by(|a, b| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    Ok(entries)
}

/// Internal function to probe media duration (similar to the public command but takes a PathBuf).
fn probe_media_internal(path: &PathBuf) -> Result<MediaMetadata, String> {
    let input = Input::open(path)
        .map_err(|e| format!("Failed to open media file: {}", e))?;
    
    let mut duration = 0.0;
    let mut width = None;
    let mut height = None;
    let mut frame_rate_num = 0;
    let mut frame_rate_den = 1;
    let mut has_video = false;
    let mut has_audio = false;

    for stream in input.streams() {
        match stream.codec().parameters().media_type() {
            Type::Video => {
                has_video = true;
                if let Some(codecpar) = stream.codec_parameters() {
                    width = Some(codecpar.width());
                    height = Some(codecpar.height());
                    // Try to get frame rate
                    if let Ok(avg_frame_rate) = stream.avg_frame_rate() {
                        frame_rate_num = avg_frame_rate.numer();
                        frame_rate_den = avg_frame_rate.denom();
                    }
                }
            }
            Type::Audio => {
                has_audio = true;
            }
            _ => {}
        }
    }

    // Get duration from the format context
    if let Ok(fmt_ctx) = input.format() {
        duration = fmt_ctx.duration() as f64 / AV_TIME_BASE as f64;
    }

    Ok(MediaMetadata {
        duration,
        width,
        height,
        frame_rate_num,
        frame_rate_den,
        has_video,
        has_audio,
    })
}
