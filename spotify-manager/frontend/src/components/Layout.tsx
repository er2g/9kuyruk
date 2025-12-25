import { Outlet, NavLink } from 'react-router-dom';
import { useStore } from '../store';

export default function Layout() {
  const { user, logout } = useStore();

  const navItems = [
    { to: '/', label: '🎵 Dashboard', icon: '🎛️' },
    { to: '/devices', label: '📱 Cihazlar', icon: '📱' },
    { to: '/playlists', label: '📋 Playlistler', icon: '📋' },
    { to: '/settings', label: '⚙️ Ayarlar', icon: '⚙️' },
  ];

  return (
    <div className="min-h-screen flex flex-col">
      {/* Header */}
      <header className="bg-spotify-black border-b border-gray-800">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold text-spotify-green">🎵 Spotify Manager</h1>
            <div className="flex items-center gap-4">
              <span className="text-gray-400">{user?.email}</span>
              <button onClick={logout} className="btn-secondary">
                Çıkış
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="bg-gray-900 border-b border-gray-800">
        <div className="container mx-auto px-4">
          <div className="flex gap-1">
            {navItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                end={item.to === '/'}
                className={({ isActive }) =>
                  `px-4 py-3 font-medium transition-colors ${
                    isActive
                      ? 'text-spotify-green border-b-2 border-spotify-green'
                      : 'text-gray-400 hover:text-white'
                  }`
                }
              >
                {item.label}
              </NavLink>
            ))}
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex-1 container mx-auto px-4 py-8">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="bg-spotify-black border-t border-gray-800 py-4 text-center text-gray-500 text-sm">
        © 2024 Spotify Manager - Web tabanlı uzaktan kontrol sistemi
      </footer>
    </div>
  );
}
