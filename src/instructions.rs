use registers::Register;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SEC(Register, u8),       // this stands for Skip-Equal-Constant
    SNEC(Register, u8),      // this stands for Skip-Not-Equal-Constant
    SER(Register, Register), // this stands for Skip-Equal-Registers
    LDC(Register, u8),       // this stands for Load-Constant
    ADDC(Register, u8),      // this stand for Add-Constant
    LDR(Register, Register), // this stands for Load-Register
    OR(Register, Register),
    AND(Register, Register),
    XOR(Register, Register),
    ADDR(Register, Register), // this stands for Add-Registers
    SUB(Register, Register),
    SHR(Register),
    SUBN(Register, Register),
    SHL(Register),
    SNE(Register, Register),
    LDI(u16), // this stands for Load-I
    JPA(u16), // this stands for Jump-Address
    RND(Register, u8),
    DRW(Register, Register, u8),
    SKP(Register),
    SKNP(Register),
    LDRD(Register), // this stands for Load-Register-Delay
    LDVK(Register), // this stands for Load-Register-Key
    LDDR(Register), // this stands for Load-Delay-Register
    LDSR(Register), // this stands for Load-Sound-Register
    ADDI(Register),
    LDIR(Register), // this stands for Load-I-Register
    LDBR(Register), // this stands for Load-B-Register
    LDRS(Register), // this stands for Load-Registers
    RDRS(Register), // this stands for Read-Registers
}

impl Instruction {
    pub fn new(instruction: u16) -> Option<Instruction> {
        let split_bits = (
            ((instruction >> 12) & 0b1111) as u8,
            ((instruction >> 8) & 0b1111) as u8,
            ((instruction >> 4) & 0b1111) as u8,
            (instruction & 0b1111) as u8,
        );

        match split_bits {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => Some(Instruction::CLS),
            // RET
            (0x0, 0x0, 0xE, 0xE) => Some(Instruction::RET),
            // SYS address
            (0x0, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Some(Instruction::SYS(address))
            }
            // JP address
            (0x1, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Some(Instruction::JP(address))
            }
            // CALL address
            (0x2, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Some(Instruction::CALL(address))
            }
            // SE Vx kk
            (0x3, register_bits, _, _) => {
                let register = Register::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Some(Instruction::SEC(register, constant))
            }
            // SE Vx kk (not equal)
            (0x4, register_bits, _, _) => {
                let register = Register::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Some(Instruction::SNEC(register, constant))
            }
            // SE Vx Vy
            (0x5, register_x_bits, register_y_bits, 0x0) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::SER(register_x, register_y))
            }
            // LD Vx kk
            (0x6, register_bits, _, _) => {
                let register = Register::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Some(Instruction::LDC(register, constant))
            }
            // ADD Vx kk
            (0x7, register_bits, _, _) => {
                let register = Register::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Some(Instruction::ADDC(register, constant))
            }
            // LD Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x0) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::LDR(register_x, register_y))
            }
            // OR Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x1) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::OR(register_x, register_y))
            }
            // AND Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x2) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::AND(register_x, register_y))
            }
            // XOR Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x3) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::XOR(register_x, register_y))
            }
            // ADD Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x4) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::ADDR(register_x, register_y))
            }
            // SUB Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x5) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::SUB(register_x, register_y))
            }
            // SHR Vx
            (0x8, register_x_bits, _, 0x6) => {
                let register_x = Register::new(register_x_bits);
                Some(Instruction::SHR(register_x))
            }
            // SUBN Vx Vy
            (0x8, register_x_bits, register_y_bits, 0x7) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::SUBN(register_x, register_y))
            }
            // SHL Vx
            (0x8, register_x_bits, _, 0xE) => {
                let register_x = Register::new(register_x_bits);
                Some(Instruction::SHL(register_x))
            }
            // SNE Vx Vy
            (0x9, register_x_bits, register_y_bits, 0x0) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::SNE(register_x, register_y))
            }
            // LD I address
            (0xA, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Some(Instruction::LDI(address))
            }
            // JP V0 address
            (0xB, _, _, _) => {
                let address = instruction & 0b0000111111111111;
                Some(Instruction::JPA(address))
            }
            // RND Vx kk
            (0xC, register_bits, _, _) => {
                let register = Register::new(register_bits);
                let constant = (instruction & 0b0000000011111111) as u8;

                Some(Instruction::RND(register, constant))
            }
            // DRW Vx Vy nibble
            (0xD, register_x_bits, register_y_bits, bytes) => {
                let register_x = Register::new(register_x_bits);
                let register_y = Register::new(register_y_bits);

                Some(Instruction::DRW(register_x, register_y, bytes))
            }
            // SKP Vx
            (0xE, register_bits, 0x9, 0xE) => {
                let register = Register::new(register_bits);
                Some(Instruction::SKP(register))
            }
            // SKNP Vx
            (0xE, register_bits, 0xA, 0x1) => {
                let register = Register::new(register_bits);
                Some(Instruction::SKNP(register))
            }
            // LD Vx DT
            (0xF, register_bits, 0x0, 0x7) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDRD(register))
            }
            // LD Vx K
            (0xF, register_bits, 0x0, 0xA) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDVK(register))
            }
            // LD DT Vx
            (0xF, register_bits, 0x1, 0x5) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDDR(register))
            }
            // LD ST Vx
            (0xF, register_bits, 0x1, 0x8) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDSR(register))
            }
            // ADD I Vx
            (0xF, register_bits, 0x1, 0xE) => {
                let register = Register::new(register_bits);
                Some(Instruction::ADDI(register))
            }
            // LD F Vx
            (0xF, register_bits, 0x2, 0x9) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDIR(register))
            }
            // LD B Vx
            (0xF, register_bits, 0x3, 0x3) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDBR(register))
            }
            //LD I Vx
            (0xF, register_bits, 0x5, 0x5) => {
                let register = Register::new(register_bits);
                Some(Instruction::LDRS(register))
            }
            (0xf, register_bits, 0x6, 0x5) => {
                let register = Register::new(register_bits);
                Some(Instruction::RDRS(register))
            }
            // Anything else
            (_, _, _, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_sys() {
        let sys = Instruction::new(0x0003);
        assert_eq!(Some(Instruction::SYS(0x3)), sys);
    }

    #[test]
    fn decode_cls() {
        let cls = Instruction::new(0x00E0);
        assert_eq!(Some(Instruction::CLS), cls);
    }

    #[test]
    fn decode_ret() {
        let ret = Instruction::new(0x00EE);
        assert_eq!(Some(Instruction::RET), ret);
    }

    #[test]
    fn decode_jp() {
        let jp = Instruction::new(0x1003);
        assert_eq!(Some(Instruction::JP(0x3)), jp);
    }

    #[test]
    fn decode_call() {
        let call = Instruction::new(0x2003);
        assert_eq!(Some(Instruction::CALL(0x3)), call);
    }

    #[test]
    fn decode_sec() {
        let sec = Instruction::new(0x3003);
        assert_eq!(Some(Instruction::SEC(Register::V0, 3)), sec);
    }

    #[test]
    fn decode_snec() {
        let snec = Instruction::new(0x4003);
        assert_eq!(Some(Instruction::SNEC(Register::V0, 3)), snec);
    }

    #[test]
    fn decode_ser() {
        let ser = Instruction::new(0x5010);
        assert_eq!(Some(Instruction::SER(Register::V0, Register::V1)), ser);
    }

    #[test]
    fn decode_ldc() {
        let ldc = Instruction::new(0x6003);
        assert_eq!(Some(Instruction::LDC(Register::V0, 3)), ldc);
    }

    #[test]
    fn decode_addc() {
        let addc = Instruction::new(0x7003);
        assert_eq!(Some(Instruction::ADDC(Register::V0, 3)), addc);
    }

    #[test]
    fn decode_ldr() {
        let ldr = Instruction::new(0x8010);
        assert_eq!(Some(Instruction::LDR(Register::V0, Register::V1)), ldr);
    }

    #[test]
    fn decode_or() {
        let or = Instruction::new(0x8011);
        assert_eq!(Some(Instruction::OR(Register::V0, Register::V1)), or);
    }

    #[test]
    fn decode_and() {
        let and = Instruction::new(0x8012);
        assert_eq!(Some(Instruction::AND(Register::V0, Register::V1)), and);
    }

    #[test]
    fn decode_xor() {
        let xor = Instruction::new(0x8013);
        assert_eq!(Some(Instruction::XOR(Register::V0, Register::V1)), xor);
    }

    #[test]
    fn decode_addr() {
        let addr = Instruction::new(0x8014);
        assert_eq!(Some(Instruction::ADDR(Register::V0, Register::V1)), addr);
    }

    #[test]
    fn decode_sub() {
        let sub = Instruction::new(0x8015);
        assert_eq!(Some(Instruction::SUB(Register::V0, Register::V1)), sub);
    }

    #[test]
    fn decode_shr() {
        let shr = Instruction::new(0x8106);
        assert_eq!(Some(Instruction::SHR(Register::V1)), shr);
    }

    #[test]
    fn decode_subn() {
        let subn = Instruction::new(0x8017);
        assert_eq!(Some(Instruction::SUBN(Register::V0, Register::V1)), subn);
    }

    #[test]
    fn decode_shl() {
        let shl = Instruction::new(0x810E);
        assert_eq!(Some(Instruction::SHL(Register::V1)), shl);
    }

    #[test]
    fn decode_sne() {
        let sne = Instruction::new(0x9010);
        assert_eq!(Some(Instruction::SNE(Register::V0, Register::V1)), sne);
    }

    #[test]
    fn decode_ldi() {
        let ldi = Instruction::new(0xA003);
        assert_eq!(Some(Instruction::LDI(0x3)), ldi);
    }

    #[test]
    fn decode_jpa() {
        let jpa = Instruction::new(0xB003);
        assert_eq!(Some(Instruction::JPA(0x3)), jpa);
    }

    #[test]
    fn decode_rnd() {
        let rnd = Instruction::new(0xC103);
        assert_eq!(Some(Instruction::RND(Register::V1, 0x3)), rnd);
    }

    #[test]
    fn decode_drw() {
        let drw = Instruction::new(0xD124);
        assert_eq!(Some(Instruction::DRW(Register::V1, Register::V2, 4)), drw);
    }

    #[test]
    fn decode_skp() {
        let skp = Instruction::new(0xE19E);
        assert_eq!(Some(Instruction::SKP(Register::V1)), skp);
    }

    #[test]
    fn decode_sknp() {
        let sknp = Instruction::new(0xE1A1);
        assert_eq!(Some(Instruction::SKNP(Register::V1)), sknp);
    }

    #[test]
    fn decode_ldrd() {
        let ldrd = Instruction::new(0xF107);
        assert_eq!(Some(Instruction::LDRD(Register::V1)), ldrd);
    }

    #[test]
    fn decode_ldvk() {
        let ldvk = Instruction::new(0xF10A);
        assert_eq!(Some(Instruction::LDVK(Register::V1)), ldvk);
    }

    #[test]
    fn decode_lddr() {
        let lddr = Instruction::new(0xF115);
        assert_eq!(Some(Instruction::LDDR(Register::V1)), lddr);
    }

    #[test]
    fn decode_ldsr() {
        let ldsr = Instruction::new(0xF118);
        assert_eq!(Some(Instruction::LDSR(Register::V1)), ldsr);
    }

    #[test]
    fn decode_addi() {
        let addi = Instruction::new(0xF11E);
        assert_eq!(Some(Instruction::ADDI(Register::V1)), addi);
    }

    #[test]
    fn decode_ldir() {
        let ldir = Instruction::new(0xF129);
        assert_eq!(Some(Instruction::LDIR(Register::V1)), ldir);
    }

    #[test]
    fn decode_ldbr() {
        let ldbr = Instruction::new(0xF133);
        assert_eq!(Some(Instruction::LDBR(Register::V1)), ldbr);
    }

    #[test]
    fn decode_ldrs() {
        let ldrs = Instruction::new(0xF555);
        assert_eq!(Some(Instruction::LDRS(Register::V5)), ldrs);
    }

    #[test]
    fn decode_rdrs() {
        let rdrs = Instruction::new(0xF565);
        assert_eq!(Some(Instruction::RDRS(Register::V5)), rdrs);
    }
}
