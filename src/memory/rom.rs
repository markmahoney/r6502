use crate::io_device::IODevice;
use crate::memory::{hl_to_addr, Memory};

pub struct ROM<const N: usize> {
    memory: Memory<N>
}

impl <const N: usize> ROM<N> {
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

impl <const N: usize> IODevice for ROM<N> {
    fn get(&self, addr: u16) -> u8 {
        self.memory.contents[addr as usize]
    }

    fn get_hl(&self, high: u8, low: u8) -> u8 {
        self.memory.contents[hl_to_addr(high, low) as usize]

    }

    fn put(&mut self, _addr: u16, _value: u8) {}

    fn put_hl(&mut self, _high: u8, _low: u8, _value: u8) {}
}

#[cfg(test)]
mod tests {
    use crate::io_device::IODevice;
    use crate::memory::rom::ROM;

    #[test]
    fn new() {
        let rom = ROM::<1>::new(None);
        assert_eq!(rom.get(0), 0);
    }

    #[test]
    fn new_allocated() {
        let rom = ROM::new(Some([255; 0x10000]));
        assert_eq!(rom.get(0), 255);
    }

    #[test]
    fn get() {
        let rom = ROM::new(Some([255; 0x10000]));
        assert_eq!(rom.get(0), 255);
        assert_eq!(rom.get(0xFFFF), 255);        
    }

    #[test]
    fn put() {
        let mut rom = ROM::<0x10000>::new(None);
        rom.put(0, 255);
        assert_eq!(rom.get(0), 0);
        rom.put(0xFFFF, 255);
        assert_eq!(rom.get(0xFFFF), 0);
    }

    #[test]
    fn get_hl() {
        let rom = ROM::<0x10000>::new(Some([255; 0x10000]));
        assert_eq!(rom.get_hl(0x00, 0x00), 255);
        assert_eq!(rom.get_hl(0x00, 0xFF), 255);
        assert_eq!(rom.get_hl(0x01, 0x00), 255);
        assert_eq!(rom.get_hl(0xFF, 0xFF), 255);
    }

    #[test]
    fn put_hl() {
        let mut rom = ROM::<0x10000>::new(None);
        rom.put_hl(0x00, 0x00, 255);
        assert_eq!(rom.get(0x00), 0);
        rom.put_hl(0x00, 0xFF, 255);
        assert_eq!(rom.get(0xFF), 0);
        rom.put_hl(0x01, 0x00, 255);
        assert_eq!(rom.get(0x0100), 0);
        rom.put_hl(0xFF, 0xFF, 255);
        assert_eq!(rom.get(0xFFFF), 0);
    }
}
