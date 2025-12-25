export interface VideoInfo {
  duration: number;
  width: number;
  height: number;
  fps: number;
  codec: string;
}

export interface ImageInfo {
  index: number;
  path: string;
  exists: boolean;
}

export interface Interval {
  imageIndex: number;
  imagePath: string;
  start: number;
  end: number;
}

export interface SrtEntry {
  index: number;
  start: string;
  end: string;
  text: string;
  start_seconds: number;
  end_seconds: number;
}

export interface RenderProgress {
  step: string;
  progress: number;
  message: string;
}
