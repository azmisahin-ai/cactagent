# Değişiklik Günlüğü

Tüm önemli değişiklikler bu dosyada belgelenir.

Format: [Keep a Changelog](https://keepachangelog.com/)
Sürümleme: [Semantic Versioning](https://semver.org/)

## [0.5.0] — 2026-09-17

### Eklendi
- `src/agent.rs` — Deneysel ReAct döngüsü
- Çok adımlı görev desteği (5 iterasyona kadar)
- Bağlam yönetimi (son 3 sonuç)
- 5 yeni test (toplam 36 test)

### Bilinen Sorunlar
- **ReAct döngüsü kararsız:** Needle v2 (26M), çok adımlı bağlamda
  olmayan araçları uyduruyor ve saçmalıyor. Bu, modelin parametre
  sınırından kaynaklanıyor. Daha büyük bir model gerekli.
- Ana akış (`main.rs`) tek adımlı olarak kalmaya devam ediyor.
  `agent.rs` deneysel olarak kütüphanede duruyor.
  
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
