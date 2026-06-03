use super::Tool;
use serde_json::Value;
use std::sync::Arc;

pub fn read_file() -> Tool {
    Tool {
        name: "read".to_string(),
        description: "Read the contents of a file at the given path".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the file to read"
                }
            },
            "required": ["path"]
        }),
        handler: Arc::new(|args: Value| {
            let path = args["path"].as_str().ok_or_else(|| {
                crate::NexusError::Tool("Missing 'path' argument".to_string())
            })?;
            let content = std::fs::read_to_string(path)
                .map_err(|e| crate::NexusError::Io(e))?;
            Ok(Value::String(content))
        }),
    }
}

pub fn write_file() -> Tool {
    Tool {
        name: "write".to_string(),
        description: "Write content to a file at the given path".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["path", "content"]
        }),
        handler: Arc::new(|args: Value| {
            let path = args["path"].as_str().ok_or_else(|| {
                crate::NexusError::Tool("Missing 'path' argument".to_string())
            })?;
            let content = args["content"].as_str().ok_or_else(|| {
                crate::NexusError::Tool("Missing 'content' argument".to_string())
            })?;
            std::fs::write(path, content)
                .map_err(|e| crate::NexusError::Io(e))?;
            Ok(Value::String(format!("Successfully wrote {} bytes to {}", content.len(), path)))
        }),
    }
}

pub fn exec_command() -> Tool {
    Tool {
        name: "exec".to_string(),
        description: "Execute a shell command and return its output".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                },
                "workdir": {
                    "type": "string",
                    "description": "Working directory for the command (optional)"
                }
            },
            "required": ["command"]
        }),
        handler: Arc::new(|args: Value| {
            let command = args["command"].as_str().ok_or_else(|| {
                crate::NexusError::Tool("Missing 'command' argument".to_string())
            })?;
            let output = std::process::Command::new("cmd")
                .args(["/C", command])
                .output()
                .map_err(|e| crate::NexusError::Io(e))?;

            let result = serde_json::json!({
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "exit_code": output.status.code().unwrap_or(-1)
            });
            Ok(result)
        }),
    }
}

pub fn web_search() -> Tool {
    Tool {
        name: "web_search".to_string(),
        description: "Search the web for information using a query".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                },
                "num_results": {
                    "type": "integer",
                    "description": "Number of results to return (default 5)"
                }
            },
            "required": ["query"]
        }),
        handler: Arc::new(|args: Value| {
            let query = args["query"].as_str().ok_or_else(|| {
                crate::NexusError::Tool("Missing 'query' argument".to_string())
            })?;
            Ok(Value::String(format!("[Web search for '{}' - requires API key configuration]", query)))
        }),
    }
}

pub fn web_fetch() -> Tool {
    Tool {
        name: "web_fetch".to_string(),
        description: "Fetch the contents of a URL".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to fetch"
                }
            },
            "required": ["url"]
        }),
        handler: Arc::new(|args: Value| {
            let url = args["url"].as_str().ok_or_else(|| {
                crate::NexusError::Tool("Missing 'url' argument".to_string())
            })?;
            Ok(Value::String(format!("[Fetch '{}' - async operation requires tokio runtime]", url)))
        }),
    }
}
