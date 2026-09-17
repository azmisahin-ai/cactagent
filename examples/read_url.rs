//! URL okuma ve metin çıkarma örneği.
//!
//! Çalıştırmak için:
//! ```bash
//! cargo run --example read_url -- "https://blog.rust-lang.org/"
//! ```

use cactagent::tools::reader::read_url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let url = if args.len() > 1 {
        args[1].clone()
    } else {
        "https://blog.rust-lang.org/".to_string()
    };

    println!("=== URL Okuma ===");
    println!("URL: {}\n", url);

    let content = read_url(&url)?;
    println!("=== ICERIK ===");
    println!("{}", content);

    Ok(())
}