type NNN = u16; // only use lowest 12 bits
type N = u8;    // only use lowest 4 bits
type X = u8;    // only use lowest 4 bits
type Y = u8;    // only use lowest 4 bits
type KK = u8;

pub enum Instruction {
    SYS(NNN),
}

pub fn decode_instruction(instruction: u16) -> Instruction {
    let split_bits = (((instruction >> 12) & 0b1111) as u8,
                               ((instruction >> 8) & 0b1111) as u8,
                               ((instruction >> 4) & 0b1111) as u8,
                               (instruction & 0b1111) as u8);

    match split_bits {
        (0x0, bits_one, bits_two, bits_three) => {
            let address = ((((bits_one as u16) << 4) | bits_two) << 4) | bits_three;
            Instruction::SYS(address)
        }
    }
}
