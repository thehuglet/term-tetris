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
    draw::{Layer, draw_fps_counter, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};
use std::io;

use crate::tetromino::{
    Rotation, Tetromino, rotate_clockwise, rotate_counter_clockwise, tetromino_bitmask,
};

struct TetrominoState {
    tetromino: Tetromino,
    rotation: Rotation,
    color: Color,
    x: i16,
    y: i16,
}

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20).title("term-tetris").limit_fps(240);
    let mut layer = Layer::new(&mut engine, 0);

    init(&mut engine)?;
    if terminal::supports_keyboard_enhancement().unwrap_or(false) {
        execute!(
            engine.stdout,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES),
        )?;
    }

    let fall_speed: f32 = 1.0;
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

        draw_tetromino(&mut layer, &controlled_tetromino);

        draw_fps_counter(&mut layer, 0, 0);
        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;

    Ok(())
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
                draw_block(layer, pixel_x, pixel_y, tetromino_state.color)
            }
        }
    }
}

/// Coordinate space: 2x2 twoxel grid
fn draw_block(layer: &mut Layer, x: i16, y: i16, color: Color) {
    let offsets = [
        (0, 0, shift_hue(color, 7.0)),
        (1, 0, scale_lightness(color, 0.8)),
        (0, 1, scale_lightness(color, 0.8)),
        (1, 1, shift_hue(scale_lightness(color, 0.6), -7.0)),
    ];

    let base_x = x as f32 * 2.0;
    let base_y = y as f32;

    for (dx, dy, color) in offsets {
        draw_twoxel(layer, base_x + dx as f32, base_y + dy as f32 * 0.5, color);
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
