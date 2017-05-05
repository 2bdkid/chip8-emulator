use std::fmt;

const MEMORY_SIZE: usize = 4096;

pub struct Chip8Memory {
    memory_bank: Box<[u8]>
}

impl Chip8Memory {
    pub fn write_value(&mut self, location: usize, value: u8) {
        if let Some(memory_reference) = self.memory_bank.get_mut(location) {
            *memory_reference = value;
        } else {
            panic!("Tried to access invalid memory location: {}", location);
        }
    }
}

impl Default for Chip8Memory {
    fn default() -> Chip8Memory {
        Chip8Memory {
            memory_bank: vec![0; MEMORY_SIZE].into_boxed_slice()
        }
    }
}

// This types only purpose is to print a usize formatted in hex
struct Address(usize);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl fmt::Debug for Chip8Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.memory_bank.iter()
                                              .enumerate()
                                              .map(|(index, value)| (Address(index), value)))
                                              .finish()
    }
}
