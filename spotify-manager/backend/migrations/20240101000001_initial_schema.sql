-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_users_email ON users(email);

-- Spotify connections table
CREATE TABLE IF NOT EXISTS spotify_connections (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    spotify_user_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_spotify_connections_user_id ON spotify_connections(user_id);

-- Devices table
CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    spotify_device_id TEXT UNIQUE NOT NULL,
    device_name TEXT NOT NULL,
    device_type TEXT NOT NULL,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT 0,
    last_seen TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_devices_user_id ON devices(user_id);
CREATE INDEX idx_devices_spotify_device_id ON devices(spotify_device_id);
CREATE INDEX idx_devices_is_primary ON devices(user_id, is_primary) WHERE is_primary = 1;

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT UNIQUE NOT NULL,
    auto_pause_on_device_lost BOOLEAN NOT NULL DEFAULT 1,
    device_lost_timeout_seconds INTEGER NOT NULL DEFAULT 30,
    enable_device_locking BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_settings_user_id ON settings(user_id);
