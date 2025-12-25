import { create } from 'zustand';
import type { User, SpotifyDevice, Settings } from '../types';

interface AppState {
  user: User | null;
  isAuthenticated: boolean;
  spotifyConnected: boolean;
  devices: SpotifyDevice[];
  settings: Settings | null;

  setUser: (user: User | null) => void;
  setAuth: (isAuth: boolean) => void;
  setSpotifyConnected: (connected: boolean) => void;
  setDevices: (devices: SpotifyDevice[]) => void;
  setSettings: (settings: Settings) => void;
  logout: () => void;
}

export const useStore = create<AppState>((set) => ({
  user: null,
  isAuthenticated: !!localStorage.getItem('access_token'),
  spotifyConnected: false,
  devices: [],
  settings: null,

  setUser: (user) => set({ user }),
  setAuth: (isAuth) => set({ isAuthenticated: isAuth }),
  setSpotifyConnected: (connected) => set({ spotifyConnected: connected }),
  setDevices: (devices) => set({ devices }),
  setSettings: (settings) => set({ settings }),
  logout: () => {
    localStorage.clear();
    set({ user: null, isAuthenticated: false, spotifyConnected: false });
  },
}));
