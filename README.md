# CactAgent 🦀🌵

[![CI](https://github.com/azmisahin-ai/cactagent/actions/workflows/ci.yml/badge.svg)](https://github.com/azmisahin-ai/cactagent/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.9.0-blue.svg)](https://github.com/azmisahin-ai/cactagent/releases)

**Tamamen yerel, tek binary'lik AI ajanı.** Sunucu yok, API anahtarı yok, bulut yok. Sadece Rust.

CactAgent, 13 MB'lık bir dil modeli (Needle v2) ve bir dizi aracı tek bir Rust binary'sinde birleştirir. İnternete çıkıp arama yapabilir, sayfaları okuyabilir ve size ham, temiz metin sunar. Hepsi CPU'da, hepsi yerel.

## ✨ Özellikler

- 🏠 **Tamamen yerel** — Hiçbir veri cihazından çıkmaz
- 📦 **Tek binary** — Ayrı sunucu, Docker, Python bağımlılığı yok
- 🚀 **Hafif** — Sadece ~7 MB binary, ~47 MB RAM
- ⚡ **Hızlı** — CPU'da milisaniyeler içinde araç seçimi
- 🔒 **Güvenli** — Sandbox'lı dosya işlemleri, kullanıcı onayı, audit log
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

## 🎯 Kullanım

### Yardım

```bash
cactagent --help
```

### Sürüm

```bash
cactagent --version
```

### Otomatik Onay (Script/CI için)

```bash
cactagent --auto-approve "Write 'test' to test.txt"
```

### Normal Kullanım

```bash
cactagent "Rust programlama dili hakkında son haberleri araştır"
cactagent "Write 'Merhaba dunya' to notes.txt"
cactagent "Read the file notes.txt"
```

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
│  │ Tools: web_search, read_url,              │  │
│  │        read_file, write_file, list_dir    │  │
│  └──────────────────────────────────────────┘  │
│                                                 │
│  ┌──────────────────────────────────────────┐  │
│  │ Security: sandbox, approval, audit log   │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

**Veri akışı:**
1. Kullanıcı sorusu → Needle → araç çağrısı
2. Araç çalıştırılır (web_search, read_url, dosya işlemleri)
3. Sonuç kullanıcıya sunulur (uydurma yok, ham veri)
4. Tüm araç çağrıları audit log'a yazılır

Detaylı mimari için: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

## 🛠️ Araçlar

| Araç | Açıklama | Durum |
|------|----------|-------|
| `web_search` | DuckDuckGo üzerinden gerçek arama | ✅ |
| `read_url` | Sayfa içeriğini oku ve readability ile temizle | ✅ |
| `read_file` | Sandbox içindeki dosyayı oku (max 1 MB) | ✅ |
| `write_file` | Sandbox içindeki dosyaya yaz (max 1 MB, onay ister) | ✅ |
| `list_dir` | Sandbox içindeki dizini listele | ✅ |
| `run_command` | Terminal komutu (whitelist ile, planlı) | 🚧 |

## 🔒 Güvenlik

### Dosya Yazma Onayı

`write_file` aracı, varsayılan olarak **kullanıcı onayı** ister:

```bash
cargo run --release -- "Write a note to notes.txt"
# Çıktı:
# === ONAY GEREKLI ===
# Islem: Dosya Yazma
# Detay: Dosya: notes.txt
# Icerik: ...
# Onaylıyor musunuz? (e/h):
```

Otomatik onay modu (script/CI için):

```bash
cargo run --release -- --auto-approve "Write a note to notes.txt"
```

### Sandbox

Tüm dosya işlemleri `./workspace/` dizini içinde sınırlıdır:
- Path traversal (`..`) reddedilir
- Absolute path reddedilir
- Windows drive letter reddedilir
- Maksimum dosya boyutu: 1 MB

### Audit Log

Tüm araç çağrıları `workspace/logs/audit.log` dosyasına kaydedilir:

```
[2026-09-17 21:36:26] web_search({"query":"Rust news"}) -> 5 sonuc
[2026-09-17 21:36:30] read_url({"url":"https://blog.rust-lang.org/"}) -> 1195 karakter
[2026-09-17 21:36:35] write_file({"path":"notes.txt","content":"test"}) -> 4 byte yazildi
```

Log dosyasını görmek için:

```bash
type workspace\logs\audit.log
```

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

### Çoklu Dil

CactAgent, kullanıcının dilini otomatik algılar ve arama sorgusunu ona göre ayarlar.

Desteklenen diller:
- `tr` — Türkçe
- `en` — İngilizce (varsayılan)
- `de` — Almanca
- `fr` — Fransızca

Manuel dil seçimi:

```bash
cactagent --lang en "Search Rust news"
cactagent --lang tr "Rust haberlerini araştır"
```

## 🗺️ Yol Haritası

- [x] Needle v2 ile araç seçimi
- [x] DuckDuckGo ile gerçek arama
- [x] URL okuma ve readability ile temizleme
- [x] Otomatik model indirme
- [x] Retry mekanizması (bağlantı hatalarına karşı)
- [x] CI + test altyapısı
- [x] Sandbox'lı dosya araçları
- [x] `write_file` onay mekanizması
- [x] Audit log
- [x] `--help`, `--version` flag'leri
- [x] **Deneysel:** ReAct çok adımlı döngü (`src/agent.rs`)
  - ⚠️ Needle v2'nin 26M parametre sınırı nedeniyle kararsız
  - Gelecekte daha büyük model ile aktif edilebilir
- [ ] Opsiyonel: özet modeli (feature flag arkasında)
- [ ] E2 Micro / mobil optimizasyonu
- [ ] Kendi kendine geliştirme modu

Detaylı yol haritası için: [docs/ROADMAP.md](docs/ROADMAP.md)

## 📚 Örnekler

`examples/` klasöründe çalıştırılabilir örnekler bulunur:

```bash
# Basit web araması
cargo run --example basic_search -- "Rust programming"

# URL okuma
cargo run --example read_url -- "https://blog.rust-lang.org/"

# Dosya işlemleri
cargo run --example file_ops

# Yeni araç ekleme rehberi
cargo run --example custom_tool
```

## 📊 Performans ve Kaynak Kullanımı

| Metrik | Değer |
|--------|-------|
| Binary boyutu | ~7 MB |
| Model boyutu | ~13 MB |
| Toplam disk | ~20 MB |
| Maksimum RAM | ~47 MB |
| CPU kullanımı | Milisaniyeler (araç seçimi) |

**E2 Micro (512 MB RAM) uyumluluğu:** ✅ Rahat çalışır (%9 bellek kullanımı)

### Ölçüm

Kendi sisteminizde ölçüm yapmak için:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\measure.ps1
```

### Pre-commit Kontrolleri

Commit öncesi format, lint ve test kontrolü:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\pre-commit.ps1
```

## 📖 Dokümantasyon

- [Mimari](docs/ARCHITECTURE.md)
- [Araç Sistemi](docs/TOOLS.md)
- [Yol Haritası](docs/ROADMAP.md)
- [Katkı Rehberi](CONTRIBUTING.md)
- [Değişiklik Günlüğü](CHANGELOG.md)

## 🤝 Katkı

Katkılar memnuniyetle karşılanır! Lütfen [CONTRIBUTING.md](CONTRIBUTING.md) dosyasını okuyun.

## 📄 Lisans

[LICENSE](LICENSE) dosyasına bakın. AGPL-3.0.

## 🙏 Teşekkürler

- [Needle](https://huggingface.co/Cactus-Compute/needle2) — Araç seçimi modeli
- [Candle](https://github.com/huggingface/candle) — Rust ML framework
- [readability](https://crates.io/crates/readability) — Metin çıkarımı
- [scraper](https://crates.io/crates/scraper) — HTML ayrıştırma
