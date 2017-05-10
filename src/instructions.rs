use super::registers::GeneralRegister;

pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SEC(GeneralRegister, u8), // this stands for Skip-Equal-Constant
}

impl Instruction {
    pub fn new(instruction: u16) -> Instruction {
        let split_bits = (((instruction >> 12) & 0b1111) as u8,
                          ((instruction >> 8) & 0b1111) as u8,
                          ((instruction >> 4) & 0b1111) as u8,
                           (instruction & 0b1111) as u8);

        match split_bits {
            (0x0, bits_one, bits_two, bits_three) => {
                let address = ((((bits_one as u16) << 4) |
                                  bits_two as u16) << 4) |
                                  bits_three as u16;
                Instruction::SYS(address)
            },
            (0x0, 0x0, 0xE, 0x0) => {
                Instruction::CLS
            },
            (0x0, 0x0, 0xE, 0xE) => {
                Instruction::RET
            },
            (0x1, bits_one, bits_two, bits_three) => {
                let address = ((((bits_one as u16) << 4) |
                                  bits_two as u16) << 4) |
                                  bits_three as u16;
                Instruction::JP(address)
            },
            (0x2, bits_one, bits_two, bits_three) => {
                let address = ((((bits_one as u16) << 4) |
                                  bits_two as u16) << 4) |
                                  bits_three as u16;
                Instruction::CALL(address)
            },
            (0x3, bits_one, bits_two, bits_three) => {
                let register = match bits_one {
                    0 => GeneralRegister::V0,
                    1 => GeneralRegister::V1,
                    2 => GeneralRegister::V2,
                    3 => GeneralRegister::V3,
                    4 => GeneralRegister::V4,
                    5 => GeneralRegister::V5,
                    6 => GeneralRegister::V6,
                    7 => GeneralRegister::V7,
                    8 => GeneralRegister::V8,
                    9 => GeneralRegister::V9,
                    10 => GeneralRegister::VA,
                    11 => GeneralRegister::VB,
                    12 => GeneralRegister::VC,
                    13 => GeneralRegister::VD,
                    14 => GeneralRegister::VE,
                    15 => GeneralRegister::VF,
                    _ => panic!("SEC instruction has invalid register: {}", bits_one),
                };

                let constant = (bits_two << 4) | bits_three;
                Instruction::SEC(register, constant)
            },
            (_, _, _, _) => panic!("Invalid instruction: {:x}{:x}{:x}{:x}",
                                   split_bits.0, split_bits.1, split_bits.2, split_bits.3),
        }
    }
}
