#[derive(Clone, Debug)]
struct SnakeSegment {}

impl SnakeSegment {
    pub fn new() -> Self {
        SnakeSegment {}
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Snake {
    segments: Vec<SnakeSegment>,
    pub directions: Vec<Direction>,
    pub head_position: (i32, i32),
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            segments: vec![SnakeSegment::new()],
            directions: Vec::new(),
            head_position: (5, 5),
        }
    }

    pub fn step(&mut self, dir: Direction) {
        let step = crate::STEP as i32;
        let (x, y) = self.head_position;
        self.head_position = match dir {
            Direction::Up => (x, y - step),
            Direction::Down => (x, y + step),
            Direction::Left => (x - step, y),
            Direction::Right => (x + step, y),
        };
        self.directions.insert(0, dir);
        while self.directions.len() > self.segments.len() {
            self.directions.pop();
        }
    }

    pub fn grow(&mut self) {
        self.segments.push(SnakeSegment::new());
    }
}
