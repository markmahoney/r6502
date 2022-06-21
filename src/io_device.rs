pub trait IODevice {
    fn get(&self, addr: u16) -> u8;
    fn get_hl(&self, high: u8, low: u8) -> u8;
    fn put(&mut self, addr: u16, value: u8);
    fn put_hl(&mut self, high: u8, low: u8, value: u8);
}
