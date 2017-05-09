extern crate chip8_virtual_machine;

use chip8_virtual_machine::chip8_machine;

fn main() {
    let mut machine = chip8_machine::Chip8Machine::new();
    machine.run();
}
