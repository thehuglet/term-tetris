use germterm::color::Color;

const fn build_tetromino_bitmask(rows: [&str; 4]) -> u16 {
    let mut bits = 0u16;
    let mut y = 0;
    while y < 4 {
        let bytes = rows[y].as_bytes();
        let mut x = 0;
        while x < 4 {
            if bytes[x] == b'X' {
                bits |= 1 << (15 - (y * 4 + x));
            }
            x += 1;
        }
        y += 1;
    }
    bits
}

#[rustfmt::skip]
pub mod tetromino_definitions {
    use crate::tetromino::build_tetromino_bitmask;

    pub const I_NORTH: u16 = build_tetromino_bitmask([
        "----",
        "XXXX",
        "----",
        "----",
    ]);

    pub const I_EAST: u16 = build_tetromino_bitmask([
        "--X-",
        "--X-",
        "--X-",
        "--X-",
    ]);

    pub const I_SOUTH: u16 = build_tetromino_bitmask([
        "----",
        "----",
        "XXXX",
        "----",
    ]);

    pub const I_WEST: u16 = build_tetromino_bitmask([
        "-x--",
        "-X--",
        "-x--",
        "-X--",
    ]);

    pub const T_NORTH: u16 = build_tetromino_bitmask([
        "-X--",
        "XXX-",
        "----",
        "----",
    ]);

    pub const T_EAST: u16 = build_tetromino_bitmask([
        "-X--",
        "-XX-",
        "-X--",
        "----",
    ]);

    pub const T_SOUTH: u16 = build_tetromino_bitmask([
        "----",
        "xxx-",
        "-x--",
        "----",
    ]);

    pub const T_WEST: u16 = build_tetromino_bitmask([
        "-x--",
        "XX--",
        "-x--",
        "----",
    ]);
}

#[rustfmt::skip]
pub const I_TETROMINO: u16 = build_tetromino_bitmask([
    "----",
    "XXXX",
    "----",
    "----",
]);

#[rustfmt::skip]
pub const O_TETROMINO: u16 = build_tetromino_bitmask([
    "XX--",
    "XX--",
    "----",
    "----",
]);

#[rustfmt::skip]
pub const T_TETROMINO: u16 = build_tetromino_bitmask([
    "XXX-",
    "-X--",
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

    normalize_mask(new)
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

    normalize_mask(new)
}

pub fn scale_lightness(c: Color, factor: f32) -> Color {
    let factor_q8 = (factor * 256.0).round() as i32;
    let factor_q8 = factor_q8.clamp(0, 511) as u16;

    let r = ((c.r() as u16).saturating_mul(factor_q8) >> 8).min(255) as u8;
    let g = ((c.g() as u16).saturating_mul(factor_q8) >> 8).min(255) as u8;
    let b = ((c.b() as u16).saturating_mul(factor_q8) >> 8).min(255) as u8;

    Color::new(r, g, b, c.a())
}

fn normalize_mask(mask: u16) -> u16 {
    let mut min_x = 4;
    let mut min_y = 4;

    for y in 0..4 {
        for x in 0..4 {
            let bit = 1 << (15 - (y * 4 + x));
            if mask & bit != 0 {
                if x < min_x {
                    min_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
            }
        }
    }

    let mut new = 0u16;
    for y in 0..4 {
        for x in 0..4 {
            let bit = 1 << (15 - (y * 4 + x));
            if mask & bit != 0 {
                let nx = x - min_x;
                let ny = y - min_y;
                new |= 1 << (15 - (ny * 4 + nx));
            }
        }
    }

    new
}
