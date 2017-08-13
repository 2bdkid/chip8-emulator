use std::fmt;

pub struct Chip8Stack {
    array: [u16; 16],
    sp: usize,
}

impl Chip8Stack {
    pub fn push(&mut self, value: u16) {
        if self.sp < 16 {
            self.array[self.sp] = value;
            self.sp += 1;
        } else {
            panic!("Tried to push address to a full stack");
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp > 0 {
            self.sp -= 1;
            self.array[self.sp + 1]
        } else {
            panic!("Tried to pop empty stack");
        }
    }

    pub fn top_address(&self) -> u16 {
        self.array[self.sp]
    }
}

impl Default for Chip8Stack {
    fn default() -> Chip8Stack {
        Chip8Stack {
            array: [0; 16],
            sp: 0,
        }
    }
}

struct Address(u16);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Debug for Chip8Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.array.iter().map(|address| Address(*address))).finish()
    }
}
