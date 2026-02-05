mod tetromino;

use germterm::{
    color::{Color, lerp},
    crossterm::{
        self,
        event::{
            Event, KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags,
            PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
        },
        execute,
    },
    draw::{Layer, draw_rect, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};
use std::io;

use crate::tetromino::{
    Tetromino, rotate_90_clockwise, rotate_90_counter_clockwise, scale_lightness, tetromino_bitmask,
};

struct TetrominoState {
    bitmask: u16,
    x: i16,
    y: i16,
    color: Color,
}

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20).title("term-tetris").limit_fps(240);
    let mut layer = Layer::new(&mut engine, 0);

    init(&mut engine)?;

    let fall_speed: f32 = 1.0;
    let mut fall_timer: f32 = 0.0;
    let mut controlled_tetromino = TetrominoState {
        bitmask: tetromino_bitmask(&Tetromino::T),
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
                    controlled_tetromino.bitmask =
                        rotate_90_counter_clockwise(controlled_tetromino.bitmask)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    controlled_tetromino.bitmask = rotate_90_clockwise(controlled_tetromino.bitmask)
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

        // draw_block(&mut layer, 8, 6);
        // draw_block(&mut layer, 9, 6);
        // draw_block(&mut layer, 9, 5);
        // draw_block(&mut layer, 10, 6);

        // draw_rect(&mut layer, 10, 10, 8, 1, Color::RED);
        // fill_screen(&mut layer, Color::BLACK);
        // draw_text(&mut layer, 14, 9, "Hello world!");
        // draw_fps_counter(&mut layer, 0, 0);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;

    Ok(())
}

/// Coordinate space: 2x2 twoxel grid
fn draw_tetromino(layer: &mut Layer, tetromino_state: &TetrominoState) {
    for tetromino_y in 0..4 {
        for tetromino_x in 0..4 {
            let pixel_x = tetromino_state.x + tetromino_x;
            let pixel_y = tetromino_state.y + tetromino_y;

            let bit = 1 << (15 - (tetromino_y * 4 + tetromino_x));
            if tetromino_state.bitmask & bit != 0 {
                draw_block(layer, pixel_x, pixel_y, tetromino_state.color)
            }
        }
    }
}

/// Coordinate space: 2x2 twoxel grid
fn draw_block(layer: &mut Layer, x: i16, y: i16, color: Color) {
    let offsets = [
        (0, 0, color),
        (1, 0, scale_lightness(color, 0.9)),
        (0, 1, scale_lightness(color, 0.9)),
        (1, 1, scale_lightness(color, 0.7)),
    ];

    let base_x = x as f32 * 2.0;
    let base_y = y as f32;

    for (dx, dy, color) in offsets {
        draw_twoxel(layer, base_x + dx as f32, base_y + dy as f32 * 0.5, color);
    }
}
