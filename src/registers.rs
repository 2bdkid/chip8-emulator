use std::fmt;

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
        f.debug_map().entry(&String::from("V0"), &U8Register(self.v0))
                     .entry(&String::from("V1"), &U8Register(self.v1))
                     .entry(&String::from("V2"), &U8Register(self.v2))
                     .entry(&String::from("V3"), &U8Register(self.v3))
                     .entry(&String::from("V4"), &U8Register(self.v4))
                     .entry(&String::from("V5"), &U8Register(self.v5))
                     .entry(&String::from("V6"), &U8Register(self.v6))
                     .entry(&String::from("V7"), &U8Register(self.v7))
                     .entry(&String::from("V8"), &U8Register(self.v8))
                     .entry(&String::from("V9"), &U8Register(self.v9))
                     .entry(&String::from("VA"), &U8Register(self.va))
                     .entry(&String::from("VB"), &U8Register(self.vb))
                     .entry(&String::from("VC"), &U8Register(self.vc))
                     .entry(&String::from("VD"), &U8Register(self.vd))
                     .entry(&String::from("VE"), &U8Register(self.ve))
                     .entry(&String::from("VF"), &U8Register(self.vf))
                     .entry(&String::from("I"), &U16Register(self.i))
                     .entry(&String::from("Delay"), &U8Register(self.delay))
                     .entry(&String::from("Sound"), &U8Register(self.sound))
                     .entry(&String::from("PC"), &U16Register(self.pc))
                     .entry(&String::from("SP"), &U8Register(self.sp))
                     .finish()
    }
}
