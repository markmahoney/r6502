use crate::io_device::IODevice;

pub struct CPU6502 {
}

impl CPU6502 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick<T: IODevice>(&mut self, address_bus: &mut T) {
        address_bus.put(0x0000, address_bus.get(0x0000) + 1 % 255);
        println!("address at 0x0000: {}", address_bus.get(0x0000));
    }
}
