mod tetromino;

use germterm::{
    color::{Color, lerp},
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Layer, draw_rect, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};
use std::io;

use crate::tetromino::{
    Tetromino, rotate_90_clockwise, rotate_90_counter_clockwise, tetromino_bitmask,
};

struct TetrominoState {
    bitmask: u16,
    x: i16,
    y: i16,
}

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20).title("term-tetris").limit_fps(240);
    let mut layer = Layer::new(&mut engine, 0);

    init(&mut engine)?;

    let fall_speed: f32 = 1.0;
    let mut fall_timer: f32 = 0.0;
    let mut controlled_tetromino = TetrominoState {
        bitmask: tetromino_bitmask(&Tetromino::I),
        x: 4,
        y: 4,
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
                    ..
                }) => {
                    controlled_tetromino.bitmask =
                        rotate_90_counter_clockwise(controlled_tetromino.bitmask)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    ..
                }) => {
                    controlled_tetromino.bitmask = rotate_90_clockwise(controlled_tetromino.bitmask)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => controlled_tetromino.x -= 1,
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
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

        draw_tetromino(
            &mut layer,
            &controlled_tetromino,
            controlled_tetromino.x,
            controlled_tetromino.y,
            Color::TEAL,
        );

        draw_rect(&mut layer, 10, 10, 8, 1, Color::RED);
        // fill_screen(&mut layer, Color::BLACK);
        // draw_text(&mut layer, 14, 9, "Hello world!");
        // draw_fps_counter(&mut layer, 0, 0);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn draw_tetromino(
    layer: &mut Layer,
    tetromino_state: &TetrominoState,
    pos_x: i16,
    pos_y: i16,
    color: Color,
) {
    for y in 0..4 {
        for x in 0..4 {
            let pixel_x = pos_x + x;
            let pixel_y = pos_y + y;

            // Checkerboard dimming to differentiate each pixel a little bit
            let color = if (x + y) % 2 == 0 {
                lerp(color, Color::BLACK.with_alpha(color.a()), 0.15)
            } else {
                color
            };

            let bit = 1 << (15 - (y * 4 + x));
            if tetromino_state.bitmask & bit != 0 {
                draw_twoxel(layer, pixel_x as f32, pixel_y as f32 * 0.5, color);
            }
        }
    }
}
