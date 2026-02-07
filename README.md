# Overlay Chronicle

Tauri desktop app for combining timed image overlays and subtitles into video output with an operator-friendly interface.

## Features

- Timeline-based image overlay placement
- Subtitle embedding workflow
- Preview-oriented UI before final render
- FFmpeg-backed export pipeline

## Stack

- Frontend: Vite + TypeScript
- Desktop runtime: Tauri (Rust)

## Development

```bash
npm install
npm run dev
```

## Build

```bash
npm run build
npm run tauri build
```

## Project Layout

- `src/`: UI and workflow logic
- `src-tauri/`: native shell and command bridge
- `video-studio/`: related assets/components
