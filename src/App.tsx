import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import FileSelector from "./components/FileSelector";
import IntervalEditor from "./components/IntervalEditor";
import PreviewPanel from "./components/PreviewPanel";
import ProgressBar from "./components/ProgressBar";
import { VideoInfo, ImageInfo, Interval, RenderProgress } from "./types";

function App() {
  const [videoPath, setVideoPath] = useState<string>("");
  const [folderPath, setFolderPath] = useState<string>("");
  const [srtPath, setSrtPath] = useState<string>("");
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [images, setImages] = useState<ImageInfo[]>([]);
  const [intervals, setIntervals] = useState<Interval[]>([]);
  const [status, setStatus] = useState<string>("Hazır");
  const [progress, setProgress] = useState<number>(0);
  const [isRendering, setIsRendering] = useState<boolean>(false);
  const [previewTime, setPreviewTime] = useState<number>(0);

  // Listen for render progress
  useEffect(() => {
    const unlisten = listen<RenderProgress>("render_progress", (event) => {
      setProgress(event.payload.progress);
      setStatus(event.payload.message);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // Select video file
  const selectVideo = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Video",
          extensions: ["mp4", "mov", "mkv", "avi"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      setVideoPath(selected);
      setStatus("Video bilgisi alınıyor...");

      try {
        const info: VideoInfo = await invoke("get_video_info", {
          videoPath: selected,
        });
        setVideoInfo(info);
        setStatus(`Video yüklendi: ${info.duration.toFixed(1)}s, ${info.width}x${info.height}`);
      } catch (error) {
        setStatus(`Hata: ${error}`);
      }
    }
  };

  // Select images folder
  const selectFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected && typeof selected === "string") {
      setFolderPath(selected);
      setStatus("Görseller taranıyor...");

      try {
        const foundImages: ImageInfo[] = await invoke("count_images_in_folder", {
          folderPath: selected,
        });
        setImages(foundImages);
        setStatus(`${foundImages.length} görsel bulundu`);
      } catch (error) {
        setStatus(`Hata: ${error}`);
      }
    }
  };

  // Select SRT file
  const selectSrt = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Altyazı",
          extensions: ["srt"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      setSrtPath(selected);
      setStatus("SRT dosyası seçildi");
    }
  };

  // Auto-fill intervals based on video duration and image count
  const autoFillIntervals = () => {
    if (!videoInfo || images.length === 0) {
      setStatus("Önce video ve görselleri seçin!");
      return;
    }

    const duration = videoInfo.duration;
    const count = images.length;
    const step = duration / count;

    const newIntervals: Interval[] = images.map((img, index) => ({
      imageIndex: img.index,
      imagePath: img.path,
      start: index * step,
      end: index < count - 1 ? (index + 1) * step : duration,
    }));

    setIntervals(newIntervals);
    setStatus(`${count} eşit aralık oluşturuldu`);
  };

  // Start rendering
  const startRender = async () => {
    if (!videoPath || !folderPath || !srtPath || intervals.length === 0) {
      setStatus("Lütfen tüm dosyaları seçin ve aralıkları girin!");
      return;
    }

    setIsRendering(true);
    setProgress(0);
    setStatus("Render başlatılıyor...");

    try {
      // Step 1: Render video with overlays
      const overlayIntervals = intervals.map((int) => ({
        image_path: int.imagePath,
        start: int.start,
        end: int.end,
      }));

      const overlayOutput = videoPath.replace(/\.(mp4|mov|mkv|avi)$/, "_overlays.mp4");

      await invoke("render_video_with_overlays", {
        videoPath,
        intervals: overlayIntervals,
        outputPath: overlayOutput,
      });

      // Step 2: Convert SRT to ASS
      setStatus("SRT → ASS dönüşümü...");
      const assPath = srtPath.replace(".srt", ".ass");
      await invoke("srt_to_ass", {
        srtPath,
        assPath,
      });

      // Step 3: Embed subtitles
      setStatus("Altyazı gömülüyor...");
      const finalOutput = overlayOutput.replace(".mp4", "_final.mp4");
      await invoke("embed_subtitles", {
        videoPath: overlayOutput,
        assPath,
        outputPath: finalOutput,
      });

      setStatus(`✓ Tamamlandı: ${finalOutput}`);
      setProgress(100);
    } catch (error) {
      setStatus(`✗ Hata: ${error}`);
    } finally {
      setIsRendering(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 text-white">
      <div className="container mx-auto p-6">
        {/* Header */}
        <div className="mb-8 text-center">
          <h1 className="text-4xl font-bold mb-2 bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
            Video Birleştirici
          </h1>
          <p className="text-gray-400">Overlay + Altyazı - Canlı Önizleme ile</p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Left Panel - Controls */}
          <div className="space-y-6">
            {/* File Selectors */}
            <div className="bg-gray-800 rounded-lg p-6 shadow-xl border border-gray-700">
              <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
                <span className="text-blue-400">📁</span> Dosya Seçimi
              </h2>
              <FileSelector
                label="Şablon Video"
                value={videoPath}
                onClick={selectVideo}
                icon="🎬"
              />
              <FileSelector
                label="Görseller Klasörü"
                value={folderPath}
                onClick={selectFolder}
                icon="🖼️"
              />
              <FileSelector
                label="Altyazı (.srt)"
                value={srtPath}
                onClick={selectSrt}
                icon="💬"
              />

              {/* Video Info */}
              {videoInfo && (
                <div className="mt-4 p-3 bg-gray-900 rounded border border-gray-600 text-sm">
                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <span className="text-gray-400">Süre:</span>{" "}
                      <span className="text-white">{videoInfo.duration.toFixed(1)}s</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Çözünürlük:</span>{" "}
                      <span className="text-white">
                        {videoInfo.width}x{videoInfo.height}
                      </span>
                    </div>
                    <div>
                      <span className="text-gray-400">FPS:</span>{" "}
                      <span className="text-white">{videoInfo.fps.toFixed(2)}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Codec:</span>{" "}
                      <span className="text-white">{videoInfo.codec}</span>
                    </div>
                  </div>
                </div>
              )}

              {/* Image Count */}
              {images.length > 0 && (
                <div className="mt-3 p-3 bg-green-900/30 rounded border border-green-600/50 text-sm">
                  <span className="text-green-400">✓</span> {images.length} görsel bulundu
                </div>
              )}
            </div>

            {/* Interval Editor */}
            <div className="bg-gray-800 rounded-lg p-6 shadow-xl border border-gray-700">
              <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
                <span className="text-purple-400">⏱️</span> Zaman Aralıkları
              </h2>
              <button
                onClick={autoFillIntervals}
                disabled={!videoInfo || images.length === 0}
                className="w-full mb-4 px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed rounded-lg font-medium transition-colors"
              >
                Otomatik Doldur
              </button>
              <IntervalEditor
                intervals={intervals}
                onChange={setIntervals}
                images={images}
                maxDuration={videoInfo?.duration || 0}
              />
            </div>

            {/* Progress */}
            <div className="bg-gray-800 rounded-lg p-6 shadow-xl border border-gray-700">
              <div className="mb-3">
                <span className="text-sm text-gray-400">Durum:</span>
                <p className="text-white font-medium">{status}</p>
              </div>
              <ProgressBar progress={progress} max={100} />
              <button
                onClick={startRender}
                disabled={isRendering || !videoPath || !srtPath || intervals.length === 0}
                className="w-full mt-4 px-6 py-3 bg-gradient-to-r from-green-500 to-emerald-600 hover:from-green-600 hover:to-emerald-700 disabled:from-gray-600 disabled:to-gray-600 disabled:cursor-not-allowed rounded-lg font-bold text-lg transition-all shadow-lg"
              >
                {isRendering ? "İşleniyor..." : "🎬 Üret"}
              </button>
            </div>
          </div>

          {/* Right Panel - Live Preview */}
          <div className="bg-gray-800 rounded-lg p-6 shadow-xl border border-gray-700">
            <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
              <span className="text-green-400">👁️</span> Canlı Önizleme
            </h2>
            <PreviewPanel
              videoPath={videoPath}
              intervals={intervals}
              srtPath={srtPath}
              currentTime={previewTime}
              onTimeChange={setPreviewTime}
              maxDuration={videoInfo?.duration || 0}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
