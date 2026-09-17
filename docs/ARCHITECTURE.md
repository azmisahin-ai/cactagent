# Mimari

## Genel Bakış

CactAgent, tek bir küçük dil modeli (Needle v2) ve bir dizi aracı tek bir Rust binary'sinde birleştirir. Hiçbir ayrı sunucu, Python bağımlılığı veya harici API yoktur.

## Bileşenler

### 1. Engine Katmanı (`src/engine/needle.rs`)

**Needle v2 (13 MB)**
- Görev: Doğal dildeki kullanıcı isteğini araç çağrısına dönüştürmek
- Çıktı: `<tool_call>[{"name":"...","arguments":{...}}]</tool_call>`
- Kullanım: `needle_engine.run(task, tools_json)`

Needle, **sadece araç seçimi** için eğitilmiş özel bir modeldir. Metin üretmez, özet yapmaz. Tek işi: kullanıcı niyetini anlayıp doğru aracı seçmek.

### 2. Tools Katmanı (`src/tools/`)

Her araç bir fonksiyon olarak tanımlanır ve JSON şeması ile modele sunulur.

**web_search(query)**
- DuckDuckGo HTML arayüzünü kullanır
- Retry mekanizması: 3 deneme, 2 sn arayla
- Çıktı: Başlık + URL + snippet listesi

**read_url(url)**
- Sayfayı indirir, readability ile ana içeriği çıkarır
- Fallback: yoğunluk bazlı HTML temizleme
- Cümle sonunda akıllıca keser (3000 karakter)

### 3. Ana Akış (`src/main.rs`)

Doğrusal, kontrollü bir akış:
1. Kullanıcı isteği → Needle → araç çağrısı
2. Araç çalıştır → sonuç
3. Sonuç → kullanıcıya ham metin

**Özetleme yok.** Model uydurmuyor, sadece gerçek metin sunuluyor.

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
read_url: 96 KB HTML → readability → 3 KB temiz metin
    ↓
Kullanıcı: Ham, gerçek metin
```

## Bağımlılıklar

| Crate | Amaç | Boyut |
|-------|------|-------|
| `needle-infer` | Needle modeli yükleme ve çıkarım | Küçük |
| `reqwest` | HTTP istekleri (native-tls) | Orta |
| `scraper` | HTML ayrıştırma | Küçük |
| `readability` | Ana içerik çıkarımı | Küçük |
| `hf-hub` | Model indirme | Küçük |
| `url` | URL ayrıştırma | Küçük |

## Performans

| İşlem | Süre (CPU) |
|-------|-----------|
| Needle araç seçimi | ~50 ms |
| Web araması | ~2 sn |
| URL okuma (readability) | ~2 sn |
| **Toplam** | **~4-5 sn** |

## Tasarım Kararları

### Neden Qwen2.5-0.5B kaldırıldı?

Sistem ilk sürümünde iki model kullanıyordu: Needle (araç seçimi) + Qwen2.5-0.5B (özetleme). Ancak:

1. **Halüsinasyon:** Qwen2.5-0.5B, özetleme görevinde sık sık kaynakta olmayan bilgiler uyduruyordu.
2. **Tekrar döngüsü:** Düşük temperature'da aynı cümleyi tekrar ediyordu.
3. **Bellek:** ~400 MB ek bellek kullanımı, E2 Micro gibi kısıtlı cihazlarda sorun yaratıyordu.
4. **Değer katmıyordu:** Ürettiği özetler, ham metinden daha az bilgi içeriyordu.

**Sonuç:** Qwen kaldırıldı, sistem sadece Needle + araçlar üzerine odaklandı. Bu, "en düşük donanımda en hızlı çalışan" hedefine daha uygun.

### Özet modeli opsiyonel olarak geri gelebilir

Gelecekte, E2 Micro'da çalışabilecek **daha küçük** (100-200 MB) bir özet modeli veya **daha iyi prompt** ile Qwen2.5-0.5B, `feature = "summarize"` flag'i arkasında opsiyonel olarak eklenebilir.
