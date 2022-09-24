use std::process::exit;

use super::assets::snake::{Direction, Snake};

pub fn generate_snake_positions(snake: &Snake) -> Vec<(i32, i32)> {
    let step = crate::STEP as i32;
    let (start_x, start_y) = snake.head_position;
    let mut positions: Vec<(i32, i32)> = Vec::new();

    let (x, y) = (start_x, start_y);
    positions.push((x, y));

    for (_, dir) in snake.directions.iter().enumerate() {
        let (x, y) = (
            positions
                .last()
                .unwrap_or_else(|| {
                    println!("Womething went wrong!");
                    exit(-1)
                })
                .0,
            positions
                .last()
                .unwrap_or_else(|| {
                    println!("Womething went wrong!");
                    exit(-1)
                })
                .1,
        );
        let (x, y) = match dir {
            Direction::Up => (x, y + step),
            Direction::Down => (x, y - step),
            Direction::Left => (x + step, y),
            Direction::Right => (x - step, y),
        };

        positions.push((x, y));
    }

    positions
}

pub fn check_collisions(snake: &Snake) -> bool {
    // lil bit buggy. fucking deal with it

    let mut positions = generate_snake_positions(snake);

    let head = positions[0];
    positions.remove(0);
    for i in positions {
        if head == i {
            return true;
        }
    }
    false
}
