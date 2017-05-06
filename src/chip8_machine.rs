use super::display;
use super::instructions;
use super::keyboard;
use super::registers;
use super::memory;

use super::registers::Register::*;

pub struct Chip8Machine {
    memory_bank: memory::Chip8Memory,
    registers: registers::Chip8Registers,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            memory_bank: memory::Chip8Memory::default(),
            registers: registers::Chip8Registers::default(),
        }
    }

    pub fn run(&mut self) {
        self.registers.write_u8(V0, 1);
        println!("{:#?}", self.registers);
    }
}
