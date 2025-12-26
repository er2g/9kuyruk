import { useState, useEffect } from 'react';
import { Sliders, Type, Image, Move, RotateCw, Maximize2 } from 'lucide-react';

interface PropertiesPanelProps {
  selectedLayer: any;
}

export default function PropertiesPanel({ selectedLayer }: PropertiesPanelProps) {
  const [activeSection, setActiveSection] = useState<'transform' | 'effect' | 'motion'>('transform');

  if (!selectedLayer) {
    return (
      <div className="panel h-full">
        <div className="panel-header">
          <span className="panel-title">Properties</span>
        </div>
        <div className="panel-content">
          <div className="text-center text-gray-600 py-12">
            <Sliders size={48} className="mx-auto mb-2 opacity-30" />
            <p className="text-sm">No layer selected</p>
            <p className="text-xs mt-1">Select a clip from timeline</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="panel h-full">
      <div className="panel-header">
        <span className="panel-title">Properties</span>
      </div>

      {/* Section Tabs */}
      <div className="flex border-b border-gray-800">
        <button
          className={`flex-1 py-2 text-xs font-medium ${
            activeSection === 'transform' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-gray-500'
          }`}
          onClick={() => setActiveSection('transform')}
        >
          Transform
        </button>
        <button
          className={`flex-1 py-2 text-xs font-medium ${
            activeSection === 'effect' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-gray-500'
          }`}
          onClick={() => setActiveSection('effect')}
        >
          Effects
        </button>
        <button
          className={`flex-1 py-2 text-xs font-medium ${
            activeSection === 'motion' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-gray-500'
          }`}
          onClick={() => setActiveSection('motion')}
        >
          Motion
        </button>
      </div>

      <div className="panel-content">
        {activeSection === 'transform' && (
          <div className="space-y-4">
            {/* Position */}
            <div className="property-group">
              <div className="property-label flex items-center gap-2">
                <Move size={12} />
                Position
              </div>
              <div className="property-row">
                <div className="flex-1">
                  <label className="text-xs text-gray-500">X</label>
                  <input type="number" className="property-input" defaultValue={0} />
                </div>
                <div className="flex-1">
                  <label className="text-xs text-gray-500">Y</label>
                  <input type="number" className="property-input" defaultValue={0} />
                </div>
              </div>
            </div>

            {/* Scale */}
            <div className="property-group">
              <div className="property-label flex items-center gap-2">
                <Maximize2 size={12} />
                Scale
              </div>
              <div className="property-row">
                <div className="flex-1">
                  <label className="text-xs text-gray-500">Width %</label>
                  <input type="number" className="property-input" defaultValue={100} />
                </div>
                <div className="flex-1">
                  <label className="text-xs text-gray-500">Height %</label>
                  <input type="number" className="property-input" defaultValue={100} />
                </div>
              </div>
              <div className="mt-2">
                <label className="flex items-center gap-2 text-xs text-gray-400 cursor-pointer">
                  <input type="checkbox" defaultChecked />
                  Uniform scaling
                </label>
              </div>
            </div>

            {/* Rotation */}
            <div className="property-group">
              <div className="property-label flex items-center gap-2">
                <RotateCw size={12} />
                Rotation
              </div>
              <input
                type="range"
                min="-180"
                max="180"
                defaultValue={0}
                className="w-full"
              />
              <input type="number" className="property-input mt-2" defaultValue={0} />
            </div>

            {/* Opacity */}
            <div className="property-group">
              <div className="property-label">Opacity</div>
              <input
                type="range"
                min="0"
                max="100"
                defaultValue={100}
                className="w-full"
              />
              <div className="flex items-center gap-2 mt-2">
                <input type="number" className="property-input flex-1" defaultValue={100} />
                <span className="text-xs text-gray-500">%</span>
              </div>
            </div>

            {/* Anchor Point */}
            <div className="property-group">
              <div className="property-label">Anchor Point</div>
              <div className="grid grid-cols-3 gap-1">
                {['TL', 'TC', 'TR', 'CL', 'C', 'CR', 'BL', 'BC', 'BR'].map((pos) => (
                  <button
                    key={pos}
                    className="btn text-xs py-1"
                  >
                    {pos}
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}

        {activeSection === 'effect' && (
          <div className="space-y-4">
            <div className="property-group">
              <div className="property-label">Applied Effects</div>
              <div className="text-xs text-gray-600">No effects applied</div>
              <button className="btn w-full mt-2">+ Add Effect</button>
            </div>

            {/* Quick Effects */}
            <div className="property-group">
              <div className="property-label">Quick Adjustments</div>

              <div className="space-y-3">
                <div>
                  <label className="text-xs text-gray-500">Brightness</label>
                  <input type="range" min="-100" max="100" defaultValue={0} className="w-full" />
                </div>

                <div>
                  <label className="text-xs text-gray-500">Contrast</label>
                  <input type="range" min="-100" max="100" defaultValue={0} className="w-full" />
                </div>

                <div>
                  <label className="text-xs text-gray-500">Saturation</label>
                  <input type="range" min="-100" max="100" defaultValue={0} className="w-full" />
                </div>

                <div>
                  <label className="text-xs text-gray-500">Blur</label>
                  <input type="range" min="0" max="100" defaultValue={0} className="w-full" />
                </div>
              </div>
            </div>
          </div>
        )}

        {activeSection === 'motion' && (
          <div className="space-y-4">
            <div className="property-group">
              <div className="property-label">Animation</div>
              <button className="btn w-full">+ Add Keyframe</button>
            </div>

            <div className="property-group">
              <div className="property-label">Motion Presets</div>
              <div className="space-y-1">
                <button className="effect-item w-full">Fade In</button>
                <button className="effect-item w-full">Fade Out</button>
                <button className="effect-item w-full">Slide In</button>
                <button className="effect-item w-full">Slide Out</button>
                <button className="effect-item w-full">Zoom In</button>
                <button className="effect-item w-full">Zoom Out</button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
