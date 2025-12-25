# Spotify Manager Backend

Rust backend for Spotify remote control and device management system.

## Features

- 🔐 JWT-based authentication
- 🎵 Spotify OAuth2 integration
- 🎮 Complete playback control (play, pause, next, previous)
- 📱 Device tracking and locking
- 📋 Playlist management
- ⚙️ User settings and preferences
- 🔌 WebSocket for real-time updates
- 💾 SQLite database

## Prerequisites

- Rust 1.70+
- SQLite3
- Spotify Developer Account

## Setup

### 1. Clone and Navigate
```bash
cd spotify-manager/backend
```

### 2. Environment Configuration
```bash
cp .env.example .env
```

Edit `.env` and add your Spotify API credentials:
1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new app
3. Copy Client ID and Client Secret
4. Add redirect URI: `https://yourdomain/api/spotify/callback`

### 3. Database Setup
```bash
# Database will be created automatically on first run
# Migrations run automatically
```

### 4. Run Development Server
```bash
cargo run
```

Server will start on `http://localhost:3000`

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login
- `POST /api/auth/refresh` - Refresh access token

### Spotify OAuth
- `GET /api/spotify/auth` - Get Spotify authorization URL
- `GET /api/spotify/callback` - OAuth callback
- `GET /api/spotify/status` - Connection status
- `POST /api/spotify/disconnect` - Disconnect Spotify

### Playback Control
- `POST /api/spotify/player/play` - Play
- `POST /api/spotify/player/pause` - Pause
- `POST /api/spotify/player/next` - Next track
- `POST /api/spotify/player/previous` - Previous track
- `GET /api/spotify/player/devices` - List devices
- `POST /api/spotify/player/transfer` - Transfer playback

### Playlists
- `GET /api/spotify/playlists` - List playlists
- `GET /api/spotify/playlists/:id` - Get playlist
- `GET /api/spotify/playlists/:id/tracks` - Get playlist tracks

### Device Management
- `GET /api/devices` - List all devices
- `GET /api/devices/:id` - Get device info
- `POST /api/devices/:id/lock` - Lock device
- `POST /api/devices/:id/unlock` - Unlock device
- `POST /api/devices/:id/set-primary` - Set as primary device

### Settings
- `GET /api/settings` - Get user settings
- `POST /api/settings` - Update settings

### WebSocket
- `WS /ws` - WebSocket connection for real-time updates

## WebSocket Messages

### Client → Server
```json
{
  "type": "ping"
}
```

### Server → Client
```json
{
  "type": "device_status_update",
  "device_id": "device_123",
  "is_active": true,
  "is_locked": false
}
```

```json
{
  "type": "device_lost",
  "device_id": "device_123",
  "device_name": "My Phone"
}
```

```json
{
  "type": "system_paused",
  "reason": "Primary device lost"
}
```

## Database Schema

### users
- id, email, password_hash, created_at, updated_at

### spotify_connections
- id, user_id, access_token, refresh_token, expires_at, spotify_user_id

### devices
- id, user_id, spotify_device_id, device_name, device_type, is_locked, is_primary, last_seen

### settings
- id, user_id, auto_pause_on_device_lost, device_lost_timeout_seconds, enable_device_locking

## Development

### Run Tests
```bash
cargo test
```

### Check Code
```bash
cargo clippy
```

### Format Code
```bash
cargo fmt
```

### Build Release
```bash
cargo build --release
```

## Deployment

### Using systemd (Linux)
```bash
# Build release
cargo build --release

# Copy binary
sudo cp target/release/spotify-manager /usr/local/bin/

# Create systemd service
sudo nano /etc/systemd/system/spotify-manager.service
```

```ini
[Unit]
Description=Spotify Manager Backend
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/spotify-manager
Environment=DATABASE_URL=/var/www/spotify-manager/spotify_manager.db
EnvironmentFile=/var/www/spotify-manager/.env
ExecStart=/usr/local/bin/spotify-manager
Restart=always

[Install]
WantedBy=multi-user.target
```

```bash
# Start service
sudo systemctl daemon-reload
sudo systemctl enable spotify-manager
sudo systemctl start spotify-manager
```

### Using Docker
```bash
# Build image
docker build -t spotify-manager .

# Run container
docker run -d \
  -p 3000:3000 \
  -v $(pwd)/spotify_manager.db:/app/spotify_manager.db \
  --env-file .env \
  --name spotify-manager \
  spotify-manager
```

## HTTPS Configuration

For production, use a reverse proxy like Nginx:

```nginx
server {
    listen 443 ssl http2;
    server_name yourdomain;

    ssl_certificate /etc/letsencrypt/live/yourdomain/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/yourdomain/privkey.pem;

    location /api/ {
        proxy_pass http://localhost:3000/api/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    location /ws {
        proxy_pass http://localhost:3000/ws;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

## Security Notes

- Always use HTTPS in production
- Change JWT_SECRET to a strong random value
- Keep Spotify credentials secure
- Use environment variables, never commit .env files
- Implement rate limiting for production
- Enable CORS only for trusted domains

## Troubleshooting

### Database locked error
```bash
# Make sure only one instance is running
pkill spotify-manager
```

### Spotify API errors
- Check if tokens are expired
- Verify API credentials
- Ensure redirect URI matches exactly

## License

MIT
