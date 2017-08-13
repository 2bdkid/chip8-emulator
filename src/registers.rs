use std::fmt;

#[derive(Default, Debug)]
pub struct Chip8Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
}

impl Chip8Registers {
    pub fn get(&self, register: Register) -> u8 {
        match register {
            Register::V0 => self.v0,
            Register::V1 => self.v1,
            Register::V2 => self.v2,
            Register::V3 => self.v3,
            Register::V4 => self.v4,
            Register::V5 => self.v5,
            Register::V6 => self.v6,
            Register::V7 => self.v7,
            Register::V8 => self.v8,
            Register::V9 => self.v9,
            Register::VA => self.va,
            Register::VB => self.vb,
            Register::VC => self.vc,
            Register::VD => self.vd,
            Register::VE => self.ve,
            Register::VF => self.vf,
        }
    }

    pub fn get_mut(&mut self, register: Register) -> &mut u8 {
        match register {
            Register::V0 => &mut self.v0,
            Register::V1 => &mut self.v1,
            Register::V2 => &mut self.v2,
            Register::V3 => &mut self.v3,
            Register::V4 => &mut self.v4,
            Register::V5 => &mut self.v5,
            Register::V6 => &mut self.v6,
            Register::V7 => &mut self.v7,
            Register::V8 => &mut self.v8,
            Register::V9 => &mut self.v9,
            Register::VA => &mut self.va,
            Register::VB => &mut self.vb,
            Register::VC => &mut self.vc,
            Register::VD => &mut self.vd,
            Register::VE => &mut self.ve,
            Register::VF => &mut self.vf,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl Register {
    pub fn new(value: u8) -> Register {
        match value {
            0 => Register::V0,
            1 => Register::V1,
            2 => Register::V2,
            3 => Register::V3,
            4 => Register::V4,
            5 => Register::V5,
            6 => Register::V6,
            7 => Register::V7,
            8 => Register::V8,
            9 => Register::V9,
            10 => Register::VA,
            11 => Register::VB,
            12 => Register::VC,
            13 => Register::VD,
            14 => Register::VE,
            15 => Register::VF,
            _ => panic!(format!("Cannot create Register from value {}", value)),
        }
    }
}