pub mod reader;
pub mod search;

pub const TOOLS_JSON: &str = r#"[{
    "name": "web_search",
    "description": "Search the web for current information.",
    "parameters": {
        "type": "object",
        "properties": {
            "query": {"type": "string", "description": "The search query"}
        },
        "required": ["query"]
    }
}]"#;