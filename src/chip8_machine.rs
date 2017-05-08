use super::display;
use super::instructions;
use super::keyboard;
use super::registers;
use super::memory;

pub struct Chip8Machine {
    memory_bank: memory::Chip8Memory,
    registers: registers::Chip8Registers,
    keyboard: keyboard::Keyboard,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            memory_bank: memory::Chip8Memory::default(),
            registers: registers::Chip8Registers::default(),
            keyboard: keyboard::Chip8Keyboard::default(),
        }
    }

    pub fn run(&mut self) {
        self.registers.push_stack(5);
        println!("{:#?}", self.registers);
    }
}
