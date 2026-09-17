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
    fn test_parse_tool_call_single() {
        let input =
            r#"<tool_call>[{"name":"web_search","arguments":{"query":"test"}}]</tool_call>"#;
        let calls = parse_tool_call(input).unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0]["name"], "web_search");
        assert_eq!(calls[0]["arguments"]["query"], "test");
    }

    #[test]
    fn test_parse_tool_call_empty() {
        let input = "<tool_call>[]</tool_call>";
        let calls = parse_tool_call(input).unwrap();
        assert_eq!(calls.len(), 0);
    }

    #[test]
    fn test_parse_tool_call_no_tags() {
        let input = "just some text without tool calls";
        assert!(parse_tool_call(input).is_none());
    }

    #[test]
    fn test_parse_tool_call_multiple() {
        let input = r#"<tool_call>[{"name":"web_search","arguments":{"query":"a"}},{"name":"read_url","arguments":{"url":"https://example.com"}}]</tool_call>"#;
        let calls = parse_tool_call(input).unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0]["name"], "web_search");
        assert_eq!(calls[1]["name"], "read_url");
    }

    #[test]
    fn test_extract_think() {
        let input = "<think>test thinking</think><tool_call>[]</tool_call>";
        let think = extract_think(input).unwrap();
        assert_eq!(think, "test thinking");
    }

    #[test]
    fn test_extract_think_none() {
        let input = "<tool_call>[]</tool_call>";
        assert!(extract_think(input).is_none());
    }

    #[test]
    fn test_extract_think_multiline() {
        let input = "<think>\nline 1\nline 2\n</think>";
        let think = extract_think(input).unwrap();
        assert_eq!(think, "line 1\nline 2");
    }
}
