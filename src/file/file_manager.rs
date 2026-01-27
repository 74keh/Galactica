use std::fs::{File, OpenOptions, create_dir_all, read_dir, remove_file};
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

use super::block_id::BlockId;
use super::page::Page;

pub struct FileManager {
    db_directory: String,
    block_size: usize,
    is_new: bool,
}

impl FileManager {
    pub fn new(db_directory: &str, block_size: usize) -> Self {
        let path = Path::new(db_directory);
        let is_new = !path.exists();

        if is_new {
            create_dir_all(db_directory).unwrap();
        }

        if let Ok(entries) = read_dir(db_directory) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("temp") {
                        let _ = remove_file(entry.path());
                    }
                }
            }
        }

        Self {
            db_directory: db_directory.to_string(),
            block_size,
            is_new,
        }
    }

    pub fn read(&self, block_id: &BlockId, page: &mut Page) -> Result<(), Error> {
        let mut file = self.get_file(block_id.file_name())?;
        file.seek(SeekFrom::Start(
            (block_id.number() as usize * self.block_size) as u64,
        ))?;
        file.read_exact(page.contents())?;
        Ok(())
    }

    pub fn write(&self, block_id: &BlockId, page: &mut Page) -> Result<(), Error> {
        let mut file = self.get_file(block_id.file_name())?;
        file.seek(SeekFrom::Start(
            (block_id.number() as usize * self.block_size) as u64,
        ))?;
        file.write_all(page.contents())?;
        file.sync_all()?;
        Ok(())
    }

    pub fn append(&self, filename: &str) -> Result<BlockId, Error> {
        let new_block_number = self.length(filename)?;
        let block_id = BlockId::new(filename, new_block_number);
        let bytes = vec![0u8; self.block_size];

        let mut file = self.get_file(block_id.file_name())?;
        file.seek(SeekFrom::Start(
            (block_id.number() as usize * self.block_size) as u64,
        ))?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        Ok(block_id)
    }

    pub fn length(&self, filename: &str) -> Result<i32, Error> {
        let file = self.get_file(filename)?;
        let len = file.metadata()?.len();
        Ok((len / self.block_size as u64) as i32)
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    fn get_file(&self, filename: &str) -> Result<File, Error> {
        let filename = Path::new(&self.db_directory).join(filename);
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&filename)
    }
}
