pub mod audit;
pub mod file_ops;
pub mod reader;
pub mod sandbox;
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
},
{
    "name": "read_url",
    "description": "Read the content of a URL and return its cleaned text.",
    "parameters": {
        "type": "object",
        "properties": {
            "url": {"type": "string", "description": "The URL to read"}
        },
        "required": ["url"]
    }
},
{
    "name": "read_file",
    "description": "Read a file from the local workspace directory. Only files inside ./workspace/ are accessible.",
    "parameters": {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Relative path inside workspace/"}
        },
        "required": ["path"]
    }
},
{
    "name": "write_file",
    "description": "Write content to a file in the local workspace directory. Only files inside ./workspace/ are accessible.",
    "parameters": {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Relative path inside workspace/"},
            "content": {"type": "string", "description": "Content to write"}
        },
        "required": ["path", "content"]
    }
},
{
    "name": "list_dir",
    "description": "List files and directories inside the workspace. Use empty path or '.' for the workspace root.",
    "parameters": {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Relative path inside workspace/"}
        },
        "required": []
    }
}]"#;
