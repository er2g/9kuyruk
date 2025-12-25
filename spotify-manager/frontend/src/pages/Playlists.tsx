import { useState, useEffect } from 'react';
import { api } from '../services/api';
import type { Playlist } from '../types';

export default function Playlists() {
  const [playlists, setPlaylists] = useState<Playlist[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadPlaylists();
  }, []);

  const loadPlaylists = async () => {
    try {
      const data = await api.getPlaylists();
      setPlaylists(data);
    } catch (error) {
      console.error('Failed to load playlists:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="text-center py-12">Yükleniyor...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">📋 Playlistler</h1>
        <button onClick={loadPlaylists} className="btn-secondary">
          🔄 Yenile
        </button>
      </div>

      {playlists.length === 0 ? (
        <div className="card text-center py-12">
          <p className="text-gray-400">Playlist bulunamadı</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {playlists.map((playlist) => (
            <div key={playlist.id} className="card hover:bg-gray-700/50 transition-colors cursor-pointer">
              {playlist.images[0] && (
                <img
                  src={playlist.images[0].url}
                  alt={playlist.name}
                  className="w-full h-48 object-cover rounded-lg mb-4"
                />
              )}
              <h3 className="font-bold text-lg mb-2">{playlist.name}</h3>
              {playlist.description && (
                <p className="text-sm text-gray-400 mb-3 line-clamp-2">{playlist.description}</p>
              )}
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-400">{playlist.tracks.total} şarkı</span>
                <span className="text-gray-500">{playlist.owner.display_name}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
