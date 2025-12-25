# 🎵 Spotify Manager - Web Tabanlı Uzaktan Kontrol Sistemi

Modern, güvenli ve kullanıcı dostu Spotify uzaktan kontrol ve cihaz yönetim sistemi.

## ✨ Özellikler

### 🎮 Oynatıcı Kontrolü
- Play, Pause, Next, Previous kontrolleri
- Aktif cihazları görüntüleme
- Cihazlar arası oynatma transferi
- Gerçek zamanlı durum güncellemeleri

### 📱 Cihaz Yönetimi
- **Cihaz Kilitleme:** İstenmeyen cihazlarda oynatımı engelle
- **Birincil Cihaz:** Özel bir cihazı izle
- **Otomatik Duraklat:** Birincil cihaz kaybolduğunda müziği duraklat
- **Gerçek Zamanlı Takip:** Cihaz durumunu anlık olarak izle

### 📋 Playlist Yönetimi
- Playlistleri görüntüleme
- Playlist detayları ve şarkılar
- Görsel önizleme

### ⚙️ Akıllı Ayarlar
- Cihaz kaybında otomatik duraklat
- Ayarlanabilir zaman aşımı
- Cihaz kilitleme kontrolü

### 🔐 Güvenlik
- JWT-based authentication
- Spotify OAuth2 entegrasyonu
- HTTPS desteği
- Token auto-refresh

## 🏗️ Teknik Yapı

### Backend
- **Framework:** Axum (Rust)
- **Database:** SQLite
- **Authentication:** JWT + Argon2
- **API:** RESTful + WebSocket
- **Spotify API:** OAuth2 + Token refresh

### Frontend
- **Framework:** React 18 + TypeScript
- **Styling:** Tailwind CSS
- **State:** Zustand
- **HTTP Client:** Axios
- **Router:** React Router v6
- **Build:** Vite

## 📋 Gereksinimler

- **Node.js** 18+
- **Rust** 1.70+
- **SQLite3**
- **Spotify Developer Account**

## 🚀 Kurulum

### 1. Spotify API Kurulumu

1. [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)'a gidin
2. Yeni bir app oluşturun
3. Client ID ve Client Secret alın
4. Redirect URI ekleyin: `https://yourdomain.com/api/spotify/callback`

### 2. Backend Kurulumu

```bash
cd spotify-manager/backend

# Environment dosyasını oluştur
cp .env.example .env

# .env dosyasını düzenle ve Spotify kredilerini ekle
nano .env

# Çalıştır
cargo run
```

Backend `http://localhost:3000` üzerinde çalışacaktır.

### 3. Frontend Kurulumu

```bash
cd spotify-manager/frontend

# Bağımlılıkları yükle
npm install

# Geliştirme sunucusunu başlat
npm run dev
```

Frontend `http://localhost:5173` üzerinde çalışacaktır.

## 📖 Kullanım

### İlk Kurulum
1. `http://localhost:5173` adresine gidin
2. Hesap oluşturun
3. Spotify'a bağlanın
4. Cihazlarınızı yönetmeye başlayın

### Cihaz Kilitleme
1. "Cihazlar" bölümüne gidin
2. Kilitlemek istediğiniz cihazı seçin
3. "Kilitle" butonuna tıklayın
4. Artık o cihazda müzik oynatılamaz

### Birincil Cihaz Ayarlama
1. "Cihazlar" bölümüne gidin
2. İzlemek istediğiniz cihazı seçin
3. "Birincil Yap" butonuna tıklayın
4. "Ayarlar"dan otomatik duraklat özelliğini etkinleştirin
5. Birincil cihaz kaybolursa müzik otomatik durur

## 🌐 Production Deployment

### Backend (Systemd ile)

```bash
# Build release
cd backend
cargo build --release

# Servisi kur
sudo cp target/release/spotify-manager /usr/local/bin/
sudo cp spotify-manager.service /etc/systemd/system/

# Başlat
sudo systemctl enable spotify-manager
sudo systemctl start spotify-manager
```

### Frontend

```bash
cd frontend
npm run build

# dist klasörünü web sunucunuza kopyalayın (nginx, apache, vb.)
```

### Nginx Konfigürasyonu

```nginx
server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    # Frontend
    root /var/www/spotify-manager/frontend/dist;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api/ {
        proxy_pass http://localhost:3000/api/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # WebSocket
    location /ws {
        proxy_pass http://localhost:3000/ws;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

## 📚 API Dokümantasyonu

Detaylı API dokümantasyonu için: `backend/README.md`

## 🔧 Geliştirme

### Backend Test
```bash
cd backend
cargo test
cargo clippy
```

### Frontend Build
```bash
cd frontend
npm run build
```

## 🐛 Sorun Giderme

### "Spotify bağlantısı başarısız"
- Spotify API kredilerini kontrol edin
- Redirect URI'nin doğru olduğundan emin olun
- Browser console'da hata mesajlarını kontrol edin

### "Cihaz bulunamadı"
- Spotify uygulamasının en az bir cihazda açık olduğundan emin olun
- "Yenile" butonuna tıklayın
- Spotify hesabınızın premium olduğundan emin olun

### "WebSocket bağlantı hatası"
- Backend sunucusunun çalıştığından emin olun
- CORS ayarlarını kontrol edin
- SSL sertifikalarını kontrol edin

## 🤝 Katkıda Bulunma

1. Fork edin
2. Feature branch oluşturun (`git checkout -b feature/amazing-feature`)
3. Commit edin (`git commit -m 'Add amazing feature'`)
4. Push edin (`git push origin feature/amazing-feature`)
5. Pull Request açın

## 📄 Lisans

MIT

## 🙏 Teşekkürler

- [Spotify Web API](https://developer.spotify.com/documentation/web-api/)
- [Axum](https://github.com/tokio-rs/axum)
- [React](https://react.dev/)
- [Tailwind CSS](https://tailwindcss.com/)

## 📞 Destek

Sorularınız veya sorunlarınız için GitHub Issues kullanabilirsiniz.

---

**Made with ❤️ and 🎵**
