use rand;
use ncurses;

use display;
use keyboard;
use registers;
use memory;
use stack;
use sprites;

use instructions::Instruction;
use registers::Register;
use keyboard::Key;
use sprites::ASCIISprite;

pub struct Chip8Machine {
    memory_bank: memory::Chip8Memory,
    registers: registers::Chip8Registers,
    keyboard: keyboard::Chip8Keyboard,
    display: display::Chip8Display,
    stack: stack::Chip8Stack,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            memory_bank: memory::Chip8Memory::default(),
            registers: registers::Chip8Registers::default(),
            keyboard: keyboard::Chip8Keyboard::default(),
            display: display::Chip8Display::default(),
            stack: stack::Chip8Stack::default(),
        }
    }

    fn run_sys(&mut self, address: u16) {
        self.registers.pc = address;
    }

    fn run_cls(&mut self) {
        self.display.clear();
    }

    fn run_ret(&mut self) {
        self.registers.pc = self.stack.pop();
        self.registers.sp -= 1;
    }

    fn run_jp(&mut self, address: u16) {
        self.registers.pc = address;
    }

    fn run_call(&mut self, address: u16) {
        self.registers.sp += 1;
        self.stack.push(self.registers.pc);
        self.registers.pc = address;
    }

    fn run_sec(&mut self, register: Register, constant: u8) {
        let register_value = self.registers.get(register);

        if register_value == constant {
            self.registers.pc += 2;
        }
    }

    fn run_snec(&mut self, register: Register, constant: u8) {
        let register_value = self.registers.get(register);

        if register_value != constant {
            self.registers.pc += 2;
        }
    }

    fn run_ser(&mut self, register_x: Register, register_y: Register) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_x_value == register_y_value {
            self.registers.pc += 2;
        }
    }

    fn run_ldc(&mut self, register: Register, constant: u8) {
        *self.registers.get_mut(register) = constant;
    }

    fn run_addc(&mut self, register: Register, constant: u8) {
        *self.registers.get_mut(register) += constant;
    }

    fn run_ldr(&mut self, register_x: Register, register_y: Register) {
        *self.registers.get_mut(register_x) = self.registers.get(register_y);
    }

    fn run_or(&mut self, register_x: Register, register_y: Register) {
        *self.registers.get_mut(register_x) |= self.registers.get(register_y);
    }

    fn run_and(&mut self, register_x: Register, register_y: Register) {
        *self.registers.get_mut(register_x) &= self.registers.get(register_y);
    }

    fn run_xor(&mut self, register_x: Register, register_y: Register) {
        *self.registers.get_mut(register_x) ^= self.registers.get(register_y);
    }

    fn run_addr(&mut self, register_x: Register, register_y: Register) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        match register_x_value.overflowing_add(register_y_value) {
            (result, false) => {
                *self.registers.get_mut(register_x) = result;
                *self.registers.get_mut(Register::VF) = 0;
            }
            (result, true) => {
                *self.registers.get_mut(register_x) = result;
                *self.registers.get_mut(Register::VF) = 1;
            }
        }
    }

    fn run_sub(&mut self, register_x: Register, register_y: Register) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_x_value > register_y_value {
            *self.registers.get_mut(Register::VF) = 1;
        } else {
            *self.registers.get_mut(Register::VF) = 0;
        }

        *self.registers.get_mut(register_x) -= register_y_value;
    }

    fn run_shr(&mut self, register: Register) {
        let register_value = self.registers.get(register);

        if register_value & 0b1 == 1 {
            *self.registers.get_mut(Register::VF) = 1;
        } else {
            *self.registers.get_mut(Register::VF) = 0;
        }

        *self.registers.get_mut(register) >>= 1;
    }

    fn run_subn(&mut self, register_x: Register, register_y: Register) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_y_value > register_x_value {
            *self.registers.get_mut(Register::VF) = 1;
        } else {
            *self.registers.get_mut(Register::VF) = 0;
        }

        *self.registers.get_mut(register_x) = register_y_value - register_x_value;
    }

    fn run_shl(&mut self, register: Register) {
        let register_value = self.registers.get(register);

        if register_value & 0b10000000 == 1 {
            *self.registers.get_mut(Register::VF) = 1;
        } else {
            *self.registers.get_mut(Register::VF) = 0;
        }

        *self.registers.get_mut(register) <<= 1;
    }

    fn run_sne(&mut self, register_x: Register, register_y: Register) {
        let register_x_value = self.registers.get(register_x);
        let register_y_value = self.registers.get(register_y);

        if register_x_value != register_y_value {
            self.registers.pc += 2;
        }
    }

    fn run_ldi(&mut self, address: u16) {
        self.registers.i = address;
    }

    fn run_jpa(&mut self, address: u16) {
        self.registers.pc = self.registers.get(Register::V0) as u16 + address;
    }

    fn run_rnd(&mut self, register: Register, constant: u8) {
        *self.registers.get_mut(register) = rand::random::<u8>() & constant;
    }

    fn run_drw(&mut self, register_x: Register, register_y: Register, bytes: u8) {
        let i_value = self.registers.i as usize;
        let mut sprite: Vec<u8> = Vec::new();

        let x = self.registers.get(register_x) as usize;
        let y = self.registers.get(register_y) as usize;

        let mut collision = false;

        for offset in 0..bytes as usize {
            let sprite_piece = self.memory_bank.read(i_value + offset);
            sprite.push(sprite_piece);
        }

        for (y_offset, layer) in sprite.iter().enumerate() {
            let mut pixels: [bool; 8] = [false; 8];

            if layer & 0b10000000 == 0b10000000 { pixels[0] = true; }
            if layer & 0b01000000 == 0b01000000 { pixels[1] = true; }
            if layer & 0b00100000 == 0b00100000 { pixels[2] = true; }
            if layer & 0b00010000 == 0b00010000 { pixels[3] = true; }
            if layer & 0b00001000 == 0b00001000 { pixels[4] = true; }
            if layer & 0b00000100 == 0b00000100 { pixels[5] = true; }
            if layer & 0b00000010 == 0b00000010 { pixels[6] = true; }
            if layer & 0b00000001 == 0b00000001 { pixels[7] = true; }

            for (x_offset, pixel) in pixels.iter().enumerate() {
                let mut x_draw_position = x + x_offset;
                let mut y_draw_position = y + y_offset;

                if x_draw_position > 63 { x_draw_position -= 64; }
                if y_draw_position > 32 { y_draw_position -= 32; }

                if self.display.draw_pixel(x_draw_position, y_draw_position, *pixel) {
                    collision = true;
                }
            }

            if collision {
                *self.registers.get_mut(Register::VF) = 1;
            } else {
                *self.registers.get_mut(Register::VF) = 0;
            }
        }
    }

    fn run_skp(&mut self, register: Register) {
        let register_value = self.registers.get(register);

        if self.keyboard.is_pressed(register_value) {
            self.registers.pc += 2;
        }
    }

    fn run_sknp(&mut self, register: Register) {
        let register_value = self.registers.get(register);

        if !self.keyboard.is_pressed(register_value) {
            self.registers.pc += 2;
        }
    }

    fn run_ldrd(&mut self, register: Register) {
        *self.registers.get_mut(register) = self.registers.delay;
    }

    fn run_ldvk(&mut self, register: Register) {
        let key = keyboard::get_key();

        match key {
            Key::Zero => *self.registers.get_mut(register) = 0,
            Key::One => *self.registers.get_mut(register) = 1,
            Key::Two => *self.registers.get_mut(register) = 2,
            Key::Three => *self.registers.get_mut(register) = 3,
            Key::Four => *self.registers.get_mut(register) = 4,
            Key::Five => *self.registers.get_mut(register) = 5,
            Key::Six => *self.registers.get_mut(register) = 6,
            Key::Seven => *self.registers.get_mut(register) = 7,
            Key::Eight => *self.registers.get_mut(register) = 8,
            Key::Nine => *self.registers.get_mut(register) = 9,
            Key::A => *self.registers.get_mut(register) = 10,
            Key::B => *self.registers.get_mut(register) = 11,
            Key::C => *self.registers.get_mut(register) = 12,
            Key::D => *self.registers.get_mut(register) = 13,
            Key::E => *self.registers.get_mut(register) = 14,
            Key::F => *self.registers.get_mut(register) = 15,
        }
    }

    fn run_lddr(&mut self, register: Register) {
        self.registers.delay = self.registers.get(register);
    }

    fn run_ldsr(&mut self, register: Register) {
        self.registers.sound = self.registers.get(register);
    }

    fn run_addi(&mut self, register: Register) {
        self.registers.i = self.registers.get(register) as u16;
    }

    fn run_ldir(&mut self, register: Register) {
        let register_value = self.registers.get(register);
        self.registers.i = sprites::get_location(ASCIISprite::new(register_value)) as u16;
    }

    fn run_ldbr(&mut self, register: Register) {
        let register_value = self.registers.get(register);
        let i_value = self.registers.i as usize;

        let ones = register_value % 10;
        let tens = (register_value / 10) % 10;
        let hundreds = (register_value / 100) % 10;

        self.memory_bank.write(i_value, hundreds);
        self.memory_bank.write(i_value + 1, tens);
        self.memory_bank.write(i_value + 2, ones);
    }

    fn run_ldrs(&mut self, register: Register) {
        let i_value = self.registers.i as usize;

        match register {
            Register::V0 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
            }
            Register::V1 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
            }
            Register::V2 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
            }
            Register::V3 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
            }
            Register::V4 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
            }
            Register::V5 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
            }
            Register::V6 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
            }
            Register::V7 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
            }
            Register::V8 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
            }
            Register::V9 => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
            }
            Register::VA => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
                self.memory_bank.write(i_value + 10, self.registers.get(Register::VA));
            }
            Register::VB => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
                self.memory_bank.write(i_value + 10, self.registers.get(Register::VA));
                self.memory_bank.write(i_value + 11, self.registers.get(Register::VB));
            }
            Register::VC => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
                self.memory_bank.write(i_value + 10, self.registers.get(Register::VA));
                self.memory_bank.write(i_value + 11, self.registers.get(Register::VB));
                self.memory_bank.write(i_value + 12, self.registers.get(Register::VC));
            }
            Register::VD => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
                self.memory_bank.write(i_value + 10, self.registers.get(Register::VA));
                self.memory_bank.write(i_value + 11, self.registers.get(Register::VB));
                self.memory_bank.write(i_value + 12, self.registers.get(Register::VC));
                self.memory_bank.write(i_value + 13, self.registers.get(Register::VD));
            }
            Register::VE => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
                self.memory_bank.write(i_value + 10, self.registers.get(Register::VA));
                self.memory_bank.write(i_value + 11, self.registers.get(Register::VB));
                self.memory_bank.write(i_value + 12, self.registers.get(Register::VC));
                self.memory_bank.write(i_value + 13, self.registers.get(Register::VD));
                self.memory_bank.write(i_value + 14, self.registers.get(Register::VE));
            }
            Register::VF => {
                self.memory_bank.write(i_value, self.registers.get(Register::V0));
                self.memory_bank.write(i_value + 1, self.registers.get(Register::V1));
                self.memory_bank.write(i_value + 2, self.registers.get(Register::V2));
                self.memory_bank.write(i_value + 3, self.registers.get(Register::V3));
                self.memory_bank.write(i_value + 4, self.registers.get(Register::V4));
                self.memory_bank.write(i_value + 5, self.registers.get(Register::V5));
                self.memory_bank.write(i_value + 6, self.registers.get(Register::V6));
                self.memory_bank.write(i_value + 7, self.registers.get(Register::V7));
                self.memory_bank.write(i_value + 8, self.registers.get(Register::V8));
                self.memory_bank.write(i_value + 9, self.registers.get(Register::V9));
                self.memory_bank.write(i_value + 10, self.registers.get(Register::VA));
                self.memory_bank.write(i_value + 11, self.registers.get(Register::VB));
                self.memory_bank.write(i_value + 12, self.registers.get(Register::VC));
                self.memory_bank.write(i_value + 13, self.registers.get(Register::VD));
                self.memory_bank.write(i_value + 14, self.registers.get(Register::VE));
                self.memory_bank.write(i_value + 15, self.registers.get(Register::VF));
            }
        }
    }

    fn run_rdrs(&mut self, register: Register) {
        let i_value = self.registers.i as usize;

        match register {
            Register::V0 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
            }
            Register::V1 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
            }
            Register::V2 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
            }
            Register::V3 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
            }
            Register::V4 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
            }
            Register::V5 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
            }
            Register::V6 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
            }
            Register::V7 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
            }
            Register::V8 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
            }
            Register::V9 => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
            }
            Register::VA => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
                *self.registers.get_mut(Register::VA) = self.memory_bank.read(i_value + 10);
            }
            Register::VB => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
                *self.registers.get_mut(Register::VA) = self.memory_bank.read(i_value + 10);
                *self.registers.get_mut(Register::VB) = self.memory_bank.read(i_value + 11);
            }
            Register::VC => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
                *self.registers.get_mut(Register::VA) = self.memory_bank.read(i_value + 10);
                *self.registers.get_mut(Register::VB) = self.memory_bank.read(i_value + 11);
                *self.registers.get_mut(Register::VC) = self.memory_bank.read(i_value + 12);
            }
            Register::VD => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
                *self.registers.get_mut(Register::VA) = self.memory_bank.read(i_value + 10);
                *self.registers.get_mut(Register::VB) = self.memory_bank.read(i_value + 11);
                *self.registers.get_mut(Register::VC) = self.memory_bank.read(i_value + 12);
                *self.registers.get_mut(Register::VD) = self.memory_bank.read(i_value + 13);
            }
            Register::VE => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
                *self.registers.get_mut(Register::VA) = self.memory_bank.read(i_value + 10);
                *self.registers.get_mut(Register::VB) = self.memory_bank.read(i_value + 11);
                *self.registers.get_mut(Register::VC) = self.memory_bank.read(i_value + 12);
                *self.registers.get_mut(Register::VD) = self.memory_bank.read(i_value + 13);
                *self.registers.get_mut(Register::VE) = self.memory_bank.read(i_value + 14);
            }
            Register::VF => {
                *self.registers.get_mut(Register::V0) = self.memory_bank.read(i_value);
                *self.registers.get_mut(Register::V1) = self.memory_bank.read(i_value + 1);
                *self.registers.get_mut(Register::V2) = self.memory_bank.read(i_value + 2);
                *self.registers.get_mut(Register::V3) = self.memory_bank.read(i_value + 3);
                *self.registers.get_mut(Register::V4) = self.memory_bank.read(i_value + 4);
                *self.registers.get_mut(Register::V5) = self.memory_bank.read(i_value + 5);
                *self.registers.get_mut(Register::V6) = self.memory_bank.read(i_value + 6);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 7);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 8);
                *self.registers.get_mut(Register::V7) = self.memory_bank.read(i_value + 9);
                *self.registers.get_mut(Register::VA) = self.memory_bank.read(i_value + 10);
                *self.registers.get_mut(Register::VB) = self.memory_bank.read(i_value + 11);
                *self.registers.get_mut(Register::VC) = self.memory_bank.read(i_value + 12);
                *self.registers.get_mut(Register::VD) = self.memory_bank.read(i_value + 13);
                *self.registers.get_mut(Register::VE) = self.memory_bank.read(i_value + 14);
                *self.registers.get_mut(Register::VF) = self.memory_bank.read(i_value + 15);
            }
        }
    }

    fn run_op(&mut self, op: &Instruction) {
        match *op {
            Instruction::SYS(address) => {
                self.run_sys(address);
            }
            Instruction::CLS => {
                self.run_cls();
            }
            Instruction::RET => {
                self.run_ret();
            }
            Instruction::JP(address) => {
                self.run_jp(address);
            }
            Instruction::CALL(address) => {
                self.run_call(address);
            }
            Instruction::SEC(register, constant) => {
                self.run_sec(register, constant);
            }
            Instruction::SNEC(register, constant) => {
                self.run_snec(register, constant);
            }
            Instruction::SER(register_x, register_y) => {
                self.run_ser(register_x, register_y);
            }
            Instruction::LDC(register, constant) => {
                self.run_ldc(register, constant);
            }
            Instruction::ADDC(register, constant) => {
                self.run_addc(register, constant);
            }
            Instruction::LDR(register_x, register_y) => {
                self.run_ldr(register_x, register_y);
            }
            Instruction::OR(register_x, register_y) => {
                self.run_or(register_x, register_y);
            }
            Instruction::AND(register_x, register_y) => {
                self.run_and(register_x, register_y);
            }
            Instruction::XOR(register_x, register_y) => {
                self.run_xor(register_x, register_y);
            }
            Instruction::ADDR(register_x, register_y) => {
                self.run_addr(register_x, register_y);
            }
            Instruction::SUB(register_x, register_y) => {
                self.run_sub(register_x, register_y);
            }
            Instruction::SHR(register) => {
                self.run_shr(register);
            }
            Instruction::SUBN(register_x, register_y) => {
                self.run_subn(register_x, register_y);
            }
            Instruction::SHL(register) => {
                self.run_shl(register);
            }
            Instruction::SNE(register_x, register_y) => {
                self.run_sne(register_x, register_y);
            }
            Instruction::LDI(address) => {
                self.run_ldi(address);
            }
            Instruction::JPA(address) => {
                self.run_jpa(address);
            }
            Instruction::RND(register, constant) => {
                self.run_rnd(register, constant);
            }
            Instruction::DRW(register_x, register_y, bytes) => {
                self.run_drw(register_x, register_y, bytes);
            }
            Instruction::SKP(register) => {
                self.run_skp(register);
            }
            Instruction::SKNP(register) => {
                self.run_sknp(register);
            }
            Instruction::LDRD(register) => {
                self.run_ldrd(register);
            }
            Instruction::LDVK(register) => {
                self.run_ldvk(register);
            }
            Instruction::LDDR(register) => {
                self.run_lddr(register);
            }
            Instruction::LDSR(register) => {
                self.run_ldsr(register);
            }
            Instruction::ADDI(register) => {
                self.run_addi(register);
            }
            Instruction::LDIR(register) => {
                self.run_ldir(register);
            }
            Instruction::LDBR(register) => {
                self.run_ldbr(register);
            }
            Instruction::LDRS(register) => {
                self.run_ldrs(register);
            }
            Instruction::RDRS(register) => {
                self.run_rdrs(register);
            }
        }
    }
    
    pub fn run(&mut self, binary_instructions: &Vec<u16>) {
        let mut instructions: Vec<Instruction> = Vec::new();

        for instruction in binary_instructions {
            instructions.push(Instruction::new(*instruction));
        }

        for instruction in &instructions {
            self.run_op(&instruction);
        }

        println!("{:#?}", self.display);
    }
    
    /*
    pub fn run(&mut self, binary_instructions: &Vec<u16>) {
        self.run_op(&Instruction::new([0x6, 0x0, 0x0]));
        self.run_op(&Instruction::new([0x6, 0x1, 0x0]));
        self.run_op(&Instruction::new([0x6, 0x2, 0xA]));
        self.run_op(&Instruction::new([0xF, 0x2, 0x2, 0x9]));
        self.run_op(&Instruction::new([0xD, 0x0, 0x1, 0x5]));

        println!("{:?}", self.display);
    }
    */
}
