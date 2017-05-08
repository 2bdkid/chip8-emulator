use std::fmt;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Chip8Display {
    pixels: [[bool; HEIGHT]; WIDTH]
}

impl Chip8Display {
    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        if x >=64 || y >= 32 {
            panic!("Tried to flip pixel that was out of range: ({}, {})", x, y);
        }

        let state = self.pixels[x][y];
        self.pixels[x][y] = !state;
    }
}

impl Default for Chip8Display {
    fn default() -> Chip8Display {
        Chip8Display {
            pixels: [[false; HEIGHT]; WIDTH]
        }
    }
}

impl fmt::Debug for Chip8Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut screen = String::new();
        for i in 0..32 {
            for j in 0..64 {
                if self.pixels[j][i] {
                    screen += "*";
                } else {
                    screen += "-";
                }
            }
            screen += "\n";
        }

        f.write_str(screen.as_ref())
    }
}
