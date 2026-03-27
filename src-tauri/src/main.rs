#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, State};
use crate::ipc::{AppState, init};

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            crate::ipc::create_timeline,
            crate::ipc::add_track,
            crate::ipc::add_clip,
            crate::ipc::get_timeline,
            crate::ipc::get_timeline_duration,
            crate::ipc::update_clip_volume,
            crate::ipc::update_clip_opacity,
            crate::ipc::remove_clip,
            crate::ipc::play_timeline,
            crate::ipc::pause_timeline,
            crate::ipc::set_playback_rate,
            crate::ipc::seek_to_time,
            crate::ipc::get_playback_state,
            crate::ipc::probe_media,
            crate::ipc::read_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
