use crate::io_device::IODevice;

use super::cpu_6502::Registers;
use super::status_register::StatusRegister;

#[derive(Clone, Copy)]
pub enum InstructionState {
    Continue,
    Finished
}

pub trait Instruction {
    fn cycle(&mut self, step: usize, reg: &mut Registers, address_bus: &mut dyn IODevice) -> InstructionState;
}

// ADC
// absolute
// TODO: handle decimal mode
struct ADC0x6D {
    adh: u8,
    adl: u8,
}
impl ADC0x6D {
    fn new() -> Option<Box<dyn Instruction>> { Some(Box::new(Self { adh: 0, adl: 0 })) }
}
impl Instruction for ADC0x6D {
    fn cycle(&mut self, cycle: usize, reg: &mut Registers, address_bus: &mut dyn IODevice) -> InstructionState {
        match cycle {
            0 => {
                self.adl = reg.data;
                InstructionState::Continue
            },
            1 => {
                self.adh = reg.data;
                InstructionState::Continue
            },
            2 => {
                (reg.adh, reg.adl) = (self.adh, self.adl);
                reg.data = address_bus.get_hl(reg.adh, reg.adl);

                let total: u16 = reg.a as u16 + reg.data as u16 + if reg.status.flag(StatusRegister::CARRY) { 1 } else { 0 };
                reg.a = total as u8;

                if total > 255 {
                    reg.status.set_flag(StatusRegister::CARRY);
                } else {
                    reg.status.clear_flag(StatusRegister::CARRY);
                }

                if reg.a == 0 {
                    reg.status.set_flag(StatusRegister::ZERO);
                } else {
                    reg.status.clear_flag(StatusRegister::ZERO);
                }

                if reg.a & 0b10000000 == 0b10000000 {
                    reg.status.set_flag(StatusRegister::NEGATIVE);
                } else {
                    reg.status.clear_flag(StatusRegister::NEGATIVE);
                }

                if (reg.a & 0b10000000) != ((total as u8) & 0b10000000) {
                    reg.status.set_flag(StatusRegister::OVERFLOW);
                } else {
                    reg.status.clear_flag(StatusRegister::OVERFLOW);
                }

                reg.pc -= 1;
                InstructionState::Continue
            },
            3 => InstructionState::Finished,
            _ => panic!("unexpected cycle for ADC0x6D: {cycle}")
        }
    }
}


// CLC
// implied
struct CLC0x18;
impl CLC0x18 {
    fn new() -> Option<Box<dyn Instruction>> { Some(Box::new(Self {})) }
}
impl Instruction for CLC0x18 {
    fn cycle(&mut self, cycle: usize, reg: &mut Registers, _address_bus: &mut dyn IODevice) -> InstructionState {
        match cycle {
            0 => {
                reg.pc -= 1;
                InstructionState::Continue
            },
            1 => {
                reg.status.clear_flag(StatusRegister::CARRY);
                InstructionState::Finished
            },
            _ => panic!("unexpected cycle for CLC0x18: {cycle}")
        }
    }
}

// CLD
// implied
struct CLD0xD8;
impl CLD0xD8 {
    fn new() -> Option<Box<dyn Instruction>> { Some(Box::new(Self {})) }
}
impl Instruction for CLD0xD8 {
    fn cycle(&mut self, cycle: usize, reg: &mut Registers, _address_bus: &mut dyn IODevice) -> InstructionState {
        match cycle {
            0 => {
                reg.pc -= 1;
                InstructionState::Continue
            },
            1 => {
                reg.status.clear_flag(StatusRegister::DECIMAL_MODE);
                InstructionState::Finished
            },
            _ => panic!("unexpected cycle for CLD0xD8: {cycle}")
        }
    }
}

// LDA
// immediate
struct LDA0xA9 {
    data: u8,
}
impl LDA0xA9 {
    fn new() -> Option<Box<dyn Instruction>> { Some(Box::new(Self { data: 0 })) }
}
impl Instruction for LDA0xA9 {
    fn cycle(&mut self, cycle: usize, reg: &mut Registers, _address_bus: &mut dyn IODevice) -> InstructionState {
        match cycle {
            0 => {
                self.data = reg.data;
                InstructionState::Continue
            },
            1 => {
                reg.a = self.data;
                if reg.a == 0 {
                    reg.status.set_flag(StatusRegister::ZERO);
                } else {
                    reg.status.clear_flag(StatusRegister::ZERO);
                }

                if reg.a & 0b10000000 == 0b10000000 {
                    reg.status.set_flag(StatusRegister::NEGATIVE);
                } else {
                    reg.status.clear_flag(StatusRegister::NEGATIVE);
                }

                InstructionState::Finished
            },
            _ => panic!("unexpected cycle for LDA0xA9: {cycle}")
        }
    }
}

// absolute
struct LDA0xAD {
    adh: u8,
    adl: u8,
}
impl LDA0xAD {
    fn new() -> Option<Box<dyn Instruction>> { Some(Box::new(Self { adh: 0, adl: 0 })) }
}
impl Instruction for LDA0xAD {
    fn cycle(&mut self, cycle: usize, reg: &mut Registers, address_bus: &mut dyn IODevice) -> InstructionState {
        match cycle {
            0 => {
                self.adl = reg.data;
                InstructionState::Continue
            },
            1 => {
                self.adh = reg.data;
                InstructionState::Continue
            },
            2 => {
                (reg.adh, reg.adl) = (self.adh, self.adl);
                reg.data = address_bus.get_hl(reg.adh, reg.adl);
                reg.a = reg.data;

                if reg.a == 0 {
                    reg.status.set_flag(StatusRegister::ZERO);
                } else {
                    reg.status.clear_flag(StatusRegister::ZERO);
                }

                if reg.a & 0b10000000 == 0b10000000 {
                    reg.status.set_flag(StatusRegister::NEGATIVE);
                } else {
                    reg.status.clear_flag(StatusRegister::NEGATIVE);
                }

                reg.pc -= 1;
                InstructionState::Continue
            },
            3 => InstructionState::Finished,
            _ => panic!("unexpected cycle for LDA0xAD: {cycle}")
        }
    }
}

// STA
// absolute
struct STA0x8D {
    adh: u8,
    adl: u8,
}
impl STA0x8D {
    fn new() -> Option<Box<dyn Instruction>> { Some(Box::new(Self { adh: 0, adl: 0 })) }
}
impl Instruction for STA0x8D {
    fn cycle(&mut self, cycle: usize, reg: &mut Registers, address_bus: &mut dyn IODevice) -> InstructionState {
        match cycle {
            0 => {
                self.adl = reg.data;
                InstructionState::Continue
            },
            1 => {
                self.adh = reg.data;
                InstructionState::Continue
            },
            2 => {
                (reg.adh, reg.adl) = (self.adh, self.adl);
                reg.data = reg.a;
                address_bus.put_hl(reg.adh, reg.adl, reg.data);
                reg.pc -= 1;
                InstructionState::Continue
            },
            3 => InstructionState::Finished,
            _ => panic!("unexpected cycle for STA0x8D: {cycle}")
        }
    }
}

pub fn find_instruction(opcode: u8) -> Option<Box<dyn Instruction>> {
    match opcode {
        0x18 => CLC0x18::new(),
        0x60 => None, // RTS, not sure how to implment this yet, so just exit when we hit it
        0x6d => ADC0x6D::new(),
        0x8d => STA0x8D::new(),
        0xa9 => LDA0xA9::new(),
        0xad => LDA0xAD::new(),
        0xd8 => CLD0xD8::new(),
        _ => None
    }
}
