pub struct Memory {
    contents: Block64,
}

type Block64 = [u8; 65536];

fn hl_to_addr(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

impl Memory {
    pub fn new(contents: Option<Block64>) -> Self {
        match contents {
            Some(contents) => Self { contents },
            None => Self { contents: [0; 65536] },
        }
    }
    
    pub fn get(&self, addr: u16) -> u8 {
        self.contents[addr as usize]
    }

    pub fn get_hl(&self, high: u8, low: u8) -> u8 {
        self.contents[hl_to_addr(high, low) as usize]
    }

    pub fn put(&mut self, addr: u16, value: u8) {
        self.contents[addr as usize] = value;
    }

    pub fn put_hl(&mut self, high: u8, low: u8, value: u8) {
        self.contents[hl_to_addr(high, low) as usize] = value;
    }
}
