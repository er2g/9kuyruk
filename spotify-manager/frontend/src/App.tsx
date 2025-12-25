import { useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { useStore } from './store';
import { api } from './services/api';
import { websocket } from './services/websocket';

// Pages
import Login from './pages/Login';
import Dashboard from './pages/Dashboard';
import DevicesPage from './pages/Devices';
import PlaylistsPage from './pages/Playlists';
import SettingsPage from './pages/Settings';

// Layout
import Layout from './components/Layout';

function App() {
  const { isAuthenticated, setSpotifyConnected, setSettings } = useStore();

  useEffect(() => {
    if (isAuthenticated) {
      // Check Spotify connection status
      api.getSpotifyStatus().then((status) => {
        setSpotifyConnected(status.connected);
      });

      // Load settings
      api.getSettings().then(setSettings);

      // Connect WebSocket
      websocket.connect();

      return () => {
        websocket.disconnect();
      };
    }
  }, [isAuthenticated, setSpotifyConnected, setSettings]);

  if (!isAuthenticated) {
    return (
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="*" element={<Navigate to="/login" replace />} />
      </Routes>
    );
  }

  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Dashboard />} />
        <Route path="devices" element={<DevicesPage />} />
        <Route path="playlists" element={<PlaylistsPage />} />
        <Route path="settings" element={<SettingsPage />} />
      </Route>
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}

export default App;
