//! Sandbox'lı dosya işlemleri örneği.
//!
//! Çalıştırmak için:
//! ```bash
//! cargo run --example file_ops
//! ```

use cactagent::tools::file_ops;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dosya Islemleri Ornegi ===\n");

    // 1. Dosya yaz
    println!("1. Dosya yaziliyor...");
    let write_result = file_ops::write_file("example_notes.txt", "Merhaba, CactAgent!")?;
    println!("   {}\n", write_result);

    // 2. Dizin listele
    println!("2. Dizin listeleniyor...");
    let listing = file_ops::list_dir(".")?;
    println!("   {}\n", listing.replace('\n', "\n   "));

    // 3. Dosya oku
    println!("3. Dosya okunuyor...");
    let content = file_ops::read_file("example_notes.txt")?;
    println!("   Icerik: {}\n", content);

    // 4. Temizlik
    println!("4. Temizlik yapiliyor...");
    // Sandbox disina cikmadan silmek icin dogrudan std::fs kullaniyoruz
    std::fs::remove_file("workspace/example_notes.txt")?;
    println!("   Temizlendi.");

    println!("\n=== ISLEM TAMAMLANDI ===");

    Ok(())
}