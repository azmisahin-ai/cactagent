use cactagent::engine::needle;
use cactagent::tools::{file_ops, reader, search, TOOLS_JSON};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI argümanlarını al
    let args: Vec<String> = std::env::args().collect();
    let user_task = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Search the web for the latest news about Rust programming language".to_string()
    };

    println!("=== CactAgent ===");
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

    // Tool call'ları ayrıştır ve her birini çalıştır
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
                        println!("[DEBUG] Web aramasi yapiliyor: {}", query);
                        match search::web_search(query) {
                            Ok(r) => {
                                println!("\n=== ARAMA SONUCLARI ===");
                                println!("{}", r);
                            }
                            Err(e) => eprintln!("Arama hatasi: {}", e),
                        }
                    } else {
                        eprintln!("Hata: 'query' parametresi eksik.");
                    }
                }
                "read_url" => {
                    if let Some(url) = args["url"].as_str() {
                        println!("[DEBUG] URL okunuyor: {}", url);
                        match reader::read_url(url) {
                            Ok(content) => {
                                println!("\n=== SAYFA ICERIGI ===");
                                println!("{}", content);
                            }
                            Err(e) => eprintln!("Okuma hatasi: {}", e),
                        }
                    } else {
                        eprintln!("Hata: 'url' parametresi eksik.");
                    }
                }
                "read_file" => {
                    if let Some(path) = args["path"].as_str() {
                        println!("[DEBUG] Dosya okunuyor: {}", path);
                        match file_ops::read_file(path) {
                            Ok(content) => {
                                println!("\n=== DOSYA ICERIGI ===");
                                println!("{}", content);
                            }
                            Err(e) => eprintln!("Dosya okuma hatasi: {}", e),
                        }
                    } else {
                        eprintln!("Hata: 'path' parametresi eksik.");
                    }
                }
                "write_file" => {
                    if let (Some(path), Some(content)) =
                        (args["path"].as_str(), args["content"].as_str())
                    {
                        println!("[DEBUG] Dosya yaziliyor: {}", path);
                        match file_ops::write_file(path, content) {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => eprintln!("Dosya yazma hatasi: {}", e),
                        }
                    } else {
                        eprintln!("Hata: 'path' ve 'content' parametreleri gerekli.");
                    }
                }
                "list_dir" => {
                    let path = args["path"].as_str().unwrap_or(".");
                    println!("[DEBUG] Dizin listeleniyor: {}", path);
                    match file_ops::list_dir(path) {
                        Ok(listing) => {
                            println!("\n=== DIZIN ICERIGI ===");
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
        println!("Nihai cikti: {}", result.text);
    }

    println!("\n=== ISLEM TAMAMLANDI ===");

    Ok(())
}

/// Arama sonuçlarından ilk HTTP URL'sini çıkarır.
/// Test amaçlı public bırakıldı.
#[allow(dead_code)]
pub fn extract_first_url(search_results: &str) -> Option<String> {
    for line in search_results.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("URL: ") {
            let url = trimmed.trim_start_matches("URL: ").trim();
            if url.starts_with("http") {
                return Some(url.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first_url_finds_url() {
        let search_results = "1. Some Title\n   URL: https://example.com\n   Snippet\n\n";
        let url = extract_first_url(search_results);
        assert_eq!(url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_extract_first_url_multiple() {
        let search_results =
            "1. First\n   URL: https://first.com\n\n2. Second\n   URL: https://second.com\n";
        let url = extract_first_url(search_results);
        assert_eq!(url, Some("https://first.com".to_string()));
    }

    #[test]
    fn test_extract_first_url_none() {
        let search_results = "No URLs here.";
        let url = extract_first_url(search_results);
        assert!(url.is_none());
    }

    #[test]
    fn test_extract_first_url_skips_non_http() {
        let search_results =
            "1. Title\n   URL: ftp://example.com\n\n2. Title 2\n   URL: https://example.com\n";
        let url = extract_first_url(search_results);
        assert_eq!(url, Some("https://example.com".to_string()));
    }
}
