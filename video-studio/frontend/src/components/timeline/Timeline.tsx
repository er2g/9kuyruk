import { useState, useRef, useEffect } from 'react';
import { Film, Volume2, Image as ImageIcon, Type } from 'lucide-react';

interface TimelineProps {
  currentTime: number;
  onTimeChange: (time: number) => void;
  onLayerSelect: (layer: any) => void;
}

interface Track {
  id: string;
  type: 'video' | 'audio' | 'overlay' | 'subtitle';
  name: string;
  clips: Clip[];
}

interface Clip {
  id: string;
  name: string;
  start: number;
  duration: number;
  asset?: string;
}

export default function Timeline({ currentTime, onTimeChange, onLayerSelect }: TimelineProps) {
  const [tracks, setTracks] = useState<Track[]>([
    { id: 'v1', type: 'video', name: 'Video 1', clips: [] },
    { id: 'v2', type: 'video', name: 'Video 2', clips: [] },
    { id: 'o1', type: 'overlay', name: 'Overlay 1', clips: [] },
    { id: 'o2', type: 'overlay', name: 'Overlay 2', clips: [] },
    { id: 's1', type: 'subtitle', name: 'Subtitles', clips: [] },
    { id: 'a1', type: 'audio', name: 'Audio 1', clips: [] },
  ]);

  const [zoom, setZoom] = useState(10); // pixels per second
  const [selectedClip, setSelectedClip] = useState<string | null>(null);
  const timelineRef = useRef<HTMLDivElement>(null);

  const duration = 60; // 60 seconds for demo
  const timelineWidth = duration * zoom;

  const handleClipClick = (clip: Clip) => {
    setSelectedClip(clip.id);
    onLayerSelect(clip);
  };

  const handleTimelineClick = (e: React.MouseEvent<HTMLDivElement>) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left - 120; // subtract track label width
    const time = Math.max(0, x / zoom);
    onTimeChange(time);
  };

  const getTrackIcon = (type: Track['type']) => {
    switch (type) {
      case 'video': return <Film size={14} />;
      case 'audio': return <Volume2 size={14} />;
      case 'overlay': return <ImageIcon size={14} />;
      case 'subtitle': return <Type size={14} />;
    }
  };

  const renderTimeRuler = () => {
    const markers = [];
    for (let i = 0; i <= duration; i += 5) {
      markers.push(
        <div
          key={i}
          className="absolute text-xs text-gray-500"
          style={{ left: `${i * zoom + 120}px`, top: '4px' }}
        >
          {formatTime(i)}
        </div>
      );
    }
    return markers;
  };

  return (
    <div className="timeline-container h-full flex flex-col">
      {/* Zoom Controls */}
      <div className="flex items-center justify-between px-4 py-2 bg-gray-900 border-b border-gray-800">
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500">Zoom:</span>
          <input
            type="range"
            min="5"
            max="50"
            value={zoom}
            onChange={(e) => setZoom(Number(e.target.value))}
            className="w-24"
          />
          <span className="text-xs text-gray-400">{zoom}px/s</span>
        </div>
        <div className="flex gap-2">
          <button className="btn text-xs py-1 px-3">Add Track</button>
        </div>
      </div>

      {/* Timeline Ruler */}
      <div className="timeline-ruler relative" style={{ paddingLeft: '120px' }}>
        {renderTimeRuler()}
      </div>

      {/* Tracks */}
      <div className="timeline-tracks flex-1 relative" ref={timelineRef}>
        <div className="relative" style={{ width: `${timelineWidth + 120}px` }}>
          {tracks.map((track) => (
            <div key={track.id} className="timeline-track">
              {/* Track Label */}
              <div className="timeline-track-label">
                {getTrackIcon(track.type)}
                <span>{track.name}</span>
              </div>

              {/* Track Content */}
              <div
                className="timeline-track-content"
                onClick={handleTimelineClick}
              >
                {track.clips.map((clip) => (
                  <div
                    key={clip.id}
                    className={`timeline-clip ${track.type} ${selectedClip === clip.id ? 'selected' : ''}`}
                    style={{
                      left: `${clip.start * zoom}px`,
                      width: `${clip.duration * zoom}px`,
                    }}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleClipClick(clip);
                    }}
                  >
                    <div className="timeline-clip-label">{clip.name}</div>
                    {track.type === 'audio' && (
                      <div className="absolute inset-0 opacity-30">
                        {/* Waveform placeholder */}
                        <svg width="100%" height="100%" className="opacity-50">
                          <path
                            d={generateWaveform(clip.duration * zoom)}
                            fill="none"
                            stroke="white"
                            strokeWidth="1"
                          />
                        </svg>
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* Playhead */}
        <div
          className="timeline-playhead"
          style={{ left: `${currentTime * zoom + 120}px` }}
        />
      </div>
    </div>
  );
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

function generateWaveform(width: number): string {
  const points = Math.floor(width / 4);
  let path = 'M 0 16';
  for (let i = 0; i < points; i++) {
    const x = i * 4;
    const y = 16 + Math.random() * 12 - 6;
    path += ` L ${x} ${y}`;
  }
  return path;
}
