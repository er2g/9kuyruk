# 🎬 Video Birleştirici - Tauri Edition

Modern, hızlı ve kullanıcı dostu video overlay + altyazı birleştirici uygulaması.

## ✨ Özellikler

### 🎯 Temel Özellikler
- **Video Overlay**: Görselleri videoya belirtilen zaman aralıklarında otomatik ekler
- **Altyazı Desteği**: SRT formatındaki altyazıları ASS'e çevirerek videoya gömer
- **Canlı Önizleme**: Render etmeden önce sonucun nasıl görüneceğini görün
- **Otomatik Hesaplama**: Görsel sayısına göre zaman aralıklarını otomatik oluşturur
- **Progress Tracking**: İşlem sırasında gerçek zamanlı ilerleme takibi

### 🚀 Ek Özellikler
- Modern ve karanlık tema arayüz
- Drag & drop dosya seçimi (yakında)
- Video bilgilerini otomatik algılama (çözünürlük, FPS, süre)
- Manuel ve otomatik zaman aralığı düzenleme
- Hızlı zaman atlama butonları
- Responsive tasarım

## 📋 Gereksinimler

### Sistem Gereksinimleri
- **FFmpeg**: Video işleme için gerekli
- **FFprobe**: Video analizi için gerekli
- **Node.js**: v18 veya üzeri
- **Rust**: v1.70 veya üzeri

### FFmpeg Kurulumu

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
- [FFmpeg indirme sayfası](https://ffmpeg.org/download.html) üzerinden binary'leri indirin
- PATH ortam değişkenine ekleyin

## 🛠️ Kurulum

### 1. Bağımlılıkları Yükle
```bash
npm install
```

### 2. Geliştirme Modunda Çalıştır
```bash
npm run tauri dev
```

### 3. Production Build
```bash
npm run tauri build
```

Build edilen uygulama `src-tauri/target/release` klasöründe olacaktır.

## 📖 Kullanım

### 1. Dosyaları Seçin
- **Şablon Video**: Ana video dosyanızı seçin (.mp4, .mov, vb.)
- **Görseller Klasörü**: Numaralandırılmış görsellerin bulunduğu klasörü seçin (1.png, 2.jpg, ...)
- **Altyazı**: SRT formatında altyazı dosyanızı seçin

### 2. Zaman Aralıklarını Ayarlayın
- **Otomatik Doldur**: Görselleri eşit aralıklara otomatik dağıtır
- **Manuel Giriş**: Her görsel için başlangıç-bitiş zamanlarını kendiniz girin

### 3. Önizleme Yapın
- Sağ paneldeki timeline'dan istediğiniz anı seçin
- "Önizleme Oluştur" butonuna basın
- Overlay ve altyazının nasıl görüneceğini kontrol edin

### 4. Üretin
- Her şey hazır olduğunda "Üret" butonuna basın
- İşlem tamamlandığında `_final.mp4` dosyası oluşturulacaktır

## 🎨 Görsel Formatları

Desteklenen görsel formatları:
- PNG (`.png`)
- JPEG (`.jpg`, `.jpeg`)
- WebP (`.webp`)

**Önemli**: Görseller numaralandırılmış olmalıdır: `1.png`, `2.jpg`, `3.png`, ...

## 📝 Altyazı Formatı

SRT formatı örneği:
```
1
00:00:00,000 --> 00:00:05,000
İlk altyazı metni

2
00:00:05,000 --> 00:00:10,000
İkinci altyazı metni
```

Uygulama bu formatı otomatik olarak ASS formatına çevirecektir.

## 🏗️ Proje Yapısı

```
video-birlestirici/
├── src/                    # Frontend (React + TypeScript)
│   ├── components/         # React bileşenleri
│   ├── types.ts           # TypeScript tipleri
│   ├── App.tsx            # Ana uygulama
│   └── main.tsx           # Entry point
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Ana Rust dosyası
│   │   ├── video.rs       # Video işleme
│   │   ├── srt.rs         # Altyazı işleme
│   │   └── ffmpeg.rs      # FFmpeg entegrasyonu
│   └── Cargo.toml         # Rust bağımlılıkları
└── package.json           # Node bağımlılıkları
```

## 🐛 Bilinen Sorunlar

- Preview oluşturma Linux'ta bazı fontlar için ek paket gerektirebilir
- Çok büyük videolar için render süresi uzun olabilir
- Windows'ta path'lerde Türkçe karakter sorun çıkarabilir

## 🤝 Katkıda Bulunma

1. Fork edin
2. Feature branch oluşturun (`git checkout -b feature/amazing-feature`)
3. Değişikliklerinizi commit edin (`git commit -m 'Add some amazing feature'`)
4. Branch'inizi push edin (`git push origin feature/amazing-feature`)
5. Pull Request açın

## 📄 Lisans

Bu proje MIT lisansı altında lisanslanmıştır.

## 🙏 Teşekkürler

- [Tauri](https://tauri.app/) - Desktop uygulama framework'ü
- [FFmpeg](https://ffmpeg.org/) - Video işleme
- [React](https://react.dev/) - UI framework
- [Tailwind CSS](https://tailwindcss.com/) - CSS framework

## 📞 İletişim

Sorularınız veya önerileriniz için issue açabilirsiniz.

---

**Not**: Bu uygulama Python versiyonundan Tauri'ye dönüştürülmüş ve canlı önizleme özelliği eklenmiştir.
