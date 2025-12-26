# 🎬 Video Studio - VPS-Optimized Video Editor & Automation Platform

Professional web-based video editing and automation platform optimized for VPS deployment (8GB RAM, 6-core CPU).

## ✨ Özellikler

### 🎥 Video Editor
- **Multi-track Timeline:** Sınırsız video, ses ve overlay track'leri
- **Drag & Drop:** Tüm elementleri sürükle-bırak ile yerleştir
- **Live Preview:** Gerçek zamanlı önizleme
- **Non-destructive Editing:** Orijinal dosyalar korunur
- **Keyframe Animation:** Gelişmiş animasyon sistemi

### 🎨 Overlay & Effects
- **Görsel Overlay:** PNG, WebP desteği ile transparan overlay
- **Text/Subtitle Editor:** Zengin metin düzenleme ve animasyonlar
- **Transitions:** Fade, wipe, dissolve ve özel geçişler
- **Filters:** Color grading, blur, sharpen, vignette
- **Masks:** Custom maskeleme desteği

### 🤖 Automation System
- **Template Engine:** Yeniden kullanılabilir video şablonları
- **Batch Processing:** Toplu video işleme
- **Workflow Builder:** Görsel automation akışları
- **Scheduled Rendering:** Zamanlanmış render işlemleri
- **API Integration:** Harici sistemlerle entegrasyon

### ☁️ Cloud Features
- **Cloud Storage:** Proje ve asset depolama
- **Collaborative Editing:** Takım çalışması desteği
- **Version Control:** Proje sürüm yönetimi
- **Cloud Rendering:** Sunucu-side rendering
- **Asset Library:** Paylaşılan asset kütüphanesi

### 🚀 Advanced Features
- **4K/8K Support:** Yüksek çözünürlük desteği
- **GPU Acceleration:** Hardware-accelerated rendering
- **Audio Mixing:** Multi-track ses düzenleme
- **Chroma Key:** Green screen desteği
- **Motion Tracking:** Otomatik nesne takibi
- **AI-powered Tools:** Otomatik altyazı, nesne tanıma

## 🏗️ Teknoloji Stack

### Backend
- **Framework:** Rust + Axum (ultra-hızlı, güvenli)
- **Video Processing:** FFmpeg + custom pipeline
- **Job Queue:** Redis + background workers
- **Database:** PostgreSQL (projects, users, assets)
- **Storage:** S3-compatible (MinIO/AWS S3)
- **Real-time:** WebSocket + Server-Sent Events
- **Cache:** Redis

### Frontend
- **Framework:** React 18 + TypeScript
- **UI Library:** Custom video editor components
- **State:** Zustand + React Query
- **Canvas:** Fabric.js / Konva.js
- **Video:** Video.js / custom player
- **Timeline:** Custom timeline engine
- **Drag & Drop:** dnd-kit
- **Styling:** Tailwind CSS + CSS Modules
- **Build:** Vite

### Infrastructure
- **Container:** Docker + Docker Compose
- **Orchestration:** Kubernetes ready
- **Monitoring:** Prometheus + Grafana
- **Logging:** ELK stack
- **CI/CD:** GitHub Actions

## 📋 Proje Yapısı

```
video-studio/
├── backend/           # Rust backend
│   ├── src/
│   │   ├── api/       # REST API endpoints
│   │   ├── video/     # Video processing
│   │   ├── jobs/      # Background job queue
│   │   ├── storage/   # File storage
│   │   ├── templates/ # Template engine
│   │   └── automation/# Automation rules
│   ├── migrations/    # Database migrations
│   └── Cargo.toml
│
├── frontend/          # React frontend
│   ├── src/
│   │   ├── components/
│   │   │   ├── editor/    # Video editor components
│   │   │   ├── timeline/  # Timeline editor
│   │   │   ├── preview/   # Preview player
│   │   │   ├── assets/    # Asset manager
│   │   │   └── automation/# Automation builder
│   │   ├── pages/
│   │   ├── services/
│   │   └── store/
│   └── package.json
│
├── docker-compose.yml # Development stack
└── README.md
```

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose 20+
- 8GB RAM minimum (6GB for Whisper service + 2GB for other services)
- 6-core CPU (E5 or equivalent)
- 50GB storage minimum

### Production Deployment (Docker)

```bash
# 1. Clone repository
git clone <repo>
cd video-studio

# 2. Configure environment
cp .env.example .env
# Edit .env with your settings (JWT_SECRET, etc.)

# 3. Deploy with Docker Compose
docker-compose up -d

# 4. Check service health
docker-compose ps
docker-compose logs -f backend
```

**Services:**
- Frontend: http://localhost (nginx)
- Backend API: http://localhost:3000
- Whisper Service: http://localhost:8001
- PostgreSQL: localhost:5432
- Redis: localhost:6379

### Development Setup

```bash
# Backend (Rust)
cd backend
cp .env.example .env
cargo build
cargo run

# Frontend (React)
cd frontend
npm install
npm run dev
# Access: http://localhost:5173

# Whisper Service (Python)
cd whisper-service
pip install -r requirements.txt
python app.py
```

## 🎯 Kullanım Senaryoları

### 1. **Sosyal Medya İçerik Üretimi**
- Template seç
- Videoları upload et
- Overlay ve text ekle
- Toplu render et
- Otomatik export

### 2. **E-ticaret Ürün Videoları**
- Ürün template'i oluştur
- CSV ile toplu ürün bilgisi import
- Otomatik video oluştur
- Tüm ürünler için batch render

### 3. **Eğitim Videoları**
- Multi-camera setup
- Screen recording entegrasyonu
- Otomatik subtitle generation
- Chapter markers
- Quiz ekleme

### 4. **Marketing Campaigns**
- Brand template'leri
- A/B testing için varyasyonlar
- Otomatik watermark ekleme
- Multi-format export
- Social media optimizasyonu

## 🔥 Gelişmiş Özellikler

### Timeline Editor
- **Magnetic Timeline:** Clip'ler otomatik hizalanır
- **Ripple Edit:** Değişiklikler timeline'ı iter
- **Multi-selection:** Çoklu clip düzenleme
- **Nested Sequences:** Sequence içinde sequence
- **Audio Waveforms:** Görsel audio dalga formu

### Overlay System
- **Smart Positioning:** Grid ve kılavuz çizgiler
- **Animation Presets:** Hazır animasyon şablonları
- **Custom Paths:** Bezier curve animasyonlar
- **Layer Groups:** Overlay gruplama
- **Blend Modes:** Photoshop-style karışım modları

### Automation Builder
```javascript
// Örnek automation rule
{
  trigger: "file_upload",
  actions: [
    { type: "apply_template", template_id: "product-video" },
    { type: "add_overlay", layer: "logo", position: "top-right" },
    { type: "add_subtitle", source: "auto-generate" },
    { type: "render", format: "1080p", codec: "h264" },
    { type: "export", destination: "s3://bucket/videos/" }
  ]
}
```

### Template System
- **Parametric Templates:** Değişken-based şablonlar
- **Template Marketplace:** Paylaşım ve satış
- **Version Control:** Template versiyonlama
- **Preview System:** Gerçek veri ile önizleme

## 📊 Performance

- **Upload:** Multi-part upload, resume support
- **Processing:** Distributed rendering
- **Streaming:** Adaptive bitrate (HLS/DASH)
- **Preview:** Low-res proxy videos
- **Export:** Hardware acceleration (NVENC, QuickSync)

## 🔐 Security

- JWT authentication
- Role-based access control (RBAC)
- Project-level permissions
- Encrypted file storage
- Rate limiting
- CORS protection

## 📈 Scalability

- Horizontal scaling (worker nodes)
- Queue-based processing
- CDN integration
- Database sharding ready
- Microservices architecture

## 🎯 VPS Optimization Features

### Resource-Efficient Design
- **Whisper Large-v3**: INT8 quantization (5GB RAM vs 10GB+)
- **Faster-Whisper**: C++ implementation, 4x faster than OpenAI Whisper
- **FFmpeg Processing**: Lightweight, CPU-only video processing
- **No Heavy ML Models**: Removed upscaling, interpolation, ML background removal
- **Chroma Key Alternative**: FFmpeg-based green screen (fast, low memory)

### Implemented Components

#### ✅ Backend (Rust/Axum)
- JWT authentication with Argon2 password hashing
- PostgreSQL database with full schema (users, projects, assets, render_jobs)
- RESTful API endpoints:
  - `/api/auth/*` - Login, register
  - `/api/projects/*` - CRUD operations
  - `/api/assets/*` - File upload with multipart, metadata extraction
  - `/api/compositions/*` - Timeline JSON storage
  - `/api/render/*` - Background job queue integration
  - `/api/templates/*` - Template management
  - `/api/ai/overlay/auto-distribute` - Auto-distribute overlay images
  - `/api/ai/subtitle/align` - Text-to-audio forced alignment (Aeneas)
  - `/api/ai/subtitle/transcribe` - Auto-transcribe with Whisper
  - `/ws` - WebSocket for real-time updates
- Background job queue with tokio mpsc channels
- Local filesystem storage backend (easily extensible to S3)
- FFmpeg integration for:
  - Video metadata extraction
  - Proxy generation (720p for editing)
  - Thumbnail generation
  - Beat detection (audio analysis)
  - Chroma key (green screen)

#### ✅ AI Features (VPS-Optimized)
- **Scene Detection**: FFmpeg-based shot detection
- **Auto-Reframe**: Smart crop for different aspect ratios
- **Color Grading**: ACES color management, LUT support
- **Beat Detection**: FFmpeg astats filter, adaptive threshold
- **Subtitle Generation**: Whisper microservice with fallback
- **Chroma Key**: Lightweight green/blue screen removal
- **AI Agent**: OpenAI GPT-4 & Gemini integration with 13 AI tools

#### ✅ Frontend (React/TypeScript)
- Project management UI
- Login/Register pages
- Asset upload with drag-drop
- Store management (Zustand)
- API client with axios
- Responsive design (Tailwind CSS)

#### ✅ Whisper Service (Python/FastAPI)
- Faster-whisper library
- INT8 quantization for RAM efficiency
- VAD filter to skip silent parts
- Microservice architecture (isolated resource usage)
- 6GB RAM limit with Docker resource constraints

### AI Tools Available
1. Auto-subtitle generation (Whisper Large-v3)
2. Scene detection & smart cuts
3. Auto-reframe for social media
4. Beat detection for music sync
5. Chroma key (green screen)
6. Color matching
7. ACES color grading
8. LUT application
9. Object tracking
10. Audio enhancement (FFmpeg filters)
11. Overlay positioning
12. Text generation
13. AI agent with function calling

## 🎓 Documentation

- **User Guide:** Web arayüzü kullanımı
- **API Reference:** REST API dokümantasyonu
- **Developer Guide:** Katkıda bulunma
- **Template Guide:** Şablon oluşturma
- **Automation Guide:** Workflow builder

## 🛠️ Development Roadmap

### Phase 1: MVP (Current)
- ✅ Basic video upload
- ✅ Timeline editor
- ✅ Overlay placement
- ✅ Subtitle support
- ✅ Basic rendering

### Phase 2: Advanced Editor
- 🔄 Transitions & effects
- 🔄 Audio mixing
- 🔄 Chroma key
- 🔄 Template system
- 🔄 Batch processing

### Phase 3: Automation
- ⏳ Workflow builder
- ⏳ API automation
- ⏳ Scheduled jobs
- ⏳ Webhooks

### Phase 4: Collaboration
- ⏳ Real-time co-editing
- ⏳ Comments & annotations
- ⏳ Version control
- ⏳ Team management

### Phase 5: AI Integration
- ⏳ Auto-subtitle generation
- ⏳ Object detection
- ⏳ Scene detection
- ⏳ Smart cropping
- ⏳ Voice cloning

## 💡 Future Ideas

- Mobile app (React Native)
- Desktop app (Tauri)
- Plugin system
- Marketplace for effects
- Live streaming integration
- Video analytics
- A/B testing tools

## 📄 License

MIT License - Production-ready for commercial use

## 🤝 Contributing

Katkıda bulunmak için pull request gönderin!

## 📧 Support

Issues için GitHub Issues kullanın.

---

**Professional video editing, simplified. 🎬✨**
