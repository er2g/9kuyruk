import {
  Save,
  Undo,
  Redo,
  Scissors,
  Copy,
  Clipboard,
  Download,
  Settings,
  Play,
  Square
} from 'lucide-react';

interface ToolbarProps {
  projectId: string;
  onSave?: () => void;
  onUndo?: () => void;
  onRedo?: () => void;
  onExport?: () => void;
}

export default function Toolbar({ projectId, onSave, onUndo, onRedo, onExport }: ToolbarProps) {
  return (
    <div className="toolbar">
      {/* File Operations */}
      <div className="toolbar-group">
        <button
          className="tool-button tooltip"
          data-tooltip="Save (Ctrl+S)"
          onClick={onSave}
        >
          <Save size={18} />
        </button>
      </div>

      {/* Edit Operations */}
      <div className="toolbar-group">
        <button
          className="tool-button tooltip"
          data-tooltip="Undo (Ctrl+Z)"
          onClick={onUndo}
        >
          <Undo size={18} />
        </button>
        <button
          className="tool-button tooltip"
          data-tooltip="Redo (Ctrl+Shift+Z)"
          onClick={onRedo}
        >
          <Redo size={18} />
        </button>
      </div>

      {/* Tools */}
      <div className="toolbar-group">
        <button className="tool-button tooltip" data-tooltip="Selection Tool (V)">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M10 2L3 22L21 14L10 2Z" />
          </svg>
        </button>
        <button className="tool-button tooltip" data-tooltip="Razor Tool (C)">
          <Scissors size={18} />
        </button>
        <button className="tool-button tooltip" data-tooltip="Hand Tool (H)">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M18 11V6a2 2 0 0 0-4 0v2a2 2 0 0 0-4 0v2a2 2 0 0 0-4 0v4c0 3.31 2.69 6 6 6h2c3.31 0 6-2.69 6-6v-3" />
          </svg>
        </button>
      </div>

      {/* Clipboard */}
      <div className="toolbar-group">
        <button className="tool-button tooltip" data-tooltip="Copy (Ctrl+C)">
          <Copy size={18} />
        </button>
        <button className="tool-button tooltip" data-tooltip="Paste (Ctrl+V)">
          <Clipboard size={18} />
        </button>
      </div>

      <div className="flex-1" />

      {/* Project Info */}
      <div className="toolbar-group">
        <span className="text-xs text-gray-500">
          Project: {projectId}
        </span>
      </div>

      {/* Export */}
      <div className="toolbar-group">
        <button
          className="btn-success px-4 py-2 rounded flex items-center gap-2"
          onClick={onExport}
        >
          <Download size={16} />
          Export
        </button>
        <button className="tool-button tooltip" data-tooltip="Settings">
          <Settings size={18} />
        </button>
      </div>
    </div>
  );
}
