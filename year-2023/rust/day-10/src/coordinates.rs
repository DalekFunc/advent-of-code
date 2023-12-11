use crate::direction::Direction;

pub type Coord = (usize, usize);

pub fn neighbour_coord_in_direction(
    dir: Direction,
    pos: (usize, usize),
    x_bound: usize,
    y_bound: usize,
) -> Option<(usize, usize)> {
    let diff = match dir {
        Direction::Up => (-1, 0),
        Direction::Left => (0, -1),
        Direction::Down => (1, 0),
        Direction::Right => (0, 1),
    };
    let x: i64 = pos.0 as i64 + diff.0;
    let y: i64 = pos.1 as i64 + diff.1;

    if x < 0 || y < 0 || x >= x_bound as i64 || y >= y_bound as i64 {
        None
    } else {
        Some((x as usize, y as usize))
    }
}
