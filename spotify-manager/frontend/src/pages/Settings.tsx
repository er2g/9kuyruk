import { useState, useEffect } from 'react';
import { api } from '../services/api';
import { useStore } from '../store';

export default function Settings() {
  const { settings, setSettings } = useStore();
  const [formData, setFormData] = useState({
    auto_pause_on_device_lost: true,
    device_lost_timeout_seconds: 30,
    enable_device_locking: true,
  });
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    if (settings) {
      setFormData(settings);
    }
  }, [settings]);

  const handleSave = async () => {
    setSaving(true);
    setSaved(false);
    try {
      const updated = await api.updateSettings(formData);
      setSettings(updated);
      setSaved(true);
      setTimeout(() => setSaved(false), 3000);
    } catch (error) {
      alert('Kaydetme başarısız');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="max-w-3xl space-y-6">
      <h1 className="text-3xl font-bold">⚙️ Ayarlar</h1>

      <div className="card space-y-6">
        <div>
          <div className="flex items-start justify-between mb-2">
            <div>
              <h3 className="font-bold text-lg">Cihaz Kaybında Otomatik Duraklat</h3>
              <p className="text-sm text-gray-400">
                Birincil cihaz kaybolduğunda müziği otomatik olarak duraklat
              </p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={formData.auto_pause_on_device_lost}
                onChange={(e) =>
                  setFormData({ ...formData, auto_pause_on_device_lost: e.target.checked })
                }
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-spotify-green/20 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-spotify-green"></div>
            </label>
          </div>
        </div>

        <div>
          <h3 className="font-bold text-lg mb-2">Cihaz Kayıp Zaman Aşımı</h3>
          <p className="text-sm text-gray-400 mb-4">
            Cihazın kaybolduğuna karar vermeden önce beklenecek süre (saniye)
          </p>
          <div className="flex items-center gap-4">
            <input
              type="range"
              min="10"
              max="300"
              value={formData.device_lost_timeout_seconds}
              onChange={(e) =>
                setFormData({ ...formData, device_lost_timeout_seconds: parseInt(e.target.value) })
              }
              className="flex-1 h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-spotify-green"
            />
            <span className="w-20 text-center bg-gray-700 rounded-lg px-3 py-2">
              {formData.device_lost_timeout_seconds}s
            </span>
          </div>
        </div>

        <div>
          <div className="flex items-start justify-between mb-2">
            <div>
              <h3 className="font-bold text-lg">Cihaz Kilitleme Özelliği</h3>
              <p className="text-sm text-gray-400">
                Cihazları kilitleme özelliğini etkinleştir
              </p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={formData.enable_device_locking}
                onChange={(e) =>
                  setFormData({ ...formData, enable_device_locking: e.target.checked })
                }
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-spotify-green/20 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-spotify-green"></div>
            </label>
          </div>
        </div>

        <div className="pt-4 border-t border-gray-700">
          <button onClick={handleSave} disabled={saving} className="btn-primary w-full">
            {saving ? 'Kaydediliyor...' : saved ? '✓ Kaydedildi!' : 'Kaydet'}
          </button>
        </div>
      </div>

      <div className="card bg-blue-900/10 border-blue-600/30">
        <h3 className="font-bold mb-2">ℹ️ Bilgi</h3>
        <ul className="text-sm text-gray-400 space-y-2">
          <li>• Ayarlar anında uygulanır ve tüm oturumlarınızda geçerlidir</li>
          <li>• Birincil cihaz ayarını "Cihazlar" bölümünden yapabilirsiniz</li>
          <li>• Kilitleme özelliği belirli cihazlarda müzik oynatımını engellemek için kullanılır</li>
        </ul>
      </div>
    </div>
  );
}
