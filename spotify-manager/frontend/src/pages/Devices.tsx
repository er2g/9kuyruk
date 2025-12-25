import { useState, useEffect } from 'react';
import { api } from '../services/api';
import type { SpotifyDevice } from '../types';

export default function Devices() {
  const [devices, setDevices] = useState<SpotifyDevice[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    try {
      const data = await api.getMyDevices();
      setDevices(data);
    } catch (error) {
      console.error('Failed to load devices:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleLock = async (device: SpotifyDevice) => {
    try {
      if (device.is_locked) {
        await api.unlockDevice(device.id);
      } else {
        await api.lockDevice(device.id);
      }
      await loadDevices();
    } catch (error) {
      alert('İşlem başarısız');
    }
  };

  const handleSetPrimary = async (device: SpotifyDevice) => {
    try {
      await api.setPrimaryDevice(device.id);
      await loadDevices();
    } catch (error) {
      alert('İşlem başarısız');
    }
  };

  if (loading) {
    return <div className="text-center py-12">Yükleniyor...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">📱 Cihaz Yönetimi</h1>
        <button onClick={loadDevices} className="btn-secondary">
          🔄 Yenile
        </button>
      </div>

      {devices.length === 0 ? (
        <div className="card text-center py-12">
          <p className="text-gray-400">Henüz cihaz kaydı yok</p>
          <p className="text-sm text-gray-500 mt-2">
            Spotify uygulamasını bir cihazda açtığınızda otomatik olarak kaydedilecektir.
          </p>
        </div>
      ) : (
        <div className="grid gap-4">
          {devices.map((device) => (
            <div key={device.id} className="card">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <h3 className="text-xl font-bold">{device.device_name}</h3>
                    {device.is_primary && (
                      <span className="px-3 py-1 bg-spotify-green/20 text-spotify-green text-sm rounded-full">
                        ⭐ Birincil Cihaz
                      </span>
                    )}
                    {device.is_locked && (
                      <span className="px-3 py-1 bg-red-900/50 text-red-400 text-sm rounded-full">
                        🔒 Kilitli
                      </span>
                    )}
                  </div>

                  <div className="text-sm text-gray-400 space-y-1">
                    <div>Tip: {device.device_type}</div>
                    <div>Son görülme: {new Date(device.last_seen).toLocaleString('tr-TR')}</div>
                    <div>ID: {device.spotify_device_id}</div>
                  </div>
                </div>

                <div className="flex gap-2">
                  {!device.is_primary && (
                    <button
                      onClick={() => handleSetPrimary(device)}
                      className="btn-secondary"
                      title="Birincil cihaz olarak ayarla"
                    >
                      ⭐ Birincil Yap
                    </button>
                  )}
                  <button
                    onClick={() => handleLock(device)}
                    className={device.is_locked ? 'btn-primary' : 'btn-danger'}
                  >
                    {device.is_locked ? '🔓 Kilidi Aç' : '🔒 Kilitle'}
                  </button>
                </div>
              </div>

              {device.is_primary && (
                <div className="mt-4 p-3 bg-blue-900/20 border border-blue-600/50 rounded-lg text-sm">
                  <p className="text-blue-300">
                    <strong>ℹ️ Birincil Cihaz:</strong> Bu cihaz kaybolursa veya bağlantısı kesilirse
                    sistem otomatik olarak müziği duraklatacaktır (ayarlar bölümünden kontrol edilebilir).
                  </p>
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      <div className="card bg-gray-900/50">
        <h3 className="font-bold mb-2">💡 Bilgi</h3>
        <ul className="text-sm text-gray-400 space-y-1">
          <li>• <strong>Birincil Cihaz:</strong> Kaybolan cihazı takip etmek için bir cihaz seçin</li>
          <li>• <strong>Kilitle:</strong> Kilitli cihazlarda müzik oynatımını engelleyin</li>
          <li>• <strong>Otomatik Kayıt:</strong> Spotify'da aktif olan cihazlar otomatik olarak burada görünür</li>
        </ul>
      </div>
    </div>
  );
}
