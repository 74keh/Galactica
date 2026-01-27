use crate::file::file_manager;

pub struct Galactica {
    file_manager: file_manager::FileManager,
}

impl Galactica {
    pub fn new(directory_name: &str, block_size: usize, _buffer_size: u64) -> Self {
        Self {
            file_manager: file_manager::FileManager::new(directory_name, block_size),
        }
    }

    pub fn file_manager(&self) -> &file_manager::FileManager {
        &self.file_manager
    }
}
