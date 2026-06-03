use crate::Result;

pub struct FtsEngine;

impl FtsEngine {
    pub fn new(_path: &str) -> Self {
        Self
    }

    pub fn index(&self, _doc_id: &str, _content: &str) -> Result<()> {
        Ok(())
    }

    pub fn search(&self, _query: &str, _limit: usize) -> Result<Vec<String>> {
        Ok(Vec::new())
    }
}
