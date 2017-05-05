pub struct Chip8Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,

    i: u16,

    delay: u8,
    sound: u8,

    pc: u16,
    sp: u8,

    stack: Box<[u16]>,
}

impl Default for Chip8Registers {
    fn default() -> Chip8Registers {
        Chip8Registers {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,

            i: 0,

            delay: 0,
            sound: 0,

            pc: 0,
            sp: 0,

            stack: vec![0; 16].into_boxed_slice(),
        }
    }
}
