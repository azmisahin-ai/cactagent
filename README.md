# CactAgent 🦀🌵

[![CI](https://github.com/azmisahin-ai/cactagent/actions/workflows/ci.yml/badge.svg)](https://github.com/azmisahin-ai/cactagent/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.5.0-blue.svg)](https://github.com/azmisahin-ai/cactagent/releases)

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
- [x] CI + test altyapısı
- [x] Sandbox'lı dosya araçları
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

Anladım, çok net. **Hiçbir dış bağımlılık (API key, üçüncü parti servis) eklemiyoruz.** Sistem tamamen kendi kendine yeten, bağımsız bir yapı olarak kalacak. Arama motoru konusunu **sonraya** bırakıyoruz; gerekirse kendi arama çözümümüzü değerlendiririz.

Bu durumda **A (Güvenlik)** ile devam ediyoruz — çünkü bu **tamamen içsel**, hiçbir dış bağımlılık gerektirmiyor.

### 🎯 v0.7.0 — Güvenlik İyileştirmeleri

**Hedef:** Kendi kendine çalışan ajanın **kontrolsüz** işlem yapmasını engellemek.

### 📋 Plan

| İyileştirme | Açıklama | Öncelik |
|-------------|----------|---------|
| **1. `write_file` onay mekanizması** | Yazmadan önce kullanıcıya sor | 🔴 Yüksek |
| **2. Audit log** | Hangi araç ne zaman çağrıldı, logla | 🟡 Orta |
| **3. Rate limiting** | Aynı aracın saniyede N kez çağrılmasını engelle | 🟢 Düşük |

### 📝 1. Adım: `write_file` Onay Mekanizması

İki mod olacak:
- **`--auto-approve`** flag'i varsa: onay sormadan yaz (CI/script kullanımı için)
- **Varsayılan:** Yazmadan önce kullanıcıya sor

### 📝 `src/tools/sandbox.rs` — Onay Modülü Ekle

Dosyanın en üstüne ekle:

```rust
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

/// Global onay modu (varsayılan: kapalı)
static AUTO_APPROVE: AtomicBool = AtomicBool::new(false);

/// Otomatik onay modunu ayarlar
pub fn set_auto_approve(enabled: bool) {
    AUTO_APPROVE.store(enabled, Ordering::SeqCst);
}

/// Otomatik onay modunda mı?
pub fn is_auto_approve() -> bool {
    AUTO_APPROVE.load(Ordering::SeqCst)
}

/// Kullanıcıdan onay ister.
/// Otomatik onay modundaysa sormaz, doğrudan true döner.
pub fn ask_approval(action: &str, details: &str) -> bool {
    if is_auto_approve() {
        println!("[AUTO-APPROVE] {}: {}", action, details);
        return true;
    }

    println!("\n=== ONAY GEREKLI ===");
    println!("Islem: {}", action);
    println!("Detay: {}", details);
    print!("\nOnayliyor musunuz? (e/h): ");
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    let trimmed = input.trim().to_lowercase();
    trimmed == "e" || trimmed == "evet" || trimmed == "y" || trimmed == "yes"
}
```

### 📝 `src/tools/file_ops.rs` — `write_file`'a Onay Ekle

`write_file` fonksiyonunu güncelle:

```rust
use super::sandbox::{ask_approval, safe_path, MAX_FILE_SIZE};

pub fn write_file(path: &str, content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = safe_path(path)?;

    // Boyut kontrolü
    if content.len() as u64 > MAX_FILE_SIZE {
        return Err(format!(
            "Icerik cok buyuk: {} byte (max {} byte)",
            content.len(),
            MAX_FILE_SIZE
        )
        .into());
    }

    // Onay iste
    let preview: String = content.chars().take(200).collect();
    let preview_display = if content.len() > 200 {
        format!("{}... (toplam {} byte)", preview, content.len())
    } else {
        preview
    };

    let details = format!("Dosya: {}\nIcerik: {}", path, preview_display);

    if !ask_approval("Dosya Yazma", &details) {
        return Err("Kullanici yazma islemini reddetti.".into());
    }

    println!("[DEBUG] Dosya yaziliyor: {:?}", full_path);

    // Parent dizinleri oluştur
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Yaz
    std::fs::write(&full_path, content)?;
    println!("[DEBUG] Yazilan boyut: {} byte", content.len());

    Ok(format!(
        "Dosya basariyla yazildi: {} ({} byte)",
        path,
        content.len()
    ))
}
```

### 📝 `src/main.rs` — `--auto-approve` Flag'i Ekle

```rust
use cactagent::engine::needle;
use cactagent::tools::sandbox;
use cactagent::tools::{file_ops, reader, search, TOOLS_JSON};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI argümanlarını al
    let args: Vec<String> = std::env::args().collect();

    // --auto-approve flag'ini kontrol et
    let auto_approve = args.iter().any(|a| a == "--auto-approve");
    if auto_approve {
        sandbox::set_auto_approve(true);
        println!("[!] Otomatik onay modu aktif. Dosya yazma onayi sorulmayacak.\n");
    }

    // Flag'leri temizle
    let filtered_args: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .collect();

    let user_task = if filtered_args.len() > 1 {
        filtered_args[1..].join(" ")
    } else {
        "Search the web for the latest news about Rust programming language".to_string()
    };

    println!("=== CactAgent v0.7.0 ===");
    println!("Gorev: {}\n", user_task);

    // Needle modelini yükle
    let needle_path = needle::ensure_model();
    let needle_engine = needle::load(&needle_path);

    // ADIM 1: Araç seçimi
    println!("=== ADIM 1: ARAC SECIMI ===");
    let result = needle_engine.run(&user_task, TOOLS_JSON);

    println!("Ham cikti:");
    println!("{}", result.text);
    println!("---");

    if let Some(think) = needle::extract_think(&result.text) {
        println!("[Dusunce] {}", think);
    }

    // Tool call'ları ayrıştır ve çalıştır
    if let Some(tool_calls) = needle::parse_tool_call(&result.text) {
        if tool_calls.is_empty() {
            println!("Model bos tool call dondurdu.");
            return Ok(());
        }

        for call in tool_calls {
            let name = call["name"].as_str().unwrap_or("bilinmeyen");
            let args = &call["arguments"];

            println!("\n=== ARAC: {} ===", name);
            println!("Argumanlar: {}\n", args);

            match name {
                "web_search" => {
                    if let Some(query) = args["query"].as_str() {
                        match search::web_search(query) {
                            Ok(r) => {
                                println!("=== ARAMA SONUCLARI ===");
                                println!("{}", r);
                            }
                            Err(e) => eprintln!("Arama hatasi: {}", e),
                        }
                    }
                }
                "read_url" => {
                    if let Some(url) = args["url"].as_str() {
                        match reader::read_url(url) {
                            Ok(content) => {
                                println!("=== SAYFA ICERIGI ===");
                                println!("{}", content);
                            }
                            Err(e) => eprintln!("Okuma hatasi: {}", e),
                        }
                    }
                }
                "read_file" => {
                    if let Some(path) = args["path"].as_str() {
                        match file_ops::read_file(path) {
                            Ok(content) => {
                                println!("=== DOSYA ICERIGI ===");
                                println!("{}", content);
                            }
                            Err(e) => eprintln!("Dosya okuma hatasi: {}", e),
                        }
                    }
                }
                "write_file" => {
                    if let (Some(path), Some(content)) =
                        (args["path"].as_str(), args["content"].as_str())
                    {
                        match file_ops::write_file(path, content) {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => eprintln!("Dosya yazma hatasi: {}", e),
                        }
                    }
                }
                "list_dir" => {
                    let path = args["path"].as_str().unwrap_or(".");
                    match file_ops::list_dir(path) {
                        Ok(listing) => {
                            println!("=== DIZIN ICERIGI ===");
                            println!("{}", listing);
                        }
                        Err(e) => eprintln!("Dizin listeleme hatasi: {}", e),
                    }
                }
                _ => {
                    eprintln!("Bilinmeyen arac: {}", name);
                }
            }
        }
    } else {
        println!("Model bir tool call uretmedi.");
    }

    println!("\n=== ISLEM TAMAMLANDI ===");

    Ok(())
}
```

### 📝 `src/tools/sandbox.rs` — Testler Ekle

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_path_simple() {
        let result = safe_path("test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_path_rejects_parent_dir() {
        let result = safe_path("../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_rejects_absolute() {
        let result = safe_path("/etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_rejects_empty() {
        let result = safe_path("");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_rejects_windows_absolute() {
        let result = safe_path("C:\\Windows\\System32");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_with_subdir() {
        let result = safe_path("subdir/test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_approve_default() {
        // Varsayılan olarak kapalı
        set_auto_approve(false);
        assert!(!is_auto_approve());
    }

    #[test]
    fn test_auto_approve_enable() {
        set_auto_approve(true);
        assert!(is_auto_approve());
        // Test sonrası sıfırla
        set_auto_approve(false);
    }

    #[test]
    fn test_ask_approval_auto_mode() {
        set_auto_approve(true);
        assert!(ask_approval("test", "detay"));
        set_auto_approve(false);
    }
}
```

### 📝 `src/tools/file_ops.rs` — Testleri Güncelle

`write_file` testlerini güncelle (onay modu nedeniyle):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::sandbox;

    fn setup_auto_approve() {
        sandbox::set_auto_approve(true);
    }

    #[test]
    fn test_write_and_read_file() {
        setup_auto_approve();
        let content = "Merhaba, dunya!";
        let write_result = write_file("test_write_read.txt", content);
        assert!(write_result.is_ok());

        let read_result = read_file("test_write_read.txt");
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        let _ = std::fs::remove_file("workspace/test_write_read.txt");
    }

    #[test]
    fn test_write_file_with_subdir() {
        setup_auto_approve();
        let result = write_file("subdir/test.txt", "content");
        assert!(result.is_ok());

        let read_result = read_file("subdir/test.txt");
        assert!(read_result.is_ok());

        let _ = std::fs::remove_file("workspace/subdir/test.txt");
        let _ = std::fs::remove_dir("workspace/subdir");
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_file("nonexistent_file_xyz.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_rejects_path_traversal() {
        setup_auto_approve();
        let result = write_file("../outside.txt", "content");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_rejects_absolute() {
        setup_auto_approve();
        let result = write_file("/tmp/outside.txt", "content");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_rejects_too_large() {
        setup_auto_approve();
        let big_content = "x".repeat(2_000_000);
        let result = write_file("big.txt", &big_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_dir_empty() {
        let result = list_dir(".");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_dir_nonexistent() {
        let result = list_dir("nonexistent_dir_xyz");
        assert!(result.is_err());
    }
}
```

### 🚀 Sıra

1. **`sandbox.rs`'e onay modülünü ekle**
2. **`file_ops.rs`'de `write_file`'a onay ekle**
3. **`main.rs`'e `--auto-approve` flag'i ekle**
4. **Testleri güncelle**
5. **Test et:**
   ```powershell
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```
6. **Manuel test:**
   ```powershell
   # Onay sormadan çalıştır (auto-approve)
   cargo run --release -- --auto-approve "Search the web for Rust news"

   # Onay sorarak çalıştır (varsayılan)
   cargo run --release -- "Write a test file"
   ```
7. **Commit + Push:**
   ```powershell
   git add .
   git commit -m "feat(security): add user approval for write_file operations"
   git push origin main
   ```

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
- Maksimum dosya boyutu: 1 MB
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

[LICENSE](LICENSE) dosyasına bakın.

## 🙏 Teşekkürler

- [Needle](https://huggingface.co/Cactus-Compute/needle2) — Araç seçimi modeli
- [Candle](https://github.com/huggingface/candle) — Rust ML framework
- [readability](https://crates.io/crates/readability) — Metin çıkarımı
- [scraper](https://crates.io/crates/scraper) — HTML ayrıştırma
