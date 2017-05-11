use std::fmt;

mod stack;

#[derive(Clone, Copy, PartialEq)]
pub enum GeneralRegister {
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

impl GeneralRegister {
    pub fn new(register: u8) -> GeneralRegister {
        match register {
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
            _ => panic!(format!("Cannot create GeneralRegister from index > 15. Got {}", register)),
        }
    }
}

#[derive(Default)]
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
    stack: stack::Chip8Stack,
}

impl Chip8Registers {
    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
        self.sp = self.stack.top_address();
    }

    pub fn pop_stack(&mut self) -> u16 {
        let popped_value = self.stack.pop();
        self.sp = self.stack.top_address();
        popped_value
    }

    pub fn get(&self, register: GeneralRegister) -> u8 {
        match register {
            GeneralRegister::V0 => self.v0,
            GeneralRegister::V1 => self.v1,
            GeneralRegister::V2 => self.v2,
            GeneralRegister::V3 => self.v3,
            GeneralRegister::V4 => self.v4,
            GeneralRegister::V5 => self.v5,
            GeneralRegister::V6 => self.v6,
            GeneralRegister::V7 => self.v7,
            GeneralRegister::V8 => self.v8,
            GeneralRegister::V9 => self.v9,
            GeneralRegister::VA => self.va,
            GeneralRegister::VB => self.vb,
            GeneralRegister::VC => self.vc,
            GeneralRegister::VD => self.vd,
            GeneralRegister::VE => self.ve,
            GeneralRegister::VF => self.vf,
        }
    }

    pub fn get_mut(&mut self, register: GeneralRegister) -> &mut u8 {
        match register {
            GeneralRegister::V0 => &mut self.v0,
            GeneralRegister::V1 => &mut self.v1,
            GeneralRegister::V2 => &mut self.v2,
            GeneralRegister::V3 => &mut self.v3,
            GeneralRegister::V4 => &mut self.v4,
            GeneralRegister::V5 => &mut self.v5,
            GeneralRegister::V6 => &mut self.v6,
            GeneralRegister::V7 => &mut self.v7,
            GeneralRegister::V8 => &mut self.v8,
            GeneralRegister::V9 => &mut self.v9,
            GeneralRegister::VA => &mut self.va,
            GeneralRegister::VB => &mut self.vb,
            GeneralRegister::VC => &mut self.vc,
            GeneralRegister::VD => &mut self.vd,
            GeneralRegister::VE => &mut self.ve,
            GeneralRegister::VF => &mut self.vf,
        }
    }
}

// wrapper around u8 to print in hexadecimal
struct U8Register(u8);

impl fmt::Debug for U8Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

// wrapper around u16 to print in hexadecimal
struct U16Register(u16);

impl fmt::Debug for U16Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

// will print the register name plus values in hexadecimal
impl fmt::Debug for Chip8Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entry(&"V0", &U8Register(self.v0))
                     .entry(&"V1", &U8Register(self.v1))
                     .entry(&"V2", &U8Register(self.v2))
                     .entry(&"V3", &U8Register(self.v3))
                     .entry(&"V4", &U8Register(self.v4))
                     .entry(&"V5", &U8Register(self.v5))
                     .entry(&"V6", &U8Register(self.v6))
                     .entry(&"V7", &U8Register(self.v7))
                     .entry(&"V8", &U8Register(self.v8))
                     .entry(&"V9", &U8Register(self.v9))
                     .entry(&"VA", &U8Register(self.va))
                     .entry(&"VB", &U8Register(self.vb))
                     .entry(&"VC", &U8Register(self.vc))
                     .entry(&"VD", &U8Register(self.vd))
                     .entry(&"VE", &U8Register(self.ve))
                     .entry(&"VF", &U8Register(self.vf))
                     .entry(&"I", &U16Register(self.i))
                     .entry(&"Delay", &U8Register(self.delay))
                     .entry(&"Sound", &U8Register(self.sound))
                     .entry(&"PC", &U16Register(self.pc))
                     .entry(&"SP", &U8Register(self.sp))
                     .entry(&"Stack", &self.stack)
                     .finish()
    }
}
