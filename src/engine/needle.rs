use needle_infer::v2_engine::V2Engine;
use serde_json::Value;

pub fn load(path: &str) -> V2Engine {
    V2Engine::load(path).expect("Needle modeli yuklenemedi.")
}

pub fn parse_tool_call(raw_output: &str) -> Option<Vec<Value>> {
    let start_tag = "<tool_call>";
    let end_tag = "</tool_call>";
    let start = raw_output.find(start_tag)? + start_tag.len();
    let end = raw_output.find(end_tag)?;
    let json_str = &raw_output[start..end];
    let parsed: Value = serde_json::from_str(json_str).ok()?;
    if let Value::Array(arr) = parsed {
        Some(arr)
    } else {
        Some(vec![parsed])
    }
}

pub fn extract_think(raw_output: &str) -> Option<String> {
    let start_tag = "<think>";
    let end_tag = "</think>";
    let start = raw_output.find(start_tag)? + start_tag.len();
    let end = raw_output.find(end_tag)?;
    Some(raw_output[start..end].trim().to_string())
}

pub fn ensure_model() -> String {
    let path = "weights/needle2.cact";

    if std::path::Path::new(path).exists() {
        println!("[DEBUG] Needle modeli mevcut: {}", path);
        return path.to_string();
    }

    println!("[DEBUG] Needle modeli Hugging Face'ten indiriliyor...");

    let api = hf_hub::api::sync::Api::new().expect("HF API baslatilamadi");
    let repo = api.model("Cactus-Compute/needle2".to_string());

    let downloaded_path = repo.get("needle2.cact").expect("needle2.cact indirilemedi");

    println!(
        "[DEBUG] Model HF cache'ine indirildi: {:?}",
        downloaded_path
    );

    std::fs::create_dir_all("weights").expect("weights klasoru olusturulamadi");
    std::fs::copy(&downloaded_path, path).expect("Model kopyalanamadi");

    println!("[DEBUG] Model kopyalandi: {}", path);
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tool_call() {
        let input =
            "<tool_call>[{\"name\":\"web_search\",\"arguments\":{\"query\":\"test\"}}]</tool_call>";
        let calls = parse_tool_call(input).unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0]["name"], "web_search");
    }

    #[test]
    fn test_extract_think() {
        let input = "<think>test thinking</think><tool_call>[]</tool_call>";
        let think = extract_think(input).unwrap();
        assert_eq!(think, "test thinking");
    }
}
