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

`main.rs`'deki `TOOLS_JSON` sabitine ekle:

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

`main.rs`'deki `match name` bloğuna ekle:

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

### read_url

**Amaç:** Bir URL'nin içeriğini okumak

**Parametreler:**
- `url` (string): Okunacak URL

**Çıktı:** Temizlenmiş metin (max 3000 karakter)

**Örnek:**
```json
{"name":"read_url","arguments":{"url":"https://blog.rust-lang.org/"}}
```

## Gelecek Araçlar

### write_file

**Amaç:** Dosyaya içerik yazmak

**Parametreler:**
- `path` (string): Dosya yolu
- `content` (string): Yazılacak içerik

### run_command

**Amaç:** Terminal komutu çalıştırmak

**Parametreler:**
- `command` (string): Çalıştırılacak komut

**Uyarı:** Güvenlik riski! Sadece güvenilir ortamlarda kullanın.