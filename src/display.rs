use std::fmt;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Chip8Display {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Chip8Display {
    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: bool) -> bool {
        if x >= 64 || y >= 32 {
            panic!("Tried to flip pixel that was out of range: ({}, {})", x, y);
        }

        let display_pixel = self
            .pixels
            .chunks_mut(WIDTH)
            .nth(y)
            .unwrap()
            .get_mut(x)
            .unwrap();
        let collision: bool = *display_pixel && pixel;

        *display_pixel ^= pixel;

        collision
    }

    pub fn draw() {
        unimplemented!();
    }

    pub fn clear(&mut self) {
        self.pixels = [false; HEIGHT * WIDTH];
    }
}

impl Default for Chip8Display {
    fn default() -> Chip8Display {
        Chip8Display {
            pixels: [false; HEIGHT * WIDTH],
        }
    }
}

impl fmt::Debug for Chip8Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut screen = String::new();

        for row in self.pixels.chunks(WIDTH) {
            for pixel in row {
                if *pixel {
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
