mod engine;
mod tools;

use tools::search::web_search;
use tools::reader::read_url;

const TOOLS_JSON: &str = r#"[{
    "name": "web_search",
    "description": "Search the web for current information.",
    "parameters": {
        "type": "object",
        "properties": {
            "query": {"type": "string", "description": "The search query"}
        },
        "required": ["query"]
    }
}]"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Needle modelini otomatik indir (yoksa)
    let needle_path = engine::ensure_needle_model();
    let needle_engine = engine::load_engine(&needle_path);

    // ADIM 1: Modelden arama sorgusu
    println!("=== ADIM 1: MODELDEN ARAMA SORGUSU ===");
    let user_task = "Search the web for the latest news about Rust programming language";

    let result = needle_engine.run(user_task, TOOLS_JSON);

    println!("Ham model cikisi:");
    println!("{}", result.text);
    println!("---");

    if let Some(think) = engine::extract_think(&result.text) {
        println!("[Dusunce] {}", think);
    }

    let search_query = if let Some(tool_calls) = engine::parse_tool_call(&result.text) {
        if let Some(call) = tool_calls.first() {
            let name = call["name"].as_str().unwrap_or("");
            if name == "web_search" {
                call["arguments"]["query"].as_str().unwrap_or("").to_string()
            } else {
                "latest news about Rust programming language".to_string()
            }
        } else {
            "latest news about Rust programming language".to_string()
        }
    } else {
        "latest news about Rust programming language".to_string()
    };

    // ADIM 2: Web araması
    println!("\n=== ADIM 2: WEB ARAMASI ===");
    println!("Arama sorgusu: {}\n", search_query);

    let search_results = match web_search(&search_query) {
        Ok(r) => r,
        Err(e) => {
            println!("Arama hatasi: {}", e);
            return Ok(());
        }
    };

    println!("=== ARAMA SONUCLARI ===");
    println!("{}", search_results);

    // ADIM 3: İlk URL'yi çıkar
    println!("=== ADIM 3: ILK URL CIKARILIYOR ===");

    let first_url = extract_first_url(&search_results);

    match first_url {
        Some(url) => {
            println!("Secilen URL: {}", url);

            // ADIM 4: URL'yi oku
            println!("\n=== ADIM 4: URL OKUNUYOR ===");
            match read_url(&url) {
                Ok(content) => {
                    println!("\n=== SAYFA ICERIGI (ilk 1000 karakter) ===");
                    let preview: String = content.chars().take(1000).collect();
                    println!("{}", preview);

                    // ADIM 5: Qwen ile özetle
                    println!("\n=== ADIM 5: QWEN ILE OZETLENIYOR ===");
                    match engine::summarize_with_qwen(&content) {
                        Ok(summary) => {
                            println!("\n=== OZET ===");
                            println!("{}", summary);
                        }
                        Err(e) => {
                            println!("Ozet hatasi: {}", e);
                        }
                    }
                }
                Err(e) => println!("Okuma hatasi: {}", e),
            }
        }
        None => {
            println!("Arama sonuclarindan URL cikarilamadi.");
        }
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