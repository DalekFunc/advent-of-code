pub type Map<'a> = Vec<&'a [u8]>;

pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Coord,
    pub dir: Direction,
}

pub fn goto(pos: (isize, isize), dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

pub fn goto_checked(
    pos: Coord,
    dir: Direction,
    row_bound: usize,
    col_bound: usize,
) -> Option<(usize, usize)> {
    let (row, col) = goto((pos.0 as isize, pos.1 as isize), dir);

    let Ok(row) = usize::try_from(row) else {
        return None;
    };
    let Ok(col) = usize::try_from(col) else {
        return None;
    };

    if row >= row_bound || col >= col_bound {
        return None;
    };

    Some((row, col))
}
