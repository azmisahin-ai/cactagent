//! Basit niyet algılama modülü.
//!
//! Needle modeli küçük olduğu için bazen başarısız olur.
//! Bu modül, basit keyword matching ile araç seçimini garanti eder.
//!
//! Hibrit yaklaşım:
//! 1. Önce keyword matching dene (hızlı, güvenilir, çok dilli)
//! 2. Eşleşme yoksa Needle modeline sor (karmaşık görevler için)

use serde_json::{json, Value};

/// Kullanıcı niyetini keyword'lerle algılar.
/// Eşleşme bulursa tool call JSON dizisi döndürür.
pub fn detect_intent(text: &str) -> Option<Value> {
    let lower = text.to_lowercase();

    // web_search keyword'leri (TR + EN)
    let search_keywords = [
        "search",
        "ara",
        "araştır",
        "arastir",
        "bul",
        "find",
        "lookup",
        "google",
        "web",
        "internet",
        "haber",
        "news",
    ];

    // read_file keyword'leri
    let read_file_keywords = ["read file", "dosya oku", "dosyayı oku", "file read"];

    // write_file keyword'leri
    let write_file_keywords = [
        "write",
        "yaz",
        "kaydet",
        "save",
        "create file",
        "dosya oluştur",
    ];

    // list_dir keyword'leri
    let list_dir_keywords = ["list", "listele", "dizin", "klasör", "directory", "folder"];

    // Öncelik sırası: URL > read_file > write_file > list_dir > web_search

    // 1. URL kontrolü (en spesifik)
    if let Some(url) = extract_url(text) {
        let url_context_keywords = ["read", "oku", "aç", "open", "fetch", "get", "url", "link"];
        if contains_any(&lower, &url_context_keywords) {
            return Some(json!([{
                "name": "read_url",
                "arguments": { "url": url }
            }]));
        }
    }

    // 2. read_file
    if contains_any(&lower, &read_file_keywords) {
        if let Some(path) = extract_path(text) {
            return Some(json!([{
                "name": "read_file",
                "arguments": { "path": path }
            }]));
        }
    }

    // 3. write_file
    if contains_any(&lower, &write_file_keywords) {
        if let Some(path) = extract_path(text) {
            return Some(json!([{
                "name": "write_file",
                "arguments": { "path": path, "content": "" }
            }]));
        }
    }

    // 4. list_dir
    if contains_any(&lower, &list_dir_keywords) {
        return Some(json!([{
            "name": "list_dir",
            "arguments": { "path": "." }
        }]));
    }

    // 5. web_search (en genel)
    if contains_any(&lower, &search_keywords) {
        return Some(json!([{
            "name": "web_search",
            "arguments": { "query": extract_search_query(text) }
        }]));
    }

    None
}

fn contains_any(text: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|k| text.contains(k))
}

/// Metinden URL çıkarır.
fn extract_url(text: &str) -> Option<String> {
    for word in text.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| {
            !c.is_alphanumeric() && c != ':' && c != '/' && c != '.' && c != '-' && c != '_'
        });
        if cleaned.starts_with("http://") || cleaned.starts_with("https://") {
            return Some(cleaned.to_string());
        }
    }
    None
}

/// Metinden dosya yolu çıkarır (basit).
fn extract_path(text: &str) -> Option<String> {
    for word in text.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| {
            !c.is_alphanumeric() && c != '.' && c != '/' && c != '_' && c != '-'
        });
        // En az bir nokta içermeli ve uzunluk > 2
        if cleaned.contains('.') && !cleaned.contains(' ') && cleaned.len() > 2 {
            return Some(cleaned.to_string());
        }
    }
    None
}

/// Arama sorgusunu temizler.
/// Baştaki "search", "ara", "bul" gibi kelimeleri kaldırır.
fn extract_search_query(text: &str) -> String {
    let prefixes = [
        "search the web for ",
        "search for ",
        "search ",
        "web'de ara ",
        "araştır ",
        "arastir ",
        "bul ",
        "ara ",
        "haberlerini araştır",
    ];

    let lower = text.to_lowercase();
    for prefix in &prefixes {
        if lower.starts_with(prefix) {
            return text[prefix.len()..].trim().to_string();
        }
    }

    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_search_english() {
        let result = detect_intent("Search the web for Rust news");
        assert!(result.is_some());
        let calls = result.unwrap();
        assert_eq!(calls[0]["name"], "web_search");
    }

    #[test]
    fn test_detect_search_turkish() {
        let result = detect_intent("Rust haberlerini araştır");
        assert!(result.is_some());
        let calls = result.unwrap();
        assert_eq!(calls[0]["name"], "web_search");
    }

    #[test]
    fn test_detect_read_url() {
        let result = detect_intent("Read https://blog.rust-lang.org/");
        assert!(result.is_some());
        let calls = result.unwrap();
        assert_eq!(calls[0]["name"], "read_url");
        assert_eq!(calls[0]["arguments"]["url"], "https://blog.rust-lang.org/");
    }

    #[test]
    fn test_detect_read_file() {
        let result = detect_intent("Read file notes.txt");
        assert!(result.is_some());
        let calls = result.unwrap();
        assert_eq!(calls[0]["name"], "read_file");
    }

    #[test]
    fn test_detect_write_file() {
        let result = detect_intent("Write to notes.txt");
        assert!(result.is_some());
        let calls = result.unwrap();
        assert_eq!(calls[0]["name"], "write_file");
    }

    #[test]
    fn test_detect_list_dir() {
        let result = detect_intent("List the directory");
        assert!(result.is_some());
        let calls = result.unwrap();
        assert_eq!(calls[0]["name"], "list_dir");
    }

    #[test]
    fn test_no_match() {
        let result = detect_intent("Hello world");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_url() {
        let url = extract_url("Read https://example.com/page");
        assert_eq!(url, Some("https://example.com/page".to_string()));
    }

    #[test]
    fn test_extract_path() {
        let path = extract_path("Read notes.txt");
        assert_eq!(path, Some("notes.txt".to_string()));
    }

    #[test]
    fn test_extract_search_query_english() {
        let query = extract_search_query("Search the web for Rust news");
        assert_eq!(query, "Rust news");
    }

    #[test]
    fn test_extract_search_query_turkish() {
        let query = extract_search_query("Rust haberlerini araştır");
        // Prefix başta değilse, metni olduğu gibi döndürür
        assert_eq!(query, "Rust haberlerini araştır");
    }

    #[test]
    fn test_extract_search_query_turkish_prefix() {
        let query = extract_search_query("araştır Rust haberleri");
        assert_eq!(query, "Rust haberleri");
    }
}
