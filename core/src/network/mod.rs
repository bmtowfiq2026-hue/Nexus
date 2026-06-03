use crate::Result;

pub struct NetworkNode;

impl NetworkNode {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn send_message(&self, _peer: &str, _data: &[u8]) -> Result<()> {
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
