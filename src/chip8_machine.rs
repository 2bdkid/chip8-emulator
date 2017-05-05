use super::display;
use super::instructions;
use super::keyboard;
use super::registers;
use super::memory;

pub struct Chip8Machine {
    memory_bank: memory::Chip8Memory,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            memory_bank: memory::Chip8Memory::default(),
        }
    }

    pub fn run(&mut self) {
        self.memory_bank.write_value(0x0, 0x45);
        println!("{:#?}", self.memory_bank);
        println!("{:#x}", self.memory_bank.read_value(0x0));
    }
}
