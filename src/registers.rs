#[derive(Clone, Copy, Debug)]
struct Register8(u8);

impl From<u8> for Register8 {
    fn from(value: u8) -> Register8 {
        Register8(value)
    }
}

#[derive(Clone, Copy, Debug)]
struct Register16(u16);

impl From<u16> for Register16 {
    fn from(value: u16) -> Register16 {
        Register16(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Registers {
    pub general: [Register8; 16],
    pub delay: Register8,
    pub sound: Register8,
    pub i: Register16,
    pub pc: Register16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            general: [0.into(); 16],
            delay: 0.into(),
            sound: 0.into(),
            i: 0.into(),
            pc: 512.into(),
        }
    }
}
