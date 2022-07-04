use crate::io_device::IODevice;
use crate::memory;
use std::ops::RangeInclusive;

struct DeviceMapping {
    range: RangeInclusive<u16>,
    device: Box<dyn IODevice>,
}

pub struct AddressDecoder {
    ranges: Vec<DeviceMapping>,
}

impl AddressDecoder {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_device(&mut self, range: RangeInclusive<u16>, device: Box<dyn IODevice>) {
        self.ranges.push(DeviceMapping { range, device })
    }

    fn get_device(&self, addr: u16) -> Option<&DeviceMapping> {
        self.ranges.iter().find(|dm| dm.range.contains(&addr))
    }

    fn get_device_mut(&mut self, addr: u16) -> Option<&mut DeviceMapping> {
        self.ranges.iter_mut().find(|dm| dm.range.contains(&addr))
    }
}

impl IODevice for AddressDecoder {
    fn get(&self, addr: u16) -> u8 {
        match self.get_device(addr) {
            Some(dm) => {
                let offset = addr - dm.range.start();
                dm.device.get(offset)
            }
            None => 0
        }
    }

    fn get_hl(&self, high: u8, low: u8) -> u8 {
        self.get(memory::hl_to_addr(high, low))
    }

    fn put(&mut self, addr: u16, value: u8) {
        match self.get_device_mut(addr) {
            Some(dm) => {
                let offset = addr - dm.range.start();
                dm.device.put(offset, value);
            }
            None => ()
        }
    }

    fn put_hl(&mut self, high: u8, low: u8, value: u8) {
        self.put(memory::hl_to_addr(high, low), value);
    }
}

#[cfg(test)]
mod tests {
    use crate::address_decoder::AddressDecoder;
    use crate::io_device::IODevice;
    use crate::memory::ram::RAM;

    #[test]
    fn new() {
        let decoder = AddressDecoder::new();
        assert_eq!(decoder.get(0x0000), 0);
    }

    #[test]
    fn add_device() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..=0x3FFF,
            Box::new(RAM::<0x2000>::new(None)),
        );
        assert_eq!(decoder.get(0x2000), 0);
    }

    #[test]
    fn get() {
        let mut decoder = AddressDecoder::new();
        decoder.add_device(
            0x2000..=0x3FFF,
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
            0x2000..=0x3FFF,
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
            0x2000..=0x3FFF,
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
            0x2000..=0x3FFF,
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
