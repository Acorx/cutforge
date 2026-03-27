# CutForge - Professional Video Editor

A cross-platform video editing suite built with Tauri v2 (Rust + React).

## Current Progress

### Backend (Rust/Tauri)
- Timeline core: Clip, Track, Timeline structs with UUIDs
- Playback controls: play/pause/seek/rate
- Media probing: FFmpeg-next integration for metadata
- File system access: directory reading, media duration detection
- IPC layer: All functionality exposed to frontend via Tauri commands

### Frontend (React/TypeScript)
- MediaBrowser panel: File/directory navigation, media preview on hover
- Timeline panel: Track headers, clip blocks, playhead, time ruler, zoom controls
- Zustand store: State management for timeline and playback
- Keyboard shortcuts: Space (play/pause), JKL scrubbing, arrow keys, zoom
- Drag & drop: From MediaBrowser to Timeline (basic implementation)

## Next Steps
1. Connect MediaBrowser drops to actual timeline clip insertion
2. Implement actual video rendering (WebGPU/WebGL preview)
3. Add audio playback (CPAL)
4. Implement effects system
5. Add project save/load (.cfproj format)
6. Enhance UI with professional styling and panels

## Quick Start
npm install
npm run dev

## Vision
Professional-grade video editing (Premiere Pro/Resolve/FCP level) with native cross-platform support.
