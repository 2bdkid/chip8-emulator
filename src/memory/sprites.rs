pub struct Chip8Sprite {
    bytes: Vec<u8>
}

impl Chip8Sprite {
    pub fn new(bytes: &[u8]) -> Chip8Sprite {
        Chip8Sprite {
            bytes: bytes.to_vec()
        }
    }

    pub fn zero() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xf0, 0x90, 0x90, 0x90, 0xf0]
        }
    }

    pub fn one() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xf0, 0x90, 0x90, 0x90, 0xf0]
        }
    }

    pub fn two() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xf0, 0x10, 0xf0, 0x80, 0xf0]
        }
    }

    pub fn three() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x10, 0xF0, 0x10, 0xF0]
        }
    }

    pub fn four() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0x90, 0x90, 0xF0, 0x10, 0x10]
        }
    }

    pub fn five() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x80, 0xF0, 0x10, 0xF0]
        }
    }

    pub fn six() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x80, 0xF0, 0x90, 0xF0]
        }
    }

    pub fn seven() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x10, 0x20, 0x40, 0x40]
        }
    }

    pub fn eight() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x90, 0xF0, 0x90, 0xF0]
        }
    }

    pub fn nine() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x90, 0xF0, 0x10, 0xF0]
        }
    }

    pub fn a() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x90, 0xF0, 0x90, 0x90]
        }
    }

    pub fn b() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xE0, 0x90, 0xE0, 0x90, 0xE0]
        }
    }
    pub fn c() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x80, 0x80, 0x80, 0xF0]
        }
    }
    pub fn d() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xE0, 0x90, 0x90, 0x90, 0xE0]
        }
    }
    pub fn e() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x80, 0xF0, 0x80, 0xF0]
        }
    }
    pub fn f() -> Chip8Sprite {
        Chip8Sprite {
            bytes: vec![0xF0, 0x80, 0xF0, 0x80, 0x80]
        }
    }
}
