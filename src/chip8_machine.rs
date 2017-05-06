use super::display;
use super::instructions;
use super::keyboard;
use super::registers;
use super::memory;
use super::stack;

pub struct Chip8Machine {
    memory_bank: memory::Chip8Memory,
    registers: registers::Chip8Registers,
    stack: stack::Chip8Stack,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            memory_bank: memory::Chip8Memory::default(),
            registers: registers::Chip8Registers::default(),
            stack: stack::Chip8Stack::default(),
        }
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
        self.registers.sp = self.stack.top_address();
    }

    pub fn pop_stack(&mut self) -> u16 {
        let popped_value = self.stack.pop();
        self.registers.sp = self.stack.top_address();
        popped_value
    }

    pub fn run(&mut self) {
        println!("{:#?}", self.registers);
    }
}
