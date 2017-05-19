use super::registers::GeneralRegister;

pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SEC(GeneralRegister, u8),               // this stands for Skip-Equal-Constant
    SNEC(GeneralRegister, u8),              // this stands for Skip-Not-Equal-Constant
    SER(GeneralRegister, GeneralRegister),  // this stands for Skip-Equal-Registers
    LDC(GeneralRegister, u8),               // this stands for Load-Constant
    ADDC(GeneralRegister, u8),              // this stand for Add-Constant
    LDR(GeneralRegister, GeneralRegister),  // this stands for Load-Register
    OR(GeneralRegister, GeneralRegister),
    AND(GeneralRegister, GeneralRegister),
    XOR(GeneralRegister, GeneralRegister),
    ADDR(GeneralRegister, GeneralRegister), // this stands for Add-Registers
    SUB(GeneralRegister, GeneralRegister),
    SHR(GeneralRegister),
    SUBN(GeneralRegister, GeneralRegister),
    SHL(GeneralRegister),
    SNE(GeneralRegister, GeneralRegister),
    LDI(u16),                               // this stands for Load-I
    JPA(u16),                               // this stands for Jump-Address
    RND(GeneralRegister, u8),
    DRW(GeneralRegister, GeneralRegister, u8),
    SKP(GeneralRegister),
    SKNP(GeneralRegister),
    LDRD(GeneralRegister),                  // this stands for Load-Register-Delay
    LDVK(GeneralRegister),                  // this stands for Load-Register-Key
    LDDR(GeneralRegister),                  // this stands for Load-Delay-Register
    LDSR(GeneralRegister),                  // this stands for Load-Sound-Register
    ADDI(GeneralRegister),
    LDIR(GeneralRegister),                  // this stands for Load-I-Register
    LDBR(GeneralRegister),                  // this stands for Load-B-Register
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

impl ToInstruction for [u8; 3] {
    /// this is for creating instructions with a constant. The last u8 is the constant
    fn to_instruction(&self) -> u16 {
        let mut instruction = 0u16;

        instruction |= (self[0] as u16) & 0b1111;
        instruction <<= 4;
        instruction |= (self[1] as u16) & 0b1111;
        instruction <<= 8;
        instruction |= (self[2] as u16) & 0b11111111;

        instruction
    }
}

impl ToInstruction for [u16; 2] {
    /// this is for creating instructions with an address. The last u16 is the address
    fn to_instruction(&self) -> u16 {
        let mut instruction = 0u16;

        instruction |= self[0] & 0b1111;
        instruction <<= 12;
        instruction |= self[1] & 0b111111111111;

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
            },
            // ADD Vx kk
            (0x7, register_bits, _, _) => {
                let register = GeneralRegister::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Instruction::ADDC(register, constant)
            },
            // LD Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x0) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::LDR(register_x, register_y)
            },
            // OR Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x1) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::OR(register_x, register_y)
            },
            // AND Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x2) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::AND(register_x, register_y)
            },
            // XOR Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x3) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::XOR(register_x, register_y)
            },
            // ADD Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x4) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::ADDR(register_x, register_y)
            },
            // SUB Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x5) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::SUB(register_x, register_y)
            },
            // SHR Vx
            (0x8, register_x_bits, _, 0x6) => {
                let register_x = GeneralRegister::new(register_x_bits);

                Instruction::SHR(register_x)
            },
            // SUBN Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x7) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::SUBN(register_x, register_y)
            },
            // SHL Vx
            (0x8, register_x_bits, _, 0xE) => {
                let register_x = GeneralRegister::new(register_x_bits);

                Instruction::SHL(register_x)
            },
            // SNE Vx Vy
            (0x9, register_x_bits, register_y_bits, 0x0) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::SNE(register_x, register_y)
            },
            // LD I address
            (0xA, _, _, _) => {
                let address = instruction & 0b0000111111111111;

                Instruction::LDI(address)
            },
            // JP V0 address
            (0xB, _, _, _) => {
                let address = instruction & 0b0000111111111111;

                Instruction::JPA(address)
            },
            // RND Vx kk
            (0xC, register_bits, _, _) => {
                let register = GeneralRegister::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Instruction::RND(register, constant)
            },
            // DRW Vx Vy nibble
            (0xD, register_x_bits, register_y_bits, bytes) => {
                let register_x = GeneralRegister::new(register_x_bits);
                let register_y = GeneralRegister::new(register_y_bits);

                Instruction::DRW(register_x, register_y, bytes)
            },
            // SKP Vx
            (0xE, register_bits, 0x9, 0xE) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::SKP(register)
            },
            // SKNP Vx
            (0xE, register_bits, 0xA, 0x1) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::SKNP(register)
            },
            // LD Vx DT
            (0xF, register_bits, 0x0, 0x7) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::LDRD(register)
            },
            // LD Vx K
            (0xF, register_bits, 0x0, 0xA) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::LDVK(register)
            },
            // LD DT Vx
            (0xF, register_bits, 0x1, 0x5) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::LDDR(register)
            },
            // LD ST Vx
            (0xF, register_bits, 0x1, 0x8) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::LDSR(register)
            },
            // ADD I Vx
            (0xF, register_bits, 0x1, 0xE) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::ADDI(register)
            },
            // LD F Vx
            (0xF, register_bits, 0x2, 0x9) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::LDIR(register)
            },
            // LD B Vx
            (0xF, register_bits, 0x3, 0x3) => {
                let register = GeneralRegister::new(register_bits);

                Instruction::LDBR(register)
            },
            // Anything else
            (_, _, _, _) => panic!("Invalid instruction: {:#x}", instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_sys() {
        let sys = Instruction::new([0x0, 0x3]);
        match sys {
            Instruction::SYS(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_cls() {
        let cls = Instruction::new([0x0, 0x0, 0xE, 0x0]);
        match cls {
            Instruction::CLS => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ret() {
        let ret = Instruction::new([0x0, 0x0, 0xE, 0xE]);
        match ret {
            Instruction::RET => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_jp() {
        let jp = Instruction::new([0x1, 0x3]);
        match jp {
            Instruction::JP(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_call() {
        let call = Instruction::new([0x2, 0x3]);
        match call {
            Instruction::CALL(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_sec() {
        let sec = Instruction::new([0x3, 0x0, 0x3]);
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
        let snec = Instruction::new([0x4, 0x0, 0x3]);
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
        let ser = Instruction::new([0x5, 0x0, 0x1, 0x0]);
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
        let ldc = Instruction::new([0x6, 0x0, 0x3]);
        match ldc {
            Instruction::LDC(register, constant) if register == GeneralRegister::V0 &&
                                                                constant == 3 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_addc() {
        let addc = Instruction::new([0x7, 0x0, 0x3]);
        match addc {
            Instruction::ADDC(register, constant) => if constant == 3 &&
                                                        register == GeneralRegister::V0 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldr() {
        let ldr = Instruction::new([0x8, 0x0, 0x1, 0x0]);
        match ldr {
            Instruction::LDR(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                           register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_or() {
        let or = Instruction::new([0x8, 0x0, 0x1, 0x1]);
        match or {
            Instruction::OR(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                          register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_and() {
        let and = Instruction::new([0x8, 0x0, 0x1, 0x2]);
        match and {
            Instruction::AND(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                           register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_xor() {
        let xor = Instruction::new([0x8, 0x0, 0x1, 0x3]);
        match xor {
            Instruction::XOR(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                           register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_addr() {
        let addr = Instruction::new([0x8, 0x0, 0x1, 0x4]);
        match addr {
            Instruction::ADDR(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                            register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_sub() {
        let sub = Instruction::new([0x8, 0x0, 0x1, 0x5]);
        match sub {
            Instruction::SUB(register_x, register_y) => if register_x == GeneralRegister::V0 &&
                                                           register_y == GeneralRegister::V1 {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_shr() {
        let shr = Instruction::new([0x8, 0x1, 0x0, 0x6]);
        match shr {
            Instruction::SHR(register) if register == GeneralRegister::V1 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_subn() {
        let subn = Instruction::new([0x8, 0x0, 0x1, 0x7]);
        match subn {
            Instruction::SUBN(register_x, register_y) if register_x == GeneralRegister::V0 &&
                                                         register_y == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_shl() {
        let shl = Instruction::new([0x8, 0x1, 0x0, 0xE]);
        match shl {
            Instruction::SHL(register) if register == GeneralRegister::V1 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_sne() {
        let sne = Instruction::new([0x9, 0x0, 0x1, 0x0]);
        match sne {
            Instruction::SNE(register_x, register_y) if register_x == GeneralRegister::V0 &&
                                                        register_y == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldi() {
        let ldi = Instruction::new([0xA, 0x3]);
        match ldi {
            Instruction::LDI(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_jpa() {
        let jpa = Instruction::new([0xB, 0x3]);
        match jpa {
            Instruction::JPA(address) if address == 0x3 => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_rnd() {
        let rnd = Instruction::new([0xC, 0x1, 0x3]);
        match rnd {
            Instruction::RND(register, constant) if register == GeneralRegister::V1 &&
                                                     constant == 0x3 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_drw() {
        let drw = Instruction::new([0xD, 0x1, 0x2, 0x4]);
        match drw {
            Instruction::DRW(register_x, register_y, bytes) if register_x == GeneralRegister::V1 &&
                                                               register_y == GeneralRegister::V2 &&
                                                               bytes == 4 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_skp() {
        let skp = Instruction::new([0xE, 0x1, 0x9, 0xE]);
        match skp {
            Instruction::SKP(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_sknp() {
        let sknp = Instruction::new([0xE, 0x1, 0xA, 0x1]);
        match sknp {
            Instruction::SKNP(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldrd() {
        let ldrd = Instruction::new([0xF, 0x1, 0x0, 0x7]);
        match ldrd {
            Instruction::LDRD(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldvk() {
        let ldvk = Instruction::new([0xF, 0x1, 0x0, 0xA]);
        match ldvk {
            Instruction::LDVK(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_lddr() {
        let lddr = Instruction::new([0xF, 0x1, 0x1, 0x5]);
        match lddr {
            Instruction::LDDR(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldsr() {
        let ldsr = Instruction::new([0xF, 0x1, 0x1, 0x8]);
        match ldsr {
            Instruction::LDSR(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_addi() {
        let addi = Instruction::new([0xF, 0x1, 0x1, 0xE]);
        match addi {
            Instruction::ADDI(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldir() {
        let ldir = Instruction::new([0xF, 0x1, 0x2, 0x9]);
        match ldir {
            Instruction::LDIR(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn decode_ldbr() {
        let ldbr = Instruction::new([0xF, 0x1, 0x3, 0x3]);
        match ldir {
            Instruction::LDBR(register) if register == GeneralRegister::V1 => {
                assert!(true);
            },
            _ => assert!(false),
        }
    }
}
