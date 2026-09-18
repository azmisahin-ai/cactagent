use cactagent::engine::needle;
use cactagent::tools::sandbox;
use cactagent::tools::{audit, file_ops, ratelimit, reader, search, TOOLS_JSON};

fn print_help() {
    println!("CactAgent v{} - Tamamen yerel AI ajani", env!("CARGO_PKG_VERSION"));
    println!();
    println!("KULLANIM:");
    println!("    cactagent [OPTIONS] \"<gorev>\"");
    println!();
    println!("OPSIYONLAR:");
    println!("    --help              Bu yardim mesajini goster");
    println!("    --version           Surum bilgisini goster");
    println!("    --auto-approve      Dosya yazma onayini atla (script/CI icin)");
    println!();
    println!("ORNEKLER:");
    println!("    cactagent \"Rust haberlerini arastir\"");
    println!("    cactagent \"Write 'Merhaba' to notes.txt\"");
    println!("    cactagent --auto-approve \"Write 'test' to test.txt\"");
    println!();
    println!("ARACLAR:");
    println!("    web_search   - DuckDuckGo uzerinden arama");
    println!("    read_url     - Sayfa icerigini oku ve temizle");
    println!("    read_file    - Sandbox icindeki dosyayi oku");
    println!("    write_file   - Sandbox icindeki dosyaya yaz");
    println!("    list_dir     - Sandbox icindeki dizini listele");
    println!();
    println!("GUVENLIK:");
    println!("    - Tum dosya islemleri ./workspace/ icinde sinirli");
    println!("    - Path traversal ve absolute path reddedilir");
    println!("    - write_file varsayilan olarak onay ister");
    println!("    - Maksimum dosya boyutu: 1 MB");
}

fn print_version() {
    println!("CactAgent v{}", env!("CARGO_PKG_VERSION"));
    println!("Lisans: {}", env!("CARGO_PKG_LICENSE"));
    println!("Repo: {}", env!("CARGO_PKG_REPOSITORY"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return Ok(());
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        print_version();
        return Ok(());
    }

    let auto_approve = args.iter().any(|a| a == "--auto-approve");
    if auto_approve {
        sandbox::set_auto_approve(true);
        println!("[!] Otomatik onay modu aktif.\n");
    }

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

    println!("=== CactAgent v{} ===", env!("CARGO_PKG_VERSION"));
    println!("Gorev: {}\n", user_task);

    let needle_path = needle::ensure_model();
    let needle_engine = needle::load(&needle_path);

    println!("=== ADIM 1: ARAC SECIMI ===");
    let result = needle_engine.run(&user_task, TOOLS_JSON);

    println!("Ham cikti:");
    println!("{}", result.text);
    println!("---");

    if let Some(think) = needle::extract_think(&result.text) {
        println!("[Dusunce] {}", think);
    }

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

            // Rate limit kontrolü
            let limit = ratelimit::limit_for_tool(name);
            if let Err(wait_secs) = ratelimit::check_rate_limit(name, limit) {
                eprintln!(
                    "[RATE LIMIT] '{}' araci cok sik cagrildi. {} saniye bekleyin.",
                    name, wait_secs
                );
                audit::log_tool_call(
                    name,
                    &args.to_string(),
                    &format!("RATE LIMIT: {} saniye", wait_secs),
                );
                continue;
            }

            let tool_result: Result<String, String> = match name {
                "web_search" => {
                    if let Some(query) = args["query"].as_str() {
                        match search::web_search(query) {
                            Ok(r) => {
                                println!("=== ARAMA SONUCLARI ===");
                                println!("{}", r);
                                Ok(format!("{} sonuc", r.lines().count()))
                            }
                            Err(e) => Err(format!("Arama hatasi: {}", e)),
                        }
                    } else {
                        Err("'query' parametresi eksik".to_string())
                    }
                }
                "read_url" => {
                    if let Some(url) = args["url"].as_str() {
                        match reader::read_url(url) {
                            Ok(content) => {
                                println!("=== SAYFA ICERIGI ===");
                                println!("{}", content);
                                Ok(format!("{} karakter", content.len()))
                            }
                            Err(e) => Err(format!("Okuma hatasi: {}", e)),
                        }
                    } else {
                        Err("'url' parametresi eksik".to_string())
                    }
                }
                "read_file" => {
                    if let Some(path) = args["path"].as_str() {
                        match file_ops::read_file(path) {
                            Ok(content) => {
                                println!("=== DOSYA ICERIGI ===");
                                println!("{}", content);
                                Ok(format!("{} karakter", content.len()))
                            }
                            Err(e) => Err(format!("Dosya okuma hatasi: {}", e)),
                        }
                    } else {
                        Err("'path' parametresi eksik".to_string())
                    }
                }
                "write_file" => {
                    if let (Some(path), Some(content)) =
                        (args["path"].as_str(), args["content"].as_str())
                    {
                        match file_ops::write_file(path, content) {
                            Ok(msg) => {
                                println!("{}", msg);
                                Ok(format!("{} byte yazildi", content.len()))
                            }
                            Err(e) => Err(format!("Dosya yazma hatasi: {}", e)),
                        }
                    } else {
                        Err("'path' ve 'content' parametreleri gerekli".to_string())
                    }
                }
                "list_dir" => {
                    let path = args["path"].as_str().unwrap_or(".");
                    match file_ops::list_dir(path) {
                        Ok(listing) => {
                            println!("=== DIZIN ICERIGI ===");
                            println!("{}", listing);
                            Ok(format!("{} girdi", listing.lines().count()))
                        }
                        Err(e) => Err(format!("Dizin listeleme hatasi: {}", e)),
                    }
                }
                _ => Err(format!("Bilinmeyen arac: {}", name)),
            };

            // Audit log
            let log_result = match &tool_result {
                Ok(r) => r.clone(),
                Err(e) => format!("HATA: {}", e),
            };
            audit::log_tool_call(name, &args.to_string(), &log_result);
        }
    } else {
        println!("Model bir tool call uretmedi.");
    }

    println!("\n=== ISLEM TAMAMLANDI ===");

    Ok(())
}
