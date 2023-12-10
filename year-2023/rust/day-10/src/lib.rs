use core::{num, panic};

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn part1(input: &[u8]) -> Result<u64> {
    let grid: Vec<&[u8]> = input.split(|b| *b == NEWLINE).collect();
    let row_bound = grid.len();
    let col_bound = grid[0].len();

    let start_coord = find_start(&grid);

    let mut cur = start_coord;
    let mut steps = 0;
    let mut from = None;
    loop {
        steps += 1;
        let (new_cur, new_from) = walk(&grid, cur, from, row_bound, col_bound);
        cur = new_cur;
        from = Some(new_from);
        if cur == start_coord {
            break;
        }
    }

    Ok(steps / 2)
}

fn is_zero_or_odd(num: usize) -> bool {
    num % 2 == 1
}

pub fn part2(input: &[u8]) -> Result<u64> {
    let grid: Vec<&[u8]> = input.split(|b| *b == NEWLINE).collect();
    let row_bound = grid.len();
    let col_bound = grid[0].len();

    let start_coord = find_start(&grid);

    let mut cur = start_coord;
    let mut from = None;
    let mut loop_coords = vec![];

    loop {
        loop_coords.push(cur);
        let (new_cur, new_from) = walk(&grid, cur, from, row_bound, col_bound);
        cur = new_cur;
        from = Some(new_from);
        if cur == start_coord {
            break;
        }
    }

    // let grid: Vec<&[u8]> = include_bytes!("../test-4-2.txt")
    //     .split(|b| *b == NEWLINE)
    //     .collect();
    let grid: Vec<&[u8]> = include_bytes!("../input-2.txt")
        .split(|b| *b == NEWLINE)
        .collect();

    // find all cells in grid which has odd intersections with loop cells in any direction;
    let mut count = 0;
    for row in 0..row_bound {
        for col in 0..col_bound {
            // not on loop
            // if row != 4 || col != 7 {
            //     continue;
            // }

            if loop_coords.contains(&(row, col)) {
                continue;
            }

            if (0..row).map(|row| grid[row][col]).all_equal() {
                continue;
            };
            if (row + 1..row_bound).map(|row| grid[row][col]).all_equal() {
                continue;
            };
            if (0..col).map(|col| grid[row][col]).all_equal() {
                continue;
            };
            if (col + 1..col_bound).map(|col| grid[row][col]).all_equal() {
                continue;
            };

            println!("{}, {}", row, col);

            // top
            // filter and count all same col, diff row < self.row
            // if odd continue, +1
            let mut num_of_odd_dir = true;

            // we track F and then L for up
            //
            if !is_zero_or_odd(
                (0..row)
                    .filter_map(|row| {
                        if loop_coords.contains(&(row, col)) {
                            Some((row, col))
                        } else {
                            None
                        }
                    })
                    // track FJ or 7L
                    .scan(None, |state, new_coord @ (row, col)| match state {
                        Some(F) => {
                            if grid[row][col] == J {
                                *state = None;
                                Some(true)
                            } else if grid[row][col] == L {
                                *state = None;
                                Some(false)
                            } else {
                                Some(false)
                            }
                        }
                        Some(C7) => {
                            if grid[row][col] == L {
                                *state = None;
                                Some(true)
                            } else if grid[row][col] == J {
                                *state = None;
                                Some(false)
                            } else {
                                Some(false)
                            }
                        }
                        Some(_) => {
                            panic!("!!")
                        }
                        None => match grid[row][col] {
                            F => {
                                *state = Some(F);
                                Some(false)
                            }
                            J => {
                                panic!("J")
                            }
                            C7 => {
                                *state = Some(C7);
                                Some(false)
                            }
                            L => {
                                panic!("L")
                            }
                            PIPE => {
                                panic!("|")
                            }
                            DASH => Some(true),
                            _ => {
                                panic!("imp")
                            }
                        },
                    })
                    .filter(|v| *v)
                    .count(),
            ) {
                // if row == 6 {
                // println!("up");
                // dbg!("!");
                // }
                num_of_odd_dir = false;
            }

            // down
            if row + 1 != row_bound {
                if !is_zero_or_odd(
                    (row + 1..row_bound)
                        .filter_map(|row| {
                            if loop_coords.contains(&(row, col)) {
                                Some((row, col))
                            } else {
                                None
                            }
                        })
                        // .map(|v| dbg!(v))
                        .scan(None, |state, new_coord @ (row, col)| match state {
                            Some(F) => {
                                if grid[row][col] == J {
                                    *state = None;
                                    Some(true)
                                } else if grid[row][col] == L {
                                    *state = None;
                                    Some(false)
                                } else {
                                    Some(false)
                                }
                            }
                            Some(C7) => {
                                if grid[row][col] == L {
                                    *state = None;
                                    Some(true)
                                } else if grid[row][col] == J {
                                    *state = None;
                                    Some(false)
                                } else {
                                    Some(false)
                                }
                            }
                            Some(_) => {
                                panic!("!!")
                            }
                            None => match grid[row][col] {
                                F => {
                                    *state = Some(F);
                                    Some(false)
                                }
                                J => {
                                    panic!("J")
                                }
                                C7 => {
                                    *state = Some(C7);
                                    Some(false)
                                }
                                L => {
                                    panic!("L")
                                }
                                PIPE => {
                                    panic!("|")
                                }
                                DASH => Some(true),
                                _ => {
                                    panic!("imp")
                                }
                            },
                        })
                        .filter(|v| *v)
                        .count(),
                ) {
                    // if row == 6 {
                    // dbg!("!");
                    // println!("down");
                    // }
                    num_of_odd_dir = false;
                }
            }

            // left
            if !is_zero_or_odd(
                (0..col)
                    .filter_map(|col| {
                        if loop_coords.contains(&(row, col)) {
                            Some((row, col))
                        } else {
                            None
                        }
                    })
                    .scan(None, |state, new_coord @ (row, col)| match state {
                        Some(F) => {
                            if grid[row][col] == J {
                                *state = None;
                                Some(true)
                            } else if grid[row][col] == C7 {
                                *state = None;
                                Some(false)
                            } else {
                                Some(false)
                            }
                        }
                        Some(L) => {
                            if grid[row][col] == C7 {
                                *state = None;
                                Some(true)
                            } else if grid[row][col] == J {
                                *state = None;
                                Some(false)
                            } else {
                                Some(false)
                            }
                        }
                        Some(_) => {
                            panic!("!!")
                        }
                        None => match grid[row][col] {
                            F => {
                                *state = Some(F);
                                Some(false)
                            }
                            J => {
                                panic!("J")
                            }
                            C7 => {
                                panic!("C7")
                            }
                            L => {
                                *state = Some(L);
                                Some(false)
                            }
                            PIPE => Some(true),
                            DASH => {
                                panic!("imp")
                            }
                            _ => {
                                panic!("imp")
                            }
                        },
                    })
                    .filter(|v| *v)
                    .count(),
            ) {
                // if row == 6 {
                // println!("left");
                // dbg!("!");
                // }
                num_of_odd_dir = false;
            }

            // right
            if col + 1 != col_bound {
                if !is_zero_or_odd(
                    (col + 1..col_bound)
                        .filter_map(|col| {
                            if loop_coords.contains(&(row, col)) {
                                Some((row, col))
                            } else {
                                None
                            }
                        })
                        .scan(None, |state, new_coord @ (row, col)| match state {
                            Some(F) => {
                                if grid[row][col] == J {
                                    *state = None;
                                    Some(true)
                                } else if grid[row][col] == C7 {
                                    *state = None;
                                    Some(false)
                                } else {
                                    Some(false)
                                }
                            }
                            Some(L) => {
                                if grid[row][col] == C7 {
                                    *state = None;
                                    Some(true)
                                } else if grid[row][col] == J {
                                    *state = None;
                                    Some(false)
                                } else {
                                    Some(false)
                                }
                            }
                            Some(_) => {
                                panic!("!!")
                            }
                            None => match grid[row][col] {
                                F => {
                                    *state = Some(F);
                                    Some(false)
                                }
                                J => {
                                    panic!("J")
                                }
                                C7 => {
                                    panic!("C7")
                                }
                                L => {
                                    *state = Some(L);
                                    Some(false)
                                }
                                PIPE => Some(true),
                                DASH => panic!("|"),
                                _ => {
                                    panic!("imp")
                                }
                            },
                        })
                        .filter(|v| *v)
                        .count(),
                ) {
                    // if row == 6 {
                    // println!("right");
                    // dbg!("!");
                    // }
                    num_of_odd_dir = false;
                }
            }

            if num_of_odd_dir {
                dbg!(num_of_odd_dir);
                println!("{}, {}", row, col);

                count += 1;
            }
        }
    }

    Ok(count)
}

const NEWLINE: u8 = b'\n';
const S: u8 = b'S';
const F: u8 = b'F';
const L: u8 = b'L';
const C7: u8 = b'7';
const J: u8 = b'J';
const DASH: u8 = b'-';
const PIPE: u8 = b'|';
const DOT: u8 = b'.';

// region:    --- Part 1

fn find_start(grid: &[&[u8]]) -> (usize, usize) {
    // find starting point
    let start_coord = grid
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find(|(_, b)| **b == S)
                .map(|(col, _)| (row, col))
        })
        .collect::<Vec<_>>();

    *start_coord.first().expect("start should exist")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

const UP_CONNECTIONS: &[u8] = &[PIPE, C7, F, S];
const LEFT_CONNECTIONS: &[u8] = &[DASH, F, L, S];
const DOWN_CONNECTIONS: &[u8] = &[L, PIPE, J, S];
const RIGHT_CONNECTIONS: &[u8] = &[DASH, C7, J, S];

impl Direction {
    const ALL: &[Direction] = &[
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    const HORIZONTAL: &[Direction] = &[Direction::Left, Direction::Right];
    const VERTICAL: &[Direction] = &[Direction::Up, Direction::Down];

    fn neighbour_coord(
        &self,
        pos: (usize, usize),
        x_bound: usize,
        y_bound: usize,
    ) -> Option<(usize, usize)> {
        let diff = match self {
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

    fn connections(&self) -> &'static [u8] {
        match self {
            Direction::Up => UP_CONNECTIONS,
            Direction::Left => LEFT_CONNECTIONS,
            Direction::Down => DOWN_CONNECTIONS,
            Direction::Right => RIGHT_CONNECTIONS,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

fn can_connect_to(pos_type: u8) -> &'static [Direction] {
    match pos_type {
        S => Direction::ALL,
        DOT => &[],
        PIPE => Direction::VERTICAL,
        DASH => Direction::HORIZONTAL,
        F => &[Direction::Right, Direction::Down],
        L => &[Direction::Up, Direction::Right],
        C7 => &[Direction::Left, Direction::Down],
        J => &[Direction::Up, Direction::Left],
        _ => panic!("unknown pipe type"),
    }
}

// FLJ7|-
fn connected(pos_type: u8, nbr_type: u8, dir: Direction) -> bool {
    can_connect_to(pos_type).contains(&dir) && dir.connections().contains(&nbr_type)
}

// walk anticlockwise, up first
fn walk(
    grid: &[&[u8]],
    pos: (usize, usize),
    from: Option<Direction>,
    x_bound: usize,
    y_bound: usize,
) -> ((usize, usize), Direction) {
    // find pipe connected anticlockwisely, starting from up
    // taking assumption S wont be in the top left border where x / y == 0.
    Direction::ALL
        .into_iter()
        .cloned()
        .filter(|dir| {
            let Some(from) = from else { return true };

            *dir != from
        })
        .find_map(|dir| {
            let Some(nbr_coord) = dir.neighbour_coord(pos, x_bound, y_bound) else {
                return None;
            };

            let pos_type = grid[pos.0][pos.1];
            let nbr_type = grid[nbr_coord.0][nbr_coord.1];

            if connected(pos_type, nbr_type, dir) {
                Some((nbr_coord, dir.opposite()))
            } else {
                None
            }
        })
        .expect("connected nbr exists")
}

// endregion: --- Part 1

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[test]
    fn test_find_start() {
        let grid: Vec<&[u8]> = include_bytes!("../test-1.txt")
            .split(|b| *b == NEWLINE)
            .collect();
        assert_eq!((1, 1), find_start(&grid));

        let grid: Vec<&[u8]> = include_bytes!("../test-2.txt")
            .split(|b| *b == NEWLINE)
            .collect();
        assert_eq!((2, 0), find_start(&grid));
    }
}
