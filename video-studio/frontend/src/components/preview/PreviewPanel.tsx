import { useRef, useEffect, useState } from 'react';

interface PreviewPanelProps {
  currentTime: number;
  isPlaying: boolean;
  onTimeUpdate: (time: number) => void;
}

export default function PreviewPanel({ currentTime, isPlaying, onTimeUpdate }: PreviewPanelProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [videoSize, setVideoSize] = useState({ width: 1920, height: 1080 });
  const [showGrid, setShowGrid] = useState(false);
  const [showSafeZones, setShowSafeZones] = useState(false);

  useEffect(() => {
    if (videoRef.current) {
      videoRef.current.currentTime = currentTime;
    }
  }, [currentTime]);

  useEffect(() => {
    if (videoRef.current) {
      if (isPlaying) {
        videoRef.current.play();
      } else {
        videoRef.current.pause();
      }
    }
  }, [isPlaying]);

  const handleTimeUpdate = () => {
    if (videoRef.current) {
      onTimeUpdate(videoRef.current.currentTime);
    }
  };

  const calculateAspectRatio = () => {
    const aspectRatio = videoSize.width / videoSize.height;
    const containerWidth = 800; // max preview width
    const containerHeight = 450; // max preview height

    let width = containerWidth;
    let height = width / aspectRatio;

    if (height > containerHeight) {
      height = containerHeight;
      width = height * aspectRatio;
    }

    return { width, height };
  };

  const previewSize = calculateAspectRatio();

  return (
    <div className="preview-container relative w-full h-full">
      {/* Preview Controls Overlay */}
      <div className="absolute top-4 right-4 z-10 flex flex-col gap-2">
        <div className="bg-black/70 rounded px-3 py-2 flex items-center gap-3">
          <label className="flex items-center gap-2 text-xs cursor-pointer">
            <input
              type="checkbox"
              checked={showGrid}
              onChange={(e) => setShowGrid(e.target.checked)}
            />
            Grid
          </label>
          <label className="flex items-center gap-2 text-xs cursor-pointer">
            <input
              type="checkbox"
              checked={showSafeZones}
              onChange={(e) => setShowSafeZones(e.target.checked)}
            />
            Safe Zones
          </label>
        </div>

        <div className="bg-black/70 rounded px-3 py-2 text-xs">
          {videoSize.width} × {videoSize.height}
        </div>
      </div>

      {/* Video Preview */}
      <div className="relative flex items-center justify-center h-full">
        <div
          className="relative bg-black"
          style={{
            width: `${previewSize.width}px`,
            height: `${previewSize.height}px`,
          }}
        >
          <video
            ref={videoRef}
            className="preview-video w-full h-full"
            onTimeUpdate={handleTimeUpdate}
            onLoadedMetadata={(e) => {
              const video = e.target as HTMLVideoElement;
              setVideoSize({
                width: video.videoWidth,
                height: video.videoHeight,
              });
            }}
          >
            {/* Video source will be set dynamically */}
          </video>

          {/* Overlay Canvas */}
          <canvas
            ref={canvasRef}
            className="preview-overlay-canvas"
            width={videoSize.width}
            height={videoSize.height}
            style={{
              width: `${previewSize.width}px`,
              height: `${previewSize.height}px`,
            }}
          />

          {/* Grid Overlay */}
          {showGrid && (
            <svg
              className="absolute inset-0 pointer-events-none"
              width="100%"
              height="100%"
            >
              {/* Thirds grid */}
              <line x1="33.33%" y1="0" x2="33.33%" y2="100%" stroke="rgba(255,255,255,0.3)" strokeWidth="1" />
              <line x1="66.66%" y1="0" x2="66.66%" y2="100%" stroke="rgba(255,255,255,0.3)" strokeWidth="1" />
              <line x1="0" y1="33.33%" x2="100%" y2="33.33%" stroke="rgba(255,255,255,0.3)" strokeWidth="1" />
              <line x1="0" y1="66.66%" x2="100%" y2="66.66%" stroke="rgba(255,255,255,0.3)" strokeWidth="1" />
              {/* Center lines */}
              <line x1="50%" y1="0" x2="50%" y2="100%" stroke="rgba(255,255,255,0.5)" strokeWidth="1" strokeDasharray="5,5" />
              <line x1="0" y1="50%" x2="100%" y2="50%" stroke="rgba(255,255,255,0.5)" strokeWidth="1" strokeDasharray="5,5" />
            </svg>
          )}

          {/* Safe Zones */}
          {showSafeZones && (
            <div className="absolute inset-0 pointer-events-none">
              <div
                className="absolute border-2 border-yellow-500/50"
                style={{
                  left: '5%',
                  right: '5%',
                  top: '5%',
                  bottom: '5%',
                }}
              />
              <div
                className="absolute border-2 border-red-500/50"
                style={{
                  left: '10%',
                  right: '10%',
                  top: '10%',
                  bottom: '10%',
                }}
              />
            </div>
          )}
        </div>
      </div>

      {/* No Video Placeholder */}
      {!videoRef.current?.src && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="text-center text-gray-600">
            <svg
              width="64"
              height="64"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1"
              className="mx-auto mb-4 opacity-30"
            >
              <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18" />
              <line x1="7" y1="2" x2="7" y2="22" />
              <line x1="17" y1="2" x2="17" y2="22" />
              <line x1="2" y1="12" x2="22" y2="12" />
              <line x1="2" y1="7" x2="7" y2="7" />
              <line x1="2" y1="17" x2="7" y2="17" />
              <line x1="17" y1="17" x2="22" y2="17" />
              <line x1="17" y1="7" x2="22" y2="7" />
            </svg>
            <p className="text-sm">No video loaded</p>
            <p className="text-xs mt-1">Drag a video from assets to timeline</p>
          </div>
        </div>
      )}
    </div>
  );
}
