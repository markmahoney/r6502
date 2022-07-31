use crate::io_device::IODevice;
use crate::cpu::status_register::StatusRegister;

use super::opcodes::{Instruction, InstructionState, find_instruction};

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: StatusRegister,

    pub pc: u16,
    pub stack: u8,

    // Technically bus-related but easier to store here
    pub adl: u8,
    pub adh: u8,
    pub data: u8,
}

pub struct CPU6502 {
    pub registers: Registers,
    pub instruction: Option<Box<dyn Instruction>>,
    pub cycle: usize,
}

impl CPU6502 {
    pub fn new() -> Self {
        Self {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                status: StatusRegister::new(0b00000000),
                pc: 0,
                stack: 0,
                adl: 0,
                adh: 0,
                data: 0,
            },
            instruction: None,
            cycle: 0,
        }
    }

    pub fn update_buses<T: IODevice>(&mut self, address_bus: &mut T) {
        (self.registers.adh, self.registers.adl) = (((self.registers.pc & 0xFF00) >> 8) as u8, (self.registers.pc & 0x00FF) as u8);
        self.registers.data = address_bus.get_hl(self.registers.adh, self.registers.adl);
    }

    pub fn next_instruction(&mut self) {
        self.instruction = find_instruction(self.registers.data);
        self.cycle = 0;
    }

    pub fn tick<T: IODevice>(&mut self, address_bus: &mut T) -> bool {
        self.update_buses(address_bus);
        self.registers.pc += 1;

        match &mut self.instruction {
            Some(instruction) => {
                match instruction.cycle(self.cycle, &mut self.registers, address_bus) {
                    InstructionState::Continue => self.cycle += 1,
                    InstructionState::Finished => self.next_instruction()
                }
            },
            _ => {
                self.next_instruction()
            }
        };

        self.instruction.is_some()
    }
}

