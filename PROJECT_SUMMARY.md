# CutForge Project Summary

## Overview
Professional cross-platform video editing software (like Adobe Premiere Pro/DaVinci Resolve) being developed with:
- **Framework**: Tauri v2 (Rust backend + React/TypeScript frontend)
- **Goal**: MVP (v0.1) with functional timeline, import/export H.264, real-time playback, basic effects

## Current Progress (2026-03-27)
- ✅ Rust & Node.js environment set up (rustc 1.63.0, cargo 0.66.0, npm 10.9.4)
- ✅ Basic Tauri project structure created:
  - Package.json with React 18 + TypeScript
  - Tauri configuration (tauri.conf.json)
  - Vite config for React/TS
  - Basic src/Main.tsx and App.tsx
  - Cargo.toml with multimedia dependencies (FFmpeg-next, GStreamer, CPAL, etc.)
- ⏳ Development server setup in progress (tauri dev encountered config validation issues)

## Technical Stack
**Backend (Rust)**:
- FFmpeg-next for video/audio decoding/encoding
- GStreamer for media pipeline alternatives
- CPAL for cross-platform audio I/O
- RubberBand/SoundTouch for audio effects
- SQLite via rusqlite for project metadata
- Vulkan/wgpu for GPU rendering

**Frontend (React/TypeScript)**:
- React 18 + TypeScript
- Vite for bundling
- Resizable panels (react-resizable-panels planned)
- Zustand for state management

## Next Steps
1. Fix tauri.conf.json configuration (remove invalid fields)
2. Set up proper Tauri dev workflow
3. Implement basic window with frameless design
4. Create timeline core in Rust
5. Build React timeline UI components

## GitHub Prep
- Repository: https://github.com/Acorx/cutforge (to be created)
- Initial commit: Basic project structure
- README with vision and setup instructions