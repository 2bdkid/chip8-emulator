use std::fmt;

const MEMORY_SIZE: usize = 4096;

pub struct Chip8Memory {
    memory_bank: [u8; MEMORY_SIZE],
}

impl Chip8Memory {
    pub fn write(&mut self, location: usize, value: u8) {
        *self.memory_bank
            .get_mut(location)
            .expect(format!("Tried to write invalid memory location: {:#x}", location).as_ref()) =
            value
    }

    pub fn read(&self, location: usize) -> u8 {
        *self.memory_bank
            .get(location)
            .expect(format!("Tried to read invalid memory location: {:#x}", location).as_ref())
    }

    pub fn read_instruction(&self, location: usize) -> u16 {
        let first = *self.memory_bank
            .get(location)
            .expect(format!("Tried to read invalid memory location: {:#x}", location).as_ref());

        let second = *self.memory_bank
            .get(location + 1)
            .expect(format!("Tried to read invalid memory location: {:#x}",
                            location + 1)
                .as_ref());

        (first as u16) << 8 | second as u16
    }
}

impl Default for Chip8Memory {
    fn default() -> Chip8Memory {
        let mut memory = [0u8; MEMORY_SIZE];

        // This is where I insert the ASCII sprites defined by chip8
        // Zero
        memory[0] = 0xF0;
        memory[1] = 0x90;
        memory[2] = 0x90;
        memory[3] = 0x90;
        memory[4] = 0xF0;
        // One
        memory[5] = 0x20;
        memory[6] = 0x60;
        memory[7] = 0x20;
        memory[8] = 0x20;
        memory[9] = 0x70;
        // Two
        memory[10] = 0xF0;
        memory[11] = 0x10;
        memory[12] = 0xF0;
        memory[13] = 0x80;
        memory[14] = 0xF0;

        // Three
        memory[15] = 0xF0;
        memory[16] = 0x10;
        memory[17] = 0xF0;
        memory[18] = 0x10;
        memory[19] = 0xF0;

        // Four
        memory[20] = 0x90;
        memory[21] = 0x90;
        memory[22] = 0xF0;
        memory[23] = 0x10;
        memory[24] = 0x10;

        // Five
        memory[25] = 0xF0;
        memory[26] = 0x80;
        memory[27] = 0xF0;
        memory[28] = 0x10;
        memory[29] = 0xF0;

        // Six
        memory[30] = 0xF0;
        memory[31] = 0x80;
        memory[32] = 0xF0;
        memory[33] = 0x90;
        memory[34] = 0xF0;

        // Seven
        memory[35] = 0xF0;
        memory[36] = 0x10;
        memory[37] = 0x20;
        memory[38] = 0x40;
        memory[39] = 0x40;

        // Eight
        memory[40] = 0xF0;
        memory[41] = 0x90;
        memory[42] = 0xF0;
        memory[43] = 0x90;
        memory[44] = 0xF0;

        // Nine
        memory[45] = 0xF0;
        memory[46] = 0x90;
        memory[47] = 0xF0;
        memory[48] = 0x10;
        memory[49] = 0xF0;

        // A
        memory[50] = 0xF0;
        memory[51] = 0x90;
        memory[52] = 0xF0;
        memory[53] = 0x90;
        memory[54] = 0x90;

        // B
        memory[55] = 0xE0;
        memory[56] = 0x90;
        memory[57] = 0xE0;
        memory[58] = 0x90;
        memory[59] = 0xE0;

        // C
        memory[60] = 0xF0;
        memory[61] = 0x80;
        memory[62] = 0x80;
        memory[63] = 0x80;
        memory[64] = 0xF0;

        // D
        memory[65] = 0xE0;
        memory[66] = 0x90;
        memory[67] = 0x90;
        memory[68] = 0x90;
        memory[69] = 0xE0;

        // E
        memory[70] = 0xF0;
        memory[71] = 0x80;
        memory[72] = 0xF0;
        memory[73] = 0x80;
        memory[74] = 0xF0;

        // F
        memory[75] = 0xF0;
        memory[76] = 0x80;
        memory[77] = 0xF0;
        memory[78] = 0x80;
        memory[79] = 0x80;

        Chip8Memory { memory_bank: memory }
    }
}

// This types only purpose is to print a usize formatted in hex
// Rust wouldn't let me reimpl fmt::Debug on usize so I made a wrapper
struct Address(usize);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

// Same deal. Just a wrapper over u8
struct Value(u8);

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Debug for Chip8Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.memory_bank
                .iter()
                .enumerate()
                .map(|(index, value)| (Address(index), Value(*value))))
            .finish()
    }
}
