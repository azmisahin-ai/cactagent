use needle_infer::v2_engine::V2Engine;
use serde_json::Value;
use candelabra::{
    download_model,
    load_tokenizer_from_repo,
    Model,
    InferenceConfig,
    run_inference,
};
use std::sync::{Arc, atomic::AtomicBool};

pub fn load_engine(path: &str) -> V2Engine {
    V2Engine::load(path).expect("Model yuklenemedi. weights/needle2.cact dosyasini kontrol et.")
}

pub fn parse_tool_call(raw_output: &str) -> Option<Vec<Value>> {
    let start_tag = "<tool_call>";
    let end_tag = "</tool_call>";
    let start = raw_output.find(start_tag)? + start_tag.len();
    let end = raw_output.find(end_tag)?;
    let json_str = &raw_output[start..end];
    let parsed: Value = serde_json::from_str(json_str).ok()?;
    if let Value::Array(arr) = parsed { Some(arr) } else { Some(vec![parsed]) }
}

pub fn extract_think(raw_output: &str) -> Option<String> {
    let start_tag = "<think>";
    let end_tag = "</think>";
    let start = raw_output.find(start_tag)? + start_tag.len();
    let end = raw_output.find(end_tag)?;
    Some(raw_output[start..end].trim().to_string())
}

pub fn summarize_with_qwen(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[DEBUG] Model indiriliyor...");
    let model_path = download_model(
        "Qwen/Qwen2.5-0.5B-Instruct-GGUF",
        "qwen2.5-0.5b-instruct-q4_k_m.gguf",
    )?;
    println!("[DEBUG] Model yolu: {:?}", model_path);

    println!("[DEBUG] Tokenizer indiriliyor...");
    let tokenizer = load_tokenizer_from_repo("Qwen/Qwen2.5-0.5B-Instruct")?;

    println!("[DEBUG] Model yukleniyor...");
    let mut model = Model::load(&model_path)?;

    let cancel_token = Arc::new(AtomicBool::new(false));

    let prompt = format!(
        "<|im_start|>system\nYou are a helpful assistant. Summarize the user's text in exactly 3 short sentences.<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
        content
    );

    let mut config = InferenceConfig::default();
    config.prompt = prompt;
    config.max_tokens = 150;
    config.temperature = 0.3;

    println!("[DEBUG] Cikarim basliyor...");
    let mut output = String::new();
    let result = run_inference(
        &mut model,
        &tokenizer,
        &config,
        cancel_token,
        |token: String| {
            output.push_str(&token);
            print!("{}", token);
            use std::io::Write;
            std::io::stdout().flush().ok();
            Ok(())
        },
    )?;

    println!("\n[DEBUG] Qwen cikarim hizi: {:.2} token/s", result.tokens_per_second);

    Ok(output.trim().to_string())
}

pub fn ensure_needle_model() -> String {
    let path = "weights/needle2.cact";
    
    if std::path::Path::new(path).exists() {
        println!("[DEBUG] Needle modeli mevcut: {}", path);
        return path.to_string();
    }
    
    println!("[DEBUG] Needle modeli Hugging Face'ten indiriliyor...");
    
    let api = hf_hub::api::sync::Api::new().expect("HF API baslatilamadi");
    let repo = api.model("Cactus-Compute/needle2".to_string());
    
    let downloaded_path = repo
        .get("needle2.cact")
        .expect("needle2.cact indirilemedi");
    
    println!("[DEBUG] Model HF cache'ine indirildi: {:?}", downloaded_path);
    
    std::fs::create_dir_all("weights").expect("weights klasoru olusturulamadi");
    std::fs::copy(&downloaded_path, path).expect("Model kopyalanamadi");
    
    println!("[DEBUG] Model kopyalandi: {}", path);
    path.to_string()
}