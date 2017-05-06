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
    I,
    Delay,
    Sound,
    PC,
    SP,
}

// TODO: impl fmt::Debug to print values in hexadecimal
#[derive(Debug)]
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
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,

    // TODO: implement a basic stack wrapper
    stack: Vec<u16>,
}

impl Chip8Registers {
    pub fn read_u8(&self, register: Register) -> u8 {
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
            Register::Delay => self.delay,
            Register::Sound => self.sound,
            Register::SP => self.sp,
            _ => panic!("Tried to read 8 bits from a 16 bit register"),
        }
    }

    pub fn write_u8(&mut self, register: Register, value: u8) {
        match register {
            Register::V0 => self.v0 = value,
            Register::V1 => self.v1 = value,
            Register::V2 => self.v2 = value,
            Register::V3 => self.v3 = value,
            Register::V4 => self.v4 = value,
            Register::V5 => self.v5 = value,
            Register::V6 => self.v6 = value,
            Register::V7 => self.v7 = value,
            Register::V8 => self.v8 = value,
            Register::V9 => self.v9 = value,
            Register::VA => self.va = value,
            Register::VB => self.vb = value,
            Register::VC => self.vc = value,
            Register::VD => self.vd = value,
            Register::VE => self.ve = value,
            Register::VF => self.vf = value,
            Register::Delay => self.delay = value,
            Register::Sound => self.sound = value,
            Register::SP => self.sp = value,
            _ => panic!("Tried to write 8 bits to a 16 bit register"),
        }
    }

    pub fn read_u16(&self, register: Register) -> u16 {
        match register {
            Register::I => self.i,
            Register::PC => self.pc,
            _ => panic!("Tried to read 16 bits from an 8 bit register"),
        }
    }

    pub fn write_u16(&mut self, register: Register, value: u16) {
        match register {
            Register::I => self.i = value,
            Register::PC => self.pc = value,
            _ => panic!("Tried to write 16 bits to an 8 bit register"),
        }
    }

    // TODO: Should push value onto stack and increase SP register
    pub fn push_stack(&mut self, value: u16) {
        unimplemented!();
    }

    // TODO: Should pop value on top of stack and decrease SP register
    pub fn pop_stack(&mut self) -> u16 {
        unimplemented!();
    }
}

impl Default for Chip8Registers {
    fn default() -> Chip8Registers {
        Chip8Registers {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,

            i: 0,

            delay: 0,
            sound: 0,

            pc: 0,
            sp: 0,

            stack: vec![0; 16],
        }
    }
}
