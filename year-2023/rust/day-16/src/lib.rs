#![allow(unused)]
#![allow(dead_code)]

use anyhow::{anyhow, Result};
use nom::{bytes::complete::is_a, character::complete::newline, multi::separated_list1, IResult};
use types::{goto_checked, Direction, Map, Ray};

mod types;

pub fn part1(input: &[u8]) -> Result<u64> {
    let (_, map) = parse_map(input).expect("parse ok");

    Ok(simulate(
        Ray {
            pos: (0, 0),
            dir: Direction::Right,
        },
        &map,
    ))
}

pub fn part2(input: &[u8]) -> Result<u64> {
    let (_, map) = parse_map(input).expect("parse ok");
    let num_row = map.len();
    let num_col = map[0].len();

    let mut initial_rays = vec![];
    initial_rays.append(
        &mut (0..num_col)
            .map(|col| Ray {
                pos: (0, col),
                dir: Direction::Down,
            })
            .collect::<Vec<Ray>>(),
    );
    initial_rays.append(
        &mut (0..num_col)
            .map(|col| Ray {
                pos: (num_row - 1, col),
                dir: Direction::Up,
            })
            .collect::<Vec<Ray>>(),
    );
    initial_rays.append(
        &mut (0..num_row)
            .map(|row| Ray {
                pos: (row, 0),
                dir: Direction::Right,
            })
            .collect::<Vec<Ray>>(),
    );
    initial_rays.append(
        &mut (0..num_row)
            .map(|row| Ray {
                pos: (row, num_col - 1),
                dir: Direction::Left,
            })
            .collect::<Vec<Ray>>(),
    );

    Ok(initial_rays
        .into_iter()
        .map(|ray| simulate(ray, &map))
        .max()
        .expect("max should exist"))
}

fn parse_map(input: &[u8]) -> IResult<&[u8], Map> {
    separated_list1(newline, is_a(".|-/\\"))(input)
}

fn simulate(ray: Ray, map: &Vec<&[u8]>) -> u64 {
    let num_row = map.len();
    let num_col = map[0].len();

    let mut visits = vec![vec![0; num_col]; num_row];
    let mut rays = vec![ray];

    while let Some(mut ray) = rays.pop() {
        loop {
            visits[ray.pos.0][ray.pos.1] += 1;

            match map[ray.pos.0][ray.pos.1] {
                // Empty tile
                b'.' => {
                    // ray should continue its path, unless it went out of the map
                    if let Some(pos) = goto_checked(ray.pos, ray.dir, num_row, num_col) {
                        ray.pos = pos;
                    } else {
                        break;
                    }
                }
                // Mirror, reflect 90 degree up or down
                b'/' => {
                    let reflected_dir = match ray.dir {
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Right => Direction::Up,
                    };

                    if let Some(pos) = goto_checked(ray.pos, reflected_dir, num_row, num_col) {
                        ray.pos = pos;
                        ray.dir = reflected_dir;
                    } else {
                        break;
                    }
                }
                b'\\' => {
                    let reflected_dir = match ray.dir {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Down,
                    };

                    if let Some(pos) = goto_checked(ray.pos, reflected_dir, num_row, num_col) {
                        ray.pos = pos;
                        ray.dir = reflected_dir;
                    } else {
                        break;
                    }
                }
                // Splitter
                b'|' => {
                    if visits[ray.pos.0][ray.pos.1] > 1 {
                        break;
                    }

                    match ray.dir {
                        Direction::Up | Direction::Down => {
                            if let Some(pos) = goto_checked(ray.pos, ray.dir, num_row, num_col) {
                                ray.pos = pos;
                            } else {
                                break;
                            }
                        }
                        Direction::Left | Direction::Right => {
                            // we push down ray to rays for later processing in down direction
                            if let Some(pos) =
                                goto_checked(ray.pos, Direction::Down, num_row, num_col)
                            {
                                rays.push(Ray {
                                    pos,
                                    dir: Direction::Down,
                                });
                            }

                            // we keep simulating the ray in up direction
                            if let Some(pos) =
                                goto_checked(ray.pos, Direction::Up, num_row, num_col)
                            {
                                ray.pos = pos;
                                ray.dir = Direction::Up;
                            } else {
                                break;
                            }
                        }
                    }
                }
                b'-' => {
                    if visits[ray.pos.0][ray.pos.1] > 1 {
                        break;
                    }

                    match ray.dir {
                        Direction::Left | Direction::Right => {
                            if let Some(pos) = goto_checked(ray.pos, ray.dir, num_row, num_col) {
                                ray.pos = pos;
                            } else {
                                break;
                            }
                        }
                        Direction::Up | Direction::Down => {
                            // we push left ray to rays for later processing in down direction
                            if let Some(pos) =
                                goto_checked(ray.pos, Direction::Left, num_row, num_col)
                            {
                                rays.push(Ray {
                                    pos,
                                    dir: Direction::Left,
                                });
                            }

                            // we keep simulating the ray in right direction
                            if let Some(pos) =
                                goto_checked(ray.pos, Direction::Right, num_row, num_col)
                            {
                                ray.pos = pos;
                                ray.dir = Direction::Right;
                            } else {
                                break;
                            }
                        }
                    }
                }
                _ => unreachable!("invalid character"),
            }
        }
    }

    visits
        .iter()
        .map(|row| row.iter().filter(|&&elem| elem > 0).count())
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}
