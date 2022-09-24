use std::process::exit;

use piston::{self, Button, Key, PressEvent, WindowSettings};
use piston_window::{self, PistonWindow, *};

mod lib;
use lib::assets::{snake::Direction, *};
use lib::main::*;

pub const STEP: i8 = 10;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [560, 480])
        .build()
        .unwrap_or_else(|_| {
            println!("Failed to create window!");
            exit(-1);
        });

    let mut snake = snake::Snake::new();
    while let Some(event) = window.next() {
        let size = window.size();
        let (width, height) = (size.width, size.height);
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let mut handle_dir = |key: Key| {
                let dir = match key {
                    Key::Up => Some(Direction::Up),
                    Key::Down => Some(Direction::Down),
                    Key::Left => Some(Direction::Left),
                    Key::Right => Some(Direction::Right),
                    _ => None,
                };
                if let Some(dir) = dir {
                    snake.step(dir);
                }
            };

            match key {
                // Directional input
                Key::Up | Key::Down | Key::Left | Key::Right => {
                    handle_dir(key);
                }
                Key::Space => snake.grow(),
                _ => (),
            }
        }
        let positions = generate_snake_positions(&snake);
        if check_collisions(&snake, (&width, &height)) {
            println!("Died");
            exit(0);
        }

        // Main game drawing call
        window.draw_2d(&event, |ctx, g2d, _| {
            let mut grad: Vec<f64> = Vec::new();
            for i in 0..positions.len() {
                grad.push(1_f64 / positions.len() as f64 * i as f64);
            }
            clear([1.0; 4], g2d);
            for (i, rec) in positions.iter().enumerate() {
                let (x, y) = rec.to_owned();
                let (x, y) = (x as f64, y as f64);
                rectangle(
                    [0.0, grad[i] as f32, 0.0, 1.0],
                    [x, y, 10.0, 10.0],
                    ctx.transform,
                    g2d,
                );
            }
        });
    }
}
