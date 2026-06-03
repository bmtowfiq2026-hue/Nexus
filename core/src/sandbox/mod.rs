use crate::Result;

pub struct Sandbox;

impl Sandbox {
    pub fn new(_config: &crate::SandboxConfig) -> Self {
        Self
    }

    pub fn execute(&self, _code: &str, _language: &str) -> Result<String> {
        Ok("[Sandbox execution - placeholder]".to_string())
    }

    pub fn execute_wasm(&self, _bytes: &[u8], _input: &[u8]) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
