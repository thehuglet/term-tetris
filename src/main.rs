mod coord_space;
mod tetromino;

use germterm::{
    color::Color,
    crossterm::{
        event::{
            Event, KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags,
            PushKeyboardEnhancementFlags,
        },
        execute, terminal,
    },
    draw::{Layer, draw_fps_counter, draw_octad, draw_rect, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};
use std::io;

use crate::{
    coord_space::TetrisBlockCoords,
    tetromino::{
        Rotation, Tetromino, rotate_clockwise, rotate_counter_clockwise, tetromino_bitmask,
    },
};

struct TetrominoState {
    tetromino: Tetromino,
    rotation: Rotation,
    color: Color,
    x: i16,
    y: i16,
}

pub const TERM_WIDTH: usize = 40;
pub const TERM_HEIGHT: usize = 24;
pub const BOARD_SIZE: (usize, usize) = (10, 20);

fn main() -> io::Result<()> {
    let mut engine = Engine::new(TERM_WIDTH as u16, TERM_HEIGHT as u16)
        .title("term-tetris")
        .limit_fps(240);
    let mut layer = Layer::new(&mut engine, 0);

    init(&mut engine)?;
    if terminal::supports_keyboard_enhancement().unwrap_or(false) {
        execute!(
            engine.stdout,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES),
        )?;
    }

    let fall_speed: f32 = 2.0;
    let mut fall_timer: f32 = 0.0;
    let mut controlled_tetromino = TetrominoState {
        tetromino: Tetromino::T,
        rotation: Rotation::North,
        x: 4,
        y: 4,
        color: Color::new(255, 0, 255, 255),
    };

    'update_loop: loop {
        for event in poll_input() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break 'update_loop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('e'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    controlled_tetromino.rotation =
                        rotate_counter_clockwise(controlled_tetromino.rotation)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    controlled_tetromino.rotation = rotate_clockwise(controlled_tetromino.rotation)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                }) => controlled_tetromino.x -= 1,
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                }) => controlled_tetromino.x += 1,
                _ => {}
            }
        }
        start_frame(&mut engine);
        let step_time: f32 = 1.0 / fall_speed;
        fall_timer += engine.delta_time;

        if fall_timer >= step_time {
            fall_timer -= step_time;
            controlled_tetromino.y += 1;
        }

        draw_tetris_board(&mut layer, 8.0, 1.0, &mut controlled_tetromino);

        // draw_fps_counter(&mut layer, 0, 0);
        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn draw_tetris_board(
    layer: &mut Layer,
    origin_pos: TetrisBlockCoords,
    controlled_tetromino: &mut TetrominoState,
) {
    // Border
    let width_octads: i16 = 45;
    let height_octads: i16 = 81;

    let x_offset_octads: i16 = -1;
    let y_offset_octads: i16 = -1;

    for y in 0..=height_octads {
        for x in 0..=width_octads {
            if x == 0 || x == width_octads || y == 0 || y == height_octads {
                let  = Tetromino
                // coord_space::octad_to_standard(x + x_offset_octads, y + y_offset_octads);
                draw_octad(layer, tx + origin_x, ty + origin_y, Color::WHITE);
            }
        }
    }

    draw_tetromino(layer, controlled_tetromino);
}

/// Coordinate space: 2x2 twoxel grid
fn draw_tetromino(layer: &mut Layer, tetromino_state: &TetrominoState) {
    let bitmask = tetromino_bitmask(tetromino_state.tetromino, tetromino_state.rotation);

    for tetromino_y in 0..4 {
        for tetromino_x in 0..4 {
            let pixel_x = tetromino_state.x + tetromino_x;
            let pixel_y = tetromino_state.y + tetromino_y;

            let bit = 1 << (15 - (tetromino_y * 4 + tetromino_x));
            if bitmask & bit != 0 {
                draw_tetris_block(layer, pixel_x, pixel_y, tetromino_state.color)
            }
        }
    }
}

/// Coordinate space: tetris-block
fn draw_tetris_block(layer: &mut Layer, x: i16, y: i16, color: Color) {
    let twoxel_offsets = [
        (0, 0, shift_hue(color, 7.0)),
        (1, 0, scale_lightness(color, 0.75)),
        (0, 1, scale_lightness(color, 0.75)),
        (1, 1, shift_hue(scale_lightness(color, 0.6), -7.0)),
    ];

    for (dx, dy, color) in twoxel_offsets {
        let (x, y) = coord_space::tetris_block_to_standard(x, y);
        let (dx, dy) = coord_space::twoxel_to_standard(dx, dy);
        draw_twoxel(layer, x + dx, y + dy, color);
    }
}

pub fn scale_lightness(c: Color, factor: f32) -> Color {
    let factor_q8 = (factor * 256.0).round() as i32;
    let factor_q8 = factor_q8.clamp(0, 511) as u16;

    let r = ((c.r() as u16).saturating_mul(factor_q8) >> 8).min(255) as u8;
    let g = ((c.g() as u16).saturating_mul(factor_q8) >> 8).min(255) as u8;
    let b = ((c.b() as u16).saturating_mul(factor_q8) >> 8).min(255) as u8;

    Color::new(r, g, b, c.a())
}

pub fn shift_hue(c: Color, degrees: f32) -> Color {
    // Approximate hue rotation in RGB space
    let deg = degrees.to_radians();
    let cos_deg = deg.cos();
    let sin_deg = deg.sin();

    let r = c.r() as f32;
    let g = c.g() as f32;
    let b = c.b() as f32;

    // Rotation matrix for approximate hue shift
    let new_r = (0.299 + 0.701 * cos_deg + 0.168 * sin_deg) * r
        + (0.587 - 0.587 * cos_deg + 0.330 * sin_deg) * g
        + (0.114 - 0.114 * cos_deg - 0.497 * sin_deg) * b;

    let new_g = (0.299 - 0.299 * cos_deg - 0.328 * sin_deg) * r
        + (0.587 + 0.413 * cos_deg + 0.035 * sin_deg) * g
        + (0.114 - 0.114 * cos_deg + 0.292 * sin_deg) * b;

    let new_b = (0.299 - 0.300 * cos_deg + 1.250 * sin_deg) * r
        + (0.587 - 0.588 * cos_deg - 1.050 * sin_deg) * g
        + (0.114 + 0.886 * cos_deg - 0.203 * sin_deg) * b;

    // Clamp to 0..255 and preserve alpha
    let r = new_r.clamp(0.0, 255.0) as u8;
    let g = new_g.clamp(0.0, 255.0) as u8;
    let b = new_b.clamp(0.0, 255.0) as u8;

    Color::new(r, g, b, c.a())
}

// pub mod coord_space {
//     pub fn octad_to_standard(x: i16, y: i16) -> (f32, f32) {
//         (x as f32 / 2.0, y as f32 / 4.0)
//     }

//     pub fn standard_to_octad(x: f32, y: f32) -> (i16, i16) {
//         ((x * 2.0).round() as i16, (y * 4.0).round() as i16)
//     }

//     pub fn tetris_block_to_standard(x: i16, y: i16) -> (f32, f32) {
//         (x as f32 * 2.0, y as f32)
//     }

//     pub fn standard_to_tetris_block(x: f32, y: f32) -> (i16, i16) {
//         ((x / 2.0).round() as i16, y.round() as i16)
//     }

//     pub fn twoxel_to_standard(x: i16, y: i16) -> (f32, f32) {
//         (x as f32, y as f32 * 0.5)
//     }
// }
