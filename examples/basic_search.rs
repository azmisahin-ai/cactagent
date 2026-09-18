//! Basit web araması örneği.
//!
//! Çalıştırmak için:
//! ```bash
//! cargo run --example basic_search -- "Rust programming"
//! ```

use cactagent::i18n;
use cactagent::tools::search::web_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let query = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Rust programming language".to_string()
    };

    // Dil algıla (veya varsayılan İngilizce)
    let lang = i18n::detect_language(&query);

    println!("=== Basit Web Aramasi ===");
    println!("Sorgu: {}", query);
    println!("Dil: {} ({})\n", lang.code(), lang.ddg_region());

    let results = web_search(&query, lang)?;
    println!("{}", results);

    Ok(())
}
