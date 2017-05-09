pub struct Chip8Sprite {
    bytes: Box<[u8]>
}

pub const CHAR_ZERO: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
pub const CHAR_ONE: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
pub const CHAR_TWO: [u8; 5] = [0xf0, 0x10, 0xf0, 0x80, 0xf0];
pub const CHAR_THREE: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
pub const CHAR_FOUR: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
pub const CHAR_FIVE: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
pub const CHAR_SIX: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
pub const CHAR_SEVEN: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
pub const CHAR_EIGHT: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
pub const CHAR_NINE: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
pub const CHAR_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
pub const CHAR_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
pub const CHAR_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
pub const CHAR_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
pub const CHAR_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
pub const CHAR_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];
