const fn build_tetromino_bitmasks(
    north: [&str; 4],
    east: [&str; 4],
    south: [&str; 4],
    west: [&str; 4],
) -> [u16; 4] {
    let mut rotation_index = 0;
    let all_rotations: [[&str; 4]; 4] = [north, east, south, west];
    let mut all_rotations_bitmasks: [u16; 4] = [0, 0, 0, 0];

    while rotation_index < 4 {
        let mut row = 0;

        while row < 4 {
            let bytes = all_rotations[rotation_index][row].as_bytes();
            let mut x = 0;
            while x < 4 {
                if bytes[x] == b'x' {
                    all_rotations_bitmasks[rotation_index] |= 1 << (15 - (row * 4 + x));
                }
                x += 1;
            }
            row += 1;
        }
        rotation_index += 1
    }
    all_rotations_bitmasks
}

#[rustfmt::skip]
mod tetromino_bitmask_definitions {
    use crate::tetromino::build_tetromino_bitmasks;

    pub const I: [u16; 4] = build_tetromino_bitmasks(
        [
            "----",
            "xxxx",
            "----",
            "----",
        ],
        [
            "--x-",
            "--x-",
            "--x-",
            "--x-",
        ],
        [
            "----",
            "----",
            "xxxx",
            "----",
        ],
        [
            "-x--",
            "-x--",
            "-x--",
            "-x--",
        ],
    );

    pub const J: [u16; 4] = build_tetromino_bitmasks(
        [
            "x---",
            "xxx-",
            "----",
            "----",
        ],
        [
            "-xx-",
            "-x--",
            "-x--",
            "----",
        ],
        [
            "----",
            "xxx-",
            "--x-",
            "----",
        ],
        [
            "-x--",
            "-x--",
            "xx--",
            "----",
        ],
    );

    pub const L: [u16; 4] = build_tetromino_bitmasks(
        [
            "--x-",
            "xxx-",
            "----",
            "----",
        ],
        [
            "-x--",
            "-x--",
            "-xx-",
            "----",
        ],
        [
            "----",
            "xxx-",
            "x---",
            "----",
        ],
        [
            "xx--",
            "-x--",
            "-x--",
            "----",
        ],
    );


    pub const O: [u16; 4] = build_tetromino_bitmasks(
        [
            "-xx-",
            "-xx-",
            "----",
            "----",
        ],
        [
            "-xx-",
            "-xx-",
            "----",
            "----",
        ],
        [
            "-xx-",
            "-xx-",
            "----",
            "----",
        ],
        [
            "-xx-",
            "-xx-",
            "----",
            "----",
        ],
    );


    pub const S: [u16; 4] = build_tetromino_bitmasks(
        [
            "-xx-",
            "xx--",
            "----",
            "----",
        ],
        [
            "-x--",
            "-xx-",
            "--x-",
            "----",
        ],
        [
            "----",
            "-xx-",
            "xx--",
            "----",
        ],
        [
            "x---",
            "xx--",
            "-x--",
            "----",
        ],
    );

    pub const T: [u16; 4] = build_tetromino_bitmasks(
        [
            "-x--",
            "xxx-",
            "----",
            "----",
        ],
        [
            "-x--",
            "-xx-",
            "-x--",
            "----",
        ],
        [
            "----",
            "xxx-",
            "-x---",
            "----",
        ],
        [
            "-x--",
            "xx--",
            "-x--",
            "----",
        ],
    );


    pub const Z: [u16; 4] = build_tetromino_bitmasks(
        [
            "xx--",
            "-xx-",
            "----",
            "----",
        ],
        [
            "--x-",
            "-xx-",
            "-x--",
            "----",
        ],
        [
            "----",
            "xx--",
            "-xx-",
            "----",
        ],
        [
            "-x--",
            "xx--",
            "x---",
            "----",
        ],
    );
}

#[derive(Clone, Copy)]
pub enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Rotation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub fn tetromino_bitmask(tetromino: Tetromino, rotation: Rotation) -> u16 {
    use tetromino_bitmask_definitions as bitmasks;

    let rotation_index: usize = rotation as usize;

    match tetromino {
        Tetromino::I => bitmasks::I[rotation_index],
        Tetromino::O => bitmasks::O[rotation_index],
        Tetromino::T => bitmasks::T[rotation_index],
        Tetromino::J => bitmasks::J[rotation_index],
        Tetromino::L => bitmasks::L[rotation_index],
        Tetromino::S => bitmasks::S[rotation_index],
        Tetromino::Z => bitmasks::Z[rotation_index],
    }
}

pub fn rotate_clockwise(rotation: Rotation) -> Rotation {
    match rotation {
        Rotation::North => Rotation::East,
        Rotation::East => Rotation::South,
        Rotation::South => Rotation::West,
        Rotation::West => Rotation::North,
    }
}

pub fn rotate_counter_clockwise(rotation: Rotation) -> Rotation {
    match rotation {
        Rotation::North => Rotation::West,
        Rotation::West => Rotation::South,
        Rotation::South => Rotation::East,
        Rotation::East => Rotation::North,
    }
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
