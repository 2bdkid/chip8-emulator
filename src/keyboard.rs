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

#[derive(Default)]
pub struct Keyboard {
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

impl Keyboard {
    pub fn is_pressed(&self, key: Key) -> bool {
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
