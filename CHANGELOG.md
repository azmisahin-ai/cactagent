# Değişiklik Günlüğü

Tüm önemli değişiklikler bu dosyada belgelenir.

Format: [Keep a Changelog](https://keepachangelog.com/)
Sürümleme: [Semantic Versioning](https://semver.org/)

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