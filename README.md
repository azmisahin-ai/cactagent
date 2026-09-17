# CactAgent 🦀🌵

[![CI](https://github.com/azmisahin-ai/cactagent/actions/workflows/ci.yml/badge.svg)](https://github.com/azmisahin-ai/cactagent/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/azmisahin-ai/cactagent/releases)

**Tamamen yerel, tek binary'lik AI ajanı.** Sunucu yok, API anahtarı yok, bulut yok. Sadece Rust.

CactAgent, 13 MB'lık bir dil modeli (Needle v2) ve bir dizi aracı tek bir Rust binary'sinde birleştirir. İnternete çıkıp arama yapabilir, sayfaları okuyabilir ve size ham, temiz metin sunar. Hepsi CPU'da, hepsi yerel.

## ✨ Özellikler

- 🏠 **Tamamen yerel** — Hiçbir veri cihazından çıkmaz
- 📦 **Tek binary** — Ayrı sunucu, Docker, Python bağımlılığı yok
- 🚀 **Hafif** — Sadece ~10 MB binary, ~50 MB RAM
- ⚡ **Hızlı** — CPU'da milisaniyeler içinde araç seçimi
- 🔧 **Genişletilebilir** — Yeni araçlar eklemek 10 satır kod
- 📱 **Mobil hazır** — E2 Micro, Raspberry Pi ve mobil cihazlarda çalışır
- 🆓 **Ücretsiz** — API anahtarı gerekmez

## 🚀 Hızlı Başlangıç

### Gereksinimler

- Rust 1.70+
- İnternet bağlantısı (ilk çalıştırmada modeli indirmek için)

### Kurulum

```bash
git clone https://github.com/azmisahin-ai/cactagent
cd cactagent
cargo build --release
```

### Çalıştırma

```bash
cargo run --release -- "Rust programlama dili hakkında son haberleri araştır"
```

İlk çalıştırmada Needle modeli otomatik indirilecek (~13 MB). Sonraki çalıştırmalarda hazır olacak.

## 🏗️ Mimari

CactAgent, tek bir küçük model (Needle v2, 13 MB) ve araçlar üzerine kuruludur:

```
┌─────────────────────────────────────────────────┐
│              CactAgent (Rust binary)            │
│                                                 │
│  ┌─────────────┐                                │
│  │ Needle v2   │  Kullanıcı niyetini anlar      │
│  │ (13 MB)     │  ve doğru aracı seçer          │
│  └─────────────┘                                │
│                                                 │
│  ┌──────────────────────────────────────────┐  │
│  │ Tools: web_search, read_url              │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

**Veri akışı:**
1. Kullanıcı sorusu → Needle → araç çağrısı
2. `web_search` → DuckDuckGo → sonuçlar
3. `read_url` → readability → temiz metin
4. Kullanıcıya ham, doğru metin sunulur (uydurma yok)

Detaylı mimari için: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

## 🛠️ Araçlar

| Araç | Açıklama | Durum |
|------|----------|-------|
| `web_search` | DuckDuckGo üzerinden gerçek arama | ✅ |
| `read_url` | Sayfa içeriğini oku ve readability ile temizle | ✅ |
| `read_file` | Sandbox içindeki dosyayı oku (max 1 MB) | ✅ |
| `write_file` | Sandbox içindeki dosyaya yaz (max 1 MB) | ✅ |
| `list_dir` | Sandbox içindeki dizini listele | ✅ |
| `run_command` | Terminal komutu (whitelist ile, planlı) | 🚧 |

## 🗺️ Yol Haritası

- [x] Needle v2 ile araç seçimi
- [x] DuckDuckGo ile gerçek arama
- [x] URL okuma ve readability ile temizleme
- [x] Otomatik model indirme
- [x] Retry mekanizması (bağlantı hatalarına karşı)
- [ ] CI + test altyapısı
- [ ] Dosya sistemi araçları (sandbox'lı)
- [ ] Çok adımlı görev döngüsü
- [ ] E2 Micro / mobil optimizasyonu
- [ ] Opsiyonel: özet modeli (feature flag arkasında)
- [ ] Kendi kendine geliştirme modu

Detaylı yol haritası için: [docs/ROADMAP.md](docs/ROADMAP.md)

## 📖 Dokümantasyon

- [Mimari](docs/ARCHITECTURE.md)
- [Araç Sistemi](docs/TOOLS.md)
- [Yol Haritası](docs/ROADMAP.md)
- [Katkı Rehberi](CONTRIBUTING.md)
- [Değişiklik Günlüğü](CHANGELOG.md)

## 🤝 Katkı

Katkılar memnuniyetle karşılanır! Lütfen [CONTRIBUTING.md](CONTRIBUTING.md) dosyasını okuyun.

## 📄 Lisans

[LICENSE](LICENSE) dosyasına bakın.

## 🙏 Teşekkürler

- [Needle](https://huggingface.co/Cactus-Compute/needle2) — Araç seçimi modeli
- [Candle](https://github.com/huggingface/candle) — Rust ML framework
- [readability](https://crates.io/crates/readability) — Metin çıkarımı
- [scraper](https://crates.io/crates/scraper) — HTML ayrıştırma
