# Değişiklik Günlüğü

Tüm önemli değişiklikler bu dosyada belgelenir.

Format: [Keep a Changelog](https://keepachangelog.com/)
Sürümleme: [Semantic Versioning](https://semver.org/)



**CHANGELOG.md**'ye v0.9.0 notu ekle:

## [0.9.0] — 2026-09-18

### Eklendi
- Çoklu dil desteği (`src/i18n.rs`)
- Otomatik dil algılama (karakter + kelime analizi)
- `--lang` flag'i (`tr`, `en`, `de`, `fr`)
- DuckDuckGo bölge parametresi (`kl`)
- Hata mesajları seçilen dile göre
- 9 yeni test (toplam 55 test)

### Diller
- Türkçe (`tr`)
- İngilizce (`en`) — varsayılan
- Almanca (`de`)
- Fransızca (`fr`)

### Neden?
- Türkçe kullanıcılar için daha iyi arama sonuçları
- DuckDuckGo bölge parametresi ile daha alakalı sonuçlar
- Tamamen içsel, dış bağımlılık yok

## [0.8.0] — 2026-09-18

### Rate Limiting

Her araç, dakikada maksimum çağrı sayısı ile sınırlıdır:

| Araç | Limit |
|------|-------|
| `web_search` | Dakikada 10 |
| `read_url` | Dakikada 20 |
| `write_file` | Dakikada 5 |
| `read_file` | Dakikada 60 |
| `list_dir` | Dakikada 60 |

Bu, DuckDuckGo gibi servislerin bizi engellemesini önler ve ajanın kontrolsüz döngüye girmesini engeller.

## [0.7.0] — 2026-09-17

### Eklendi
- `write_file` için kullanıcı onayı (`--auto-approve` ile atlanabilir)
- Audit log (`workspace/logs/audit.log`) — tüm araç çağrıları kaydedilir
- `--help` ve `--version` flag'leri
- Hata mesajları iyileştirildi (kullanıcıya çözüm önerileri)
- Pre-commit kontrol script'i (`scripts/pre-commit.ps1`)

### Değişti
- `sandbox.rs`'e onay modülü eklendi (`AUTO_APPROVE` atomic flag)
- `main.rs`'de flag işleme basitleştirildi
- README'ye güvenlik ve audit log bölümleri eklendi

### Güvenlik
- `write_file` artık varsayılan olarak onay ister
- Path traversal, absolute path, Windows drive letter korumaları
- Sandbox dışına çıkma girişimleri engellenir

### Test
- 39 test (3 yeni: onay modülü, audit log, leap year)

### Bilinen Sorunlar
- **DuckDuckGo rate limit (HTTP 202):** Yoğun kullanımda geçici olarak
  çalışmaz. Kod tarafında retry + 5 sn bekleme var, ama yoğun kullanımda
  birkaç dakika beklemek gerekebilir.
- **Needle v2 (26M) sınırı:** Belirsiz ifadelerde olmayan araçları
  uydurabiliyor (örneğin "Rust news" → "create_newsletter_item").
  Bu, modelin parametre sınırından kaynaklanıyor, yazılım hatası değil.
  Çözüm: Net ifadeler kullanmak.

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

## [0.3.0] — 2026-09-17

### Eklendi
- GitHub Actions CI workflow (`.github/workflows/ci.yml`)
- `cargo fmt`, `cargo clippy`, `cargo test` otomatik kontrolleri
- 21 test (needle parse, URL cleaning, text processing)
- `examples/` klasörü (basic_search, read_url, file_ops, custom_tool)

### Değişti
- Test kapsamı genişletildi
- README'ye CI rozeti eklendi

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
