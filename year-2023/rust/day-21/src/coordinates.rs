#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn neighbours(&self, row_bound: usize, col_bound: usize) -> Vec<Coord> {
        let up = (self.y != 0).then(|| (self.y - 1, self.x));
        let down = (self.y != row_bound - 1).then(|| (self.y + 1, self.x));
        let left = (self.x != 0).then(|| (self.y, self.x - 1));
        let right = (self.x != col_bound - 1).then(|| (self.y, self.x + 1));

        [up, down, left, right]
            .into_iter()
            .map(|maybe_coord| maybe_coord.and_then(|coord| Some(Coord::from(coord))))
            .flatten()
            .collect()
    }
}

impl From<(usize, usize)> for Coord {
    fn from((y, x): (usize, usize)) -> Self {
        Self { x, y }
    }
}

//
// #[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
// pub struct Path {
//     steps: [Option<Coord>; 3],
// }

// impl Path {
//     pub fn empty() -> Self {
//         Self::default()
//     }

//     pub fn push(&mut self, value: Coord) {
//         self.steps[0] = self.steps[1];
//         self.steps[1] = self.steps[2];
//         self.steps[2] = Some(value);
//     }
// }

// impl AsRef<[Coord]> for Path {
//     fn as_ref(&self) -> &[Coord] {
//         &self.steps
//     }
// }
