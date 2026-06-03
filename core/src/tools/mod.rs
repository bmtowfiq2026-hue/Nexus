pub mod builtin;

use crate::providers::ToolDefinition;
use crate::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub type ToolResult = Result<Value>;

#[derive(Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub handler: Arc<dyn Fn(Value) -> ToolResult + Send + Sync>,
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("parameters", &self.parameters)
            .finish()
    }
}

pub struct ToolDispatcher {
    tools: HashMap<String, Tool>,
}

impl ToolDispatcher {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Tool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    pub fn execute(&self, name: &str, args: Value) -> ToolResult {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| crate::NexusError::Tool(format!("Tool '{}' not found", name)))?;
        (tool.handler)(args)
    }

    pub fn get_definitions(&self) -> Vec<ToolDefinition> {
        self.tools
            .values()
            .map(|t| ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            })
            .collect()
    }

    pub fn register_builtins(&mut self) {
        self.register(builtin::read_file());
        self.register(builtin::write_file());
        self.register(builtin::exec_command());
        self.register(builtin::web_search());
        self.register(builtin::web_fetch());
    }
}

impl std::fmt::Debug for ToolDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolDispatcher")
            .field("tool_count", &self.tools.len())
            .finish()
    }
}
