export interface User {
  id: string;
  email: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  user_id: string;
  email: string;
}

export interface SpotifyDevice {
  id: string;
  spotify_device_id: string;
  device_name: string;
  device_type: string;
  is_locked: boolean;
  is_primary: boolean;
  last_seen: string;
}

export interface SpotifyPlayer Device {
  id: string;
  is_active: boolean;
  is_private_session: boolean;
  is_restricted: boolean;
  name: string;
  type: string;
  volume_percent?: number;
}

export interface Playlist {
  id: string;
  name: string;
  description?: string;
  images: SpotifyImage[];
  tracks: {
    total: number;
  };
  owner: {
    id: string;
    display_name?: string;
  };
}

export interface SpotifyImage {
  url: string;
  height?: number;
  width?: number;
}

export interface Track {
  id: string;
  name: string;
  artists: Array<{ id: string; name: string }>;
  album: {
    id: string;
    name: string;
    images: SpotifyImage[];
  };
  duration_ms: number;
}

export interface Settings {
  auto_pause_on_device_lost: boolean;
  device_lost_timeout_seconds: number;
  enable_device_locking: boolean;
}

export interface WsMessage {
  type: 'device_status_update' | 'playback_state_changed' | 'device_lost' | 'system_paused' | 'ping' | 'pong';
  device_id?: string;
  device_name?: string;
  is_active?: boolean;
  is_locked?: boolean;
  is_playing?: boolean;
  reason?: string;
}
