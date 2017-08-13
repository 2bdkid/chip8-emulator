#[derive(Clone, Copy)]
pub enum Key {
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

pub trait toKey {
    fn to_key(&self) -> Key;
}

impl toKey for Key {
    fn to_key(&self) -> Key {
        *self
    }
}

impl toKey for u8 {
    fn to_key(&self) -> Key {
        match *self {
            0 => Key::Zero,
            1 => Key::One,
            2 => Key::Two,
            3 => Key::Three,
            4 => Key::Four,
            5 => Key::Five,
            6 => Key::Six,
            7 => Key::Seven,
            8 => Key::Eight,
            9 => Key::Nine,
            10 => Key::A,
            11 => Key::B,
            12 => Key::C,
            13 => Key::D,
            14 => Key::E,
            15 => Key::F,
            key @ _ => panic!(format!("Tried to create invalid key from {}", key)),
        }
    }
}

#[derive(Default)]
pub struct Chip8Keyboard {
    zero: bool,
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    five: bool,
    six: bool,
    seven: bool,
    eight: bool,
    nine: bool,
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
}

impl Chip8Keyboard {
    pub fn is_pressed<T: toKey>(&self, key: T) -> bool {
        let key = key.to_key();

        match key {
            Key::Zero => self.zero,
            Key::One => self.one,
            Key::Two => self.two,
            Key::Three => self.three,
            Key::Four => self.four,
            Key::Five => self.five,
            Key::Six => self.six,
            Key::Seven => self.seven,
            Key::Eight => self.eight,
            Key::Nine => self.nine,
            Key::A => self.a,
            Key::B => self.b,
            Key::C => self.c,
            Key::D => self.d,
            Key::E => self.e,
            Key::F => self.f,
        }
    }

    // TODO: poll keys and update inner variables. Possibly use ncurses for this
    pub fn poll(&mut self) {
        unimplemented!();
    }
}

pub fn get_key() -> Key {
    unimplemented!();
}
