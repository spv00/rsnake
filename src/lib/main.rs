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
                    println!("Something went wrong!");
                    exit(-1);
                })
                .0,
            positions
                .last()
                .unwrap_or_else(|| {
                    println!("Something went wrong!");
                    exit(-1);
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

pub fn check_collisions(snake: &Snake, dims: (&f64, &f64)) -> bool {
    // lil bit buggy. fucking deal with it

    let mut positions = generate_snake_positions(snake);
    let dims = (dims.0.round() as i32, dims.1.round() as i32);

    let head = positions[0];

    if 0 > head.0 || head.0 > dims.0 {
        return true;
    }
    if 0 > head.1 || head.1 > dims.1 {
        return true;
    }
    positions.remove(0);
    for i in positions {
        if head == i {
            return true;
        }
    }
    false
}
