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
        }
    }

    pub fn run(&mut self) {
        let op = Instruction::new([0x1, 0x3]);
        self.run_op(op);
        println!("{:#?}", self.registers);
    }
}
