pub mod ram;
pub mod rom;

use std::fs;

struct Memory<const N: usize> {
    pub contents: [u8; N],
}

impl <const N: usize> Memory<N> {
    pub fn new(contents: Option<[u8; N]>) -> Self {
        match contents {
            Some(contents) => Self { contents },
            None => Self { contents: [0; N] },
        }
    }

    /// Create a new Memory buffer with the contents set to that of the specified file. Note that the file contents will be truncated if they
    /// exceed the length of the contents buffer.
    pub fn new_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let mut memory = Self::new(None);
        memory.set_from_file(filename).map(|_| memory)
    }

    /// Update the contents buffer from a file. Note that the file contents will be truncated if they exceed the length of the content buffer.
    pub fn set_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let contents = fs::read(filename)?;

        // TODO: is there a way to do this in chunks, or with something like memcpy?
        for i in 0..self.contents.len() {
            self.contents[i] = contents[i];
        }
        
        Ok(())
    }
}

/// Convert an address represented by seperate high and low u8 values into a u16 value.
pub fn hl_to_addr(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}
