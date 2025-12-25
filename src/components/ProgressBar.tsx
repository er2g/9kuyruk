import React from "react";

interface ProgressBarProps {
  progress: number;
  max: number;
}

const ProgressBar: React.FC<ProgressBarProps> = ({ progress, max }) => {
  const percentage = max > 0 ? (progress / max) * 100 : 0;

  return (
    <div className="w-full">
      <div className="flex justify-between text-sm text-gray-400 mb-1">
        <span>İlerleme</span>
        <span>{percentage.toFixed(1)}%</span>
      </div>
      <div className="w-full h-3 bg-gray-700 rounded-full overflow-hidden">
        <div
          className="h-full bg-gradient-to-r from-blue-500 to-purple-600 transition-all duration-300 ease-out"
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  );
};

export default ProgressBar;
