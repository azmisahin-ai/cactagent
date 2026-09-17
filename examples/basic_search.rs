//! Basit web araması örneği.
//!
//! Çalıştırmak için:
//! ```bash
//! cargo run --example basic_search -- "Rust programming"
//! ```

use cactagent::tools::search::web_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let query = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Rust programming language".to_string()
    };

    println!("=== Basit Web Aramasi ===");
    println!("Sorgu: {}\n", query);

    let results = web_search(&query)?;
    println!("{}", results);

    Ok(())
}
