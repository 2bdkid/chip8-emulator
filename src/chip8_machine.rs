use display;
use instructions;
use keyboard;
use registers;
use memory;

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
        self.memory_bank.write_value(0x0, 69);
        println!("{:#?}", self.memory_bank);
    }
}
