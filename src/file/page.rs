use std::string::FromUtf8Error;

pub struct Page {
    byte_buffer: Vec<u8>,
}

impl Page {
    pub fn new(block_size: usize) -> Self {
        Page { byte_buffer: vec![0; block_size] }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Page { byte_buffer: bytes }
    }

    pub fn read_i32(&self, offset: usize) -> i32 {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&self.byte_buffer[offset..offset + 4]);
        i32::from_be_bytes(buf)
    }

    pub fn write_i32(&mut self, offset: usize, value: i32) {
        self.byte_buffer[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }

    pub fn read_bytes(&self, offset: usize) -> &[u8] {
        let len = self.read_i32(offset);
        &self.byte_buffer[offset + 4..offset + 4 + len as usize]
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        self.byte_buffer[offset..offset + 4].copy_from_slice(&(bytes.len() as i32).to_be_bytes());
        self.byte_buffer[offset + 4..offset + 4 + bytes.len()].copy_from_slice(bytes);
    }

    pub fn read_string(&self, offset: usize) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.read_bytes(offset).to_vec())
    }

    pub fn write_string(&mut self, offset: usize, str: &str) {
        self.write_bytes(offset, str.as_bytes());
    }

    pub fn max_length(len: usize) -> usize {
        let bytes_per_char = 1;
        4 + len * bytes_per_char
    }

    pub(in crate::file) fn contents(&mut self) -> &mut Vec<u8> {
        &mut self.byte_buffer
    }
}
