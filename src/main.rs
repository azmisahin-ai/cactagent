use cactagent::engine::needle;
use cactagent::tools::sandbox;
use cactagent::tools::{file_ops, reader, search, TOOLS_JSON};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI argümanlarını al
    let args: Vec<String> = std::env::args().collect();

    // --auto-approve flag'ini kontrol et
    let auto_approve = args.iter().any(|a| a == "--auto-approve");
    if auto_approve {
        sandbox::set_auto_approve(true);
        println!("[!] Otomatik onay modu aktif. Dosya yazma onayi sorulmayacak.\n");
    }

    // Flag'leri temizle
    let filtered_args: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .collect();

    let user_task = if filtered_args.len() > 1 {
        filtered_args[1..].join(" ")
    } else {
        "Search the web for the latest news about Rust programming language".to_string()
    };

    println!("=== CactAgent v0.7.0 ===");
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

    // Tool call'ları ayrıştır ve çalıştır
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
                        match search::web_search(query) {
                            Ok(r) => {
                                println!("=== ARAMA SONUCLARI ===");
                                println!("{}", r);
                            }
                            Err(e) => eprintln!("Arama hatasi: {}", e),
                        }
                    }
                }
                "read_url" => {
                    if let Some(url) = args["url"].as_str() {
                        match reader::read_url(url) {
                            Ok(content) => {
                                println!("=== SAYFA ICERIGI ===");
                                println!("{}", content);
                            }
                            Err(e) => eprintln!("Okuma hatasi: {}", e),
                        }
                    }
                }
                "read_file" => {
                    if let Some(path) = args["path"].as_str() {
                        match file_ops::read_file(path) {
                            Ok(content) => {
                                println!("=== DOSYA ICERIGI ===");
                                println!("{}", content);
                            }
                            Err(e) => eprintln!("Dosya okuma hatasi: {}", e),
                        }
                    }
                }
                "write_file" => {
                    if let (Some(path), Some(content)) =
                        (args["path"].as_str(), args["content"].as_str())
                    {
                        match file_ops::write_file(path, content) {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => eprintln!("Dosya yazma hatasi: {}", e),
                        }
                    }
                }
                "list_dir" => {
                    let path = args["path"].as_str().unwrap_or(".");
                    match file_ops::list_dir(path) {
                        Ok(listing) => {
                            println!("=== DIZIN ICERIGI ===");
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
    }

    println!("\n=== ISLEM TAMAMLANDI ===");

    Ok(())
}
