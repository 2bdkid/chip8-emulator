use super::display;
use super::instructions;
use super::keyboard;
use super::registers;
use super::memory;

pub struct Chip8Machine {
    memory_bank: memory::Chip8Memory,
    registers: registers::Chip8Registers,
    keyboard: keyboard::Chip8Keyboard,
    display: display::Chip8Display,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            memory_bank: memory::Chip8Memory::default(),
            registers: registers::Chip8Registers::default(),
            keyboard: keyboard::Chip8Keyboard::default(),
            display: display::Chip8Display::default(),
        }
    }

    pub fn run(&mut self) {
        self.display.flip_pixel(63, 31);
        println!("{:?}", self.display);
    }
}
