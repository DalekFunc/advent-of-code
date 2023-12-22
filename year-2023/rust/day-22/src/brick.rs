use std::{cmp::Ordering, path::Iter};

use crate::coordinates::Coord3D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    X,
    Y,
    Z,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Brick {
    pub start: Coord3D,
    pub end: Coord3D,
}

impl Brick {
    pub fn new(p1: Coord3D, p2: Coord3D) -> Self {
        if p2.x < p1.x || p2.y < p1.y || p2.z < p1.z {
            Brick { start: p2, end: p1 }
        } else {
            Brick { start: p1, end: p2 }
        }
    }

    pub fn length(&self) -> u32 {
        match self.orientation() {
            Orientation::X => self.end.x - self.start.x + 1,
            Orientation::Y => self.end.y - self.start.y + 1,
            Orientation::Z => self.end.z - self.start.z + 1,
            Orientation::None => 1,
        }
    }

    pub fn orientation(&self) -> Orientation {
        if self.start.x != self.end.x {
            Orientation::X
        } else if self.start.y != self.end.y {
            Orientation::Y
        } else if self.start.z != self.end.z {
            Orientation::Z
        } else {
            Orientation::None
        }
    }

    pub fn blocks(&self) -> Vec<Coord3D> {
        match self.orientation() {
            Orientation::X => (self.start.x..=self.end.x)
                .map(|x| Coord3D { x, ..self.start })
                .collect(),
            Orientation::Y => (self.start.y..=self.end.y)
                .map(|y| Coord3D { y, ..self.start })
                .collect(),
            Orientation::Z => (self.start.z..=self.end.z)
                .map(|z| Coord3D { z, ..self.start })
                .collect(),
            Orientation::None => vec![self.start],
        }
    }
}

pub fn z_order(lhs: &Brick, rhs: &Brick) -> Ordering {
    let lhs_z = std::cmp::min(lhs.start.z, lhs.end.z);
    let rhs_z = std::cmp::min(rhs.start.z, rhs.end.z);

    lhs_z.cmp(&rhs_z)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::parser::block;

    use super::*;

    #[rstest]
    #[case(Ordering::Less, "1,0,1~1,2,1", "0,0,2~2,0,2")]
    #[case(Ordering::Equal, "1,0,1~1,2,1", "0,0,1~2,0,1")]
    #[case(Ordering::Equal, "1,1,1~1,1,9", "0,0,1~2,0,1")]
    #[case(Ordering::Greater, "1,1,6~1,1,3", "0,0,1~2,0,1")]
    fn test_order_by_z(#[case] expected: Ordering, #[case] lhs: &str, #[case] rhs: &str) {
        let (_, lhs) = block(lhs).unwrap();
        let (_, rhs) = block(rhs).unwrap();

        assert_eq!(expected, z_order(&lhs, &rhs));
    }
}
