use super::registers::GeneralRegister;

pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SEC(GeneralRegister, u8),  // this stands for Skip-Equal-Constant
    SNEC(GeneralRegister, u8), // this stands for Skip-Not-Equal-Constant
    SER(GeneralRegister, GeneralRegister), // this stands for Skip-Equal-Registers
    LDC(GeneralRegister, u8), // this stands for Load Constant
}

pub trait ToInstruction {
    fn to_instruction(&self) -> u16;
}

impl ToInstruction for u16 {
    fn to_instruction(&self) -> u16 {
        *self
    }
}

impl ToInstruction for [u8; 4] {
    /// this function concatenates the last 4 bits of each u8 in self into a single u16
    fn to_instruction(&self) -> u16 {
        /*  one-liner madness
        (((((((self[0] as u16 & 0b1111)  << 4) |
              (self[1] as u16 & 0b1111)) << 4) |
              (self[2] as u16 & 0b1111)) << 4) |
              (self[3] as u16 & 0b1111))
        */

        let mut instruction = 0u16;

        instruction |= (self[0] as u16) & 0b1111;
        instruction <<= 4;
        instruction |= (self[1] as u16) & 0b1111;
        instruction <<= 4;
        instruction |= (self[2] as u16) & 0b1111;
        instruction <<= 4;
        instruction |= (self[3] as u16) & 0b1111;

        instruction
    }
}

impl Instruction {
    pub fn new<T: ToInstruction>(instruction: T) -> Instruction {
        let instruction = instruction.to_instruction();
        let split_bits = (((instruction >> 12) & 0b1111) as u8,
                          ((instruction >> 8) & 0b1111) as u8,
                          ((instruction >> 4) & 0b1111) as u8,
                           (instruction & 0b1111) as u8);

        match split_bits {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                Instruction::CLS
            },
            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                Instruction::RET
            },
            // SYS address
            (0x0, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Instruction::SYS(address)
            },
            // JP address
            (0x1, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Instruction::JP(address)
            },
            // CALL address
            (0x2, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Instruction::CALL(address)
            },
            // SE Vx kk
            (0x3, register_bits, _, _) => {
                let register = GeneralRegister::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Instruction::SEC(register, constant)
            },
            // SE Vx kk (not equal)
            (0x4, register_bits, _, _) => {
                let register = GeneralRegister::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Instruction::SNEC(register, constant)
            },
            // SE Vx Vy
            (0x5, register_x_bits, register_y_bits, 0x0) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::SER(register_x, register_y)
            },
            // LD Vx kk
            (0x6, register_bits, _, _) => {
                let register = GeneralRegister::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Instruction::LDC(register, constant)
            }
            // Anything else
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
            Instruction::SEC(register, constant) if constant == 3 &&
                                                    register == GeneralRegister::V0 => {
                assert!(true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_snec() {
        let snec = Instruction::new(0b0100000000000011);
        match snec {
            Instruction::SNEC(register, constant) if constant == 3  &&
                                                     register == GeneralRegister::V0 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ser() {
        let ser = Instruction::new(0b0101000000010000);
        match ser {
            Instruction::SER(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                           register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldc() {
        let ldc = Instruction::new(0b0110000000000011);
        match ldc {
            Instruction::LDC(register, constant) if register == GeneralRegister::V0 &&
                                                                constant == 3 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }
}
