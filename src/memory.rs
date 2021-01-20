use std::fmt;
use std::error::Error;
use sprites::Sprite;
use refinement::{Refinement, Predicate};

const MEMORY_SIZE: usize = 4096;

#[derive(Copy, Clone, Debug)]
pub struct InvalidAddress(usize);

impl Error for InvalidAddress {}

impl fmt::Display for InvalidAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid address {:#}", self.0)
    }
}

struct LessThan4k;

impl Predicate<usize> for LessThan4k {
    fn test(addr: &usize) -> bool {
        addr < 4096
    }
}

pub type Address = Refinement<usize, LessThan4k>;

impl Address {
    fn next(&self) -> Result<Self, InvalidAddress> {
        match self + 1 {
            Some(addr) => Ok(addr),
            None => Err(InvalidAddress(self.as_inner() + 1)),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:#}", self.as_inner())
    }
}

pub struct Memory {
    memory_bank: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn write(&mut self, addr: Address, value: u8) {
        self.memory_bank[addr.as_inner()] = value;
    }

    pub fn read(&self, addr: Address) -> u8 {
        self.memory_bank[addr.as_inner()]
    }

    pub fn read_instruction(&self, addr: Address) -> Result<u16, InvalidAddress> {
        let msb = self.read(addr);
        let lsb = self.read(addr.next()?);
        Ok((msb as u16) << 8 | lsb as u16)
    }

    pub fn fill_program(&self, program: &[u8]) -> Result<(), InvalidAddress> {
        for (i, &b) in program.iter().enumerate() {
            self.write(Address::new(512 + i)?, b);
        }
        Ok(())
    }

    pub fn sprite(&self, sprite: Sprite) -> &[u8] {
        let start = sprite as usize * 5;
        &self.memory_bank[start..start + 5]
    }
}

impl Memory {
    fn new() -> Memory {
        let mut memory = [0u8; MEMORY_SIZE];

        // ascii sprites
        // zero
        memory[0] = 0xF0;
        memory[1] = 0x90;
        memory[2] = 0x90;
        memory[3] = 0x90;
        memory[4] = 0xF0;
        // one
        memory[5] = 0x20;
        memory[6] = 0x60;
        memory[7] = 0x20;
        memory[8] = 0x20;
        memory[9] = 0x70;
        // two
        memory[10] = 0xF0;
        memory[11] = 0x10;
        memory[12] = 0xF0;
        memory[13] = 0x80;
        memory[14] = 0xF0;

        // three
        memory[15] = 0xF0;
        memory[16] = 0x10;
        memory[17] = 0xF0;
        memory[18] = 0x10;
        memory[19] = 0xF0;

        // four
        memory[20] = 0x90;
        memory[21] = 0x90;
        memory[22] = 0xF0;
        memory[23] = 0x10;
        memory[24] = 0x10;

        // five
        memory[25] = 0xF0;
        memory[26] = 0x80;
        memory[27] = 0xF0;
        memory[28] = 0x10;
        memory[29] = 0xF0;

        // six
        memory[30] = 0xF0;
        memory[31] = 0x80;
        memory[32] = 0xF0;
        memory[33] = 0x90;
        memory[34] = 0xF0;

        // seven
        memory[35] = 0xF0;
        memory[36] = 0x10;
        memory[37] = 0x20;
        memory[38] = 0x40;
        memory[39] = 0x40;

        // eight
        memory[40] = 0xF0;
        memory[41] = 0x90;
        memory[42] = 0xF0;
        memory[43] = 0x90;
        memory[44] = 0xF0;

        // nine
        memory[45] = 0xF0;
        memory[46] = 0x90;
        memory[47] = 0xF0;
        memory[48] = 0x10;
        memory[49] = 0xF0;

        // a
        memory[50] = 0xF0;
        memory[51] = 0x90;
        memory[52] = 0xF0;
        memory[53] = 0x90;
        memory[54] = 0x90;

        // b
        memory[55] = 0xE0;
        memory[56] = 0x90;
        memory[57] = 0xE0;
        memory[58] = 0x90;
        memory[59] = 0xE0;

        // c
        memory[60] = 0xF0;
        memory[61] = 0x80;
        memory[62] = 0x80;
        memory[63] = 0x80;
        memory[64] = 0xF0;

        // d
        memory[65] = 0xE0;
        memory[66] = 0x90;
        memory[67] = 0x90;
        memory[68] = 0x90;
        memory[69] = 0xE0;

        // e
        memory[70] = 0xF0;
        memory[71] = 0x80;
        memory[72] = 0xF0;
        memory[73] = 0x80;
        memory[74] = 0xF0;

        // f
        memory[75] = 0xF0;
        memory[76] = 0x80;
        memory[77] = 0xF0;
        memory[78] = 0x80;
        memory[79] = 0x80;

        Memory {
            memory_bank: memory,
        }
    }
}
