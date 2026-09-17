//! Yeni bir araç ekleme rehberi.
//!
//! Bu örnek, CactAgent'a nasıl yeni bir araç ekleneceğini gösterir.
//!
//! Çalıştırmak için:
//! ```bash
//! cargo run --example custom_tool
//! ```

/// Örnek araç: Basit bir hesaplayıcı.
///
/// Gerçek bir araç şu formatta olmalı:
/// - Girdi: string parametreler
/// - Çıktı: Result<String, Box<dyn Error>>
fn simple_calculator(expression: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Basit bir toplama örneği: "5+3"
    if let Some((a, b)) = expression.split_once('+') {
        let a: i64 = a.trim().parse()?;
        let b: i64 = b.trim().parse()?;
        Ok(format!("{} + {} = {}", a, b, a + b))
    } else {
        Err("Sadece '+' destekleniyor. Örnek: '5+3'".into())
    }
}

fn main() {
    println!("=== Yeni Arac Ekleme Rehberi ===\n");

    println!("Bu ornek, CactAgent'a nasil yeni bir arac ekleyeceginizi gosterir.\n");

    println!("ADIM 1: Arac fonksiyonunu yazin");
    println!("  src/tools/my_tool.rs:");
    println!("    pub fn my_tool(param: &str) -> Result<String, Box<dyn Error>> {{");
    println!("        // ... islem");
    println!("        Ok(\"sonuc\".to_string())");
    println!("    }}\n");

    println!("ADIM 2: tools/mod.rs'a ekleyin");
    println!("    pub mod my_tool;\n");

    println!("ADIM 3: JSON semasini TOOLS_JSON'a ekleyin");
    println!("    {{");
    println!("        \"name\": \"my_tool\",");
    println!("        \"description\": \"Ne yaptigini acikla\",");
    println!("        \"parameters\": {{ ... }}");
    println!("    }}\n");

    println!("ADIM 4: main.rs'de cagriyi isleyin");
    println!("    match name {{");
    println!("        \"my_tool\" => {{");
    println!("            if let Some(param) = args[\"param\"].as_str() {{");
    println!("                my_tool::my_tool(param).unwrap_or_else(|e| format!(\"Hata: {{}}\", e))");
    println!("            }} else {{");
    println!("                \"Hata: param eksik\".to_string()");
    println!("            }}");
    println!("        }}");
    println!("        // ...");
    println!("    }}\n");

    println!("=== ORNEK CALISTIRMA ===\n");

    // Örnek aracı çalıştır
    println!("Ornek arac: simple_calculator");
    match simple_calculator("5+3") {
        Ok(result) => println!("  Girdi: '5+3' -> Cikti: {}", result),
        Err(e) => println!("  Hata: {}", e),
    }

    match simple_calculator("10+20") {
        Ok(result) => println!("  Girdi: '10+20' -> Cikti: {}", result),
        Err(e) => println!("  Hata: {}", e),
    }

    match simple_calculator("invalid") {
        Ok(result) => println!("  Girdi: 'invalid' -> Cikti: {}", result),
        Err(e) => println!("  Girdi: 'invalid' -> Hata: {}", e),
    }
}