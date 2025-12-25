# 🚀 Kurulum ve Çalıştırma Rehberi

## Sistem Gereksinimleri

### 1. FFmpeg ve FFprobe Kurulumu

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install ffmpeg
```

**Fedora/RHEL:**
```bash
sudo dnf install ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
1. [FFmpeg indirme sayfası](https://ffmpeg.org/download.html) üzerinden indirin
2. Binary'leri bir klasöre çıkartın
3. PATH'e ekleyin: Sistem Özellikleri → Gelişmiş → Ortam Değişkenleri → Path

### 2. Node.js ve Rust Kurulumu

**Node.js (v18+):**
- [nodejs.org](https://nodejs.org/) üzerinden indirin

**Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Tauri Sistem Bağımlılıkları

**Ubuntu/Debian:**
```bash
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

**Arch Linux:**
```bash
sudo pacman -Syu
sudo pacman -S --needed \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips
```

**macOS:**
```bash
# XCode Command Line Tools gerekli
xcode-select --install
```

**Windows:**
- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) kurulumu gerekli
- WebView2 otomatik kurulur

## Proje Kurulumu

### 1. Bağımlılıkları Yükleyin
```bash
npm install
```

### 2. Geliştirme Modunda Çalıştırın
```bash
npm run tauri dev
```

Bu komut:
- Frontend'i derler (Vite)
- Rust backend'i derler
- Uygulama penceresini açar
- Hot-reload ile değişiklikleri izler

### 3. Production Build
```bash
npm run tauri build
```

Build çıktıları:
- **Linux**: `src-tauri/target/release/bundle/deb/` - .deb paketi
- **macOS**: `src-tauri/target/release/bundle/dmg/` - .dmg dosyası
- **Windows**: `src-tauri/target/release/bundle/msi/` - .msi installer

## Sık Karşılaşılan Sorunlar

### Problem: "pkg-config not found"
**Çözüm (Linux):**
```bash
sudo apt install pkg-config
```

### Problem: "webkit2gtk not found"
**Çözüm (Ubuntu/Debian):**
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

### Problem: "FFmpeg bulunamadı"
**Çözüm:**
Terminal'de `ffmpeg -version` komutunu çalıştırın:
- Çıktı yoksa FFmpeg kurulumu yapın
- Varsa ama uygulama bulamıyorsa PATH'e eklemeyi kontrol edin

### Problem: Windows'ta Türkçe karakter sorunları
**Çözüm:**
- Dosya yollarında Türkçe karakter kullanmaktan kaçının
- Veya dosya yollarını UTF-8 olarak encode edin

### Problem: Preview oluşturulamıyor
**Çözüm (Linux):**
Fontlar eksik olabilir:
```bash
sudo apt install fontconfig fonts-dejavu-core
```

## Geliştirme İpuçları

### Frontend Geliştirme
```bash
# Sadece frontend'i çalıştırmak için
npm run dev
```

### Backend (Rust) Geliştirme
```bash
cd src-tauri
cargo check  # Syntax kontrolü (hızlı)
cargo build  # Derleme
cargo test   # Testleri çalıştır
```

### Debug Modu
Tauri dev modunda açılan pencerede:
- **macOS/Linux**: `Ctrl+Shift+I` veya `F12`
- **Windows**: `F12`

Developer Console açılır ve React DevTools kullanılabilir.

## Performans Optimizasyonu

### Build Boyutunu Küçültme
```bash
# Strip debug symbols
npm run tauri build -- --bundles deb
```

### Render Hızını Artırma
1. FFmpeg preset ayarı: `src-tauri/src/ffmpeg.rs` dosyasında:
   ```rust
   "-preset".to_string(), "ultrafast".to_string(),  // Hızlı ama büyük dosya
   // veya
   "-preset".to_string(), "slow".to_string(),       // Yavaş ama küçük dosya
   ```

2. CRF değeri (kalite): Daha yüksek = daha küçük dosya
   ```rust
   "-crf".to_string(), "23".to_string(),  // Default (iyi kalite)
   "-crf".to_string(), "28".to_string(),  // Daha düşük kalite, küçük dosya
   ```

## Test Verisi Hazırlama

### Örnek Görseller
```bash
mkdir test_images
# Görselleri numaralandırarak kopyalayın
cp image1.png test_images/1.png
cp image2.jpg test_images/2.jpg
cp image3.png test_images/3.png
```

### Örnek SRT Dosyası
`test.srt` oluşturun:
```
1
00:00:00,000 --> 00:00:05,000
İlk altyazı metni burada

2
00:00:05,000 --> 00:00:10,000
İkinci altyazı metni burada

3
00:00:10,000 --> 00:00:15,000
Üçüncü altyazı metni
```

## Yardım ve Destek

- **Sorun Bildirme**: GitHub Issues kullanın
- **Dokümantasyon**: README.md dosyasına bakın
- **Tauri Dokümantasyonu**: https://tauri.app/v1/guides/
- **FFmpeg Dokümantasyonu**: https://ffmpeg.org/documentation.html

## Güncelleme

### Bağımlılıkları Güncelleme
```bash
# NPM paketleri
npm update

# Cargo crates
cd src-tauri
cargo update
```

### Tauri Versiyonu Güncelleme
```bash
# NPM'deki Tauri CLI
npm install @tauri-apps/cli@latest -D

# Cargo.toml'deki bağımlılıklar
cd src-tauri
cargo upgrade  # cargo-edit gerektirir
```

## Lisans ve Katkı

Bu proje MIT lisansı altındadır. Katkıda bulunmak için:
1. Fork edin
2. Feature branch oluşturun
3. Değişikliklerinizi commit edin
4. Pull request gönderin

---

**İyi çalışmalar! 🎬**
