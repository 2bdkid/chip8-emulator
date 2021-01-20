use chip8_virtual_machine::Machine;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    //let mut instructions: Vec<u16> = Vec::new();

    let program_path = env::args_os()
        .nth(1)
        .expect("Please specify program binary");
    let mut program_file = File::open(program_path).expect("File not found");

    let mut program_data = Vec::new();
    program_file
        .read_to_end(&mut program_data)
        .expect("Could not read file");

    // Because Rust only lets me read files in as u8's,
    // I take 2 at a time and concatenate them into a u16
    /*
    for instruction_pair in program_data.chunks(2) {
        let first = instruction_pair[0] as u16;
        let second = instruction_pair[1] as u16;

        let mut instruction = 0u16;
        instruction |= first << 8;
        instruction |= second;

        instructions.push(instruction);
    }
    */

    let mut machine = Chip8Machine::new();
    machine.load_memory(&program_data);
    machine.run();
}
