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

    pub fn run(&mut self) {
        let op = instructions::Instruction::new(0b0000000000000011);

        match op {
            Instruction::SYS(address) => {
                self.registers.pc = address;
            },
            Instruction::CLS => {
                // clear the screen
                unimplemented!();
            },
            Instruction::RET => {
                self.registers.pc = self.registers.pop_stack();
                self.registers.sp -= 1;
            },
            Instruction::JP(address) => {
                self.registers.pc = address;
            },
            Instruction::CALL(address) => {
                self.registers.sp += 1;
                let pc = self.registers.pc;
                self.registers.push_stack(pc);
                self.registers.pc = address;
            },
            Instruction::SEC(register, constant) => {
                  match register {
                      GeneralRegister::V0 => {
                            if self.registers.v0 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V1 => {
                            if self.registers.v1 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V2 => {
                            if self.registers.v2 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V3 => {
                            if self.registers.v3 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V4 => {
                            if self.registers.v4 == constant {
                                self.registers.pc +=2;
                            }
                      },GeneralRegister::V5 => {
                            if self.registers.v5 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V6 => {
                            if self.registers.v6 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V7 => {
                            if self.registers.v7 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V8 => {
                            if self.registers.v8 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::V9 => {
                            if self.registers.v9 == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::VA => {
                            if self.registers.va == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::VB => {
                            if self.registers.vb == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::VC => {
                            if self.registers.vd == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::VD => {
                            if self.registers.vd == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::VE => {
                            if self.registers.ve == constant {
                                self.registers.pc +=2;
                            }
                      },
                      GeneralRegister::VF => {
                            if self.registers.vf == constant {
                                self.registers.pc +=2;
                            }
                      },
                  }
            },
        }
    }
}
