import { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import EditorToolbar from '../components/editor/Toolbar';
import Timeline from '../components/timeline/Timeline';
import PreviewPanel from '../components/preview/PreviewPanel';
import AssetPanel from '../components/assets/AssetPanel';
import PropertiesPanel from '../components/editor/PropertiesPanel';
import { Play, Pause, SkipBack, SkipForward, Download } from 'lucide-react';

export default function Editor() {
  const { projectId } = useParams();
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [selectedLayer, setSelectedLayer] = useState<any>(null);

  return (
    <div className="h-screen flex flex-col bg-gray-950">
      {/* Top Toolbar */}
      <EditorToolbar projectId={projectId!} />

      {/* Main Editor Area */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left Sidebar - Assets */}
        <div className="w-80 bg-gray-900 border-r border-gray-800 overflow-y-auto">
          <AssetPanel />
        </div>

        {/* Center - Preview & Timeline */}
        <div className="flex-1 flex flex-col">
          {/* Preview */}
          <div className="flex-1 bg-black flex items-center justify-center p-4">
            <PreviewPanel
              currentTime={currentTime}
              isPlaying={isPlaying}
              onTimeUpdate={setCurrentTime}
            />
          </div>

          {/* Playback Controls */}
          <div className="bg-gray-900 border-t border-gray-800 px-4 py-3 flex items-center gap-4">
            <button className="p-2 hover:bg-gray-800 rounded">
              <SkipBack size={20} />
            </button>
            <button
              onClick={() => setIsPlaying(!isPlaying)}
              className="p-3 bg-blue-600 hover:bg-blue-700 rounded-lg"
            >
              {isPlaying ? <Pause size={24} /> : <Play size={24} />}
            </button>
            <button className="p-2 hover:bg-gray-800 rounded">
              <SkipForward size={20} />
            </button>

            <div className="flex-1 mx-4">
              <div className="flex items-center gap-2 text-sm text-gray-400">
                <span>{formatTime(currentTime)}</span>
                <div className="flex-1 h-1 bg-gray-800 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-blue-600"
                    style={{ width: `${(currentTime / duration) * 100}%` }}
                  />
                </div>
                <span>{formatTime(duration)}</span>
              </div>
            </div>

            <button className="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg flex items-center gap-2">
              <Download size={18} />
              Export
            </button>
          </div>

          {/* Timeline */}
          <div className="h-80 bg-gray-900 border-t border-gray-800">
            <Timeline
              currentTime={currentTime}
              onTimeChange={setCurrentTime}
              onLayerSelect={setSelectedLayer}
            />
          </div>
        </div>

        {/* Right Sidebar - Properties */}
        <div className="w-80 bg-gray-900 border-l border-gray-800 overflow-y-auto">
          <PropertiesPanel selectedLayer={selectedLayer} />
        </div>
      </div>
    </div>
  );
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}
