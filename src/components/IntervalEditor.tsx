import React from "react";
import { Interval, ImageInfo } from "../types";

interface IntervalEditorProps {
  intervals: Interval[];
  onChange: (intervals: Interval[]) => void;
  images: ImageInfo[];
  maxDuration: number;
}

const IntervalEditor: React.FC<IntervalEditorProps> = ({
  intervals,
  onChange,
  images,
  maxDuration,
}) => {
  const handleChange = (index: number, field: "start" | "end", value: string) => {
    const numValue = parseFloat(value) || 0;
    const newIntervals = [...intervals];
    newIntervals[index] = {
      ...newIntervals[index],
      [field]: numValue,
    };
    onChange(newIntervals);
  };

  const addInterval = () => {
    if (images.length === 0) return;

    const lastEnd = intervals.length > 0 ? intervals[intervals.length - 1].end : 0;
    const nextImage = images[intervals.length % images.length];

    onChange([
      ...intervals,
      {
        imageIndex: nextImage.index,
        imagePath: nextImage.path,
        start: lastEnd,
        end: Math.min(lastEnd + 5, maxDuration),
      },
    ]);
  };

  const removeInterval = (index: number) => {
    onChange(intervals.filter((_, i) => i !== index));
  };

  const parseManualInput = (text: string) => {
    const lines = text.trim().split("\n");
    const newIntervals: Interval[] = [];

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      const match = line.match(/^(\d+(?:[.,]\d+)?)\s*-\s*(\d+(?:[.,]\d+)?)$/);

      if (match && images[i]) {
        const start = parseFloat(match[1].replace(",", "."));
        const end = parseFloat(match[2].replace(",", "."));

        if (start < end) {
          newIntervals.push({
            imageIndex: images[i].index,
            imagePath: images[i].path,
            start,
            end,
          });
        }
      }
    }

    if (newIntervals.length > 0) {
      onChange(newIntervals);
    }
  };

  return (
    <div className="space-y-3">
      {/* Manual Text Input */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-300 mb-2">
          Manuel Giriş (her satır: başlangıç-bitiş)
        </label>
        <textarea
          className="w-full px-3 py-2 bg-gray-900 border border-gray-600 rounded-lg text-white font-mono text-sm focus:outline-none focus:ring-2 focus:ring-purple-500 scrollbar-thin"
          rows={5}
          placeholder="0-7.3&#10;7.3-14.6&#10;14.6-21.9"
          onBlur={(e) => {
            if (e.target.value.trim()) {
              parseManualInput(e.target.value);
            }
          }}
        />
      </div>

      <div className="h-64 overflow-y-auto scrollbar-thin space-y-2">
        {intervals.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            Henüz aralık eklenmedi
            <br />
            <span className="text-sm">Otomatik doldur veya manuel ekle</span>
          </div>
        ) : (
          intervals.map((interval, index) => (
            <div
              key={index}
              className="flex items-center gap-2 p-3 bg-gray-900 rounded-lg border border-gray-600"
            >
              <span className="text-purple-400 font-bold text-sm w-8">#{interval.imageIndex}</span>
              <input
                type="number"
                step="0.1"
                value={interval.start.toFixed(1)}
                onChange={(e) => handleChange(index, "start", e.target.value)}
                className="w-20 px-2 py-1 bg-gray-800 border border-gray-600 rounded text-sm text-white focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
              <span className="text-gray-500">→</span>
              <input
                type="number"
                step="0.1"
                value={interval.end.toFixed(1)}
                onChange={(e) => handleChange(index, "end", e.target.value)}
                className="w-20 px-2 py-1 bg-gray-800 border border-gray-600 rounded text-sm text-white focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
              <span className="text-gray-500 text-xs">
                ({(interval.end - interval.start).toFixed(1)}s)
              </span>
              <button
                onClick={() => removeInterval(index)}
                className="ml-auto px-2 py-1 bg-red-600 hover:bg-red-700 rounded text-sm transition-colors"
              >
                ✕
              </button>
            </div>
          ))
        )}
      </div>

      <button
        onClick={addInterval}
        disabled={images.length === 0}
        className="w-full px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed rounded-lg font-medium transition-colors"
      >
        + Aralık Ekle
      </button>
    </div>
  );
};

export default IntervalEditor;
