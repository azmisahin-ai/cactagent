use cactagent::engine::needle;
use cactagent::tools::{reader, search, TOOLS_JSON};

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

    // Tool call'u ayrıştır
    let search_query = if let Some(tool_calls) = needle::parse_tool_call(&result.text) {
        if let Some(call) = tool_calls.first() {
            let name = call["name"].as_str().unwrap_or("");
            if name == "web_search" {
                call["arguments"]["query"]
                    .as_str()
                    .unwrap_or("")
                    .to_string()
            } else {
                user_task.clone()
            }
        } else {
            user_task.clone()
        }
    } else {
        user_task.clone()
    };

    // ADIM 2: Web araması
    println!("\n=== ADIM 2: WEB ARAMASI ===");
    println!("Sorgu: {}\n", search_query);

    let search_results = match search::web_search(&search_query) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Arama hatasi: {}", e);
            return Ok(());
        }
    };

    println!("{}", search_results);

    // ADIM 3: İlk URL'yi çıkar
    let first_url = extract_first_url(&search_results);

    match first_url {
        Some(url) => {
            println!("=== ADIM 3: URL OKUNUYOR ===");
            println!("URL: {}\n", url);

            match reader::read_url(&url) {
                Ok(content) => {
                    println!("=== SAYFA ICERIGI ===\n");
                    println!("{}", content);
                }
                Err(e) => eprintln!("Okuma hatasi: {}", e),
            }
        }
        None => eprintln!("Arama sonuclarindan URL cikarilamadi."),
    }

    println!("\n=== ISLEM TAMAMLANDI ===");

    Ok(())
}

fn extract_first_url(search_results: &str) -> Option<String> {
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
