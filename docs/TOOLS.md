# Araç Sistemi

## Araç Nedir?

CactAgent'da bir "araç", modelin çağırabileceği bir Rust fonksiyonudur. Her araç:
1. Bir **isim** ve **açıklama** içerir
2. **Parametrelerini** JSON şeması ile tanımlar
3. Çalıştırıldığında **string sonuç** döndürür

## Yeni Araç Ekleme

### 1. Araç Fonksiyonunu Yaz

`src/tools/my_tool.rs`:

```rust
pub fn my_tool(param: &str) -> Result<String, Box<dyn std::error::Error>> {
    // ... işlem
    Ok("sonuc".to_string())
}
```

### 2. `tools/mod.rs`'a Ekle

```rust
pub mod my_tool;
```

### 3. JSON Şemasını Tanımla

`src/tools/mod.rs`'daki `TOOLS_JSON` sabitine ekle:

```json
{
    "name": "my_tool",
    "description": "Ne yaptığını açıkla",
    "parameters": {
        "type": "object",
        "properties": {
            "param": {"type": "string", "description": "Parametre açıklaması"}
        },
        "required": ["param"]
    }
}
```

### 4. Çağrıyı İşle

`src/main.rs`'deki `match name` bloğuna ekle:

```rust
"my_tool" => {
    if let Some(param) = args["param"].as_str() {
        my_tool::my_tool(param).unwrap_or_else(|e| format!("Hata: {}", e))
    } else {
        "Hata: param eksik".to_string()
    }
}
```

## Mevcut Araçlar

### web_search

**Amaç:** İnternette arama yapmak

**Parametreler:**
- `query` (string): Arama sorgusu

**Çıktı:** En fazla 5 sonuç, her biri başlık + URL + snippet

**Örnek:**
```json
{"name":"web_search","arguments":{"query":"Rust programming language"}}
```

**Notlar:**
- DuckDuckGo HTML arayüzü kullanılır
- Retry: 3 deneme, 2 sn arayla
- `ConnectionReset` hatalarına karşı dayanıklı

### read_url

**Amaç:** Bir URL'nin içeriğini okumak ve temizlemek

**Parametreler:**
- `url` (string): Okunacak URL

**Çıktı:** Temizlenmiş metin (max 3000 karakter, cümle sonunda kesilir)

**Örnek:**
```json
{"name":"read_url","arguments":{"url":"https://blog.rust-lang.org/"}}
```

**Notlar:**
- Önce `readability` denenir, başarısız olursa yoğunluk bazlı fallback
- Breadcrumb, menü, reklam temizlenir
- Cümle sonunda akıllıca kesilir

## Gelecek Araçlar

### read_file

**Amaç:** Sandbox'lı dizinden dosya okumak

**Parametreler:**
- `path` (string): `./workspace/` içindeki dosya yolu

**Güvenlik:**
- Path traversal koruması (`..` yasak)
- Sadece `./workspace/` dizini

### write_file

**Amaç:** Sandbox'lı dizine dosya yazmak

**Parametreler:**
- `path` (string): `./workspace/` içindeki dosya yolu
- `content` (string): Yazılacak içerik

**Güvenlik:**
- Path traversal koruması
- Sadece `./workspace/` dizini
- Kullanıcı onayı (opsiyonel)

### run_command

**Amaç:** Terminal komutu çalıştırmak

**Parametreler:**
- `command` (string): Çalıştırılacak komut

**Güvenlik:**
- **Whitelist bazlı izin sistemi** (sadece belirli komutlar)
- Shell injection koruması
- Çalışma dizini sınırlaması
- Timeout

**Uyarı:** Bu araç en son eklenecek ve güvenlik incelemesinden geçecek.
