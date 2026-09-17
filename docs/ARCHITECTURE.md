# Mimari

## Genel Bakış

CactAgent, iki küçük dil modelini tek bir Rust binary'sinde birleştirir. Hiçbir ayrı sunucu, Python bağımlılığı veya harici API yoktur.

## Bileşenler

### 1. Engine Katmanı (`src/engine/`)

**Needle v2 (13 MB)**
- Görev: Doğal dildeki kullanıcı isteğini araç çağrısına dönüştürmek
- Çıktı: `<tool_call>[{"name":"...","arguments":{...}}]</tool_call>`
- Kullanım: `needle_engine.run(task, tools_json)`

**Qwen2.5-0.5B (~350 MB)**
- Görev: Metin özetleme, yanıt üretme
- Çıktı: Serbest metin
- Kullanım: `summarize_with_qwen(content)`

### 2. Tools Katmanı (`src/tools/`)

Her araç bir fonksiyon olarak tanımlanır ve JSON şeması ile modele sunulur.

**web_search(query)**
- DuckDuckGo HTML arayüzünü kullanır
- Retry mekanizması: 3 deneme, 2 sn arayla
- Çıktı: Başlık + URL + snippet listesi

**read_url(url)**
- Sayfayı indirir, HTML'i temizler
- `<article>`, `<main>`, `<body>` önceliği
- Cümle sonunda akıllıca keser (3000 karakter)

### 3. Agent Katmanı (`src/agent.rs`)

Şu an doğrusal bir akış:
1. Kullanıcı isteği → Needle → araç çağrısı
2. Araç çalıştır → sonuç
3. Sonuç → Qwen → özet
4. Özet → kullanıcı

Gelecekte: Çok adımlı döngü (ReAct pattern).

## Veri Akışı

```
Kullanıcı: "Rust haberlerini araştır"
    ↓
Needle: {"name":"web_search","arguments":{"query":"Rust news"}}
    ↓
web_search: DuckDuckGo → 5 sonuç
    ↓
İlk URL: https://blog.rust-lang.org/
    ↓
read_url: 96 KB HTML → 2957 karakter temiz metin
    ↓
Qwen: "This is the main Rust blog..."
    ↓
Kullanıcı: Özet
```

## Bağımlılıklar

| Crate | Amaç | Boyut |
|-------|------|-------|
| `needle-infer` | Needle modeli yükleme | Küçük |
| `candelabra` | Qwen yükleme + çıkarım | Orta |
| `reqwest` | HTTP istekleri | Orta |
| `scraper` | HTML ayrıştırma | Küçük |
| `hf-hub` | Model indirme | Küçük |

## Performans

| İşlem | Süre (CPU) | Süre (GPU, tahmini) |
|-------|-----------|---------------------|
| Needle araç seçimi | ~50 ms | ~10 ms |
| Web araması | ~2 sn | ~2 sn |
| URL okuma | ~2 sn | ~2 sn |
| Qwen özet (50 token) | ~6 sn | ~1 sn |
| **Toplam** | **~10 sn** | **~5 sn** |
