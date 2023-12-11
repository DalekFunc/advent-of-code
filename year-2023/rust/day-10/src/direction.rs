#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    pub const HORIZONTAL: [Direction; 2] = [Direction::Left, Direction::Right];
    pub const VERTICAL: [Direction; 2] = [Direction::Up, Direction::Down];

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}
