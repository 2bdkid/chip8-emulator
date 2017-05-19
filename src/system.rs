use super::rand;

use super::display;
use super::instructions;
use super::keyboard;
use super::registers;
use super::memory;

use instructions::Instruction;
use registers::GeneralRegister;

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

    fn run_sys(&mut self, address: u16) {
        self.registers.pc = address;
    }

    fn run_cls(&self) {
        // clear the display
        unimplemented!();
    }

    fn run_ret(&mut self) {
        self.registers.pc = self.registers.pop_stack();
        self.registers.sp -= 1;
    }

    fn run_jp(&mut self, address: u16) {
        self.registers.pc = address;
    }

    fn run_call(&mut self, address: u16) {
        self.registers.sp += 1;
        let pc = self.registers.pc;
        self.registers.push_stack(pc);
        self.registers.pc = address;
    }

    fn run_sec(&mut self, register: GeneralRegister, constant: u8) {
        let register_value = self.registers.get(register);

        if register_value == constant {
            self.registers.pc += 2;
        }
    }

    fn run_snec(&mut self, register: GeneralRegister, constant: u8) {
        let register_value = self.registers.get(register);

        if register_value != constant {
            self.registers.pc += 2;
        }
    }

    fn run_ser(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_x_value == register_y_value {
            self.registers.pc += 2;
        }
    }

    fn run_ldc(&mut self, register: GeneralRegister, constant: u8) {
        *self.registers.get_mut(register) = constant;
    }

    fn run_addc(&mut self, register: GeneralRegister, constant: u8) {
        *self.registers.get_mut(register) += constant;
    }

    fn run_ldr(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        *self.registers.get_mut(register_x) = self.registers.get(register_y);
    }

    fn run_or(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        *self.registers.get_mut(register_x) |= self.registers.get(register_y);
    }

    fn run_and(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        *self.registers.get_mut(register_x) &= self.registers.get(register_y);
    }

    fn run_xor(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        *self.registers.get_mut(register_x) ^= self.registers.get(register_y);
    }

    fn run_addr(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        match register_x_value.overflowing_add(register_y_value) {
            (result, false) => {
                *self.registers.get_mut(register_x) = result;
                *self.registers.get_mut(GeneralRegister::VF) = 0;
            },
            (result, true) => {
                *self.registers.get_mut(register_x) = result;
                *self.registers.get_mut(GeneralRegister::VF) = 1;
            },
        }
    }

    fn run_sub(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_x_value > register_y_value {
            *self.registers.get_mut(GeneralRegister::VF) = 1;
        } else {
            *self.registers.get_mut(GeneralRegister::VF) = 0;
        }

        *self.registers.get_mut(register_x) -= register_y_value;
    }

    fn run_shr(&mut self, register: GeneralRegister) {
        if self.registers.get(register) & 0b1 == 1 {
            *self.registers.get_mut(GeneralRegister::VF) = 1;
        } else {
            *self.registers.get_mut(GeneralRegister::VF) = 0;
        }

        *self.registers.get_mut(register) >>= 1;
    }

    fn run_subn(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_y_value > register_x_value {
            *self.registers.get_mut(GeneralRegister::VF) = 1;
        } else {
            *self.registers.get_mut(GeneralRegister::VF) = 0;
        }

        *self.registers.get_mut(register_x) = register_y_value - register_x_value;
    }

    fn run_shl(&mut self, register: GeneralRegister) {
        if self.registers.get(register) & 0b10000000 == 1 {
            *self.registers.get_mut(GeneralRegister::VF) = 1;
        } else {
            *self.registers.get_mut(GeneralRegister::VF) = 0;
        }

        *self.registers.get_mut(register) <<= 1;
    }

    fn run_sne(&mut self, register_x: GeneralRegister, register_y: GeneralRegister) {
        if self.registers.get(register_x) != self.registers.get(register_y) {
            self.registers.pc += 2;
        }
    }

    fn run_ldi(&mut self, address: u16) {
        self.registers.i = address;
    }

    fn run_jpa(&mut self, address: u16) {
        self.registers.pc = self.registers.get(GeneralRegister::V0) as u16 + address;
    }

    fn run_rnd(&mut self, register: GeneralRegister, constant: u8) {
        *self.registers.get_mut(register) = rand::random::<u8>() & constant;
    }

    fn run_drw(&mut self, register_x: GeneralRegister, register_y: GeneralRegister, bytes: u8) {
        // dear god
        unimplemented!();
    }

    fn run_skp(&mut self, register: GeneralRegister) {
        let register_value = self.registers.get(register);

        if self.keyboard.is_pressed(register_value) {
            self.registers.pc += 2;
        }
    }

    fn run_sknp(&mut self, register: GeneralRegister) {
        let register_value = self.registers.get(register);

        if !self.keyboard.is_pressed(register_value) {
            self.registers.pc += 2;
        }
    }

    fn run_ldrd(&mut self, register: GeneralRegister) {
        *self.registers.get_mut(register) = self.registers.delay;
    }

    fn run_ldvk(&mut self, register: GeneralRegister) {
        unimplemented!();
    }

    fn run_lddr(&mut self, register: GeneralRegister) {
        self.registers.delay = self.registers.get(register);
    }

    fn run_ldsr(&mut self, register: GeneralRegister) {
        self.registers.sound = self.registers.get(register);
    }

    fn run_addi(&mut self, register: GeneralRegister) {
        self.registers.i += self.registers.get(register) as u16;
    }

    fn run_ldir(&mut self, register: GeneralRegister) {
        unimplemented!();
    }

    fn run_ldbr(&mut self, register: GeneralRegister) {
        let register_value = self.registers.get(register);
        let i_value = self.registers.i;

        let ones = register_value % 10;
        let tens = (register_value / 10) % 10;
        let hundreds = (register_value / 100) % 10;

        self.memory_bank.write(i_value as usize, hundreds);
        self.memory_bank.write((i_value + 1) as usize, tens);
        self.memory_bank.write((i_value + 2) as usize, ones);
    }

    fn run_op(&mut self, op: Instruction) {
        match op {
            Instruction::SYS(address) => {
                self.run_sys(address);
            },
            Instruction::CLS => {
                self.run_cls();
            },
            Instruction::RET => {
                self.run_ret();
            },
            Instruction::JP(address) => {
                self.run_jp(address);
            },
            Instruction::CALL(address) => {
                self.run_call(address);
            },
            Instruction::SEC(register, constant) => {
                self.run_sec(register, constant);
            },
            Instruction::SNEC(register, constant) => {
                self.run_snec(register, constant);
            },
            Instruction::SER(register_x, register_y) => {
                self.run_ser(register_x, register_y);
            },
            Instruction::LDC(register, constant) => {
                self.run_ldc(register, constant);
            },
            Instruction::ADDC(register, constant) => {
                self.run_addc(register, constant);
            },
            Instruction::LDR(register_x, register_y) => {
                self.run_ldr(register_x, register_y);
            },
            Instruction::OR(register_x, register_y) => {
                self.run_or(register_x, register_y);
            },
            Instruction::AND(register_x, register_y) => {
                self.run_and(register_x, register_y);
            },
            Instruction::XOR(register_x ,register_y) => {
                self.run_xor(register_x, register_y);
            },
            Instruction::ADDR(register_x, register_y) => {
                self.run_addr(register_x, register_y);
            },
            Instruction::SUB(register_x, register_y) => {
                self.run_sub(register_x, register_y);
            },
            Instruction::SHR(register) => {
                self.run_shr(register);
            },
            Instruction::SUBN(register_x, register_y) => {
                self.run_subn(register_x, register_y);
            },
            Instruction::SHL(register) => {
                self.run_shl(register);
            },
            Instruction::SNE(register_x, register_y) => {
                self.run_sne(register_x, register_y);
            },
            Instruction::LDI(address) => {
                self.run_ldi(address);
            },
            Instruction::JPA(address) => {
                self.run_jpa(address);
            },
            Instruction::RND(register, constant) => {
                self.run_rnd(register, constant);
            },
            Instruction::DRW(register_x, register_y, bytes) => {
                self.run_drw(register_x, register_y, bytes);
            },
            Instruction::SKP(register) => {
                self.run_skp(register);
            },
            Instruction::SKNP(register) => {
                self.run_sknp(register);
            },
            Instruction::LDRD(register) => {
                self.run_ldrd(register);
            },
            Instruction::LDVK(register) => {
                self.run_ldvk(register);
            },
            Instruction::LDDR(register) => {
                self.run_lddr(register);
            },
            Instruction::LDSR(register) => {
                self.run_ldsr(register);
            },
            Instruction::ADDI(register) => {
                self.run_addi(register);
            },
            Instruction::LDIR(register) => {
                self.run_ldir(register);
            },
            Instruction::LDBR(register) => {
                self.run_ldbr(register);
            },
        }
    }

    pub fn run(&mut self) {
        let op = Instruction::new([0x1, 0x3]);
        self.run_op(op);
        println!("{:#?}", self.registers);
    }
}
