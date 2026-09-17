use cactagent::agent::Agent;
use cactagent::engine::needle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI argümanlarını al
    let args: Vec<String> = std::env::args().collect();
    let user_task = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Search the web for the latest news about Rust programming language".to_string()
    };

    println!("=== CactAgent v0.5.0 ===");
    println!("Gorev: {}\n", user_task);

    // Needle modelini yükle
    let needle_path = needle::ensure_model();
    let needle_engine = needle::load(&needle_path);

    // Ajanı oluştur ve çalıştır
    let agent = Agent::new(needle_engine);
    let result = agent.run(&user_task)?;

    println!("=== NIHAI CIKTI ===");
    println!("{}", result);

    Ok(())
}
