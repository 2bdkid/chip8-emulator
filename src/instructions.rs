pub enum Instruction {
    SYS(u16),
}

pub fn decode_instruction(instruction: u16) -> Instruction {
    let split_bits = (((instruction >> 12) & 0b1111),
                      ((instruction >> 8) & 0b1111),
                      ((instruction >> 4) & 0b1111),
                       (instruction & 0b1111));

    match split_bits {
        (0x0, bits_one, bits_two, bits_three) => {
            let address = ((((bits_one) << 4) | bits_two) << 4) | bits_three;
            Instruction::SYS(address)
        },
        (_, _, _, _) => panic!("Invalid instruction: {:#x}{:#x}{:#x}{:#x}",
                               split_bits.0, split_bits.1, split_bits.2, split_bits.3),
    }
}
