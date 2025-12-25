import axios, { AxiosInstance } from 'axios';
import type { AuthResponse, SpotifyDevice, SpotifyPlayerDevice, Playlist, Track, Settings } from '../types';

class ApiClient {
  private client: AxiosInstance;

  constructor() {
    this.client = axios.create({
      baseURL: '/api',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add auth token to requests
    this.client.interceptors.request.use((config) => {
      const token = localStorage.getItem('access_token');
      if (token) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    });

    // Handle token refresh on 401
    this.client.interceptors.response.use(
      (response) => response,
      async (error) => {
        if (error.response?.status === 401) {
          const refreshToken = localStorage.getItem('refresh_token');
          if (refreshToken) {
            try {
              const { data } = await axios.post<AuthResponse>('/api/auth/refresh', { refresh_token: refreshToken });
              localStorage.setItem('access_token', data.access_token);
              localStorage.setItem('refresh_token', data.refresh_token);
              error.config.headers.Authorization = `Bearer ${data.access_token}`;
              return this.client.request(error.config);
            } catch {
              localStorage.clear();
              window.location.href = '/login';
            }
          }
        }
        return Promise.reject(error);
      }
    );
  }

  // Auth
  async login(email: string, password: string) {
    const { data } = await this.client.post<AuthResponse>('/auth/login', { email, password });
    return data;
  }

  async register(email: string, password: string) {
    const { data } = await this.client.post<AuthResponse>('/auth/register', { email, password });
    return data;
  }

  // Spotify
  async getSpotifyAuthUrl() {
    const { data } = await this.client.get<{ url: string }>('/spotify/auth');
    return data.url;
  }

  async getSpotifyStatus() {
    const { data } = await this.client.get<{ connected: boolean; spotify_user_id?: string }>('/spotify/status');
    return data;
  }

  async disconnectSpotify() {
    await this.client.post('/spotify/disconnect');
  }

  // Player
  async play() {
    await this.client.post('/spotify/player/play');
  }

  async pause() {
    await this.client.post('/spotify/player/pause');
  }

  async next() {
    await this.client.post('/spotify/player/next');
  }

  async previous() {
    await this.client.post('/spotify/player/previous');
  }

  async getDevices() {
    const { data } = await this.client.get<{ devices: SpotifyPlayerDevice[] }>('/spotify/player/devices');
    return data.devices;
  }

  async transferPlayback(deviceId: string, play: boolean = false) {
    await this.client.post('/spotify/player/transfer', { device_id: deviceId, play });
  }

  // Playlists
  async getPlaylists() {
    const { data } = await this.client.get<{ items: Playlist[] }>('/spotify/playlists');
    return data.items;
  }

  async getPlaylist(id: string) {
    const { data } = await this.client.get<Playlist>(`/spotify/playlists/${id}`);
    return data;
  }

  async getPlaylistTracks(id: string) {
    const { data } = await this.client.get<{ items: Array<{ track: Track }> }>(`/spotify/playlists/${id}/tracks`);
    return data.items.map(item => item.track);
  }

  // Devices
  async getMyDevices() {
    const { data } = await this.client.get<SpotifyDevice[]>('/devices');
    return data;
  }

  async lockDevice(id: string) {
    await this.client.post(`/devices/${id}/lock`);
  }

  async unlockDevice(id: string) {
    await this.client.post(`/devices/${id}/unlock`);
  }

  async setPrimaryDevice(id: string) {
    await this.client.post(`/devices/${id}/set-primary`);
  }

  // Settings
  async getSettings() {
    const { data } = await this.client.get<Settings>('/settings');
    return data;
  }

  async updateSettings(settings: Partial<Settings>) {
    const { data } = await this.client.post<Settings>('/settings', settings);
    return data;
  }
}

export const api = new ApiClient();
