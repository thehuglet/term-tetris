const fn build_tetromino_bitmask(rows: [&str; 4]) -> u16 {
    let mut bits = 0u16;
    let mut y = 0;
    while y < 4 {
        let bytes = rows[y].as_bytes();
        let mut x = 0;
        while x < 4 {
            if bytes[x] == b'o' {
                bits |= 1 << (15 - (y * 4 + x));
            }
            x += 1;
        }
        y += 1;
    }
    bits
}

#[rustfmt::skip]
pub const I_TETROMINO: u16 = build_tetromino_bitmask([
    "----",
    "oooo",
    "----",
    "----",
]);

#[rustfmt::skip]
pub const O_TETROMINO: u16 = build_tetromino_bitmask([
    "oo--",
    "oo--",
    "----",
    "----",
]);

#[rustfmt::skip]
pub const T_TETROMINO: u16 = build_tetromino_bitmask([
    "ooo-",
    "-o--",
    "----",
    "----",
]);

#[rustfmt::skip]
pub const J_TETROMINO: u16 = build_tetromino_bitmask([
    "-o--",
    "-o--",
    "oo--",
    "----",
]);

#[rustfmt::skip]
pub const L_TETROMINO: u16 = build_tetromino_bitmask([
    "o---",
    "o---",
    "oo--",
    "----",
]);

#[rustfmt::skip]
pub const S_TETROMINO: u16 = build_tetromino_bitmask([
    "-oo-",
    "oo--",
    "----",
    "----",
]);

#[rustfmt::skip]
pub const Z_TETROMINO: u16 = build_tetromino_bitmask([
    "oo--",
    "-oo-",
    "----",
    "----",
]);

pub enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

pub fn tetromino_bitmask(tetromino: &Tetromino) -> u16 {
    match tetromino {
        Tetromino::I => I_TETROMINO,
        Tetromino::O => O_TETROMINO,
        Tetromino::T => T_TETROMINO,
        Tetromino::J => J_TETROMINO,
        Tetromino::L => L_TETROMINO,
        Tetromino::S => S_TETROMINO,
        Tetromino::Z => Z_TETROMINO,
    }
}

pub fn rotate_90_clockwise(mask: u16) -> u16 {
    let mut new = 0u16;

    for y in 0..4 {
        for x in 0..4 {
            let bit = 1 << (15 - (y * 4 + x));
            if mask & bit != 0 {
                // CW rotation
                let new_x = 3 - y;
                let new_y = x;
                new |= 1 << (15 - (new_y * 4 + new_x));
            }
        }
    }

    new
}

pub fn rotate_90_counter_clockwise(mask: u16) -> u16 {
    let mut new = 0u16;

    for y in 0..4 {
        for x in 0..4 {
            let bit = 1 << (15 - (y * 4 + x));
            if mask & bit != 0 {
                // CW: (x, y) -> (y, 3 - x)
                new |= 1 << (15 - (y + (3 - x) * 4));
            }
        }
    }

    new
}
