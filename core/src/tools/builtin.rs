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

            let output = if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(["/C", command])
                    .output()
            } else {
                std::process::Command::new("sh")
                    .args(["-c", command])
                    .output()
            }
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
            let num_results = args["num_results"].as_i64().unwrap_or(5) as usize;

            let api_key = std::env::var("SERPAPI_API_KEY").ok();
            let use_tavily = std::env::var("TAVILY_API_KEY").ok();

            if let Some(key) = api_key {
                if let Ok(json) = search_serpapi(query, num_results, &key) {
                    return Ok(json);
                }
            }
            if let Some(key) = use_tavily {
                if let Ok(json) = search_tavily(query, num_results, &key) {
                    return Ok(json);
                }
            }

            if let Ok(json) = search_duckduckgo(query, num_results) {
                return Ok(json);
            }

            Ok(Value::String(
                format!("[Web search for '{}' failed - no search API available. Set SERPAPI_API_KEY or TAVILY_API_KEY environment variable]", query)
            ))
        }),
    }
}

fn search_duckduckgo(query: &str, num_results: usize) -> Result<Value, String> {
    let url = format!("https://lite.duckduckgo.com/lite/?q={}", urlencode(query));
    let body = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    let html = body.text().map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    for line in html.lines() {
        if line.contains("href=\"") && line.contains("</a>") {
            if let Some(href_start) = line.find("href=\"") {
                let href_start = href_start + 6;
                if let Some(href_end) = line[href_start..].find('"') {
                    let link = &line[href_start..href_start + href_end];
                    let text = line
                        .split("</a>")
                        .next()
                        .unwrap_or("")
                        .rsplit('>')
                        .next()
                        .unwrap_or("")
                        .to_string();
                    results.push(serde_json::json!({
                        "title": text,
                        "url": link
                    }));
                    if results.len() >= num_results {
                        break;
                    }
                }
            }
        }
    }
    Ok(serde_json::json!({
        "query": query,
        "results": results,
        "source": "duckduckgo"
    }))
}

fn search_serpapi(query: &str, num_results: usize, api_key: &str) -> Result<Value, String> {
    let url = format!(
        "https://serpapi.com/search?q={}&api_key={}&num={}",
        urlencode(query), api_key, num_results
    );
    let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    let json: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    Ok(json)
}

fn search_tavily(query: &str, num_results: usize, api_key: &str) -> Result<Value, String> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.tavily.com/search")
        .json(&serde_json::json!({
            "api_key": api_key,
            "query": query,
            "max_results": num_results
        }))
        .send()
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    Ok(json)
}

fn urlencode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
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

            let resp = reqwest::blocking::get(url)
                .map_err(|e| crate::NexusError::Tool(format!("Failed to fetch URL: {}", e)))?;

            let status = resp.status().as_u16();
            let content_type = resp
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown")
                .to_string();

            let body = resp.text().map_err(|e| {
                crate::NexusError::Tool(format!("Failed to read response body: {}", e))
            })?;

            let truncated = if body.len() > 50000 {
                format!("{}... [truncated from {} bytes]", &body[..50000], body.len())
            } else {
                body
            };

            Ok(serde_json::json!({
                "url": url,
                "status": status,
                "content_type": content_type,
                "content": truncated
            }))
        }),
    }
}
