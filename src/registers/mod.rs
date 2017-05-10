use std::fmt;

mod stack;

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

#[derive(Default)]
pub struct Chip8Registers {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8,
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
