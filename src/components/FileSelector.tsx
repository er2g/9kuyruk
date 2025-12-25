import React from "react";

interface FileSelectorProps {
  label: string;
  value: string;
  onClick: () => void;
  icon?: string;
}

const FileSelector: React.FC<FileSelectorProps> = ({ label, value, onClick, icon }) => {
  const displayValue = value ? value.split("/").pop() || value : "Dosya seçilmedi";
  const isSelected = Boolean(value);

  return (
    <div className="mb-4">
      <label className="block text-sm font-medium text-gray-300 mb-2">
        {icon && <span className="mr-2">{icon}</span>}
        {label}
      </label>
      <div className="flex gap-2">
        <input
          type="text"
          value={displayValue}
          readOnly
          title={value}
          className={`flex-1 px-3 py-2 rounded-lg border ${
            isSelected
              ? "bg-gray-900 border-green-600 text-white"
              : "bg-gray-700 border-gray-600 text-gray-400"
          } focus:outline-none focus:ring-2 focus:ring-blue-500`}
        />
        <button
          onClick={onClick}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg font-medium transition-colors whitespace-nowrap"
        >
          Seç
        </button>
      </div>
      {value && (
        <p className="mt-1 text-xs text-gray-500 truncate" title={value}>
          {value}
        </p>
      )}
    </div>
  );
};

export default FileSelector;
