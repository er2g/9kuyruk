import { useState } from 'react';
import { Upload, Folder, Film, Image, Music, FileText } from 'lucide-react';
import { useDropzone } from 'react-dropzone';

export default function AssetPanel() {
  const [activeTab, setActiveTab] = useState<'media' | 'effects' | 'text'>('media');
  const [assets, setAssets] = useState<any[]>([]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop: (files) => {
      console.log('Files dropped:', files);
      // TODO: Upload to server
      const newAssets = files.map(file => ({
        id: Math.random().toString(),
        name: file.name,
        type: file.type.startsWith('video/') ? 'video' :
              file.type.startsWith('image/') ? 'image' :
              file.type.startsWith('audio/') ? 'audio' : 'other',
        thumbnail: URL.createObjectURL(file),
      }));
      setAssets([...assets, ...newAssets]);
    },
  });

  return (
    <div className="panel h-full">
      <div className="panel-header">
        <span className="panel-title">Project Assets</span>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-800">
        <button
          className={`flex-1 py-2 text-xs font-medium ${
            activeTab === 'media' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-gray-500'
          }`}
          onClick={() => setActiveTab('media')}
        >
          Media
        </button>
        <button
          className={`flex-1 py-2 text-xs font-medium ${
            activeTab === 'effects' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-gray-500'
          }`}
          onClick={() => setActiveTab('effects')}
        >
          Effects
        </button>
        <button
          className={`flex-1 py-2 text-xs font-medium ${
            activeTab === 'text' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-gray-500'
          }`}
          onClick={() => setActiveTab('text')}
        >
          Text
        </button>
      </div>

      <div className="panel-content">
        {activeTab === 'media' && (
          <>
            {/* Upload Area */}
            <div
              {...getRootProps()}
              className={`drop-zone mb-4 ${isDragActive ? 'active' : ''}`}
            >
              <input {...getInputProps()} />
              <Upload size={32} className="mx-auto mb-2 opacity-50" />
              <p className="text-sm">
                {isDragActive ? 'Drop files here' : 'Drag & drop files or click to browse'}
              </p>
              <p className="text-xs mt-1">Video, Audio, Images</p>
            </div>

            {/* Asset Grid */}
            {assets.length === 0 ? (
              <div className="text-center text-gray-600 py-12">
                <Folder size={48} className="mx-auto mb-2 opacity-30" />
                <p className="text-sm">No assets yet</p>
              </div>
            ) : (
              <div className="asset-grid">
                {assets.map((asset) => (
                  <div
                    key={asset.id}
                    className="asset-item"
                    draggable
                    onDragStart={(e) => {
                      e.dataTransfer.setData('asset', JSON.stringify(asset));
                    }}
                  >
                    <div className="relative w-full h-full bg-gray-800">
                      {asset.type === 'image' && (
                        <img
                          src={asset.thumbnail}
                          alt={asset.name}
                          className="asset-thumbnail"
                        />
                      )}
                      {asset.type === 'video' && (
                        <div className="flex items-center justify-center h-full">
                          <Film size={32} className="text-gray-600" />
                        </div>
                      )}
                      {asset.type === 'audio' && (
                        <div className="flex items-center justify-center h-full">
                          <Music size={32} className="text-gray-600" />
                        </div>
                      )}
                    </div>
                    <div className="asset-name">{asset.name}</div>
                  </div>
                ))}
              </div>
            )}
          </>
        )}

        {activeTab === 'effects' && (
          <div className="space-y-2">
            <div className="effect-category">
              <div className="effect-category-title">Color Correction</div>
              <div className="effect-list">
                <button className="effect-item">Brightness & Contrast</button>
                <button className="effect-item">Hue & Saturation</button>
                <button className="effect-item">Color Balance</button>
                <button className="effect-item">LUT</button>
              </div>
            </div>

            <div className="effect-category">
              <div className="effect-category-title">Blur & Sharpen</div>
              <div className="effect-list">
                <button className="effect-item">Gaussian Blur</button>
                <button className="effect-item">Motion Blur</button>
                <button className="effect-item">Sharpen</button>
                <button className="effect-item">Unsharp Mask</button>
              </div>
            </div>

            <div className="effect-category">
              <div className="effect-category-title">Transform</div>
              <div className="effect-list">
                <button className="effect-item">Scale</button>
                <button className="effect-item">Position</button>
                <button className="effect-item">Rotation</button>
                <button className="effect-item">Crop</button>
              </div>
            </div>

            <div className="effect-category">
              <div className="effect-category-title">Transitions</div>
              <div className="effect-list">
                <button className="effect-item">Fade</button>
                <button className="effect-item">Dissolve</button>
                <button className="effect-item">Wipe</button>
                <button className="effect-item">Slide</button>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'text' && (
          <div className="space-y-2">
            <button className="btn w-full flex items-center justify-center gap-2">
              <FileText size={16} />
              Add Text Layer
            </button>

            <div className="mt-4 space-y-2">
              <div className="effect-category-title">Text Presets</div>
              <div className="effect-list">
                <button className="effect-item">Lower Third</button>
                <button className="effect-item">Title Card</button>
                <button className="effect-item">End Credits</button>
                <button className="effect-item">Subtitle</button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
