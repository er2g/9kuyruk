import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Interval, SrtEntry } from "../types";
import { convertFileSrc } from "@tauri-apps/api/core";

interface PreviewPanelProps {
  videoPath: string;
  intervals: Interval[];
  srtPath: string;
  currentTime: number;
  onTimeChange: (time: number) => void;
  maxDuration: number;
}

const PreviewPanel: React.FC<PreviewPanelProps> = ({
  videoPath,
  intervals,
  srtPath,
  currentTime,
  onTimeChange,
  maxDuration,
}) => {
  const [previewUrl, setPreviewUrl] = useState<string>("");
  const [subtitles, setSubtitles] = useState<SrtEntry[]>([]);
  const [currentSubtitle, setCurrentSubtitle] = useState<string>("");
  const [currentOverlay, setCurrentOverlay] = useState<string>("");
  const [isGenerating, setIsGenerating] = useState(false);

  // Load subtitles when SRT file is selected
  useEffect(() => {
    if (srtPath) {
      loadSubtitles();
    }
  }, [srtPath]);

  // Update current subtitle and overlay based on time
  useEffect(() => {
    // Find current subtitle
    const sub = subtitles.find(
      (s) => currentTime >= s.start_seconds && currentTime <= s.end_seconds
    );
    setCurrentSubtitle(sub?.text || "");

    // Find current overlay
    const overlay = intervals.find(
      (i) => currentTime >= i.start && currentTime <= i.end
    );
    setCurrentOverlay(overlay?.imagePath || "");
  }, [currentTime, subtitles, intervals]);

  const loadSubtitles = async () => {
    try {
      const subs: SrtEntry[] = await invoke("parse_srt_file", { srtPath });
      setSubtitles(subs);
    } catch (error) {
      console.error("Altyazı yüklenemedi:", error);
    }
  };

  const generatePreview = async () => {
    if (!videoPath) return;

    setIsGenerating(true);
    try {
      const tempPath = `/tmp/preview_${Date.now()}.jpg`;

      await invoke("generate_preview_frame", {
        videoPath,
        overlayPath: currentOverlay || null,
        timestamp: currentTime,
        outputPath: tempPath,
        subtitleText: currentSubtitle || null,
      });

      // Convert to asset URL for display
      setPreviewUrl(convertFileSrc(tempPath));
    } catch (error) {
      console.error("Preview oluşturulamadı:", error);
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="space-y-4">
      {/* Preview Display */}
      <div className="aspect-[9/16] bg-gray-900 rounded-lg overflow-hidden border-2 border-gray-700 flex items-center justify-center">
        {previewUrl ? (
          <img
            src={previewUrl}
            alt="Preview"
            className="w-full h-full object-contain"
          />
        ) : (
          <div className="text-center text-gray-500">
            <div className="text-6xl mb-4">🎬</div>
            <p>Önizleme burada görünecek</p>
            <p className="text-sm mt-2">Zaman seçip "Önizleme Oluştur"a basın</p>
          </div>
        )}
      </div>

      {/* Timeline Slider */}
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-300">
          Zaman: {currentTime.toFixed(1)}s / {maxDuration.toFixed(1)}s
        </label>
        <input
          type="range"
          min="0"
          max={maxDuration}
          step="0.1"
          value={currentTime}
          onChange={(e) => onTimeChange(parseFloat(e.target.value))}
          className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
          disabled={!videoPath}
        />
      </div>

      {/* Preview Info */}
      <div className="grid grid-cols-2 gap-3 text-sm">
        <div className="p-3 bg-gray-900 rounded border border-gray-600">
          <div className="text-gray-400 mb-1">Overlay</div>
          <div className="text-white truncate" title={currentOverlay}>
            {currentOverlay ? currentOverlay.split("/").pop() : "Yok"}
          </div>
        </div>
        <div className="p-3 bg-gray-900 rounded border border-gray-600">
          <div className="text-gray-400 mb-1">Altyazı</div>
          <div className="text-white truncate" title={currentSubtitle}>
            {currentSubtitle || "Yok"}
          </div>
        </div>
      </div>

      {/* Generate Button */}
      <button
        onClick={generatePreview}
        disabled={!videoPath || isGenerating}
        className="w-full px-4 py-3 bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 disabled:from-gray-600 disabled:to-gray-600 disabled:cursor-not-allowed rounded-lg font-bold transition-all shadow-lg"
      >
        {isGenerating ? "Oluşturuluyor..." : "🔍 Önizleme Oluştur"}
      </button>

      {/* Quick Time Buttons */}
      {intervals.length > 0 && (
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-300">Hızlı Atlama</label>
          <div className="grid grid-cols-3 gap-2">
            {intervals.slice(0, 6).map((interval, index) => (
              <button
                key={index}
                onClick={() => {
                  const midPoint = (interval.start + interval.end) / 2;
                  onTimeChange(midPoint);
                }}
                className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-xs transition-colors"
              >
                #{interval.imageIndex} ({interval.start.toFixed(1)}s)
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default PreviewPanel;
