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
            (0x0, 0x0, 0xE, 0x0) => {
                Instruction::CLS
            },
            (0x0, 0x0, 0xE, 0xE) => {
                Instruction::RET
            },
            (0x0, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Instruction::SYS(address)
            },
            (0x1, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Instruction::JP(address)
            },
            (0x2, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Instruction::CALL(address)
            },
            (0x3, register_bits, _, _) => {
                let register = match register_bits {
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
                    _ => panic!("SEC instruction has invalid register: {}", register_bits),
                };

                let constant = (instruction & 0b0000000011111111) as u8;
                Instruction::SEC(register, constant)
            },
            (_, _, _, _) => panic!("Invalid instruction: {:x}{:x}{:x}{:x}",
                                   split_bits.0, split_bits.1, split_bits.2, split_bits.3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_sys() {
        let sys = Instruction::new(0b0000_0000_0000_0011);
        match sys {
            Instruction::SYS(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_cls() {
        let cls = Instruction::new(0b0000000011100000);
        match cls {
            Instruction::CLS => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ret() {
        let ret = Instruction::new(0b0000000011101110);
        match ret {
            Instruction::RET => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_jp() {
        let jp = Instruction::new(0b0001000000000011);
        match jp {
            Instruction::JP(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_call() {
        let call = Instruction::new(0b0010000000000011);
        match call {
            Instruction::CALL(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_sec() {
        let sec = Instruction::new(0b0011000000000011);
        match sec {
            Instruction::SEC(register, constant) if constant == 3 => {
                match register {
                    GeneralRegister::V0 => assert!(true),
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }
}
