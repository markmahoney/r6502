pub struct StatusRegister {
    pub flags: u8,
}

impl StatusRegister {
    pub const CARRY: u8 =        0b00000001;
    pub const ZERO: u8 =         0b00000010;
    pub const IRQ_DISABLE: u8 =  0b00000100;
    pub const DECIMAL_MODE: u8 = 0b00001000;
    pub const BRK_COMMAND: u8 =  0b00010000;
    pub const OVERFLOW: u8 =     0b01000000;
    pub const NEGATIVE: u8 =     0b10000000;

    pub fn new(flags: u8) -> Self {
        Self { flags }
    }

    pub fn flag(&self, flag: u8) -> bool {
        self.flags & flag == flag
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }

    pub fn clear_flag(&mut self, flag: u8) {
        self.flags &= flag ^ 0b11111111;
    }
}
