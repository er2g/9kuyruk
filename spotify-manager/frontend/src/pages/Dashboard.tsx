import { useState, useEffect } from 'react';
import { api } from '../services/api';
import { useStore } from '../store';
import type { SpotifyPlayerDevice } from '../types';

export default function Dashboard() {
  const { spotifyConnected, setSpotifyConnected } = useStore();
  const [authUrl, setAuthUrl] = useState('');
  const [devices, setDevices] = useState<SpotifyPlayerDevice[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (spotifyConnected) {
      loadDevices();
    } else {
      api.getSpotifyAuthUrl().then(setAuthUrl);
    }
  }, [spotifyConnected]);

  const loadDevices = async () => {
    try {
      const data = await api.getDevices();
      setDevices(data);
    } catch (error) {
      console.error('Failed to load devices:', error);
    }
  };

  const handleConnect = () => {
    window.open(authUrl, '_blank', 'width=600,height=800');
    // Poll for connection status
    const interval = setInterval(async () => {
      const status = await api.getSpotifyStatus();
      if (status.connected) {
        setSpotifyConnected(true);
        clearInterval(interval);
      }
    }, 2000);
  };

  const handleDisconnect = async () => {
    await api.disconnectSpotify();
    setSpotifyConnected(false);
  };

  const handlePlay = async () => {
    setLoading(true);
    try {
      await api.play();
    } catch (error: any) {
      alert(error.response?.data || 'Oynatma başarısız');
    } finally {
      setLoading(false);
    }
  };

  const handlePause = async () => {
    setLoading(true);
    try {
      await api.pause();
    } catch (error: any) {
      alert(error.response?.data || 'Duraklat ma başarısız');
    } finally {
      setLoading(false);
    }
  };

  if (!spotifyConnected) {
    return (
      <div className="max-w-2xl mx-auto">
        <div className="card text-center">
          <div className="text-6xl mb-4">🎵</div>
          <h2 className="text-2xl font-bold mb-4">Spotify'a Bağlan</h2>
          <p className="text-gray-400 mb-6">
            Spotify hesabınızı bağlayarak müziklerinizi uzaktan kontrol edebilirsiniz.
          </p>
          <button onClick={handleConnect} className="btn-primary">
            Spotify'a Bağlan
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Connection Status */}
      <div className="card flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-3 h-3 bg-spotify-green rounded-full animate-pulse-green"></div>
          <span className="text-spotify-green font-medium">Spotify Bağlı</span>
        </div>
        <button onClick={handleDisconnect} className="btn-danger">
          Bağlantıyı Kes
        </button>
      </div>

      {/* Player Controls */}
      <div className="card">
        <h2 className="text-xl font-bold mb-4">Oynatıcı Kontrolü</h2>
        <div className="flex justify-center gap-4">
          <button onClick={() => api.previous()} className="btn-secondary" disabled={loading}>
            ⏮️ Önceki
          </button>
          <button onClick={handlePlay} className="btn-primary px-8" disabled={loading}>
            ▶️ Oynat
          </button>
          <button onClick={handlePause} className="btn-secondary" disabled={loading}>
            ⏸️ Duraklat
          </button>
          <button onClick={() => api.next()} className="btn-secondary" disabled={loading}>
            ⏭️ Sonraki
          </button>
        </div>
      </div>

      {/* Active Devices */}
      <div className="card">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold">Aktif Cihazlar</h2>
          <button onClick={loadDevices} className="btn-secondary">
            🔄 Yenile
          </button>
        </div>
        {devices.length === 0 ? (
          <p className="text-gray-400 text-center py-8">
            Aktif cihaz bulunamadı. Spotify uygulamasını bir cihazda açın.
          </p>
        ) : (
          <div className="space-y-2">
            {devices.map((device) => (
              <div
                key={device.id}
                className={`p-4 rounded-lg border ${
                  device.is_active
                    ? 'bg-spotify-green/10 border-spotify-green'
                    : 'bg-gray-900 border-gray-700'
                }`}
              >
                <div className="flex items-center justify-between">
                  <div>
                    <div className="font-medium">{device.name}</div>
                    <div className="text-sm text-gray-400">
                      {device.type} • {device.volume_percent}% ses
                    </div>
                  </div>
                  {device.is_active && (
                    <span className="text-spotify-green font-medium">🎵 Çalıyor</span>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
