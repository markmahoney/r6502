use crate::io_device::IODevice;
use std::ops::Range;

struct DeviceMapping {
    range: Range<u16>,
    device: Box<dyn IODevice>,
}

pub struct AddressDecoder {
    ranges: Vec<DeviceMapping>,
}

fn hl_to_addr(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}
    
impl AddressDecoder {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_device(&mut self, range: Range<u16>, device: Box<dyn IODevice>) {
        self.ranges.push(DeviceMapping { range, device })
    }

    fn get_device(&mut self, addr: u16) -> Option<&mut DeviceMapping> {
        self.ranges.iter_mut().find(|dm| dm.range.contains(&addr))
    }

    pub fn get(&mut self, addr: u16) -> u8 {
        match self.get_device(addr) {
            Some(dm) => {
                let offset = addr - dm.range.start;
                dm.device.get(offset)
            }
            None => 0
        }
    }

    pub fn get_hl(&mut self, high: u8, low: u8) -> u8 {
        self.get(hl_to_addr(high, low))
    }

    pub fn put(&mut self, addr: u16, value: u8) {
        match self.get_device(addr) {
            Some(dm) => {
                let offset = addr - dm.range.start;
                dm.device.put(offset, value);
            }
            None => ()
        }
    }

    pub fn put_hl(&mut self, high: u8, low: u8, value: u8) {
        self.put(hl_to_addr(high, low), value);
    }
}

#[cfg(test)]
mod tests {
    use crate::address_decoder::AddressDecoder;
    use crate::RAM;

    #[test]
    fn new() {
        let mut decoder = AddressDecoder::new();
        assert_eq!(decoder.get(0x0000), 0);
    }

    #[test]
    fn add_device() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..0x4000,
            Box::new(RAM::<0x2000>::new(None)),
        );
        assert_eq!(decoder.get(0x2000), 0);
    }

    #[test]
    fn get() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..0x4000,
            Box::new(RAM::<0x2000>::new(Some([255; 0x2000]))),
        );
        assert_eq!(decoder.get(0x0000), 0);
        assert_eq!(decoder.get(0x2000), 255);
        assert_eq!(decoder.get(0x3FFF), 255);
        assert_eq!(decoder.get(0x4000), 0);
    }

    #[test]
    fn put() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..0x4000,
            Box::new(RAM::<0x2000>::new(None)),
        );
        decoder.put(0x0000, 255);
        assert_eq!(decoder.get(0x0000), 0);
        decoder.put(0x2000, 255);
        assert_eq!(decoder.get(0x2000), 255);
        decoder.put(0x3FFF, 255);
        assert_eq!(decoder.get(0x3FFF), 255);
        decoder.put(0x4000, 255);
        assert_eq!(decoder.get(0x4000), 0);
    }

    #[test]
    fn get_hl() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..0x4000,
            Box::new(RAM::<0x2000>::new(None)),
        );
        decoder.put(0x0000, 255);
        assert_eq!(decoder.get_hl(0x00, 0x00), 0);
        decoder.put(0x2000, 255);
        assert_eq!(decoder.get_hl(0x20, 0x00), 255);
        decoder.put(0x3FFF, 255);
        assert_eq!(decoder.get_hl(0x3F, 0xFF), 255);
        decoder.put(0x4000, 255);
        assert_eq!(decoder.get_hl(0x40, 0x00), 0);
    }

    #[test]
    fn put_hl() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..0x4000,
            Box::new(RAM::<0x2000>::new(None)),
        );
        decoder.put_hl(0x00, 0x00, 255);
        assert_eq!(decoder.get(0x0000), 0);
        decoder.put_hl(0x20, 0x00, 255);
        assert_eq!(decoder.get(0x2000), 255);
        decoder.put_hl(0x3F, 0xFF, 255);
        assert_eq!(decoder.get(0x3FFF), 255);
        decoder.put_hl(0x40, 0x00, 255);
        assert_eq!(decoder.get(0x4000), 0);
    }
}
