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
