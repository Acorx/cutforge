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
}