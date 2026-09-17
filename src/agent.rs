//! Ajan döngüsü (ReAct pattern).
//!
//! Bu modül, çok adımlı görevleri yönetir:
//! 1. Modele görevi + bağlamı ver
//! 2. Model bir araç çağırırsa çalıştır
//! 3. Sonucu bağlama ekle
//! 4. Model "bitti" diyene kadar tekrarla

use needle_infer::v2_engine::V2Engine;
use serde_json::Value;

use crate::engine::needle;
use crate::tools::{file_ops, reader, search, TOOLS_JSON};

/// Maksimum iterasyon sayısı (sonsuz döngüyü engeller)
pub const DEFAULT_MAX_ITERATIONS: usize = 5;

/// Bağlamda tutulacak maksimum araç sonucu sayısı
pub const MAX_CONTEXT_RESULTS: usize = 3;

/// Ajan yapılandırması
pub struct Agent {
    engine: V2Engine,
    max_iterations: usize,
}

impl Agent {
    /// Yeni bir ajan oluşturur
    pub fn new(engine: V2Engine) -> Self {
        Self {
            engine,
            max_iterations: DEFAULT_MAX_ITERATIONS,
        }
    }

    /// Maksimum iterasyon sayısını ayarlar
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Görevi çok adımlı döngü ile çalıştırır
    pub fn run(&self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("=== AJAN DONGUSU BASLIYOR ===");
        println!("Gorev: {}\n", task);

        // Bağlam: geçmiş araç sonuçları
        let mut context: Vec<String> = Vec::new();
        let mut final_output = String::new();

        for iteration in 1..=self.max_iterations {
            println!("--- Iterasyon {}/{} ---", iteration, self.max_iterations);

            // Modele verilecek girdi: görev + bağlam
            let input = build_input(task, &context);

            // Modeli çalıştır
            let result = self.engine.run(&input, TOOLS_JSON);

            // Düşünce bloğunu göster
            if let Some(think) = needle::extract_think(&result.text) {
                println!("[Dusunce] {}", think);

                // "BITTI" sinyali kontrol et
                let think_upper = think.to_uppercase();
                if think_upper.contains("BITTI")
                    || think_upper.contains("TAMAM")
                    || think_upper.contains("DONE")
                {
                    println!("[Bitti] Model gorevi tamamladigini belirtti.");
                    final_output = think;
                    break;
                }
            }

            // Tool call'ları ayrıştır
            let tool_calls = match needle::parse_tool_call(&result.text) {
                Some(calls) if !calls.is_empty() => calls,
                Some(_) => {
                    println!("[Bitti] Model bos tool call dondurdu.");
                    break;
                }
                None => {
                    println!("[Bitti] Model tool call uretmedi.");
                    break;
                }
            };

            // Her aracı çalıştır
            let mut iteration_results = Vec::new();

            for call in tool_calls {
                let name = call["name"].as_str().unwrap_or("bilinmeyen");
                let args = &call["arguments"];

                println!("[Arac] {} | Args: {}", name, args);

                let output = match execute_tool(name, args) {
                    Ok(o) => o,
                    Err(e) => format!("Hata: {}", e),
                };

                // Sonucu kısalt (bağlam şişmesin)
                let preview: String = output.chars().take(500).collect();
                println!("[Sonuc] {}\n", preview);

                iteration_results.push(format!("[{} sonucu]:\n{}", name, output));
            }

            // Sonuçları bağlama ekle
            for result in iteration_results {
                context.push(result);
            }

            // Bağlamı sınırla (en son N sonucu tut)
            if context.len() > MAX_CONTEXT_RESULTS {
                let excess = context.len() - MAX_CONTEXT_RESULTS;
                context.drain(0..excess);
            }
        }

        println!("=== AJAN DONGUSU BITTI ===\n");

        if final_output.is_empty() {
            final_output = context.join("\n\n");
        }

        Ok(final_output)
    }
}

/// Modele verilecek girdiyi oluşturur (görev + bağlam)
pub fn build_input(task: &str, context: &[String]) -> String {
    if context.is_empty() {
        task.to_string()
    } else {
        let context_str = context.join("\n\n");
        format!(
            "{}\n\nOnceki arac sonuclari:\n{}\n\nSadece tanimli araclari kullan (web_search, read_url, read_file, write_file, list_dir). Devam et veya BITTI yaz.",
            task, context_str
        )
    }
}

/// Bir araç çağrısını çalıştırır
pub fn execute_tool(name: &str, args: &Value) -> Result<String, Box<dyn std::error::Error>> {
    match name {
        "web_search" => {
            let query = args["query"].as_str().ok_or("'query' parametresi eksik")?;
            search::web_search(query)
        }
        "read_url" => {
            let url = args["url"].as_str().ok_or("'url' parametresi eksik")?;
            reader::read_url(url)
        }
        "read_file" => {
            let path = args["path"].as_str().ok_or("'path' parametresi eksik")?;
            file_ops::read_file(path)
        }
        "write_file" => {
            let path = args["path"].as_str().ok_or("'path' parametresi eksik")?;
            let content = args["content"]
                .as_str()
                .ok_or("'content' parametresi eksik")?;
            file_ops::write_file(path, content)
        }
        "list_dir" => {
            let path = args["path"].as_str().unwrap_or(".");
            file_ops::list_dir(path)
        }
        _ => Err(format!("Bilinmeyen arac: {}", name).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_input_empty_context() {
        let input = build_input("test task", &[]);
        assert_eq!(input, "test task");
    }

    #[test]
    fn test_build_input_with_context() {
        let context = vec!["[web_search sonucu]:\nresult".to_string()];
        let input = build_input("test task", &context);
        assert!(input.contains("test task"));
        assert!(input.contains("Onceki arac sonuclari"));
        assert!(input.contains("web_search sonucu"));
        assert!(input.contains("Sadece tanimli araclari kullan"));
    }

    #[test]
    fn test_execute_tool_unknown() {
        let args = serde_json::json!({});
        let result = execute_tool("unknown_tool", &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Bilinmeyen"));
    }

    #[test]
    fn test_execute_tool_missing_param() {
        let args = serde_json::json!({});
        let result = execute_tool("web_search", &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_tool_list_dir_default() {
        let args = serde_json::json!({});
        let result = execute_tool("list_dir", &args);
        // Workspace erişilebilir olmalı (boş veya dolu)
        assert!(result.is_ok());
    }
}
