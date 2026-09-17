use candelabra::{
    download_model, load_tokenizer_from_repo, run_inference, InferenceConfig, Model,
};
use std::sync::{atomic::AtomicBool, Arc};

pub fn summarize(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[DEBUG] Qwen modeli indiriliyor...");
    let model_path = download_model(
        "Qwen/Qwen2.5-0.5B-Instruct-GGUF",
        "qwen2.5-0.5b-instruct-q4_k_m.gguf",
    )?;

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

    println!(
        "\n[DEBUG] Qwen cikarim hizi: {:.2} token/s",
        result.tokens_per_second
    );

    Ok(output.trim().to_string())
}