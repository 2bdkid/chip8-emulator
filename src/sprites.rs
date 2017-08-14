pub struct Chip8Sprite {
    bytes: Vec<u8>,
}

pub fn get_location(sprite: ASCIISprite) -> usize {
    // these are the corresponding indexes to the start of the sprites in memory
    match sprite {
        ASCIISprite::Zero => 0,
        ASCIISprite::One => 5,
        ASCIISprite::Two => 10,
        ASCIISprite::Three => 15,
        ASCIISprite::Four => 20,
        ASCIISprite::Five => 25,
        ASCIISprite::Six => 30,
        ASCIISprite::Seven => 35,
        ASCIISprite::Eight => 40,
        ASCIISprite::Nine => 45,
        ASCIISprite::A => 50,
        ASCIISprite::B => 55,
        ASCIISprite::C => 60,
        ASCIISprite::D => 65,
        ASCIISprite::E => 70,
        ASCIISprite::F => 75,
    }
}

impl Chip8Sprite {
    pub fn new(bytes: &[u8]) -> Chip8Sprite {
        Chip8Sprite { bytes: bytes.to_vec() }
    }

    pub fn zero() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xf0, 0x90, 0x90, 0x90, 0xf0] }
    }

    pub fn one() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xf0, 0x90, 0x90, 0x90, 0xf0] }
    }

    pub fn two() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xf0, 0x10, 0xf0, 0x80, 0xf0] }
    }

    pub fn three() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x10, 0xF0, 0x10, 0xF0] }
    }

    pub fn four() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0x90, 0x90, 0xF0, 0x10, 0x10] }
    }

    pub fn five() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x80, 0xF0, 0x10, 0xF0] }
    }

    pub fn six() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x80, 0xF0, 0x90, 0xF0] }
    }

    pub fn seven() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x10, 0x20, 0x40, 0x40] }
    }

    pub fn eight() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x90, 0xF0, 0x90, 0xF0] }
    }

    pub fn nine() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x90, 0xF0, 0x10, 0xF0] }
    }

    pub fn a() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x90, 0xF0, 0x90, 0x90] }
    }

    pub fn b() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xE0, 0x90, 0xE0, 0x90, 0xE0] }
    }
    pub fn c() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x80, 0x80, 0x80, 0xF0] }
    }
    pub fn d() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xE0, 0x90, 0x90, 0x90, 0xE0] }
    }
    pub fn e() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x80, 0xF0, 0x80, 0xF0] }
    }
    pub fn f() -> Chip8Sprite {
        Chip8Sprite { bytes: vec![0xF0, 0x80, 0xF0, 0x80, 0x80] }
    }
}

#[derive(Copy, Clone)]
pub enum ASCIISprite {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl ASCIISprite {
    pub fn new(sprite: u8) -> ASCIISprite {
        match sprite {
            0 => ASCIISprite::Zero,
            1 => ASCIISprite::One,
            2 => ASCIISprite::Two,
            3 => ASCIISprite::Three,
            4 => ASCIISprite::Four,
            5 => ASCIISprite::Five,
            6 => ASCIISprite::Six,
            7 => ASCIISprite::Seven,
            8 => ASCIISprite::Eight,
            9 => ASCIISprite::Nine,
            10 => ASCIISprite::A,
            11 => ASCIISprite::B,
            12 => ASCIISprite::C,
            13 => ASCIISprite::D,
            14 => ASCIISprite::E,
            15 => ASCIISprite::F,
            _ => panic!(format!("Tried to create ASCII sprite from value {}", sprite)),
        }
    }
}
