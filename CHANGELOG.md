# Değişiklik Günlüğü

Tüm önemli değişiklikler bu dosyada belgelenir.

Format: [Keep a Changelog](https://keepachangelog.com/)
Sürümleme: [Semantic Versioning](https://semver.org/)


## [0.6.0] — 2026-09-17

### Eklendi
- `scripts/measure.ps1` — Binary boyutu ve bellek ölçüm script'i
- README'de "Performans ve Kaynak Kullanımı" bölümü
- `docs/ARCHITECTURE.md`'de E2 Micro uyumluluk tablosu

### Ölçüm Sonuçları
- Binary boyutu: 7.24 MB
- Maksimum RAM: 46.79 MB
- Toplam disk: ~20 MB
- **E2 Micro (512 MB) uyumluluğu: ✅**

### Notlar
- Windows'ta Linux ARM cross-compilation ertelendi
- Hedef cihazda native derleme öneriliyor

## [0.5.0] — 2026-09-17

### Eklendi
- `src/agent.rs` — Deneysel ReAct döngüsü (çok adımlı görevler)
- 5 yeni test (toplam 36 test)
- Rate limit (HTTP 202) algılama ve uzun bekleme

### Değişti
- `main.rs` tek adımlı akışa döndü (stabilite için)
- README'de ReAct "deneysel" olarak işaretlendi

### Bilinen Sorunlar
- **ReAct döngüsü kararsız:** Needle v2 (26M), çok adımlı bağlamda
  olmayan araçları uyduruyor. Bu, modelin parametre sınırından
  kaynaklanıyor, yazılım hatası değil. Daha büyük model gerekli.
- **DuckDuckGo rate limit:** Yoğun kullanımda 202 döndürüyor.
  Retry mekanizması ile kısmen çözüldü, tam çözüm için alternatif
  arama sağlayıcısı gerekli.
  
## [0.4.0] — 2026-09-17

### Eklendi
- `read_file` aracı (sandbox'lı, max 1 MB)
- `write_file` aracı (sandbox'lı, max 1 MB)
- `list_dir` aracı (sandbox'lı)
- `sandbox.rs` güvenlik modülü:
  - Path traversal (`..`) koruması
  - Absolute path koruması
  - Windows drive letter koruması
  - Workspace otomatik oluşturma
- 13 yeni test (toplam 35 test)
- CI/CD pipeline (GitHub Actions)

### Değişti
- README'ye CI, lisans, sürüm rozetleri eklendi
- `docs/TOOLS.md` genişletildi

## [0.2.0] — 2026-09-17

### Değişti
- **BREAKING:** Qwen2.5-0.5B bağımlılığı kaldırıldı
- Odak: sadece Needle v2 + araçlar
- Binary boyutu ~50 MB → ~10 MB
- RAM kullanımı ~500 MB → ~50 MB
- Derleme süresi ~2 dk → ~45 sn

### Kaldırıldı
- `candelabra` bağımlılığı
- `engine/qwen.rs` modülü
- Özetleme adımı (opsiyonel olarak geri gelebilir)

### Eklendi
- `readability` ile daha temiz metin çıkarımı
- Breadcrumb ve navigasyon temizleme
- `docs/ARCHITECTURE.md`'de tasarım kararları bölümü

### Neden?
Qwen2.5-0.5B, özetleme görevinde yeterince iyi değildi:
- Halüsinasyon yapıyordu ("düşüncelerinizle birlikte doğru yerdesiniz" gibi)
- Tekrar döngüsüne giriyordu (düşük temperature'da aynı cümleyi tekrarlıyordu)
- E2 Micro gibi kısıtlı cihazlarda bellek sorunu yaratıyordu
- Ürettiği özetler, ham metinden daha az bilgi içeriyordu

Odak, "en düşük donanımda en hızlı çalışan" temel ajana kaydırıldı.

## [0.1.0] — 2026-09-17

### Eklendi
- Needle v2 model entegrasyonu (araç seçimi)
- Qwen2.5-0.5B model entegrasyonu (metin özetleme)
- `web_search` aracı (DuckDuckGo)
- `read_url` aracı (HTML temizleme)
- Otomatik model indirme (Hugging Face)
- Retry mekanizması (3 deneme, 2 sn arayla)
- Modüler proje yapısı
- Kapsamlı dokümantasyon

### Bilinen Sorunlar
- DuckDuckGo bazen bağlantı sıfırlayabilir (retry ile çözülüyor)
- CPU-only çıkarım yavaş (GPU desteği planlı)
- Özet kalitesi model boyutuyla sınırlı
