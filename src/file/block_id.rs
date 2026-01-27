#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockId {
    file_name: String,
    block_number: i32,
}

impl BlockId {
    pub fn new(file_name: &str, block_number: i32) -> Self {
        Self {
            file_name: file_name.to_string(),
            block_number,
        }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn number(&self) -> i32 {
        self.block_number
    }
}
