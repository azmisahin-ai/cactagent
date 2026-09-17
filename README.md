# CactAgent 🦀🌵

**Tamamen yerel, tek binary'lik AI ajanı.** Sunucu yok, API anahtarı yok, bulut yok. Sadece Rust.

CactAgent, iki küçük dil modelini tek bir Rust binary'sinde birleştirir:
- **Needle v2** (13 MB) — Araç seçimi ve görev planlama
- **Qwen2.5-0.5B** (~350 MB) — Metin özetleme ve yanıt üretme

İnternete çıkıp arama yapabilir, sayfaları okuyabilir ve öğrendiklerini özetleyebilir. Hepsi CPU'da, hepsi yerel.

## ✨ Özellikler

- 🏠 **Tamamen yerel** — Hiçbir veri cihazından çıkmaz
- 📦 **Tek binary** — Ayrı sunucu, Docker, Python bağımlılığı yok
- 🚀 **Hızlı** — CPU'da 10+ token/s, GPU ile 50+ token/s
- 🔧 **Genişletilebilir** — Yeni araçlar eklemek 10 satır kod
- 📱 **Mobil hazır** — Candle + Vulkan/Metal desteği
- 🆓 **Ücretsiz** — API anahtarı gerekmez

## 🚀 Hızlı Başlangıç

### Gereksinimler

- Rust 1.70+
- İnternet bağlantısı (ilk çalıştırmada modelleri indirmek için)

### Kurulum

```bash
git clone https://github.com/alen/cactagent
cd cactagent
cargo build --release
```

### Çalıştırma

```bash
cargo run --release -- "Rust programlama dili hakkında son haberleri araştır"
```

İlk çalıştırmada modeller otomatik indirilecek (~400 MB). Sonraki çalıştırmalarda hazır olacak.

## 🏗️ Mimari

```
┌─────────────────────────────────────────────────┐
│              CactAgent (Rust binary)            │
│                                                 │
│  ┌─────────────┐    ┌──────────────────────┐   │
│  │ Needle v2   │    │ Qwen2.5-0.5B         │   │
│  │ Araç seçimi │    │ Metin üretimi        │   │
│  └─────────────┘    └──────────────────────┘   │
│                                                 │
│  ┌──────────────────────────────────────────┐  │
│  │ Tools: web_search, read_url, ...          │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

Detaylı mimari için: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

## 🛠️ Araçlar

| Araç | Açıklama | Durum |
|------|----------|-------|
| `web_search` | DuckDuckGo üzerinden arama | ✅ |
| `read_url` | Sayfa içeriğini oku ve temizle | ✅ |
| `write_file` | Dosyaya yaz | 🚧 Planlı |
| `read_file` | Dosyadan oku | 🚧 Planlı |
| `run_command` | Terminal komutu çalıştır | 🚧 Planlı |

## 🗺️ Yol Haritası

- [x] Needle v2 ile araç seçimi
- [x] Qwen2.5-0.5B ile metin özetleme
- [x] DuckDuckGo arama
- [x] URL okuma ve temizleme
- [ ] Dosya sistemi araçları
- [ ] Çok adımlı görev döngüsü
- [ ] CUDA/Metal GPU desteği
- [ ] Mobil portu (Android/iOS)
- [ ] WebAssembly derlemesi
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
- [Qwen2.5](https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct) — Metin üretim modeli
- [Candle](https://github.com/huggingface/candle) — Rust ML framework
- [candelabra](https://crates.io/crates/candelabra) — Yüksek seviyeli çıkarım API'si
