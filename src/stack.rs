#[derive(Debug)]
pub struct Chip8Stack {
    array: Box<[u16]>,
    sp: u8,
}

impl Chip8Stack {
    pub fn push(&mut self, value: u16) {
        if self.sp < 16 {
            self.array[self.sp as usize] = value;
            self.sp += 1;
        } else {
            panic!("Tried to push address to a full stack");
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp > 0 {
            self.sp -= 1;
            self.array[self.sp as usize]
        } else {
            panic!("Tried to pop empty stack");
        }
    }

    pub fn top_address(&self) -> u8 {
        if self.sp == 0 {
            0
        } else {
            self.sp - 1
        }
    }
}

impl Default for Chip8Stack {
    fn default() -> Chip8Stack {
        Chip8Stack {
            array: vec![0; 16].into_boxed_slice(),
            sp: 0,
        }
    }
}
