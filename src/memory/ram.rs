use crate::io_device::IODevice;
use crate::memory::{hl_to_addr, Memory};

pub struct RAM<const N: usize> {
    memory: Memory<N>
}

impl <const N: usize> RAM<N> {
    pub fn new(memory: Option<[u8; N]>) -> Self {
        Self { memory: Memory::<N>::new(memory) }
    }

    pub fn new_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Memory::new_from_file(filename).map(|mem| Self { memory: mem })
    }

    pub fn set_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        self.memory.set_from_file(filename)
    }
}

impl <const N: usize> IODevice for RAM<N> {
    fn get(&self, addr: u16) -> u8 {
        self.memory.contents[addr as usize]
    }

    fn get_hl(&self, high: u8, low: u8) -> u8 {
        self.memory.contents[hl_to_addr(high, low) as usize]

    }

    fn put(&mut self, addr: u16, value: u8) {
        self.memory.contents[addr as usize] = value;
    }

    fn put_hl(&mut self, high: u8, low: u8, value: u8) {
        self.memory.contents[hl_to_addr(high, low) as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::io_device::IODevice;
    use crate::memory::ram::RAM;

    #[test]
    fn new() {
        let ram = RAM::<1>::new(None);
        assert_eq!(ram.get(0), 0);
    }

    #[test]
    fn new_allocated() {
        let ram = RAM::new(Some([255; 0x10000]));
        assert_eq!(ram.get(0), 255);
    }

    #[test]
    fn get() {
        let ram = RAM::new(Some([255; 0x10000]));
        assert_eq!(ram.get(0), 255);
        assert_eq!(ram.get(0xFFFF), 255);        
    }

    #[test]
    fn put() {
        let mut ram = RAM::<0x10000>::new(None);
        ram.put(0, 255);
        assert_eq!(ram.get(0), 255);
        ram.put(0xFFFF, 255);
        assert_eq!(ram.get(0xFFFF), 255);
    }

    #[test]
    fn get_hl() {
        let mut ram = RAM::<0x10000>::new(None);
        ram.put(0x0000, 255);
        assert_eq!(ram.get_hl(0x00, 0x00), 255);
        ram.put(0x00FF, 255);
        assert_eq!(ram.get_hl(0x00, 0xFF), 255);
        ram.put(0x0100, 255);
        assert_eq!(ram.get_hl(0x01, 0x00), 255);
        ram.put(0xFFFF, 255);
        assert_eq!(ram.get_hl(0xFF, 0xFF), 255);
    }

    #[test]
    fn put_hl() {
        let mut ram = RAM::<0x10000>::new(None);
        ram.put_hl(0x00, 0x00, 255);
        assert_eq!(ram.get(0x00), 255);
        ram.put_hl(0x00, 0xFF, 255);
        assert_eq!(ram.get(0xFF), 255);
        ram.put_hl(0x01, 0x00, 255);
        assert_eq!(ram.get(0x0100), 255);
        ram.put_hl(0xFF, 0xFF, 255);
        assert_eq!(ram.get(0xFFFF), 255);
    }
}
