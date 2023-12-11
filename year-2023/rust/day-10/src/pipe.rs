use crate::direction::Direction;

pub const NEWLINE: u8 = b'\n';
pub const S: u8 = b'S';
pub const F: u8 = b'F';
pub const L: u8 = b'L';
pub const C7: u8 = b'7';
pub const J: u8 = b'J';
pub const DASH: u8 = b'-';
pub const PIPE: u8 = b'|';
pub const DOT: u8 = b'.';

pub type PipeType = u8;

pub fn can_connect_to(pos_type: PipeType) -> &'static [Direction] {
    match pos_type {
        S => &Direction::ALL,
        DOT => &[],
        PIPE => &Direction::VERTICAL,
        DASH => &Direction::HORIZONTAL,
        F => &[Direction::Right, Direction::Down],
        L => &[Direction::Up, Direction::Right],
        C7 => &[Direction::Left, Direction::Down],
        J => &[Direction::Up, Direction::Left],
        _ => panic!("unknown pipe type"),
    }
}
pub fn connections(dir: Direction) -> &'static [PipeType] {
    match dir {
        Direction::Up => &[PIPE, C7, F, S],
        Direction::Left => &[DASH, F, L, S],
        Direction::Down => &[L, PIPE, J, S],
        Direction::Right => &[DASH, C7, J, S],
    }
}

pub fn connected(pos_type: PipeType, nbr_type: PipeType, dir: Direction) -> bool {
    can_connect_to(pos_type).contains(&dir) && connections(dir).contains(&nbr_type)
}
